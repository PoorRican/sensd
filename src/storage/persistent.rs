use std::fs::File;
use std::io::{BufReader, BufWriter};
use crate::errors::{Error, ErrorKind, Result};
use crate::io::LogType;
use crate::storage::{Container, Containerized, MappedCollection};

// trait that expresses an interface to save or load from disk
pub trait Persistent {
    // save data to disk
    fn save(&self, path: Option<String>) -> Result<()>;

    // load from disk
    fn load(&mut self, path: Option<String>) -> Result<()>;
}

// Implement save/load operations for `LogType`
impl Persistent for LogType {
    fn save(&self, path: Option<String>) -> Result<()> {
        if self.is_empty() {
            Err(Error::new(ErrorKind::ContainerEmpty, "`sensors` is empty. Cannot save."))
        } else {

            let file = File::options().write(true).open(&path.clone().unwrap())
                .unwrap_or_else(move |_| {
                    File::create(&path.clone().unwrap()).unwrap();
                    File::options().write(true).open(&path.clone().unwrap()).unwrap()
                });
            let writer = BufWriter::new(file);

            dbg!(serde_json::to_string(&self.inner)?);
            match serde_json::to_writer_pretty(writer, &self.inner) {
                Ok(_) => println!("Saved"),
                Err(e) =>
                    return Err(Error::new(ErrorKind::SerializationError,
                                          e.to_string().as_str()))
            }
            Ok(())
        }
    }

    fn load(&mut self, path: Option<String>) -> Result<()> {
        if self.is_empty() {
            let file = File::open(&path.unwrap())?;
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