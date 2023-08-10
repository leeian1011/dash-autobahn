use serde::{Serialize, Deserialize};

use super::codes::ActionCode;

pub trait Dashable {
    fn dash(&self) -> ActionCode;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lane {

    pub lane: String,
    pub index: u32,
    pub nickname: String,
}


impl Dashable for Lane {
    fn dash(&self) -> ActionCode {
        todo!("Implement 'goto'")
    }
}
