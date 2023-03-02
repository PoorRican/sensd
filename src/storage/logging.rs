use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Iter;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::{writable_or_create, Deferred, Deferrable};
use crate::io::{IOEvent, IdType, DeviceType, DeferredDevice, DeviceTraits};
use crate::settings::Settings;
use crate::storage::{Container, MappedCollection, Persistent};

// Defines a type alias `LogType` for a container of IOEvent and DateTime<Utc> objects.
pub type LogType = Container<IOEvent, DateTime<Utc>>;

/// Define the `Deferred` type as an Arc of a Mutex wrapping the generic type `T`.
pub type LogContainer = Vec<Deferred<OwnedLog>>;

const FILETYPE: &str = ".json";

pub trait HasLog {
    fn log(&self) -> Option<Deferred<OwnedLog>>;

    fn add_to_log(&self, event: IOEvent) {
        let log = self.log().expect("No log is associated");
        log.try_lock().unwrap()
            .push(event.timestamp, event).expect("Unknown error when adding event to log");
    }
}

// Encapsulates a `LogType` alongside a weak reference to a `Device`
#[derive(Serialize, Deserialize, Default)]
pub struct OwnedLog {
    id: IdType,
    #[serde(skip)]
    owner: Option<Weak<Mutex<DeviceType>>>,
    #[serde(skip)]
    settings: Arc<Settings>,

    log: LogType,
}

impl OwnedLog {
    pub fn owner(&self) -> DeferredDevice {
        // TODO: handle error if owner is None or if Weak has no Strong
        self.owner.clone().unwrap().upgrade().unwrap()
    }

    pub fn set_owner(&mut self, owner: Weak<Mutex<DeviceType>>) {
        self.owner = Some(owner);
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
        let owner = self.owner();
        format!(
            "{}_{}_{}{}",
            self.settings.log_fn_prefix.clone(),
            owner.name().as_str(),
            self.id.to_string().as_str(),
            FILETYPE
        )
    }

    pub fn iter(&self) -> Iter<DateTime<Utc>, IOEvent> {
        self.log.iter()
    }

    pub fn orphan(&self) -> bool {
        match self.owner {
            Some(_) => false,
            None => true,
        }
    }
}

impl MappedCollection<IOEvent, DateTime<Utc>> for OwnedLog {
    fn push(&mut self, key: DateTime<Utc>, data: IOEvent) -> Result<&mut IOEvent, ErrorType> {
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
    fn save(&self, path: &Option<String>) -> Result<(), ErrorType> {
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
                    return Err(Error::new(ErrorKind::SerializationError, msg.as_str()));
                }
            }
            Ok(())
        }
    }

    fn load(&mut self, path: &Option<String>) -> Result<(), ErrorType> {
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

impl Deferrable for OwnedLog {
    type Inner = OwnedLog;
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(self))
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

// Testing
#[cfg(test)]
mod tests {
    use crate::builders::DeviceLogBuilder;
    use crate::helpers::Deferred;
    use crate::io::{Device, IOKind, IdType, DeviceType, IODirection, IOType};
    use crate::storage::{MappedCollection, OwnedLog, Persistent};
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, thread};
    use std::ops::Deref;
    use crate::action::IOCommand;

    fn add_to_log(device: &Deferred<DeviceType>, log: &Deferred<OwnedLog>, count: usize) {
        for _ in 0..count {
            let binding = device.lock().unwrap();
            let event = match binding.deref() {
                DeviceType::Input(inner) => inner.generate_event(IOType::default()),
                DeviceType::Output(inner) => inner.generate_event(IOType::default()),
            };
            log.lock().unwrap().push(event.timestamp, event).unwrap();
            thread::sleep(Duration::from_nanos(1)); // add delay so that we don't finish too quickly
        }
    }

    #[test]
    fn test_load_save() {
        const SENSOR_NAME: &str = "test";
        const ID: IdType = 32;
        const COUNT: usize = 10;
        const COMMAND: IOCommand = IOCommand::Input(move || IOType::default());

        /* NOTE: More complex `IOEvent` objects *could* be checked, but we are trusting `serde`.
        These tests only count the number of `IOEvent`'s added. */

        let filename;
        // test save
        {
            let builder = DeviceLogBuilder::new(SENSOR_NAME, &ID, &Some(IOKind::Flow),
                                                &IODirection::Input, &COMMAND, None);
            let (device, log) = builder.get();
            add_to_log(&device, &log, COUNT);
            let _log = log.lock().unwrap();
            _log.save(&None).unwrap();

            // save filename for later
            filename = _log.filename();
            // check that file exists
            assert!(Path::new(&filename).exists());
        };

        // test load
        // build back up then load
        {
            let builder = DeviceLogBuilder::new(SENSOR_NAME, &ID, &Some(IOKind::Flow),
                                                &IODirection::Input, &COMMAND, None);
            let (_device, log) = builder.get();
            let mut _log = log.lock().unwrap();
            _log.load(&None).unwrap();

            // check count of `IOEvent`
            assert_eq!(COUNT, _log.length() as usize);
        };

        fs::remove_file(filename).unwrap();
    }
}