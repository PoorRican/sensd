use std::error::Error as _Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    ContainerEmpty(Option<String>),
    ContainerNotEmpty(Option<String>),
    SerializationError(Option<String>),
}

impl Error {
    pub fn new(error_type: Error, message: &str) -> Box<dyn _Error> {
        let msg = String::from(message);
        Box::new( match error_type {
            Self::ContainerEmpty(ref _msg) => Self::ContainerEmpty(Some(msg)),
            Self::ContainerNotEmpty(ref _msg) => Self::ContainerNotEmpty(Some(msg)),
            Self::SerializationError(ref _msg) => Self::SerializationError(Some(msg)),
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ContainerEmpty(ref message) => {
                if let Some(ref message) = message {
                    write!(f, "Container is empty: {}", message)
                } else {
                    write!(f, "Container is empty")
                }
            },
            Self::ContainerNotEmpty(ref message) => {
                if let Some(ref message) = message {
                    write!(f, "Container is not empty: {}", message)
                } else {
                    write!(f, "Container is not empty")
                }
            },
            Self::SerializationError(ref message) => {
                if let Some(ref message) = message {
                    write!(f, "Error from Serializer: {}", message)
                } else {
                    write!(f, "Error during serialization")
                }
            }
        }
    }
}

impl _Error for Error {}