mod command;

mod config;

mod flow;

use std::process;

use config::error::{INIT_ERROR_CODE};

use flow::{init::{Script}, branch::Branch};
use command::{args::{Action}, gitv2::GitV2, browser::Browser};

fn using_store_strategy () {

    if ! Script::is_initiated() {
        match Script::create() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("{}", e);
                process::exit(INIT_ERROR_CODE);
            }
        }
    } else {
        println!("{}", "Lab Flow is already initiated");
        Script::show();
    }

}

fn inspect_action () -> Option<Action> {

    let args: Vec<String> = std::env::args().collect();

    let action = Action::new(&args);

    match &action {
        
        Some(action) => {
            
            println!("Action: {:?}", action);
            
            match action {
                Action::Init => {
                    println!("[DEBUG] Init");
                },
                Action::Start(branch, _) => {
                    println!("[DEBUG] Start: {:?}", branch);
                },
                Action::Finish(branch) => {
                    println!("[DEBUG] Finish: with name {}{}", branch.prefix().unwrap_or(String::new()), branch.name());                    
                },
            }
            
        },
        None => {
            println!("No action");
        }
    }

    println!("[DEBUG] Action: {:?}", &action);

    action    

}

fn test_git_remote () {

    match Browser::merge_request(&Branch::Feature(String::from("jack")), &Branch::Develop("develop".to_string())) {
        Ok(url) => {
            println!("URL: {}", url);
        },
        Err(e) => {
            eprintln!("[ERROR] {}", e);
        }
    }

}

fn main() {

    println!("#########");
    println!("Lab Flow");
    println!("#########\r\n");

    // test_git_remote();
    //using_store_strategy();

    // if 1 != 1 {
    //     using_store_strategy();
    // }

    if 1 != 2 {
        match inspect_action() {
            Some(action) => { 
                match action.execute() {
                    Ok(_) => println!("[DEBUG] executed!"),
                    Err(e) => {
                        eprintln!("[ERROR] {}", e);
                    }
                } 
            },
            None => {}
        }
        
    }

}
