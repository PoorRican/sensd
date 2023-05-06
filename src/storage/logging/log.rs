//! Datalogging of `IOEvent` objects
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::writable_or_create;
use crate::io::{DeviceMetadata, IOEvent, IdType};
use crate::settings::Settings;
use crate::storage::{EventCollection, Persistent};


/// Default filetype suffix.
///
/// Used by `Log::filename()`, but this should probably be moved to settings
const FILETYPE: &str = ".json";


/// Log abstraction of `IOEvent` keyed by datetime
///
/// Encapsulates a `LogType` alongside a weak reference to a `Device`
#[derive(Serialize, Deserialize)]
pub struct Log {
    // TODO: split logs using ID
    id: IdType,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    settings: Arc<Settings>,

    log: EventCollection,
}

impl Log {
    /// Full path to log file.
    ///
    /// No directories or files are created by this function.
    ///
    /// # Args:
    /// path: Optional argument to override typical storage path
    fn full_path(&self, path: &Option<String>) -> String {
        let prefix = path.as_ref().unwrap_or_else(|| &self.settings.data_root);
        let dir = Path::new(prefix);

        let full_path = dir.join(self.filename());
        String::from(full_path.to_str().unwrap())
    }

    pub fn new(metadata: &DeviceMetadata, settings: Option<Arc<Settings>>) -> Self {
        let id = metadata.id;
        let name = metadata.name.clone();
        let log = EventCollection::default();

        Self {
            id,
            name,
            log,
            settings: settings.unwrap_or_else(|| Arc::new(Settings::default())),
        }
    }

    /// Generate generic filename based on settings, owner, and id
    pub fn filename(&self) -> String {
        format!(
            "{}_{}_{}{}",
            self.settings.log_fn_prefix.clone(),
            self.name,
            self.id.to_string().as_str(),
            FILETYPE
        )
    }

    /// Iterator for log
    pub fn iter(&self) -> Iter<DateTime<Utc>, IOEvent> {
        self.log.iter()
    }

    pub fn push(
        &mut self,
        event: IOEvent,
    ) -> Result<&mut IOEvent, ErrorType> {
        match self.log.entry(event.timestamp) {
            Entry::Occupied(_) => Err(Error::new(ErrorKind::ContainerError, "Key already exists")),
            Entry::Vacant(entry) => Ok(entry.insert(event)),
        }
    }
}

// Implement save/load operations for `Log`
impl Persistent for Log {
    fn save(&self, path: &Option<String>) -> Result<(), ErrorType> {
        if self.log.is_empty() {
            Err(Error::new(
                ErrorKind::ContainerEmpty,
                format!("Log for '{}'. Nothing to save.", self.name).as_str(),
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

            let buff: Log = match serde_json::from_reader(reader) {
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

// Testing
#[cfg(test)]
mod tests {
    use crate::action::IOCommand;
    use crate::helpers::Def;
    use crate::io::{Device, DeviceType, GenericInput, IOKind, IdType, RawValue};
    use crate::storage::{Chronicle, Log, Persistent};
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, thread};

    fn add_to_log(device: &DeviceType, log: &Def<Log>, count: usize) {
        for _ in 0..count {
            let event = match device {
                DeviceType::Input(inner) => inner.generate_event(RawValue::default()),
                DeviceType::Output(inner) => inner.generate_event(RawValue::default()),
            };
            log.lock().unwrap().push(event).unwrap();
            thread::sleep(Duration::from_nanos(1)); // add delay so that we don't finish too quickly
        }
    }

    #[test]
    fn test_load_save() {
        const SENSOR_NAME: &str = "test";
        const ID: IdType = 32;
        const COUNT: usize = 10;
        const COMMAND: IOCommand = IOCommand::Input(move || RawValue::default());

        /* NOTE: More complex `IOEvent` objects *could* be checked, but we are trusting `serde`.
        These tests only count the number of `IOEvent`'s added. */

        let filename;
        // test save
        {
            let device = GenericInput::new(String::from(SENSOR_NAME), ID, Some(IOKind::Flow))
                .set_command(COMMAND)
                .init_log(None);
            let log = device.log().unwrap();

            let device = device.into_variant();

            add_to_log(&device, &log, COUNT);
            let _log = log.lock().unwrap();
            _log.save(&None).unwrap();

            // save filename for later
            filename = _log.full_path(&None);
            // check that file exists
            assert!(Path::new(&filename).exists());
        };

        // test load
        // build back up then load
        {
            let device = GenericInput::new(String::from(SENSOR_NAME), ID, Some(IOKind::Flow))
                .set_command(COMMAND)
                .init_log(None);
            let log = device.log().unwrap();

            let mut _log = log.lock().unwrap();
            _log.load(&None).unwrap();

            // check count of `IOEvent`
            assert_eq!(COUNT, _log.iter().count() as usize);
        };

        fs::remove_file(filename).unwrap();
    }
}
