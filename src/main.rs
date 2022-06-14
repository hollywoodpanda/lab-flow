mod command;

mod config;

mod flow;

use std::process;

use config::file::{
    create_config_file,
    contains_config_file,
    get_config_value,
    MAIN_BRANCH_NAME_KEY,
};

use config::error::{INIT_ERROR_CODE};

use command::gitv2::{GitV2};

use flow::branch::{Branch};

fn main() {
    println!("##############");
    println!("git lab plugin");
    println!("##############\r\n");
    if !contains_config_file() {
        match create_config_file() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
                process::exit(INIT_ERROR_CODE);
            }
        }
    } else {
        println!("{}", "Configuration file \".git/lab.conf\" already exists");
        println!("Main branch: {}", get_config_value(MAIN_BRANCH_NAME_KEY).unwrap());
        println!("Develop branch: {}", get_config_value("DEVELOP_BRANCH_NAME").unwrap());
        println!("Release branch: {}", get_config_value("RELEASE_BRANCH_NAME").unwrap());
        println!("Bugfix branch: {}", get_config_value("BUGFIX_BRANCH_NAME").unwrap());
        println!("Hotfix branch: {}", get_config_value("HOTFIX_BRANCH_NAME").unwrap());
        println!("Feature branch: {}", get_config_value("FEATURE_BRANCH_NAME").unwrap());

        println!("[DEBUG] Testing GitV2");

        let branch = Branch::Bugfix("AN_ISSUE".to_string());

        match branch.source() {
            Ok(branches) => {
                println!("[DEBUG] Source branches: {:?}", branches);
            },
            Err(e) => {
                println!("[DEBUG] Error: {}", e);
            }
        }


    }
}
