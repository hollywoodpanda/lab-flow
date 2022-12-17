use regex::Regex;
use crate::config::constants::BRANCH_ALREADY_EXISTS_SUFFIX;
use crate::config::store::{Store};
use crate::command::gitv2::{GitV2};

pub const FEATURE_BRANCH_NAME_KEY: &str = "lab.flow.branch.feature";
pub const BUGFIX_BRANCH_NAME_KEY: &str = "lab.flow.branch.bugfix";
pub const HOTFIX_BRANCH_NAME_KEY: &str = "lab.flow.branch.hotfix";
pub const RELEASE_BRANCH_NAME_KEY: &str = "lab.flow.branch.release";
pub const DEVELOP_BRANCH_NAME_KEY: &str = "lab.flow.branch.develop";
pub const MAIN_BRANCH_NAME_KEY: &str = "lab.flow.branch.main";

pub enum Script {}

impl Script {

    ///
/// Checks if the repository is already initiated.
/// It checks if the feature, bugfix, hotfix, release,
/// develop and main branch names have values in the store.
/// 
pub fn is_initiated () -> bool {

    // 1. Checking if the feature branch name is stored.
    match Store::get(FEATURE_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    // 2. Checking if the bugfix branch name is stored.
    match Store::get(BUGFIX_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    // 3. Checking if the hotfix branch name is stored.
    match Store::get(HOTFIX_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    // 4. Checking if the release branch name is stored.
    match Store::get(RELEASE_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    // 5. Checking if the develop branch name is stored.
    match Store::get(DEVELOP_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    // 6. Checking if the main branch name is stored.
    match Store::get(MAIN_BRANCH_NAME_KEY) {
        Ok(_) => {},
        Err(_) => return false
    }

    return true

}

pub fn show () {

    let feature_branch_name = match Store::get(FEATURE_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    let bugfix_branch_name = match Store::get(BUGFIX_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    let hotfix_branch_name = match Store::get(HOTFIX_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    let release_branch_name = match Store::get(RELEASE_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    let develop_branch_name = match Store::get(DEVELOP_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    let main_branch_name = match Store::get(MAIN_BRANCH_NAME_KEY) {
        Ok(value) => value,
        Err(_) => String::from("[NOT FOUND]")
    };

    println!("The feature branch prefix is {}", feature_branch_name);
    println!("The bugfix branch prefix is {}", bugfix_branch_name);
    println!("The hotfix branch prefix is {}", hotfix_branch_name);
    println!("The release branch prefix is {}", release_branch_name);
    println!("The develop branch name is {}", develop_branch_name);
    println!("The main branch name is {}", main_branch_name);

}

pub fn create () -> Result<(), Box<dyn std::error::Error>> {

    let mut used_names: Vec<String> = Vec::new();
    let mut file_string: String;

    // 1. Initializing the git repository.
    match GitV2::init() {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 2. Reading the feature branch name.
    let feature_branch_name = read_branch_name(
        "Enter the prefix of the feature branches (feature/):",
        "feature/",
        true,
        &used_names
    )?;

    // 3. Adding the feature branch name to the list of used names.
    used_names.push(feature_branch_name.clone());

    // 4. Reading the bugfix branch name.
    let bugfix_branch_name = read_branch_name(
        "Enter the prefix of the bugfix branches (bugfix/):",
        "bugfix/",
        true,
        &used_names
    )?;

    // 5. Adding the bugfix branch name to the list of used names.
    used_names.push(bugfix_branch_name.clone());

    // 6. Reading the hotfix branch name.
    let hotfix_branch_name = read_branch_name(
        "Enter the prefix of the hotfix branches (hotfix/):",
        "hotfix/",
        true,
        &used_names
    )?;

    // 7. Adding the hotfix branch name to the list of used names.
    used_names.push(hotfix_branch_name.clone());

    // 8. Reading the release branch name.
    let release_branch_name = read_branch_name(
        "Enter the prefix of the release branches (release/):",
        "release/",
        true,
        &used_names
    )?;

    // 9. Adding the release branch name to the list of used names.
    used_names.push(release_branch_name.clone());

    // 10. Reading the develop branch name.
    let develop_branch_name = read_branch_name(
        "Enter the name of the develop branch (develop):",
        "develop",
        false,
        &used_names
    )?;

    // 11. Adding the develop branch name to the list of used names.
    used_names.push(develop_branch_name.clone());

    // 12. Reading the main branch name.
    let main_branch_name = read_branch_name(
        "Enter the name of the  main branch (main):",
        "main",
        false,
        &used_names
    )?;

    // 13. We don't need this vector anymore.
    used_names.clear();

    // 14. Creating and using the main branch
    match create_branch_main(&main_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    // 15. Pushing the main branch to the remote repository.
    push_branch(&main_branch_name);

    // 16. Initial commit at the main branch.
    match stage_and_commit_all_files() {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    // 17. Creating and using the develop branch.
    match create_branch_develop(&develop_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    // 18. Pushing the develop branch to the remote repository.
    push_branch(&develop_branch_name);

    // 19. Store the branch names.
    match store_branch_names(
        &feature_branch_name,
        &bugfix_branch_name,
        &hotfix_branch_name,
        &release_branch_name,
        &develop_branch_name,
        &main_branch_name
    ) {
        Ok(_) => {},
        Err(e) => return Err(e)
    }
    
    // 20. Operation successfull!
    Ok(())

}

}

#[derive(Debug)]
pub struct InitError {
    pub message: String,
}

impl InitError {
    pub fn new(message: String) -> InitError {
        InitError {
            message
        }
    }
}

impl std::error::Error for InitError {}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn is_already_exists_message (message: &str) -> bool {
    message.contains(BRANCH_ALREADY_EXISTS_SUFFIX)
}

fn is_given_branch_name_valid(branch_name: &str) -> bool {

    let valid_branch_name_regex: Regex = match Regex::new(r"^[A-Za-z]{1,}[A_Za-z0-9_]*[A-Za-z0-9]{1,}[/]{0,1}$") {

        Ok(output) => output,
        Err(_) => return false

    };

    valid_branch_name_regex.is_match(branch_name)

}

fn read_branch_name (
    message: &str,
    default_branch_name: &str,
    add_suffix_slash: bool,
    already_used_names: &Vec<String>,
) -> Result<String, Box<dyn std::error::Error>> {

    let mut branch_name = String::new();

    // We loop here so that if the user enters a branch name
    // that is already taken, we keep asking for a new one.
    loop {

        // 1. Show the given message to the user.
        println!("{}", message);

        // 2. Read the user input.
        match std::io::stdin().read_line(&mut branch_name) {
            Ok(_) => {},
            Err(e) => return Err(Box::new(e))
        }

        // 3. Remove the trailing newline character.
        branch_name = branch_name.trim().to_string();

        // 4. If the user did not enter anything, we use the default branch name.
        if branch_name.is_empty() {
            branch_name = default_branch_name.to_string();
        }

        // 5. If the user entered a branch name that is already 
        // taken, we keep asking for a new one.
        if add_suffix_slash && !branch_name.ends_with("/") {
            branch_name = branch_name + "/";
        }

        // 6. Removing the trailing slash if the user entered one
        // as we shouldn't accept it.
        if !add_suffix_slash && branch_name.ends_with("/") {
            branch_name = branch_name.trim_end_matches("/").to_string();
        }

        // 7. If the user entered a branch name that is already taken, we keep asking for a new one.
        if already_used_names.contains(&branch_name) || ! add_suffix_slash && already_used_names.contains(&format!("{}/", &branch_name)) {
            println!("The branch name '{}' is already taken. Please enter a new one.", branch_name);
            branch_name = String::new();
            continue;
        }

        // 8. Validating if the given name is acceptable.
        if ! is_given_branch_name_valid(&branch_name) {
            println!("The branch name '{}' is not valid. Please enter a new one.", branch_name);
            branch_name = String::new();
            continue;
        }

        // 9. If we got here it means we have a valid 
        // name for the branch. We can escape.
        break;

    }

    Ok(branch_name)
    
}

fn stage_and_commit_all_files () -> Result<(), Box<InitError>> {

    match GitV2::add(vec![".".to_string()]) {

        Ok(_) => {

            // We need to commit the staged files.
            match GitV2::commit(String::from("Initial commit")) {
                Ok(_) => {},
                Err(e) => {

                    println!("We could not commit the staged files. Please do it manually. Error: {}", e);

                }
            };

        },
        Err(e) => {

            let error_message = format!("We could not stage all files. Please do it manually. Error: {}", e);

            println!("{}", &error_message);

            return Err(Box::new(InitError::new(error_message)));

        }

    }

    Ok(())

}

fn push_branch (branch_name: &str) {

    match GitV2::push(branch_name) {
        Ok(_) => {},
        Err(_) => {

            println!("We could not push the branch '{}' to the remote repository. Please do it manually.", branch_name);

        }
    };
}

fn create_branch_develop (develop_branch_name: &str) -> Result<(), Box<InitError>> {

    match GitV2::branch(Option::None, develop_branch_name) {
        Ok(_) => {

            match GitV2::checkout(Option::None, develop_branch_name, false) {
                Ok(_) => {},
                Err(e) => {

                    let error_message = format!("We could not checkout to the develop branch. Error: {}", e);

                    eprintln!("{}", &error_message);

                    return Err(Box::new(InitError::new(error_message)));

                }
            }

        },
        Err(err) => {

            let error_message = format!("We could not create the develop branch. Error: {}", err);

            eprintln!("{}", &error_message);

            if ! is_already_exists_message(&err) {

                return Err(Box::new(InitError::new(error_message)));

            }

        }
    }

    Ok(())

}

fn create_branch_main (main_branch_name: &str) -> Result<(), Box<InitError>> {

    match GitV2::checkout(
        Option::None,
        &main_branch_name,
        true
    ) {
        Ok(_) => {},
        Err(e) => {

            // If the error is because the branch already 
            // exists, we just checkout to it.
            if is_already_exists_message(&e) {

                match GitV2::checkout(
                    Option::None,
                    &main_branch_name,
                    false
                ) {
                    Ok(_) => {},
                    Err(e) => {

                        println!("The main branch already exists, but we could not checkout to it. Error: {}", e);

                        return Err(Box::new(InitError::new(e)));

                    }
                }

            }

        }
    }

    Ok(())

}

fn store_branch_names (
    feature_branch_name: &str,
    bugfix_branch_name: &str,
    hotfix_branch_name: &str,
    release_branch_name: &str,
    develop_branch_name: &str,
    main_branch_name: &str
) -> Result<(), Box<InitError>> {

    // 1. Storing the feature branch "prefix".
    match Store::add(FEATURE_BRANCH_NAME_KEY, feature_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 2. Storing the bugfix branch "prefix".
    match Store::add(BUGFIX_BRANCH_NAME_KEY, bugfix_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 3. Storing the hotfix branch "prefix".
    match Store::add(HOTFIX_BRANCH_NAME_KEY, hotfix_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 4. Storing the release branch "prefix".
    match Store::add(RELEASE_BRANCH_NAME_KEY, release_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 5. Storing the develop branch name.
    match Store::add(DEVELOP_BRANCH_NAME_KEY, develop_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    // 6. Storing the main branch name.
    match Store::add(MAIN_BRANCH_NAME_KEY, main_branch_name) {
        Ok(_) => {},
        Err(e) => return Err(Box::new(InitError::new(e)))
    }

    Ok(())

}