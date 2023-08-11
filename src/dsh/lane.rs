use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lane {

    pub lane: String,
    pub index: u32,
    pub nickname: String,
}

impl Lane {
    pub fn dash() -> () {
        todo!();
    }
}
