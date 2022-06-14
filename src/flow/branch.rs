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

fn get_config_branch (prefix: Option<String>, name: &str) -> Result<Branch, String> {
    
    let develop_name = match get_config_value(DEVELOP_BRANCH_NAME_KEY) {
        Ok(develop_name) => develop_name,
        Err(_) => return Err("Develop branch name not found".to_string())
    };

    let main_name = match get_config_value(MAIN_BRANCH_NAME_KEY) {
        Ok(main_name) => main_name,
        Err(_) => return Err("Main branch name not found".to_string())
    };

    let release_prefix = match get_config_value(RELEASE_BRANCH_NAME_KEY) {
        Ok(release_prefix) => release_prefix,
        Err(_) => return Err("Release branch prefix not found".to_string())
    };

    let feature_prefix = match get_config_value(FEATURE_BRANCH_NAME_KEY) {
        Ok(feature_prefix) => feature_prefix,
        Err(_) => return Err("Feature branch prefix not found".to_string())
    };

    let hotfix_prefix = match get_config_value(HOTFIX_BRANCH_NAME_KEY) {
        Ok(hotfix_prefix) => hotfix_prefix,
        Err(_) => return Err("Hotfix branch prefix not found".to_string())
    };

    let bugfix_prefix = match get_config_value(BUGFIX_BRANCH_NAME_KEY) {
        Ok(bugfix_prefix) => bugfix_prefix,
        Err(_) => return Err("Bugfix branch prefix not found".to_string())
    };

    match prefix {
        Some(prefix) => {
            if prefix == feature_prefix {
                return Ok(Branch::Feature(name.to_string()));
            } else if prefix == hotfix_prefix {
                return Ok(Branch::Hotfix(name.to_string()));
            } else if prefix == bugfix_prefix {
                return Ok(Branch::Bugfix(name.to_string()));
            } else if prefix == release_prefix {
                return Ok(Branch::Release(name.to_string()));
            } else {
                return Err(format!("Unknown branch prefix '{}'", prefix));
            }
        },
        None => {
            if name == develop_name {
                return Ok(Branch::Develop(name.to_string()));
            } else if name == main_name {
                return Ok(Branch::Main(name.to_string()));
            } else {
                return Err(format!("Unknown branch name {}", name));
            }
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
pub enum Branch {
    Feature(String),
    Hotfix(String),
    Bugfix(String),
    Release(String),
    Develop(String),
    Main(String),
}

impl Branch {

    pub fn from (branch_full_name: &str) -> Branch {

        let prefix = match branch_full_name.split("/").next() {
            Some(prefix) => Some(format!("{}/", prefix)),
            None => None,
        };
        let name = match branch_full_name.split("/").last() {
            Some(name) => name,
            None => branch_full_name,
        };

        let prefix = match prefix {
            Some(prefix) => {
                if prefix.starts_with(name) {
                    None
                } else {
                    Some(prefix)
                }
            },
            None => None,
        };

        return get_config_branch(prefix, name).unwrap();
       
    }

    pub fn name (&self) -> &str {
        match self {
            Branch::Feature(name) => name,
            Branch::Hotfix(name) => name,
            Branch::Bugfix(name) => name,
            Branch::Release(name) => name,
            Branch::Develop(name) => name,
            Branch::Main(name) => name,
        }
    }

    pub fn prefix (&self) -> Option<String> {
        match self {
            Branch::Feature(_) => {
                match get_config_value(FEATURE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Hotfix(_) => {
                match get_config_value(HOTFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Bugfix(_) => {
                match get_config_value(BUGFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Release(_) => {
                match get_config_value(RELEASE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }

}

pub trait Sourceable {
    fn source (&self, released: bool) -> Result<Vec<Branch>, &str>;
}

impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Sourceable for Branch {

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

        let release_prefix = match get_config_value(RELEASE_BRANCH_NAME_KEY) {
            Ok(release_prefix) => release_prefix,
            Err(_) => return Err("Release branch prefix not found")
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

                // FIXME: We don't have the suffix part of the branch name
                // FIXME: Find out the higher "release/{VERSION}" branch and use its name as the suffix
                // ... if using git lab, there'll be only one release branch
                if released {
                    branches.push(Branch::Release(release_prefix));
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