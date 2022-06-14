use crate::flow::branch::{Branch};
use crate::command::runner::{Runner};

pub enum Git {}

pub trait Gitable {
    fn create_branch (branch: Branch) -> Result<(), String>;
    fn start_branch (branch: Branch) -> Result<(), String>;
    fn finish_branch (branch: Branch) -> Result<(), String>;
    fn publish_branch (branch: Branch) -> Result<(), String>;
    fn commit (message: String) -> Result<(), String>;
    fn add (file_names: Vec<String>) -> Result<(), String>;
    fn init () -> Result<(), String>;
    fn status () -> Result<(), String>;
    fn checkout (branch: &Branch, create: bool) -> Result<(), String>;
}

impl Gitable for Git {

    fn checkout (branch: &Branch, create: bool) -> Result<(), String> {
        let command = if create {
            format!("git checkout -b {}", branch.name())
        } else {
            format!("git checkout {}", branch.name())
        };
        match Runner::run(&command) {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        }
    }

    fn status () -> Result<(), String> {
        match Runner::run("git status") {
            Ok(msg) => {
                println!("Status ok: {}", msg);
                Ok(())
            },
            Err(e) => Err(e)
        }
    }

    fn init () -> Result<(), String> {

        match Git::status() {
            Ok(_) => {
                println!("Git is already initiated");
                Ok(())
            },
            Err(error_message) => {
                println!("Git is not initiated: {}", error_message);
                match Runner::run("git init") {
                    Ok(_) => {
                        println!("Git is initiated");
                        Ok(())
                    },
                    Err(e) => Err(e)
                }
            }
        }

    }

    fn add (file_names: Vec<String>) -> Result<(), String> {
        match Runner::run(&format!("git add {}", file_names.join(" "))) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    fn commit (message: String) -> Result<(), String> {

        match Runner::run(&format!("git commit -m \"{}\"", message)) {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }

    }
    
    fn create_branch (branch: Branch) -> Result<(), String> {
        
        let branch_name = branch.name();

        let command = format!("git branch {}", branch_name);
        let result = Runner::run(&command);

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }

    }

    fn start_branch (branch: Branch) -> Result<(), String> {
        // 1. Checkout source branch
        let source_branches = match branch.source() {
            Ok(source_branches) => source_branches,
            Err(_) => return Err("Error getting source branches".to_string())
        };

        // FIXME: Fragile code: using the first source branch!
        match Runner::run(format!("git checkout {}", source_branches[0]).as_str()) {
            Ok(_) => {},
            Err(err) => return Err(err)
        };
        
        // 2. Pull source branch
        match Runner::run("git pull") {
            Ok(_) => {},
            Err(err) => return Err(err)
        };
        // 3. Create target branch using the given name and prefix
        let prefix = match branch.prefix() {
            Some(prefix) => prefix,
            None => "".to_string()
        };

        // 4. Figure out the branch name
        let branch_name = branch.name();

        // 5. Add the prefix to the name
        let branch_name = format!("{}{}", prefix, branch_name);

        // 6. Start the branch
        match Runner::run(format!("git checkout -b {}", branch_name).as_str()) {
            Ok(_) => {},
            Err(err) => return Err(err)
        };
        
        Ok(())

    }

    fn finish_branch (branch: Branch) -> Result<(), String> {
        // 1. push the branch to the remote
        // 2. Create the merge request, at git lab, to the source branch
        // 3. Delete the local branch
        // 4. checkout the "develop" branch
        Err("Ops!".to_string())
    }

    fn publish_branch (branch: Branch) -> Result<(), String> {
        Err("Not implemented!".to_string())
    }

}