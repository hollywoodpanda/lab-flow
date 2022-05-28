use crate::flow::branch::{Branch, Sourceable};
use crate::command::runner::{Runnable, Runner};

pub enum Git {}

pub trait Gitable {
    fn create_branch (branch: Branch) -> Result<(), String>;
    fn start_branch (&self, branch: Branch) -> Result<(), String>;
    fn finish_branch (&self, branch: Branch) -> Result<(), String>;
    fn publish_branch (&self, branch: Branch) -> Result<(), String>;
}

impl Gitable for Git {
    
    fn create_branch (branch: Branch) -> Result<(), String> {
        
        let branch_name = match branch {
            Branch::Feature(name) => name,
            Branch::Hotfix(name) => name,
            Branch::Bugfix(name) => name,
            Branch::Release(name) => name,
            Branch::Develop(name) => name,
            Branch::Main(name) => name,
        };

        let command = format!("git branch {}", branch_name);
        let result = Runner::run(&command);

        match result {
            Ok(_) => Ok(()),
            Err(error) => Err(error)
        }

    }

    fn start_branch (&self, branch: Branch) -> Result<(), String> {
        // 1. Checkout source branch
        let source_branch = match branch.source(false) {
            Ok(source_branch) => source_branch,
            Err(_) => return Err("Error getting source branch".to_string())
        };

        // FIXME: Fragile code: using the first source branch!
        match Runner::run(format!("git checkout -b {}", source_branch[0]).as_str()) {
            Ok(msg) => println!("{}", msg),
            Err(_) => return Err("Error checking out source branch".to_string())
        };
        
        // 2. Pull source branch
        match Runner::run("git pull") {
            Ok(msg) => println!("{}", msg),
            Err(_) => return Err("Error pulling source branch".to_string())
        };
        // 3. Create target branch using the given name and prefix
        let prefix = match branch.prefix() {
            Ok(prefix) => prefix,
            Err(_) => return Err("Error getting branch prefix".to_string())
        };
        match Runner::run(format!("git checkout -b {}{}", prefix, branch).as_str()) {
            Ok(msg) => println!("{}", msg),
            Err(_) => return Err("Error creating target branch".to_string())
        };
        
        Ok(())

    }

    fn finish_branch (&self, branch: Branch) -> Result<(), String> {
        Err("Not implemented!".to_string())
    }

    fn publish_branch (&self, branch: Branch) -> Result<(), String> {
        Err("Not implemented!".to_string())
    }

}