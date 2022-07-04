use std::process::{Command, Output};
use std::env;

pub trait Browseable {

    fn open (url: &str);

}

pub enum Runner {}

pub enum Browser {}

impl Runner {

    pub fn open (url: &str) -> Result<String, String> {

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

        Runner::run(os_command.as_str())

    }

    pub fn run (command: &str) -> Result<String, String> {

        println!("{}", command);

        let command_result: Result<Output, std::io::Error> = if cfg!(target_os = "windows") {
            run_for_windows(command)
        } else {
            run_for_nix(command)
        };

        match command_result {

            Ok(output) => {

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                if stderr.len() > 0 {
                    Err(stderr.to_string())
                } else {
                    Ok(stdout.to_string())
                }

            },

            Err(command_error) => Err(format!("{:?}", command_error))

        }

    }

}

fn run_for_windows (command: &str) -> Result<Output, std::io::Error> {

    match Command::new("cmd")
            .args(["/C", command])
            .output() {

        Ok(res) => Ok(res),
        
        Err(cmd_error) => Err(cmd_error)

    }

}

fn run_for_nix (command: &str) -> Result<Output, std::io::Error> {

    match Command::new("sh")
            .args(["-c", command])
            .output() {

        Ok(res) => {
            Ok(res)
        },

        Err(sh_error) => Err(sh_error)

    }
        
}