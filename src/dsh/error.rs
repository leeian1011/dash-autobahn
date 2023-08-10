use std::fmt;

pub trait DasherError: fmt::Debug{
    fn log(&self) -> ();
}

#[derive(Debug)]
pub struct DashError {
    pub message: String,
}

impl DashError {
    pub fn new(message: String) -> Self {
        DashError {
            message,
        }
    }
}

impl DasherError for DashError {
    fn log(&self) -> () {
        eprintln!("dsh: {}", self.message)
    }
}

impl DasherError for std::io::Error {
    fn log(&self) -> () {
        eprintln!("dsh: could not load lanes.\n{:?}", self)
    }
}

impl DasherError for serde_json::Error {
    fn log(&self) -> () {
        eprintln!("dsh: could not read/write lanes.\n{:?}", self)
    }
}

impl DasherError for std::string::FromUtf8Error {
    fn log(&self) -> () {
        eprintln!("dsh: could not parse into string.\n{:?}", self)
    }
}

impl DasherError for std::env::VarError {
    fn log(&self) -> () {
        eprintln!("dsh: {:?}", self)
    }
}




