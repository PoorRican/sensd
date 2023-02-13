use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

use crate::errors::{Error, ErrorKind, Result};
use crate::helpers::{writable_or_create, Deferred};
use crate::io::{Device, IOEvent, IdType, InputType};
use crate::settings::Settings;
use crate::storage::{Container, MappedCollection, Persistent};

// Defines a type alias `LogType` for a container of IOEvent and DateTime<Utc> objects.
pub type LogType = Container<IOEvent, DateTime<Utc>>;

/// Define the `Deferred` type as an Arc of a Mutex wrapping the generic type `T`.
pub type LogContainer = Vec<Deferred<OwnedLog>>;

const FILETYPE: &str = ".json";

// Encapsulates a `LogType` alongside a weak reference to a `Device`
#[derive(Serialize, Deserialize, Default)]
pub struct OwnedLog {
    id: IdType,
    #[serde(skip)]
    owner: Option<Weak<Mutex<InputType>>>,
    #[serde(skip)]
    settings: Arc<Settings>,

    log: LogType,
}

impl OwnedLog {
    pub fn owner(&self) -> Deferred<InputType> {
        // TODO: handle error if owner is None or if Weak has no Strong
        self.owner.clone().unwrap().upgrade().unwrap()
    }

    pub fn set_owner(&mut self, owner: Deferred<InputType>) {
        self.owner = Some(Arc::downgrade(&owner));
    }

    /// Append filename to path
    fn full_path(&self, path: &Option<String>) -> String {
        let prefix = path.clone().unwrap_or_else(|| String::from(""));

        format!("{}{}", prefix, self.filename())
    }

    pub fn new(id: IdType, settings: Option<Arc<Settings>>) -> Self {
        let owner = None;
        let log = LogType::default();
        Self {
            id,
            owner,
            log,
            settings: settings.unwrap_or_else(|| Arc::new(Settings::default())),
        }
    }

    pub fn filename(&self) -> String {
        let owner: Deferred<InputType> = self.owner();
        format!(
            "{}_{}_{}{}",
            self.settings.log_fn_prefix.clone(),
            owner.lock().unwrap().name().as_str(),
            self.id.to_string().as_str(),
            FILETYPE
        )
    }

    pub fn iter(&self) -> Iter<DateTime<Utc>, IOEvent> {
        self.log.iter()
    }
}

impl MappedCollection<IOEvent, DateTime<Utc>> for OwnedLog {
    fn push(&mut self, key: DateTime<Utc>, data: IOEvent) -> Result<()> {
        self.log.push(key, data)
    }

    fn get(&self, key: DateTime<Utc>) -> Option<&IOEvent> {
        self.log.get(key)
    }

    fn remove(&mut self, key: DateTime<Utc>) -> Option<IOEvent> {
        self.log.remove(key)
    }

    fn is_empty(&self) -> bool {
        self.log.is_empty()
    }

    fn length(&self) -> usize {
        self.log.length()
    }
}

// Implement save/load operations for `LogType`
impl Persistent for OwnedLog {
    fn save(&self, path: &Option<String>) -> Result<()> {
        if self.log.is_empty() {
            Err(Error::new(
                ErrorKind::ContainerEmpty,
                "Log is empty. Will not save.",
            ))
        } else {
            let file = writable_or_create(self.full_path(path));
            let writer = BufWriter::new(file);

            match serde_json::to_writer_pretty(writer, &self) {
                Ok(_) => println!("Saved"),
                Err(e) => {
                    let msg = e.to_string();
                    dbg!(msg.clone());
                    return Err(Error::new(
                        ErrorKind::SerializationError,
                        msg.as_str(),
                    ))
                }
            }
            Ok(())
        }
    }

    fn load(&mut self, path: &Option<String>) -> Result<()> {
        if self.log.is_empty() {
            let file = File::open(self.full_path(path).deref())?;
            let reader = BufReader::new(file);

            let buff: OwnedLog = match serde_json::from_reader(reader) {
                Ok(data) => data,
                Err(e) => {
                    return Err(Error::new(
                        ErrorKind::SerializationError,
                        e.to_string().as_str(),
                    ))
                }
            };
            self.log = buff.log;
            Ok(())
        } else {
            Err(Error::new(
                ErrorKind::ContainerNotEmpty,
                "Cannot load objects into non-empty container",
            ))
        }
    }
}

impl std::fmt::Debug for OwnedLog {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Log {{ name: {}, log length: {} }}",
            self.filename(),
            self.log.length()
        )
    }
}
