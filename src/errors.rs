use std::error::Error as _Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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
            Self::ContainerError => "Error from `Container` method",
            Self::ContainerEmpty => "Container is empty",
            Self::ContainerNotEmpty => "Container is not empty",
            Self::SerializationError => "Error during serialization",
        };

        match self.message {
            Some(message) => write!(f, "{}: {}", pretext, message),
            _ => write!(f, pretext),
        }
    }
}

impl _Error for Error {}