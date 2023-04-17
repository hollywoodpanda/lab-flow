mod command;

mod config;

mod flow;

use command::args::{Action};

fn inspect_action () -> Option<Action> {

    let args: Vec<String> = std::env::args().collect();

    Action::new(&args)

}

fn main() {

    println!("\r\n#########");
    println!("Lab Flow");
    println!("#########\r\n");

    match inspect_action() {
        Some(action) => { 
            match action.execute() {
                Ok(_) => success!("Done!\r\n"),
                Err(e) => {
                    error!("{}", e);
                }
            } 
        },
        None => {}
    }
        
}
