use crate::command::runner::{Runner};

///
/// A wrapper around the git config command,
/// to store and retrieve persisted configuration
/// from the current git repository.
/// 
pub enum Store {}

impl Store {

    ///
    /// Checks if the given branch name exists in the
    /// current git repository. It is based on the
    /// assumption that the 'git config' command will
    /// give us an error if the branch does not exist.
    /// 
    pub fn exists (branch_name: &str) -> bool {
        match Store::get(branch_name) {
            Ok(_) => true,
            Err(_) => false
        }
    }

    /// 
    /// Retrieves the value of the given branch name
    /// from the current git repository. Gives an error
    /// if the branch does not exist in the store.
    /// 
    pub fn get (branch_name: &str) -> Result<String, String> {
        Runner::run(&format!("git config --local --get {}", branch_name))
    }

    ///
    /// Adds the given value to the given branch name
    /// for the current git repository. Gives an error
    /// if the 'git config' command is not successful.
    /// It overwrites the value if the branch already
    /// have one stored.
    /// 
    pub fn add (branch_name: &str, value: &str) -> Result<String, String> {
        Runner::run(&format!("git config --local --add {} {}", branch_name, value))
    }

}