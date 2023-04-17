use regex::Regex;
use crate::{command::runner::{Runner}, config::constants::COMMIT_HASH_REGEX_PATTERN, info, working, error};

/**
 * git-flow vs git (Very cool comparison)
 * https://gist.github.com/JamesMGreene/cdd0ac49f90c987e45ac
 */

///
/// GitV2 is a wrapper around the git command line tool.
/// It provides a set of functions that can be used to
/// interact with git repositories.
/// 
pub enum GitV2 {}

///
/// This is the GitV2 default implementation
/// 
impl GitV2 {

    pub fn is_remote () -> bool {

        match Runner::run("git remote -v") {
            Ok(remote) => {

                if ! remote.trim().is_empty() {
                    true
                } else {
                    false
                }

            },
            Err(_) => false
        }

    }

    pub fn remote_push_url () -> Option<String> {

        match Runner::run("git remote -v") {
            Ok(remote) => {

                info!("Raw remote is {}", remote);

                let regex = match Regex::new(r"origin\s+(?P<url>.+)\s+\(push\)") {

                    Ok(regex) => regex,
                    Err(_) => return None

                };

                let captures = regex.captures(&remote);

                if let Some(captures) = captures {
                    Some(captures["url"].to_string())
                } else {
                    None
                }

            },
            Err(_) => None
        }

    }

    pub fn exists_local (branch_fullname: &str) -> bool {
        match Runner::run(&format!("git branch --list {}", branch_fullname)) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    pub fn exists_remote (branch_fullname: &str) -> bool {
        match Runner::run(&format!("git ls-remote --heads origin {}", branch_fullname)) {
            Ok(remote_response) => {
                let regex = match Regex::new(COMMIT_HASH_REGEX_PATTERN) {
                    Ok(regex) => regex,
                    Err(_) => return false
                };
                let captures = regex.captures(&remote_response);
                if let Some(_) = captures {
                    return true;
                }
                false
            },
            Err(_) => false
        }
    }

    pub fn exists (branch_fullname: &str) -> bool {
        
        if GitV2::exists_local(branch_fullname) {
            return true;
        }
        
        return GitV2::exists_remote(branch_fullname);

    }

    ///
    /// Pushes changes to the remote repository
    /// 
    /// ### Parameters
    /// 
    /// * `branch_name` - The name of the branch to be pushed
    /// * `first_push` - If it is the first push, the `-u` flag is used
    /// 
    /// ### Returns
    /// 
    /// * `Result<String, String>` - The output of the git command
    /// 
    /// FIXME: Should receive the prefix and name as separate parameters!
    pub fn push (branch_fullname: &str, first_push: bool) -> Result<String, String> {
        if first_push {
            Runner::run(&format!("git push -u origin {}", branch_fullname))
        } else {
            Runner::run(&format!("git push origin {}", branch_fullname))
        }
    }

    ///
    /// Retrieve changes from the remote repository
    ///
    #[allow(dead_code)]
    pub fn pull (branch_name: &str) -> Result<String, String> {
        Runner::run(&format!("git pull origin {}", branch_name))
    }

    ///
    /// Returns the status of the current git repository
    /// 
    pub fn status () -> Result<String, String> {
        Runner::run("git status") 
    }

    ///
    /// Checks if the current directory is a git repository.
    /// If it is not, it will initiate a new git repository
    ///  
    pub fn init () -> Result<String, String> {

        match GitV2::status() {
            Ok(output) => {
                info!("Git is already initiated");
                Ok(output)
            },
            Err(error_message) => {
                working!("Git is not initiated: {}", error_message);
                match Runner::run("git init") {
                    Ok(output) => {
                        info!("Git is initiated");
                        Ok(output)
                    },
                    Err(e) => Err(e)
                }
            }
        }

    }

    ///
    /// Adds the given files to the staging area
    /// 
    pub fn add (file_names: Vec<String>) -> Result<String, String> {
        match Runner::run(&format!("git add {}", file_names.join(" "))) {
            Ok(output) => Ok(output),
            Err(e) => Err(e)
        }
    }

    pub fn merge_local (
        source_branch_prefix: Option<&str>, 
        source_branch_name: &str,
        target_branch_prefix: Option<&str>,
        target_branch_name: &str
    ) -> Result<String, String> {

        let source_branch_prefix = match source_branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let target_branch_prefix = match target_branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        return Runner::run(
            &format!(
                "git fetch . {}{}:{}{}", 
                source_branch_prefix, 
                source_branch_name, 
                target_branch_prefix, 
                target_branch_name
            )
        );

    }

    ///
    /// Commits the changes with the given message
    /// 
    pub fn commit (message: String, allow_empty: bool) -> Result<String, String> {

        let allow_empty = match allow_empty {
            true => "--allow-empty",
            false => "",
        };

        return Runner::run(&format!("git commit -m \"{}\" {}", message, allow_empty));

    }

    ///
    /// Checks out the branch with the given prefix (optional) and name
    /// 
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

    ///
    /// Removes the given local branch.
    /// 
    /// ### Parameters
    /// 
    /// * `branch_prefix` - The prefix of the branch to be removed
    /// 
    /// ### Returns
    /// 
    /// * `Result<String, String>` - The output of the git command
    /// 
    /// ### Example
    /// 
    /// ```rust
    /// match GitV2::remove_local_branch(Some("feature/"), "my-feature") {
    ///     Ok(output) => success!("Branch removed: {}", output),
    ///     Err(error_message) => error!("Error removing branch: {}", error_message)
    /// }
    /// ```
    /// 
    pub fn remove_local_branch (branch_prefix: Option<&str>, branch_name: &str) -> Result<String, String> {

        let branch_prefix = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let command = format!("git branch -D {}{}", branch_prefix, branch_name);

        Runner::run(&command)

    }

    ///
    /// Creates a new branch with the given prefix (optional) and name
    /// 
    pub fn branch (branch_prefix: Option<&str>, branch_name: &str) -> Result<String, String> {

        let branch_prefix = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let command = format!("git branch {}{}", branch_prefix, branch_name);

        Runner::run(&command)

    }

    ///
    /// Returns a list of commits that are only in the branch
    /// FIXME: Using unwrap! 😱
    /// 
    pub fn exclusive_commits (branch_prefix: Option<&str>, branch_name: &str) -> Result<Vec<String>, String> {

        let branch_prefix =match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };

        let branch_full_name = format!("{}{}", branch_prefix, branch_name);
        
        let command = format!(
            "git log {} --not $(git for-each-ref --format='%(refname:short)' refs/heads/ | grep -v \"{}\") \"$@\"", 
            branch_full_name, 
            branch_full_name
        );
    
        let branch_only_commits_result = match Runner::run(&command) {
            Ok(output) => output,
            Err(err) => {
                error!("{}", err);
                return Err(format!("{}", err));
            },
        };
    
        let regex: Regex = match Regex::new(COMMIT_HASH_REGEX_PATTERN) {
            Ok(regex) => regex,
            Err(err) => {
                error!("{}", err);
                return Err(format!("{}", err));
            }
        };
    
        // FIXME: No unwrap
        Ok(
            regex
                .captures_iter(&branch_only_commits_result)
                .filter(|capture| capture.get(0).is_some())
                .map(|capture| capture.get(0).unwrap().as_str().to_string())
                .collect()
        )
    
    }
    
    ///
    /// Returns a list of commits that are in the branch using the given limit
    /// 
    pub fn all_commits (branch_prefix: Option<&str>, branch_name: &str, limit: u8) -> Result<Vec<String>, String> {
        
        let branch_prefix = match branch_prefix {
            Some(prefix) => prefix,
            None => "",
        };
        
        let branch_full_name = format!("{}{}", branch_prefix, branch_name);
        
        let command = format!(
            "git log {} -n {} --pretty=oneline", 
            branch_full_name, 
            limit
        );
    
        let branch_commits_result = match Runner::run(&command) {
            Ok(output) => output,
            Err(err) => {
                error!("{}", err);
                return Err(format!("{}", err));
            }
        };
    
        let regex = match Regex::new(COMMIT_HASH_REGEX_PATTERN) {
            Ok(regex) => regex,
            Err(err) => {
                error!("{}", err);
                return Err(format!("{}", err));
            }
        };
    
        // FIXME: No unwrap
        Ok(
            regex  
                .captures_iter(&branch_commits_result)
                .filter(|capture| capture.get(0).is_some())
                .map(|capture| capture.get(0).unwrap().as_str().to_string())
                .collect()
        )
    }
    
    ///
    /// Returns a list of branches that contain the given commit.
    /// The given branch is excluded from the list.
    /// 
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