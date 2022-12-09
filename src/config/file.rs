use crate::command::gitv2::{GitV2};

use crate::config::constants::{BRANCH_ALREADY_EXISTS_SUFFIX};

pub const FEATURE_BRANCH_NAME_KEY: &str = "FEATURE_BRANCH_NAME";
pub const HOTFIX_BRANCH_NAME_KEY: &str = "HOTFIX_BRANCH_NAME";
pub const BUGFIX_BRANCH_NAME_KEY: &str = "BUGFIX_BRANCH_NAME";
pub const RELEASE_BRANCH_NAME_KEY: &str = "RELEASE_BRANCH_NAME";
pub const DEVELOP_BRANCH_NAME_KEY: &str = "DEVELOP_BRANCH_NAME";
pub const MAIN_BRANCH_NAME_KEY: &str = "MAIN_BRANCH_NAME";

pub const CONFIG_FILE_PATH: &str = ".git/lab.conf";

///
/// The Configuration struct, containing the key 
/// and value of a configuration.
/// 
pub struct Configuration<'a> {

    key: &'a str,
    value: &'a str

}

///
/// The FileError struct, containing the error message.
/// 
#[derive(Debug)]
pub struct FileError {
    message: String
}

impl std::error::Error for FileError {}

///
/// The implementation of FileError.
/// 
impl FileError {
    fn new(message: String) -> FileError {
        FileError {
            message
        }
    }
}

///
/// Allowing the FileError to be displayed.
/// 
impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

///
/// Checks if the given error message contains the suffix that indicates 
/// that the branch already exists.
/// 
fn does_branch_already_exists (error_message: &str) -> bool {
    error_message.contains(BRANCH_ALREADY_EXISTS_SUFFIX)
}

///
/// Reads the configuration file as a string.
/// 
fn read_config_file () -> Result<String, Box<dyn std::error::Error>> {

    let file_string = std::fs::read_to_string(CONFIG_FILE_PATH)?;

    Ok(file_string)

}

///
/// Parses the configuration file and returns a vector of Configuration objects.
/// 
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

///
/// Reads a line from the standard input, checks if the branch name is already used and, if not,
/// returns the branch name. It may include a slash (or not) at the end of the branch name.
/// 
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

///
/// Creates the lab-flow configuration file for the project.
/// 
pub fn create_config_file () -> Result<(), Box<dyn std::error::Error>> {

    let mut names: Vec<String> = Vec::new();
    let mut file_string: String;
    
    match GitV2::init() {
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

    file_string = file_string + &format!("{}={}\n", DEVELOP_BRANCH_NAME_KEY, &develop_name);

    // Clone is needed. We need to keep using develop_name
    names.push(develop_name.clone());
    
    let main_name = read_branch_name(
        "Enter the name of the main branch (main):",
        "main",
        false,
        &names
    )?;

    match GitV2::checkout(Option::None, &main_name, true) {
        Ok(_) => {},
        Err(e) => {

            if does_branch_already_exists(&e) {
                match GitV2::checkout(Option::None, &main_name, false) {
                    Ok(_) => {},
                    Err(e) => {
                        let error_message = format!("Error checking out the main branch: {}", e);
                        println!("{}", &error_message);
                        return Err(Box::new(FileError::new(error_message)));
                    }
                }
            }

        }
    }

    match GitV2::push(&main_name) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error pushing to the remote repository: {}", e);
        }
    }

    match GitV2::add(vec![".".to_string()]) {
        Ok(_) => {
            match GitV2::commit("Initial commit".to_string()) {
                Ok(_) => {},
                Err(err) => {
                    eprintln!("Failed to create an initial commit. It may be there is nothing to commit 🤷‍♂️ {}", err);
                }
            }
        },
        Err(err) => {
            eprintln!("Error adding files for the initial commit: {}", err);
            return Err(Box::new(FileError::new(err)));
        }
    }

    match GitV2::branch(Option::None, &develop_name) {
        Ok(_) => {},
        Err(e) => {

            let error_message: String = format!("Error creating the \"develop\" branch: {}", e);

            eprintln!("{}", &error_message);

            if ! does_branch_already_exists(&error_message) {
                return Err(Box::new(FileError::new(format!("{}", &error_message))));
            }

            //  Ignoring error if the branch already exists.
            {}

        }
    }

    match GitV2::push(&develop_name) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error pushing to the remote repository: {}", e);
        }
    }

    file_string = file_string + &format!("{}={}\n", MAIN_BRANCH_NAME_KEY, main_name);

    match std::fs::write(CONFIG_FILE_PATH, file_string) {
        Ok(_) => {},
        Err(e) => {
            return Err(Box::new(FileError::new(format!("Error writing the configuration file: {}", e))));
        }
    }

    match GitV2::checkout(Option::None, &develop_name, false) {
        Ok(_) => {},
        Err(e) => {
            println!("Couldn't checkout the \"develop\" branch. {}", e);
        }
    }

    Ok(())

}

///
/// Reads the configuration file and returns the configuration identified by the key, if found.
/// Otherwise returns an error.
/// 
pub fn get_config_value (key: &str) -> Result<String, Box<dyn std::error::Error>> {

    let file_string = read_config_file()?;

    let config_vec = parse_config_file(&file_string)?;

    let config_value = config_vec.iter().find(|config| config.key == key);

    match config_value {

        Some(config) => Ok(config.value.to_string()),

        None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found")))

    }

}

///
/// Checks if the configuration file exists.
/// 
pub fn contains_config_file () -> bool {

    std::path::Path::new(CONFIG_FILE_PATH).exists()

}
