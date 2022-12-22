use crate::flow::branch::Branch;
use crate::config::store::Store;
use crate::config::constants::{
    FEATURE_BRANCH_NAME_KEY,
    BUGFIX_BRANCH_NAME_KEY,
    HOTFIX_BRANCH_NAME_KEY,
    RELEASE_BRANCH_NAME_KEY,
    DEVELOP_BRANCH_NAME_KEY,
    MAIN_BRANCH_NAME_KEY,
};


pub struct Args {
    pub is_init: bool,
    pub is_branch_related: bool,
    pub branch: Option<Branch>,
    pub release: Option<Branch>,
    pub files: Option<Vec<String>>
}

#[deprecated(note = "Not sure this is really needed")]
struct BranchConfig {
    develop: String,
    main: String,
    release: String,
    feature: String,
    hotfix: String,
    bugfix: String
}

impl Args {

    fn get_branch_from_store (key: &str) -> Result<String, String> {
        match Store::get(key) {
            Ok(branch_name) => Ok(String::from(branch_name.trim())),
            Err(_) => Err(format!("{} not found", key))
        }
    }

    fn get_branch_from_text (arg: &str) -> Option<Branch> {
        
        let develop_config = match Args::get_branch_from_store(DEVELOP_BRANCH_NAME_KEY) {
            Ok(develop_name) => develop_name,
            Err(_) => return None
        };

        let main_config = match Args::get_branch_from_store(MAIN_BRANCH_NAME_KEY) {
            Ok(main_name) => main_name,
            Err(_) => return None
        };

        let release_config = match Args::get_branch_from_store(RELEASE_BRANCH_NAME_KEY) {
            Ok(release_name) => release_name,
            Err(_) => return None
        };

        let feature_config = match Args::get_branch_from_store(FEATURE_BRANCH_NAME_KEY) {
            Ok(feature_name) => feature_name,
            Err(_) => return None
        };

        let hotfix_config = match Args::get_branch_from_store(HOTFIX_BRANCH_NAME_KEY) {
            Ok(hotfix_name) => hotfix_name,
            Err(_) => return None
        };

        let bugfix_config = match Args::get_branch_from_store(BUGFIX_BRANCH_NAME_KEY) {
            Ok(bugfix_name) => bugfix_name,
            Err(_) => return None
        };

        let branch: Branch = match arg {
            _ if arg == develop_config => Branch::Develop(String::from(arg)),
            _ if arg == main_config => Branch::Main(String::from(arg)),
            _ if arg.starts_with(&release_config) => Branch::Release(String::from(arg)),
            _ if arg.starts_with(&feature_config) => Branch::Feature(String::from(arg)),
            _ if arg.starts_with(&hotfix_config) => Branch::Hotfix(String::from(arg)),
            _ if arg.starts_with(&bugfix_config) => Branch::Bugfix(String::from(arg)),
            _ => return None
        };

        Some(branch)

    }

    fn extract_branch (args: &Vec<String>) -> Option<Branch> {
        match args.len() {
            0 ..= 1 => None,
            _ => Args::get_branch_from_text(&args[1])
        }
    }

    fn extract_is_branch_related (args: &Vec<String>) -> bool {
        Args::extract_branch(args).is_some()
    }

    fn extract_release_branch (args: &Vec<String>) -> Option<Branch> {

        let release_name: Option<String> = match args.len() {
            0 ..= 2 => None,
            _ => {

                let slice: &[String] = &args[2..];

                let found = slice.iter().position(|arg| arg == &"--release");

                match found {
                    Some(index) => {

                        match slice.len() {
                            n if n > index + 1 => {

                                println!("[DEBUG] Release branch name: {}", &slice[index + 1]);

                                match Args::get_branch_from_store(RELEASE_BRANCH_NAME_KEY) {
                                    Ok(release_name) => {

                                        if !&slice[index + 1].starts_with(&release_name) {
                                            println!("[WARN] The given release branch is not a release branch or it is not configured as such.");
                                            None
                                        } else {
                                            Some(String::from(&slice[index + 1]))
                                        }

                                    },
                                    Err(_) => None
                                }

                            },
                            _ => None
                        }

                    },
                    None => None
                }

            }
        };

        match release_name {
            Some(name) => Args::get_branch_from_text(&name),
            None => None
        }

    }

    pub fn new (args: Vec<String>) -> Self {

        let is_init: bool = match args.len() {
            0 ..= 1 => false,
            _ => args[1] == "init"
        };

        let branch: Option<Branch> = Args::extract_branch(&args);

        let is_branch_related: bool = Args::extract_is_branch_related(&args);

        let release = Args::extract_release_branch(&args);

        let release: Option<Branch> = Args::extract_release_branch(&args);
        
        Args {
            is_init,
            is_branch_related,
            branch,
            release: release,
            files: None
        }
        
    }

}