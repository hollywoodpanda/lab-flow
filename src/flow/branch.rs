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

use crate::command::gitv2::GitV2;

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

    pub fn source (&self) -> Result<Vec<Branch>, String> {
        
        let branch_name: String = String::from(self.name());
        let branch_prefix = match self.prefix() {
            Some(prefix) => prefix,
            None => "".to_string(),
        };
         
        // 1. Get the commits only on the branch
        let branch_only_commits = match GitV2::exclusive_commits(&branch_prefix, &branch_name) {
            Ok(commits) => commits,
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        // 2. Get the first 100 commits in the branch
        let branch_commits = match GitV2::all_commits(&branch_prefix, &branch_name, 100) {
            Ok(commits) => commits,
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        // 3. Get the first of all commits that differs from commits only in the current branch.
        let first_commit_not_in_branch: String = match branch_commits
            .iter()
            .filter(|commit| !branch_only_commits.contains(commit))
            .collect::<Vec<&String>>()
            .first() {
                Some(commit) => commit.to_string(),
                None => String::from(&branch_commits[branch_commits.len() - 1]),
            };
            
        // 4. Get all the commits on branch, stop at the first
        // one not in the branch_commits. This commit contains
        // the source branches!
        let source_branches = match GitV2::source_branches(
            &first_commit_not_in_branch, 
            &branch_prefix, 
            &branch_name
        ) {
            Ok(branches) => {
                branches
                    .iter()
                    .map(|branch_name| Branch::from(branch_name))
                    .collect()
            },
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        Ok(source_branches)

    }

}



impl Display for Branch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
