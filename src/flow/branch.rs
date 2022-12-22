use std::fmt::Display;

use crate::config::constants::{
    FEATURE_BRANCH_NAME_KEY,
    BUGFIX_BRANCH_NAME_KEY,
    HOTFIX_BRANCH_NAME_KEY,
    RELEASE_BRANCH_NAME_KEY,
    DEVELOP_BRANCH_NAME_KEY,
    MAIN_BRANCH_NAME_KEY,
};

use crate::config::store::{Store};

use crate::command::gitv2::GitV2;

fn get_config_branch (prefix: Option<String>, name: &str) -> Result<Branch, String> {
    
    let develop_name:String = match Store::get(DEVELOP_BRANCH_NAME_KEY) {
        Ok(develop_name) => String::from(develop_name.trim()),
        Err(_) => return Err("Develop branch name not found".to_string())
    };

    let main_name: String = match Store::get(MAIN_BRANCH_NAME_KEY) {
        Ok(main_name) => String::from(main_name.trim()),
        Err(_) => return Err("Main branch name not found".to_string())
    };

    let release_prefix: String = match Store::get(RELEASE_BRANCH_NAME_KEY) {
        Ok(release_prefix) => String::from(release_prefix.trim()),
        Err(_) => return Err("Release branch prefix not found".to_string())
    };

    let feature_prefix: String = match Store::get(FEATURE_BRANCH_NAME_KEY) {
        Ok(feature_prefix) => String::from(feature_prefix.trim()),
        Err(_) => return Err("Feature branch prefix not found".to_string())
    };

    let hotfix_prefix: String = match Store::get(HOTFIX_BRANCH_NAME_KEY) {
        Ok(hotfix_prefix) => String::from(hotfix_prefix.trim()),
        Err(_) => return Err("Hotfix branch prefix not found".to_string())
    };

    let bugfix_prefix: String = match Store::get(BUGFIX_BRANCH_NAME_KEY) {
        Ok(bugfix_prefix) => String::from(bugfix_prefix.trim()),
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

            println!("[DEBUG] The name is \"{}\"", name);

            if name == &develop_name {
                return Ok(Branch::Develop(name.to_string()));
            } else if name == &main_name {
                return Ok(Branch::Main(name.to_string()));
            } else {
                return Err(format!("Unknown branch name {}  (not {} or {})", name, &main_name, &develop_name));
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

    // FIXME: Should return an Option<Branch>!
    // FIXME: With the Option<Branch>, we don't need the unwrap. We return None
    pub fn from (branch_full_name: &str) -> Branch {

        let name: &str = match branch_full_name.split("/").last() {
            Some(name) => name,
            None => branch_full_name,
        };

        let prefix: Option<String> = match branch_full_name.split("/").next() {
            Some(prefix) => {

                if prefix.starts_with(name) {
                    None
                } else {
                    Some(format!("{}/", prefix))
                }

            },
            None => None,
        };

        println!("[DEBUG] name: {:?}", name);

        println!("[DEBUG] prefix: {:?}", prefix);

        // FIXME: no unwrap
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
                match Store::get(FEATURE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Hotfix(_) => {
                match Store::get(HOTFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Bugfix(_) => {
                match Store::get(BUGFIX_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            Branch::Release(_) => {
                match Store::get(RELEASE_BRANCH_NAME_KEY) {
                    Ok(prefix) => Some(prefix),
                    Err(_) => None,
                }
            },
            _ => None,
        }
    }

    pub fn source (&self) -> Result<Vec<Branch>, String> {
        
        let branch_name: String = String::from(self.name());

        // Lives in memory while the function is running (?).
        let mut inner_prefix = String::from("");

        // exclusive_commits(...) uses Option<&str> and we have
        // Option<String>. This, below, is the necessary conversion.
        let branch_prefix = match self.prefix() {
            Some(prefix) => {
                inner_prefix = prefix.clone();
                Some(inner_prefix.as_str())
            },
            None => None,
        };
         
        // 1. Get the commits only on the branch
        let branch_only_commits = match GitV2::exclusive_commits(branch_prefix, &branch_name) {
            Ok(commits) => commits,
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        // 2. Get the first 100 commits in the branch
        let branch_commits = match GitV2::all_commits(branch_prefix, &branch_name, 100) {
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
        let branch_prefix: &str = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };    

        let source_branches = match GitV2::source_branches(
            &first_commit_not_in_branch, 
            branch_prefix, 
            &branch_name
        ) {
            Ok(branches) => {
                branches
                    .iter()
                    .map(|branch_name| Branch::from(branch_name))
                    .collect()
            },
            Err(err) => {
                println!("Error fetching the source branches: {}", err);
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
