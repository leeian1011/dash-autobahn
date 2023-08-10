#![allow(unused_variables, dead_code)]
use crate::dsh::error::DashError;

use super::{lane::Lane, error::DasherError};
use std::{collections::BTreeMap,
        io::{Read, Write},
        fs::File, env,
        
};
pub enum IndexNickname {
    Index(u32),
    Nickname(String),
}

impl From<u32> for IndexNickname {
    fn from(value: u32) -> Self {
        IndexNickname::Index(value)
    }
}

impl From<String> for IndexNickname {
    fn from(value: String) -> Self {
        IndexNickname::Nickname(value)
    }
}

pub struct Dasher {
   pub cache: BTreeMap<u32, Lane>
}

impl Dasher {

    fn load_lanes() -> Result<BTreeMap<u32, Lane>, Box<dyn DasherError>> {
        let mut path = match env::var("DASH_CACHE_PATH") {
            Ok(string) => string,
            Err(env_err) => return Err(Box::new(env_err)),
        };

        path.push_str("lanes.json");
        let mut cache: String = String::new();
        match File::open(path) {
            Ok(mut file) => {
                match file.read_to_string(&mut cache) {
                    Ok(_) => {},
                    Err(read_err) => return Err(Box::new(read_err))
                }
            },
            Err(io_err) => return Err(Box::new(io_err)),
        };
        
        let loaded_lanes: Vec<Lane> = match serde_json::from_str::<Vec<Lane>>(&cache) {
            Ok(lanes) => lanes,
            Err(serde_err) => return Err(Box::new(serde_err)),
        };
        
        let tree_map: BTreeMap<u32, Lane> = loaded_lanes.into_iter()
            .map(|lane| (lane.index, lane))
            .collect::<BTreeMap<u32, Lane>>();

        Ok(tree_map)
    }

    pub fn new() -> Result<Dasher, Box<dyn DasherError>> {
        Ok(Dasher {
            cache: match Dasher::load_lanes() {
                Ok(dasher_cache) => dasher_cache,
                Err(dasher_err) => return Err(dasher_err),
            }
        })
    }

    pub fn save_lanes(&self) -> Result<(), Box<dyn DasherError>> {
        let cached_dash: String = match serde_json::to_string(&self.cache.values().collect::<Vec<&Lane>>()) {
            Ok(cache_json) => cache_json,
            Err(serde_err) => return Err(Box::new(serde_err))
        };

        let mut path = match env::var("DASH_CACHE_PATH") {
            Ok(path) => path,
            Err(env_err) => return Err(Box::new(env_err)),
        };

        path.push_str("lanes.json"); 

        match File::create(path) {
            Ok(mut file) => match file.write_all(&cached_dash.as_bytes()) {
                Ok(_) => return Ok(()),
                Err(io_err) => return Err(Box::new(io_err))
            }
            Err(file_err) => return Err(Box::new(file_err))
        }
    }

    pub fn add_lane(&mut self, lane: Lane) -> () {
        self.cache.insert(lane.index, lane);
    }
     
    pub fn remove_lane(&mut self, identifier: IndexNickname) -> Result<String, Box<dyn DasherError>>
            {
                match identifier {
                    IndexNickname::Index(index) => {
                        match self.cache.remove(&index) {
                            Some(_) => Ok(format!("dsh: lane at index {index} removed.")),
                            None => Err(Box::new(DashError::new(format!("lane does not exist at index {index}")))),
                        }
                    },
                    IndexNickname::Nickname(nickname) => {
                        let lanes: Vec<&Lane> = self.cache.values().collect();
                        let mut index: i32 = -1;
                        for lane in lanes {
                            if lane.nickname == nickname {
                                index = lane.index as i32;
                            }
                        };

                        if index == -1 {
                            return Err(Box::new(DashError::new(format!("lane nicknamed '{nickname}' does not exist"))));
                        }
                        
                        match self.cache.remove(&(index as u32)) {
                            Some(_) => {
                                return Ok(format!("dsh: lane nicknamed {nickname} removed."));
                            },
                            None => {
                                return Err(Box::new(DashError::new(format!("lane could not be removed"))));
                            },
                        };
                    }
                }
            }

    pub fn get_key(&self) -> u32 {
        let keys: Vec<&u32> = self.cache.keys().collect::<Vec<&u32>>();
        let mut index: usize = 0;

        loop {

            if keys.len() == 0 || keys[0] != &0 {
                return 0
            }

            if index == keys.len() - 1 {
                return keys[index] + 1
            }

            if keys[index + 1] > &(keys[index] + 1) {
                return keys[index] + 1
            }else{
                index += 1;
            }
        }
    }

    pub fn validate(&self, input: &str) -> Result<(), Box<dyn DasherError>> {
        if input == "" {
            return Ok(())
        }

        for (_, pairs) in &self.cache {
            if pairs.lane == input {
                return Err(Box::new(DashError::new("lane already exists!".to_string())));
            } else if pairs.nickname == input {
                return Err(Box::new(DashError::new(format!("lane with nickname '{input}' already exists!"))));
            }
        }

        return Ok(())
    }

    pub fn swap(&mut self, first_index: IndexNickname, second_index: IndexNickname) -> Result<(), Box<dyn DasherError>> {
        match first_index {
            IndexNickname::Index(index_one) => {
                match second_index {
                    IndexNickname::Index(index_two) => {
                        let mut lane_one = if let Some(lane) = self.cache.remove(&index_one) {
                            lane
                        } else {
                            return Err(Box::new(DashError::new(format!("dsh: lane does not exist at index {index_one}"))));
                        };

                        let mut lane_two = if let Some(lane) = self.cache.remove(&index_one) {
                            lane
                        } else {
                            lane_one.index = index_two;
                            self.add_lane(lane_one);
                            return Ok(());
                        };

                        lane_one.index = index_two;
                        lane_two.index = index_one;

                        self.add_lane(lane_one);
                        self.add_lane(lane_two);
                        return Ok(());
                    },
                    IndexNickname::Nickname(nickname) => {
                        let mut lane = if let Some(lane) = self.cache.remove(&index_one) {
                            lane
                        } else {
                            return Err(Box::new(DashError::new(format!("dsh: does not exist at lane {index_one}"))));
                        };

                        lane.nickname = nickname;

                        self.add_lane(lane);
                        return Ok(())
                    }
                }
            },
            IndexNickname::Nickname(nickname_one) => {
                match second_index {
                    IndexNickname::Nickname(nickname_two) => {
                        let lanes = self.cache.values()
                    }
                }
            }
        }
    }
}










