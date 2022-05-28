use std::fmt::Display;

use crate::config::file::{
    get_config_value, 
    DEVELOP_BRANCH_NAME_KEY, 
    MAIN_BRANCH_NAME_KEY, 
    RELEASE_BRANCH_NAME_KEY, 
    FEATURE_BRANCH_NAME_KEY,
    HOTFIX_BRANCH_NAME_KEY,
    BUGFIX_BRANCH_NAME_KEY,
};

#[derive(Debug, PartialEq)]
pub enum Branch {
    Feature(String),
    Hotfix(String),
    Bugfix(String),
    Release(String),
    Develop(String),
    Main(String),
}

pub trait Sourceable {
    fn source (&self, released: bool) -> Result<Vec<Branch>, &str>;
    fn prefix (&self) -> Result<String, &str>;
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Sourceable for Branch {

    fn prefix (&self) -> Result<String, &str> {
        match self {
            Branch::Feature(name) => {
                match get_config_value(FEATURE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err("Feature branch prefix not found in configuration file")
                }
            },
            Branch::Hotfix(name) => {
                match get_config_value(HOTFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err("Hotfix branch prefix not found in configuration file")
                }
            },
            Branch::Bugfix(name) => {
                match get_config_value(BUGFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err("Bugfix branch prefix not found in configuration file")
                }
            },
            Branch::Release(name) => {
                match get_config_value(RELEASE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Ok(prefix),
                    Err(_) => Err("Release branch prefix not found in configuration file")
                }
            },
            _ => Err("No prefix for this branch")
        }
    }

    fn source (&self, released: bool) -> Result<Vec<Branch>, &str> {

        let mut branches: Vec<Branch> = Vec::new();

        let develop_name = match get_config_value(DEVELOP_BRANCH_NAME_KEY) {
            Ok(develop_name) => develop_name,
            Err(_) => return Err("Develop branch name not found")
        };

        let main_name = match get_config_value(MAIN_BRANCH_NAME_KEY) {
            Ok(main_name) => main_name,
            Err(_) => return Err("Main branch name not found")
        };

        let release_name = match get_config_value(RELEASE_BRANCH_NAME_KEY) {
            Ok(release_name) => release_name,
            Err(_) => return Err("Release branch name not found")
        };
        

        match self {

            Branch::Feature(_) => {
                branches.push(Branch::Develop(develop_name));
                let branches = branches;
                Ok(branches)
            },

            Branch::Hotfix(_) => {
                branches.push(Branch::Main(main_name));
                branches.push(Branch::Develop(develop_name));
                let branches = branches;
                Ok(branches)
            },

            Branch::Bugfix(_) => {

                if released {
                    branches.push(Branch::Release(release_name));
                }

                branches.push(Branch::Develop(develop_name));

                let branches = branches;
                Ok(branches)

            },

            Branch::Release(_) => {
                branches.push(Branch::Main(main_name));
                Ok(branches)
            },

            _ => Err("No source branch found")

        }

    }

}