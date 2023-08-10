mod dsh;

use std::env::args;

use dsh::{
dasher::Dasher,
get_current_directory,
OptionCode,
get_opt,
lane::Lane};

use crate::dsh::dasher::IndexNickname;


fn main() {
    let env_args:Vec<String> = args().collect();
    match get_opt(&env_args) {
        OptionCode::Add => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(dasher_err) => {
                    println!("dsh: could not load lanes");
                    return dasher_err.log();
                },
            };
            
            let directory = match get_current_directory() {
                Ok(directory) => match dsh.validate(&directory){
                    Ok(_) => directory,
                    Err(dash_error) => return dash_error.log(),
                },
                Err(process_err) => return process_err.log(),
            };

            let nickname = if let true = env_args.len() == 3 {
                match dsh.validate(&env_args[2]) {
                    Ok(_) => env_args[2].clone(),
                    Err(validate_err) => return validate_err.log(),
                }
            } else {
                "".to_string()
            };


            let new_lane: Lane = {
                Lane {
                    index: dsh.get_key(),
                    nickname,
                    lane: directory,
                }
            };

            dsh.add_lane(new_lane);
            dsh.save_lanes();
        },
        OptionCode::List => {
            let dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => return load_err.log(),
            };

            let data: Vec<(u32, String)> = dsh.cache.iter()
                .map(|(_, lane)| {
                    (lane.index, lane.lane.clone())
                })
                .collect::<Vec<_>>();
            
            data.iter().for_each(|(index, directory)| {
                print!("{}: {}", index, directory);
            });
        },
        OptionCode::Move => {},
        OptionCode::Dash => { println!("whats good bro") },
        OptionCode::Help => {},
        OptionCode::Remove => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => return load_err.log(),
            };
            
            let identifier: IndexNickname = match env_args[2].trim().parse::<u32>() {
                Ok(index) => IndexNickname::from(index),
                Err(_) => IndexNickname::from(env_args[2].clone()),
            };
            
            match dsh.remove_lane(identifier) {
                Ok(message) =>  {
                    println!("{message}")
                },
                Err(remove_err) => return remove_err.log(),
            }

            dsh.save_lanes();
        },
        OptionCode::CommandError => {},
    }
}

