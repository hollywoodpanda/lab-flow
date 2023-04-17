use crate::config::store::Store;
use crate::flow::branch::Branch;
use crate::flow::init::Script;

use crate::command::gitv2::GitV2;

use crate::config::constants::DEVELOP_BRANCH_NAME_KEY;

use super::browser::Browser;

#[derive(Debug)]
pub enum Action {
    Init,
    Start(Branch, Option<Branch>),
    Finish(Branch),
    // TODO: Publish(Branch),
    // TODO: Pull(Branch),
    // TODO: Track(Branch),    
}

impl Action {

    fn branch_prefix (args: &Vec<String>) -> Option<&str> {

        if args.len() >= 2 {
            return Some(&args[1]);
        }

        None

    }

    fn action_text (args: &Vec<String>) -> Option<&str> {

        if args.len() >= 2 && args[1] == "init" {
            return Some(&args[1]);
        }

        if args.len() >= 3 {
            return Some(&args[2]);
        }

        None

    }

    fn branch_name (args: &Vec<String>) -> Option<&str> {
        
        if args.len() >= 4 {
            return Some(&args[3]);
        }

        None

    }

    fn branch (branch_type: &str, name: &str) -> Option<Branch> {

        match branch_type.to_lowercase().as_str() {
            "feature" => Some(Branch::Feature(name.to_string())),
            "hotfix" => Some(Branch::Hotfix(name.to_string())),
            "bugfix" => Some(Branch::Bugfix(name.to_string())),
            "release" => Some(Branch::Release(name.to_string())),
            _ => None
        }

    }

    fn calculate_branch (args: &Vec<String>) -> Option<Branch> {

        let branch_prefix = Self::branch_prefix(args);
        let branch_name = Self::branch_name(args);

        match (branch_prefix, branch_name) {

            (Some(prefix), Some(name)) => {

                return Self::branch(prefix, name);

            },
            _ => None
        }

    }

    ///
    /// Creates a new Action from the given arguments.
    /// 
    /// # Argument
    /// * `args` The console arguments used in the git flow command
    /// 
    /// # Returns
    /// 
    /// * `Some(Action)` The Action, if it could be parsed - or None.
    /// 
    /// # Example
    /// 
    /// ```rust
    /// use crate::command::args::Action;
    /// 
    /// let args: Vec<String> = std::env::args().collect();
    /// 
    /// let my_action = Action::new(&args);
    /// ```
    /// 
    /// @return The Action, if it could be parsed
    pub fn new (args: &Vec<String>) -> Option<Self> {

        // No arguments? Somethings is fishy!
        if args.len() <= 0 {
            return None;
        }

        // Let's see what action we're dealing with
        match Self::action_text(&args) {

            // git flow init ...
            Some("init") => Some(Action::Init),

            // git flow <action> finish <branch> ...
            Some("finish") => {

                match Self::calculate_branch(&args) {
                    Some(branch) => Some(Action::Finish(branch)),
                    None => None
                }

            },

            // git flow <action> start <branch> ...
            Some("start") => {

                match Self::calculate_branch(&args) {
                    Some(branch) => Some(Action::Start(branch, None)),
                    None => None
                }

            },

            // Unrecognized action 🫣
            _ => None,

        }

    }

    fn init () -> Result<(), String> {
        
        if Script::is_initiated() {
            return Ok(());
        }

        match Script::create() {
            Ok(_) => Ok(()),
            // TODO: Improve this error message
            Err(_) => Err(String::from("Couldn't initialize lab flow."))
        }

    }

    fn start (branch: &Branch, source_branch: &Option<Branch>) -> Result<(), String> {

        println!("[DEBUG] Start called!");

        let mut prefix_text: String = String::new();

        match branch.prefix() {
            Some(pfx) => {
                prefix_text = pfx;
            },
            _ => {},
        };

        let prefix: Option<&str> = match prefix_text.as_str() {
            "" => None,
            _ => Some(&prefix_text)
        };

        println!("[DEBUG] Prefix is {:?}", &prefix);

        let develop_name = match Store::get(DEVELOP_BRANCH_NAME_KEY) {
            Ok(name) => name,
            Err(e) => { return Err(e); }
        };

        println!("[DEBUG] Develop name is {}", develop_name);

        // Vamos pra develop...
        match GitV2::checkout(None, &develop_name, false) {
            Ok(_) => {},
            Err(e) => { return Err(e); }
        }

        println!("[DEBUG] Checkout to develop done!");

        // Criamos a branch nova...
        match GitV2::checkout(prefix, branch.name(), true) {
            Ok(_) => {},
            Err(e) => { return Err(e); }
        }

        println!("[DEBUG] Checkout of branch {} done!", &branch.name());

        // Damos push caso exista o remote
        match GitV2::push(&format!("{}{}", &prefix_text, &branch.name()), true) {
            Ok(_) => {},
            Err(_) => { println!("[DEBUG] Error pushing to remote... Is there a remote server?"); }
        }

        println!("[DEBUG] Push done!");

        Ok(())

    }

    fn finish (branch: &Branch) -> Result<(), String> {

        let branch_prefix_string = match branch.prefix() {
            Some(pfx) => Some(pfx.clone()),
            None => None
        };

        let branch_prefix_str = match &branch_prefix_string {
            Some(pfx) => pfx.as_str(),
            None => ""
        };

        let branch_prefix_option = match branch_prefix_str {
            "" => None,
            _ => Some(branch_prefix_str)
        };

        let branch_fullname = format!("{}{}", branch_prefix_str, branch.name());

        let branch_sources = match branch.source() {
            Ok(sources) => sources,
            Err(e) => { return Err(e); }
        };

        // 1. Tem remoto?
        if GitV2::is_remote() {

            println!("[DEBUG] Branch fullname: {}", &branch_fullname);

            // 1.1. Se não tem, criamos branch no remoto. Se tem, atualizamos no remoto            
            match GitV2::push(&branch_fullname, ! GitV2::exists_remote(&branch_fullname)) {
                Ok(_) => { println!("[DEBUG] Branch {} pushed to remote!", &branch_fullname); },
                Err(e) => { return Err(e); }
            }

            branch_sources.iter().for_each(|source| {
                match Browser::merge_request(&branch, source) {
                    Ok(_) => {},
                    Err(e) => { println!("[ERROR] Something weird while opening merge request 🫣: {}", e); }
                }
            });

        } else { // Não tem remoto!
            // 1.3. Se não tem, mergeamos branch nas sources
            branch_sources.iter().for_each(|target_branch| {

                let target_branch_prefix = match branch.prefix() {
                    Some(pfx) => Some(pfx.clone()),
                    None => None
                };

                let target_branch_prefix = match &target_branch_prefix {
                    Some(pfx) => Some(pfx.as_str()),
                    None => None
                };

                match GitV2::merge_local(
                    branch_prefix_option,
                    &branch.name(), 
                    target_branch_prefix, 
                    &target_branch.name()
                ) {
                    Ok(_) => {},
                    Err(e) => { println!("[ERROR] Something weird while merging local branches 🫣: {}", e); }
                }
            });
        }

            
        // 1.2. Vamos para a develop...
        match Store::get(DEVELOP_BRANCH_NAME_KEY) {
            Ok(develop_name) => {
                match GitV2::checkout(None, &develop_name, false) {
                    Ok(_) => {},
                    Err(e) => { return Err(e); }
                }
            },
            Err(e) => { return Err(e); }
        }

        // 1.3. ... e agora podemos remover a branch local (já estamos na develop)
        match GitV2::remove_local_branch(branch_prefix_option, &branch.name()) {
            Ok(_) => { println!("[DEBUG] Branch {} removed from local", &branch_fullname); },
            Err(e) => { return Err(e); }
        }
            

        // 2. Checkout da develop

        // 3. Se deu certo, removemos a branch local

        Ok(())

    }
    
    pub fn execute (&self) -> Result<(), String> {

        match self {

            Action::Init => Self::init(),
            Action::Start(branch, source) => Self::start(branch, source),
            Action::Finish(branch) => Self::finish(branch),

        }

    }

}

