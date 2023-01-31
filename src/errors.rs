use std::error::Error as _Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum Error {
    ContainerEmpty(Option<String>),
    ContainerNotEmpty(Option<String>),
}

impl Error {
    pub fn new(error_type: Error, message: &str) -> Box<dyn _Error> {
        let msg = String::from(message);
        Box::new( match error_type {
            Error::ContainerEmpty(ref _msg) => Error::ContainerEmpty(Some(msg)),
            Error::ContainerNotEmpty(ref _msg) => Error::ContainerNotEmpty(Some(msg)),
        })
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ContainerEmpty(ref message) => {
                if let Some(ref message) = message {
                    write!(f, "Container is empty: {}", message)
                } else {
                    write!(f, "Container is empty")
                }
            },
            Error::ContainerNotEmpty(ref message) => {
                if let Some(ref message) = message {
                    write!(f, "Container is not empty: {}", message)
                } else {
                    write!(f, "Container is not empty")
                }
            },
        }
    }
}

impl _Error for Error {}