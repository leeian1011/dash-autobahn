use std::{fmt, io::{stdout, Write}, process::exit};

use crate::ShellSignal;

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
        eprintln!("dsh: could not parse into string.{:?}", self)
    }
}

impl DasherError for std::env::VarError {
    fn log(&self) -> () {
        eprintln!("dsh: {:?}", self)
    }
}


impl DasherError for DashError {
    fn log(&self) -> () {
        let message = format!("dsh: {}\n", self.message);

        let mut stdout = stdout().lock();

        if let Err(io_err) = stdout.write_all(message.as_bytes()) {
            io_err.log();
            exit(ShellSignal::ERROR as i32);
        }
    }
}


