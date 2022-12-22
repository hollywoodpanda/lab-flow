mod command;

mod config;

mod flow;

use std::process;

use config::error::{INIT_ERROR_CODE};

use flow::init::{Script};
use command::args::Args;

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

fn inspect_args () {
    
        let args: Vec<String> = std::env::args().collect();
    
        let args = Args::new(args);
    
        let branch_name = match args.branch {
            Some(branch) => String::from(branch.name()),
            None => "".to_string()
        };

        let release_branch = match args.release {
            Some(release) => String::from(release.name()),
            None => "".to_string()
        };

        println!("Branch: {}", branch_name);
        println!("Release branch (if informed): {}", release_branch);
        println!("Is init: {}", args.is_init);
        println!("Files: {:?}", args.files);
    
}

fn main() {

    println!("#########");
    println!("Lab Flow");
    println!("#########\r\n");

    //using_store_strategy();

    inspect_args();
    
}
