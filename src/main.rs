mod dsh;

use std::env::args;

use dsh::{
dasher::Dasher,
get_current_directory,
OptionCode,
get_opt,
lane::Lane};


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
                Ok(directory) => match dsh.validate_directory(&directory){
                    Ok(_) => directory,
                    Err(dash_error) => return dash_error.log(),
                },
                Err(process_err) => return process_err.log(),
            };
            
            let new_lane: Lane = {
                Lane {
                    index: dsh.get_key(),
                    nickname: String::from(""),
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
                println!("{}: {}", index, directory);
            });
        },
        OptionCode::Move => {},
        OptionCode::Dash => {},
        OptionCode::Help => {},
        OptionCode::Remove => {},
        OptionCode::CommandError => {},
    }
}

