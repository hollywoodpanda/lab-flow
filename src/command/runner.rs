use std::process::{Command, Output};
use std::env;

pub trait Runnable {

    fn run (command: &str) -> Result<String, String>;

}

pub trait Browseable {

    fn open (url: &str);

}

pub enum Runner {}

pub enum Browser {}

impl Runnable for Runner {

    fn run (command: &str) -> Result<String, String> {

        let command_result: Result<Output, std::io::Error> = if cfg!(target_os = "windows") {
            run_for_windows(command)
        } else {
            run_for_nix(command)
        };

        match command_result {

            Ok(output) => {

                // FIXME: Aceitar outros encodings
                match String::from_utf8(output.stdout) {

                    Ok(stdout) => Ok(stdout),

                    Err(stderr) => {
                        let stderr_message = format!("{:?}", stderr);
                        println!("{}", stderr_message);
                        Err(stderr_message)
                    }

                }

            },

            Err(command_error) => {
                let error_message = format!("{:?}", command_error);
                println!("{}", error_message);
                return Err(error_message);
            }

        }

    }

}

impl Browseable for Browser {

    fn open (url: &str) {

        let os_command: String;

        match env::consts::OS {

            "windows" => {
                os_command = format!("rundll32 url.dll,FileProtocolHandler {}", url);
            },

            "macos" => {
                os_command = format!("open -u \"{}\"", url);
            },

            _ => {
                os_command = format!("xdg-open {}", url);
            }

        }

        Runner::run(os_command.as_str()).ok();

    }

}

fn run_for_windows (command: &str) -> Result<Output, std::io::Error> {

    println!("[DEBUG][WIN] Running command: {}", command);

    match Command::new("cmd")
            .args(["/C", command])
            .output() {

        Ok(res) => Ok(res),
        
        Err(cmd_error) => Err(cmd_error)

    }

}

fn run_for_nix (command: &str) -> Result<Output, std::io::Error> {

    println!("[DEBUG][NIX] Running command: {}", command);

    match Command::new("sh")
            .args(["-c", command])
            .output() {

        Ok(res) => Ok(res),

        Err(sh_error) => Err(sh_error)

    }
    
}