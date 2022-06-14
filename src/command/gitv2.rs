use regex::Regex;
use crate::command::runner::{Runner};

pub enum GitV2 {}

impl GitV2 {

    pub fn status () -> Result<String, String> {
        Runner::run("git status") 
    }

    pub fn init () -> Result<String, String> {

        match GitV2::status() {
            Ok(output) => {
                println!("Git is already initiated");
                Ok(output)
            },
            Err(error_message) => {
                println!("Git is not initiated: {}", error_message);
                match Runner::run("git init") {
                    Ok(output) => {
                        println!("Git is initiated");
                        Ok(output)
                    },
                    Err(e) => Err(e)
                }
            }
        }

    }

    pub fn checkout (branch_prefix: Option<&str>, branch_name: &str, create: bool) -> Result<String, String> {

        let branch_prefix = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let command = if create {
            format!("git checkout -b {}{}", branch_prefix, branch_name)
        } else {
            format!("git checkout {}{}", branch_prefix, branch_name)
        };

        Runner::run(&command)

    }

    pub fn branch (branch_prefix: Option<&str>, branch_name: &str) -> Result<String, String> {

        let branch_prefix = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let command = format!("git branch {}{}", branch_prefix, branch_name);

        Runner::run(&command)

    }

    pub fn exclusive_commits (branch_prefix: &str, branch_name: &str) -> Result<Vec<String>, String> {

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
    
    pub fn all_commits (branch_prefix: &str, branch_name: &str, limit: u8) -> Result<Vec<String>, String> {
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
    
    pub fn source_branches (commit: &str, branch_prefix: &str, branch_name: &str) -> Result<Vec<String>, String> {
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
                        .collect()
                )
            },
            Err(err) => Err(err)
        }
    }

}