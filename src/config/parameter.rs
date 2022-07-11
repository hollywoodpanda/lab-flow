use std::collections::HashMap;

#[derive(Debug)]
pub enum Interpreter {}

impl Interpreter {

    pub fn get_parameter (args: Vec<String>) -> Option<(String, (String, String))> {

        let args_size = args.len();

        println!("[DEBUG] args_size: {}", args_size);

        let args_position = &args.clone();

        match args_size {

            2 => {

                return match args[1].as_str() {

                    "init" => Some((String::from("init"), (String::from(""), String::from("")))),
                    _ => None    

                };

            },

            4 => {

                return match args[1].as_str() {

                    "start" => {

                        Some((String::from("start"), (args[2].to_string(), args[3].to_string())))
    
                    },
                    "finish" => {
    
                        Some((String::from("finish"), (args[2].to_string(), args[3].to_string())))
    
                    },
                    _ => None

                };

            },

            _ => None
            
        }

    }

}