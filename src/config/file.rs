use crate::command::git::{Git, Gitable};
use crate::command::runner::Runner;
use crate::flow::branch::Branch;

use crate::config::constants::{BRANCH_ALREADY_EXISTS_SUFFIX};

pub const FEATURE_BRANCH_NAME_KEY: &str = "FEATURE_BRANCH_NAME";
pub const HOTFIX_BRANCH_NAME_KEY: &str = "HOTFIX_BRANCH_NAME";
pub const BUGFIX_BRANCH_NAME_KEY: &str = "BUGFIX_BRANCH_NAME";
pub const RELEASE_BRANCH_NAME_KEY: &str = "RELEASE_BRANCH_NAME";
pub const DEVELOP_BRANCH_NAME_KEY: &str = "DEVELOP_BRANCH_NAME";
pub const MAIN_BRANCH_NAME_KEY: &str = "MAIN_BRANCH_NAME";

pub const CONFIG_FILE_PATH: &str = ".git/lab.conf";

pub struct Configuration<'a> {

    key: &'a str,
    value: &'a str

}
#[derive(Debug)]
pub struct FileError {
    message: String
}

impl std::error::Error for FileError {}

impl FileError {
    fn new(message: String) -> FileError {
        FileError {
            message
        }
    }
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn read_config_file () -> Result<String, Box<dyn std::error::Error>> {

    let file_string = std::fs::read_to_string(CONFIG_FILE_PATH)?;

    Ok(file_string)

}

fn parse_config_file (file_string: &str) -> Result<Vec<Configuration>, Box<dyn std::error::Error>> {

    let mut config_vec: Vec<Configuration> = Vec::new();

    for line in file_string.lines() {

        let mut config_line = line.split("=");

        let key = config_line.next().unwrap();
        let value = config_line.next().unwrap();

        config_vec.push(Configuration { key, value });

    }

    Ok(config_vec)

}

fn read_branch_name (
    message: &str, 
    default_name: &str,
    include_slash: bool,
    used_names: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {

    let mut branch_name = String::new();
    
    loop {

        println!("{}", message);

        std::io::stdin().read_line(&mut branch_name)?;
        // Removing the newline character
        branch_name.pop();

        if branch_name.is_empty() {
            branch_name = default_name.to_string();
        }

        if include_slash && !branch_name.ends_with("/") {
            branch_name = branch_name + "/";
        } else if !include_slash && branch_name.ends_with("/") {
            branch_name.pop();
        }

        if used_names.contains(&branch_name) {
            println!("Branch name already used, please choose another one.");
            branch_name = String::new();
        } else {
            break;
        }

    }

    Ok(branch_name)

}

pub fn create_config_file () -> Result<(), Box<dyn std::error::Error>> {

    let mut names: Vec<String> = Vec::new();
    let mut file_string: String;
    
    match Git::init() {
        Ok(_) => {},
        Err(e) => return Err(Box::new(FileError::new(e)))
    }

    let feature_name = read_branch_name(
        "Enter the prefix of the feature branches (feature/):", 
        "feature/",
        true,
        &names
    )?;

    file_string = format!("{}={}\n", FEATURE_BRANCH_NAME_KEY, feature_name);
    names.push(feature_name);
    
    let bugfix_name = read_branch_name(
        "Enter the prefix of the bugfix branches (bugfix/):",
        "bugfix/",
        true,
        &names
    )?;

    file_string = file_string + &format!("{}={}\n", BUGFIX_BRANCH_NAME_KEY, bugfix_name);
    names.push(bugfix_name);
    
    let hotfix_name = read_branch_name(
        "Enter the prefix of the hotfix branches (hotfix/):",
        "hotfix/",
        true,
        &names
    )?;

    file_string = file_string + &format!("{}={}\n", HOTFIX_BRANCH_NAME_KEY, hotfix_name);
    names.push(hotfix_name);
    
    let release_name = read_branch_name(
        "Enter the prefix of the release branches (release/):",
        "release/",
        true,
        &names
    )?;

    file_string = file_string + &format!("{}={}\n", RELEASE_BRANCH_NAME_KEY, release_name);
    names.push(release_name);
    
    let develop_name = read_branch_name(
        "Enter the name of the develop branch (develop):",
        "develop",
        false,
        &names
    )?;

    file_string = file_string + &format!("{}={}\n", DEVELOP_BRANCH_NAME_KEY, develop_name);
    
    let develop_name_to_create = develop_name.clone();
    let develop_name_to_push = develop_name.clone();
    let default_branch_name = develop_name.clone();  
    names.push(develop_name);
    
    let main_name = read_branch_name(
        "Enter the name of the main branch (main):",
        "main",
        false,
        &names
    )?;

    // TODO: Do we need to checkout?
    match Git::checkout(&Branch::Main(main_name.clone()), true) {
        Ok(_) => {},
        Err(e) => {
            println!("[ERROR] {}", e);
        }
    }

    match Runner::run(&format!("git push origin {}", main_name)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    match Git::add(vec![".".to_string()]) {
        Ok(_) => {
            match Git::commit("Initial commit".to_string()) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("{}", err);
                    return Err(Box::new(FileError::new(err)));
                }
            }
        },
        Err(err) => {
            eprintln!("{}", err);
            return Err(Box::new(FileError::new(err)));
        }
    }

    match Git::create_branch(Branch::Develop(develop_name_to_create)) {
        Ok(_) => {},
        Err(e) => {
            
            let error_message: String = format!("{}", e);

            eprintln!("{}", error_message);

            if ! error_message.contains(BRANCH_ALREADY_EXISTS_SUFFIX) {
                return Err(Box::new(FileError::new("Error creating develop branch".to_string())));
            }

            println!("Branch already exists.");

            //  Ignoring error if the branch already exists.
            {}

        }
    }

    // TODO: Check if the origin name may vary and configure it in the config file
    match Runner::run(&format!("git push origin {}", develop_name_to_push)) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("{}", e);
        }
    }

    file_string = file_string + &format!("{}={}\n", MAIN_BRANCH_NAME_KEY, main_name);

    std::fs::write(CONFIG_FILE_PATH, file_string)?;

    // FIXME: Do we need to checkout?
    match Git::checkout(&Branch::Develop(default_branch_name), false) {
        Ok(_) => {},
        Err(e) => {
            println!("[ERROR] {}", e);
        }
    }
    
    Ok(())

}

pub fn get_config_value (key: &str) -> Result<String, Box<dyn std::error::Error>> {

    let file_string = read_config_file()?;

    let config_vec = parse_config_file(&file_string)?;

    let config_value = config_vec.iter().find(|config| config.key == key);

    match config_value {

        Some(config) => Ok(config.value.to_string()),

        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found")))

    }

}

pub fn contains_config_file () -> bool {

    std::path::Path::new(CONFIG_FILE_PATH).exists()

}
