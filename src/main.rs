mod dsh;

use std::env::args;

use dsh::{
dasher::Dasher,
get_current_directory,
OptionCode,
get_opt,
lane::Lane};

use crate::dsh::IndexNickname;


fn main() {
    let env_args:Vec<String> = args().collect();
    match get_opt(&env_args) {
        OptionCode::Add => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(dasher_err) => {
                    return dasher_err.log();
                },
            };
            
            let directory = match get_current_directory() {
                Ok(directory) => match dsh.validate(&directory){
                    Ok(_) => directory.trim().to_string(),
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
            dsh.save_lanes().unwrap();
        },
        OptionCode::List => {
            let dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => return load_err.log(),
            };

            let data: Vec<(u32, String, String)> = dsh.cache.iter()
                .map(|(_, lane)| {
                    (lane.index, lane.nickname.clone(), lane.lane.clone())
                })
                .collect::<Vec<_>>();
            
                println!("dsh:");

            data.iter().for_each(|(index, nickname, directory)| {
                if nickname != "" {
                    println!("{}:{}", index, nickname);
                } else {
                    println!("{}:{}", index, directory);
                }
            });
        },
        OptionCode::Move => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => return load_err.log(),
            };
            let index_one = match env_args[2].trim().parse::<u32>() {
                Ok(index) => IndexNickname::from(index),
                Err(_) => IndexNickname::from(env_args[2].clone()),
            };

            let index_two = match env_args[3].trim().parse::<u32>() {
                Ok(index) => IndexNickname::from(index),
                Err(_) => IndexNickname::from(env_args[3].clone()),
            };    
            
            match dsh.swap(index_one, index_two) {
                Ok(_) => {},
                Err(swap_err) => return swap_err.log(),
            };

            dsh.save_lanes();
        },
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

