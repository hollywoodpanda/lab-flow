use regex::Regex;
use crate::flow::branch::{Branch};
use crate::command::runner::{Runner};
use crate::config::file::{*};

fn get_branch_name (branch: &Branch) -> String {
    match branch {
        Branch::Feature(name) => name.to_string(),
        Branch::Hotfix(name) => name.to_string(),
        Branch::Bugfix(name) => name.to_string(),
        Branch::Release(name) => name.to_string(),
        Branch::Develop(name) => name.to_string(),
        Branch::Main(name) => name.to_string(),
    }
}

fn get_branch_prefix_or_name (branch: &Branch) -> Result<String, String> {
    let branch_prefix_config_name = match branch {
        Branch::Feature(_) => FEATURE_BRANCH_NAME_KEY,
        Branch::Hotfix(_) => HOTFIX_BRANCH_NAME_KEY,
        Branch::Bugfix(_) => BUGFIX_BRANCH_NAME_KEY,
        Branch::Release(_) => RELEASE_BRANCH_NAME_KEY,
        Branch::Develop(_) => DEVELOP_BRANCH_NAME_KEY,
        Branch::Main(_) => MAIN_BRANCH_NAME_KEY,
    };
    match get_config_value(branch_prefix_config_name) {
        Ok(prefix) => Ok(prefix),
        Err(err) => {
            let error_message = format!("{}", err);
            println!("[ERROR] {}", error_message);
            return Err(error_message);
        },
    }
}

fn get_branch_only_commits (branch_prefix: &str, branch_name: &str) -> Result<Vec<String>, String> {

    let branch_full_name = format!("{}{}", branch_prefix, branch_name);
    
    let command = format!(
        "git log {} --not $(git for-each-ref --format='%(refname:short)' refs/heads/ | grep -v \"{}\") \"$@\"", 
        branch_full_name, 
        branch_full_name
    );

    let branch_only_commits_result = match Runner::run(&command) {
        Ok(output) => output,
        Err(err) => {
            println!("[ERROR] {}", err);
            return Err(format!("{}", err));
        },
    };

    let regex: Regex = match Regex::new(r"[A-Za-z0-9]{40}") {
        Ok(regex) => regex,
        Err(err) => {
            println!("[ERROR] {}", err);
            return Err(format!("{}", err));
        }
    };

    // FIXME: No unwrap
    Ok(
        regex
            .captures_iter(&branch_only_commits_result)
            .map(|capture| capture.get(0).unwrap().as_str().to_string())
            .collect()
    )

}

fn get_branch_commits (branch_prefix: &str, branch_name: &str, limit: u8) -> Result<Vec<String>, String> {
    let branch_full_name = format!("{}{}", branch_prefix, branch_name);
    let command = format!(
        "git log {} -n {} --pretty=oneline", 
        branch_full_name, 
        limit
    );

    let branch_commits_result = match Runner::run(&command) {
        Ok(output) => output,
        Err(err) => {
            println!("[ERROR] {}", err);
            return Err(format!("{}", err));
        }
    };

    let regex = match Regex::new(r"[A-Za-z0-9]{40}") {
        Ok(regex) => regex,
        Err(err) => {
            println!("[ERROR] {}", err);
            return Err(format!("{}", err));
        }
    };

    // FIXME: No unwrap
    Ok(
        regex  
            .captures_iter(&branch_commits_result)
            .map(|capture| capture.get(0).unwrap().as_str().to_string())
            .collect()
    )
}

fn get_branches_in_commit (commit: &str, branch_prefix: &str, branch_name: &str) -> Result<Vec<String>, String> {
    match Runner::run(&format!("git branch -a --contains {}", commit)) {
        Ok(output) => {
            Ok(
                output
                    .lines()
                    .map(|line| line.replace("*", "").trim().to_string())
                    .filter(|branch_name_from_command| {

                        let original_branch_name = branch_name_from_command.to_string();
                        let branch_full_name = format!("{}{}", branch_prefix, branch_name);

                        original_branch_name != branch_full_name

                    })
                    .map(|branch_name| branch_name.trim().to_string())
                    .collect()
            )
        },
        Err(err) => Err(err)
    }
}

pub enum GitV2 {}

impl GitV2 {

    pub fn source (branch: &Branch) -> Result<Vec<Branch>, String> {
        
        let branch_name: String = String::from(branch.name());
        let branch_prefix = match branch.prefix() {
            Some(prefix) => prefix,
            None => "".to_string(),
        };
         
        // 1. Get the commits only on checked out branch
        let branch_only_commits = match get_branch_only_commits(&branch_prefix, &branch_name) {
            Ok(commits) => commits,
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        let branch_commits = match get_branch_commits(&branch_prefix, &branch_name, 100) {
            Ok(commits) => commits,
            Err(err) => {
                println!("[ERROR] {}", err);
                return Err(err);
            },
        };

        // 2. Get the first of all commits that differs from commits only in the HEAD branch.
        let first_shared_commit: String = match branch_commits
            .iter()
            .filter(|commit| !branch_only_commits.contains(commit))
            .collect::<Vec<&String>>()
            .first() {
                Some(commit) => commit.to_string(),
                None => String::from(&branch_commits[branch_commits.len() - 1]),
            };
            
        // 2. Get all the commits on branch, stop at the first
        // one not in the branch_commits. This commit contains
        // the source branches!
        let source_branches = match get_branches_in_commit(&first_shared_commit, &branch_prefix, &branch_name) {
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