use std::process::Output;

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

///
/// The key and value of a configuration.
/// 
#[derive(Debug)]
#[deprecated]
pub struct Configuration<'a> {
    pub key: &'a str,
    pub value: &'a str,
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

    let valid_branch_name_regex: Regex = match Regex::new(r"^[A-Za-z]{1,}[A_Za-z0-9_]*[A-Za-z0-9]{1,}$") {

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

        // 5. If the user entered a branch name that is already taken, we keep asking for a new one.
        if already_used_names.contains(&branch_name) {
            println!("The branch name '{}' is already taken. Please enter a new one.", branch_name);
            branch_name = String::new();
            continue;
        }

        // 6. Validating if the given name is acceptable.
        if ! is_given_branch_name_valid(&branch_name) {
            println!("The branch name '{}' is not valid. Please enter a new one.", branch_name);
            branch_name = String::new();
            continue;
        }


        // 7. If the user entered a branch name that is already 
        // taken, we keep asking for a new one.
        if add_suffix_slash && !branch_name.ends_with("/") {
            branch_name = branch_name + "/";
        }

        // 8. If we got here it means we have a valid 
        // name for the branch. We can escape.
        break;

    }

    Ok(branch_name)
    
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
                    Err(e) => return Err(Box::new(InitError::new(e)))
                }

            }

        }
    }
    
    return Err(Box::new(InitError::new(String::from("Function not finished yet!"))))

}

