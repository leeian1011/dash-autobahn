pub struct FileError {
    message: String,
}

impl FileError {
    pub fn new(message: &str) -> self {
        FileError {
            message: message.to_string(),
        }
    }

pub trait DashError {
    fn get_help() -> ();
}

impl DashError for FileError {
    fn get_help() -> () {
        todo!();
    }
}
