pub mod codes;
pub mod dasher;
pub mod lane;
pub mod error;

use std::process::{Command, Stdio};

use crate::dsh::error::DasherError;

pub fn get_current_directory() -> Result<String, Box<dyn DasherError>>{
    let process = match Command::new("pwd").stdout(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(command_err) => return Err(Box::new(command_err)),
    };
    
    match process.wait_with_output() {
        Ok(output) => match String::from_utf8(output.stdout) {
            Ok(directory) => Ok(directory),
            Err(string_err) => return Err(Box::new(string_err)),
        },
        Err(process_err) => Err(Box::new(process_err)),
    }
}


#[derive(Debug)]
pub enum OptionCode {
    Add,
    Remove,
    List,
    Move,
    Dash,
    Help,
    CommandError,
}

pub fn get_opt(args: &Vec<String>) -> OptionCode {
    if args.len() <= 1 {
        return OptionCode::Help
    }

    match args[1].as_str() {
        "a" => {
            if args.len() > 3 {
                OptionCode::CommandError
            }else {
                OptionCode::Add
            }
        },

        "rm" => {
            if args.len() != 3 {
                OptionCode::CommandError
            }else{
                OptionCode::Remove
            }
        },

        "ls" => {
            if args.len() > 3 {
                OptionCode::CommandError
            }else{
                OptionCode::List
            }
        },

        "mv" => {
            if args.len() != 4 {
                OptionCode::CommandError
            }else{
                OptionCode::Move
            }
        },

        "h" => OptionCode::Help,
        _ => OptionCode::Dash,
}
}


