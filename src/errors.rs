use std::error::Error as _Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum ErrorKind {
    ContainerError,
    ContainerEmpty,
    ContainerNotEmpty,
    SerializationError,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Box<dyn _Error> {
        let message = String::from(msg);
        Box::new( Error { kind, message })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let pretext = match self.kind {
            ErrorKind::ContainerError => "Error from `Container` method",
            ErrorKind::ContainerEmpty => "Container is empty",
            ErrorKind::ContainerNotEmpty => "Container is not empty",
            ErrorKind::SerializationError => "Error during serialization",
        };

        write!(f, "{}: {}", pretext, self.message)
    }
}

impl _Error for Error {}