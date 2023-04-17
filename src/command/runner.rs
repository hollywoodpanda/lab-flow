use std::process::{Command, Output};
use std::env;

use crate::working;

pub enum Runner {}

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

        working!("{}", command);

        let command_result: Result<Output, std::io::Error> = if cfg!(target_os = "windows") {
            run_for_windows(command)
        } else {
            run_for_nix(command)
        };

        match command_result {

            Ok(output) => {

                if output.status.success() {
                    Ok(String::from_utf8_lossy(&output.stdout).to_string())
                } else {
                    Err(String::from_utf8_lossy(&output.stderr).to_string())
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