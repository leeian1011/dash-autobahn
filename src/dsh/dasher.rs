#![allow(unused_variables, dead_code)]
use super::{codes::ActionCode, lane::Lane, error::{FileError, DashError}};
use std::collections::BTreeMap;
use std::{fs::File, io::{Read, Write, Error}};
pub struct Dasher {
    cache: BTreeMap<String, Lane>
}

impl Dasher {
    fn load_lanes() -> Result<BTreeMap<String, Lane>, DashError> {
        let mut loaded_lanes: Vec<Lane> = Vec::new();
        let mut tree_map: BTreeMap<String, Lane> = BTreeMap::<String, Lane>::new();
        let mut cache: String = String::new();
        match File::open("cache/lanes.json") {
            Ok(file) => file.read_to_string(&mut cache),
            Err(_) => FileError::new("could not load lanes from cache"),
        }

        loaded_lanes = serde_json::from_str::<Vec<Lane>>(&cache)
            .unwrap_or_else(|serde_err| {
                FileError::new(format!("could not load dasher via serde: {}", e).as_str())
            });
        
        loaded_lanes.into_iter()
            .map(|lane|
                 tree_map.insert(lane.get_index(), lane)
            );

        Ok(map)
    }
    
    fn save_lanes() -> bool {
        todo!();
    }

    pub fn new(&self) -> Dasher {
        Dasher {
            cache: Dasher::load_lanes().unwrap_or_else(|e| panic!("Unhandled File Read Error: {e}")),
        }
    }

    fn insert_new_lane(&mut self, lane: Lane) -> bool {
        todo!();
    }

}




///[`Dashable`] is a trait that allows [`Lanes`] to run "dash".
pub trait Dashable {
    fn dash(&self) -> ActionCode;
}


