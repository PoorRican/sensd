use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::errors::{Error, ErrorKind, Result};
use crate::storage::logging::LogType;
use crate::storage::{Container, Containerized, MappedCollection};

// trait that expresses an interface to save or load from disk
pub trait Persistent {
    // save data to disk
    fn save(&self, path: Option<String>) -> Result<()>;

    // load from disk
    fn load(&mut self, path: Option<String>) -> Result<()>;
}