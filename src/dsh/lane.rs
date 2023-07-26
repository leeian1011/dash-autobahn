use serde::{Serialize, Deserialize};

use super::codes::ActionCode;
use super::dasher::Dashable;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lane {
    nickname: Option<String>,
    index: u64,
    lane: String,
}

impl Lane {
    pub fn new() -> Self {
        Lane {
            nickname: todo!(),
            index: todo!(),
            lane: todo!(),
        }
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

    pub fn get_nickname(&self) -> &Option<String> {
        &self.nickname
    }

    pub fn get_lane(&self) -> &String {
        &self.lane
    }
}

impl Dashable for Lane {
    fn dash(&self) -> ActionCode {
        todo!("Implement 'goto'")
    }
}
