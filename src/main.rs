mod dsh;

use std::{env::args, process::exit, io::Write};

use dsh::{
dasher::Dasher,
get_current_directory,
OptionCode,
get_opt,
lane::Lane};

use crate::dsh::{IndexNickname, error::DasherError};

enum ShellSignal {
    ADD,
    REMOVE,
    LIST,
    MOVE,
    DASH,
    HELP,
    ERROR,
}

fn main() {
    let env_args:Vec<String> = args().collect();
    match get_opt(&env_args) {
        OptionCode::Add => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(dasher_err) => {
                    dasher_err.log();
                    exit(ShellSignal::ERROR as i32);
                },
            };
            
            let directory = match get_current_directory() {
                Ok(directory) => match dsh.validate(&directory){
                    Ok(_) => {
                        directory
                    },
                    Err(dash_error) => { 
                        dash_error.log();
                        exit(ShellSignal::ERROR as i32);
                    }
                },
                Err(process_err) => { 
                    process_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            };
            let nickname = if let true = env_args.len() == 3 {
                match dsh.validate(&env_args[2]) {
                    Ok(_) => env_args[2].clone(),
                    Err(validate_err) => {
                        validate_err.log();
                        exit(ShellSignal::ERROR as i32);
                    }
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

            if let Err(save_error) = dsh.save_lanes() {
                save_error.log();
                exit(ShellSignal::ERROR as i32);
            };

            exit(ShellSignal::ADD as i32);
        },
        OptionCode::List => {
            let dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => {
                    load_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            };

            let data: Vec<(u32, String, String)> = dsh.iter()
                .map(|lane| {
                    (lane.index, lane.nickname.clone(), lane.lane.clone())
                })
                .collect::<Vec<_>>();
            
                let message = format!("dsh:");
                let mut stdout = std::io::stdout().lock();
                if let Err(write_err) = stdout.write_all(message.as_bytes()) {
                    write_err.log();
                    exit(ShellSignal::ERROR as i32);
                }

            data.iter().for_each(|(index, nickname, directory)| {
                if nickname != "" {
                    let message = format!("\\n{}:{}", index, nickname);
                    if let Err(write_err) = stdout.write_all(message.as_bytes()) {
                        write_err.log();
                        exit(ShellSignal::ERROR as i32);
                    }
                } else {
                    let message = format!("\\n{}:{}", index, directory);
                    if let Err(write_err) = stdout.write_all(message.as_bytes()) {
                        write_err.log();
                        exit(ShellSignal::ERROR as i32);
                    }
                }
            });

            exit(ShellSignal::LIST as i32);
        },
        OptionCode::Move => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => {
                    load_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
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
                Err(swap_err) => {
                    swap_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            };

            if let Err(save_error) = dsh.save_lanes() {
                save_error.log();
                exit(ShellSignal::ERROR as i32);
            };

            exit(ShellSignal::MOVE as i32);
        },
        OptionCode::Dash => {
            let destination = match env_args[1].trim().parse::<u32>() {
                Ok(index) => IndexNickname::from(index),
                Err(_) => IndexNickname::from(env_args[1].clone()),
            };

            let dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => {
                    load_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            };

            let lanes: Vec<&Lane> = dsh.iter().collect::<Vec<_>>();

            match destination {
                IndexNickname::Index(index) => {
                    for lane in lanes {
                        if lane.index == index {
                            match lane.dash() {
                                Ok(_) => exit(ShellSignal::DASH as i32),
                                Err(dash_err) => {
                                    dash_err.log();
                                    exit(ShellSignal::ERROR as i32);
                                }
                            }
                        }
                    }
                    println!("dsh: lane with index {index} does not exist");
                    exit(ShellSignal::ERROR as i32);
                },
                IndexNickname::Nickname(nickname) => {
                    for lane in lanes {
                        if lane.nickname == nickname {
                            match lane.dash() {
                                Ok(_) => exit(ShellSignal::DASH as i32),
                                Err(dash_err) => {
                                    dash_err.log();
                                    exit(ShellSignal::ERROR as i32);
                                }
                            }
                        }
                    }
                    println!("dsh: lane with nickname {nickname} does not exist");
                    exit(ShellSignal::ERROR as i32);
                },
            }
        },
        OptionCode::Help => { exit(ShellSignal::HELP as i32); },
        OptionCode::Remove => {
            let mut dsh: Dasher = match Dasher::new() {
                Ok(dasher) => dasher,
                Err(load_err) => {
                    load_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            };
            
            let identifier: IndexNickname = match env_args[2].trim().parse::<u32>() {
                Ok(index) => IndexNickname::from(index),
                Err(_) => IndexNickname::from(env_args[2].clone()),
            };
            
            match dsh.remove_lane(identifier) {
                Ok(message) =>  {
                    println!("{message}")
                },
                Err(remove_err) => {
                    remove_err.log();
                    exit(ShellSignal::ERROR as i32);
                }
            }

            if let Err(save_error) = dsh.save_lanes() {
                save_error.log();
                exit(ShellSignal::ERROR as i32);
            };

            exit(ShellSignal::REMOVE as i32);
        },
        OptionCode::CommandError => {},
    }
}

