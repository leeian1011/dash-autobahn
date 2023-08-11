use std::io::Write;
use crate::dsh::DasherError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Lane {

    pub lane: String,
    pub index: u32,
    pub nickname: String,
}

impl Lane {
    pub fn dash(&self) -> Result<(), Box<dyn DasherError>> {
        let mut writer = std::io::stdout().lock();
        match writer.write_all(self.lane.as_bytes()) {
            Ok(_) => return Ok(()),
            Err(io_err) => return Err(Box::new(io_err)),
        }
    }
}
