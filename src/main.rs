mod command;

mod config;

mod flow;

use std::process;

use config::error::{INIT_ERROR_CODE};

use flow::init::{Script};

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

fn main() {

    println!("#########");
    println!("Lab Flow");
    println!("#########\r\n");

    using_store_strategy();
    
}
