use chrono::{DateTime, Utc};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::errors;
use crate::errors::{Error, ErrorKind};
use crate::helpers::{Deferred, write_or_create};
use crate::io::{IdType, IOEvent};
use crate::storage::{Container, MappedCollection, Persistent};

pub type LogType = Container<IOEvent, DateTime<Utc>>;

/// Define the `Deferred` type as an Arc of a Mutex wrapping the generic type `T`.
pub type LogContainer = Container<Deferred<LogType>, IdType>;

// Implement save/load operations for `LogType`
impl Persistent for LogType {
    fn save(&self, path: &Option<String>) -> errors::Result<()> {
        if self.is_empty() {
            Err(Error::new(ErrorKind::ContainerEmpty, "Log is empty. Will not save."))
        } else {

            let file = write_or_create(path.clone().unwrap());
            let writer = BufWriter::new(file);

            match serde_json::to_writer_pretty(writer, &self) {
                Ok(_) => println!("Saved"),
                Err(e) =>
                    return Err(Error::new(ErrorKind::SerializationError,
                                          e.to_string().as_str()))
            }
            Ok(())
        }
    }

    fn load(&mut self, path: &Option<String>) -> errors::Result<()> {
        if self.is_empty() {
            let file = File::open(&path.clone().unwrap())?;
            let reader = BufReader::new(file);
            match serde_json::from_reader(reader) {
                Ok(log) => self.inner = log,
                Err(e) =>
                    return Err(Error::new(ErrorKind::SerializationError,
                                          e.to_string().as_str()))
            }
            Ok(())
        } else {
            Err(Error::new(ErrorKind::ContainerNotEmpty, "Cannot load objects into non-empty container"))
        }
    }
}
