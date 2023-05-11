use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::path::Path;

use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::writable_or_create;
use crate::io::{DeviceMetadata, IOEvent, IdType};
use crate::settings;
use crate::settings::RootPath;
use crate::storage::{EventCollection, Persistent, FILETYPE};


/// A record of [`IOEvent`]s from a single device keyed by datetime
///
/// Encapsulates a [`EventCollection`] along with information of originating source.
///
/// # Usage
///
/// Since log is used in multiple places throughout the input and control action lifecycle, it should be wrapped
/// behind `Def`.
#[derive(Serialize, Deserialize, Default)]
pub struct Log {
    // TODO: split logs using ID
    id: IdType,
    #[serde(skip)]
    name: String,
    #[serde(skip)]
    root_path: Option<RootPath>,

    log: EventCollection,
}

impl Log {

    /// Constructor for [`Log`]
    ///
    /// # Parameters
    ///
    /// - `metadata`: Reference to [`DeviceMetadata`] of originating device
    ///
    /// # Returns
    ///
    /// Empty log with identity attributes belonging to given device.
    pub fn new(metadata: &DeviceMetadata) -> Self
    {
        let id = metadata.id;
        let name = metadata.name.clone();
        let log = EventCollection::default();
        let root_path = None;

        Self {
            id,
            name,
            log,
            root_path,
        }
    }

    /// Helper method which returns internal `root_path` or default
    fn root(&self) -> String {
        if self.root_path.is_some() {
            self.root_path.as_ref().unwrap().to_string()
        } else {
            settings::DATA_ROOT.to_string()
        }
    }

    /// Full path to log file
    ///
    /// # Parameters:
    ///
    /// - `path`: Optional argument to override typical storage path defined by [`Settings`]. When passed,
    ///           filename is appended to given path.
    ///
    /// # Issues
    ///
    /// - See [#126](https://github.com/PoorRican/sensd/issues/126) which implements validation of `path`.
    ///
    /// # Returns
    ///
    /// `String` of full path *including filename*
    fn full_path(&self, path: &Option<String>) -> String {
        let root = self.root();
        let prefix = path.as_ref().unwrap_or(&root);
        let dir = Path::new(prefix);

        let full_path = dir.join(self.filename());
        String::from(full_path.to_str().unwrap())
    }

    /// Generate generic filename based on settings, owner, and id
    ///
    /// # Returns
    ///
    /// A formatted filename as `String` with JSON filetype prefix.
    ///
    /// # See Also
    ///
    /// - [`FILETYPE`] for definition of filetype suffix
    fn filename(&self) -> String {
        format!(
            "{}_{}_{}{}",
            settings::LOG_FN_PREFIX,
            self.name,
            self.id.to_string().as_str(),
            FILETYPE
        )
    }

    /// Iterator over keys and values
    ///
    /// # Returns
    ///
    /// Iterator that returns ([`DateTime<Utc>`], [`IOEvent`]).
    pub fn iter(&self) -> Iter<DateTime<Utc>, IOEvent> {
        self.log.iter()
    }

    /// Push a new event to log
    ///
    /// # Parameters
    ///
    /// - `event`: new event to append
    ///
    /// # Returns
    ///
    /// A `Result` that contains:
    ///
    /// - `Ok`: with a reference to inserted log is inserted when [`IOEvent.timestamp`] does not exist in log
    /// - `Err`: with an [`ErrorKind::ContainerError`] error if timestamp already exists in log
    pub fn push(
        &mut self,
        event: IOEvent,
    ) -> Result<&mut IOEvent, ErrorType> {
        match self.log.entry(event.timestamp) {
            Entry::Occupied(_) => Err(Error::new(ErrorKind::ContainerError, "Key already exists")),
            Entry::Vacant(entry) => Ok(entry.insert(event)),
        }
    }

    pub fn root_path(&self) -> Option<RootPath> {
        self.root_path.clone()
    }

    pub fn set_root(&mut self, root: RootPath) {
        self.root_path = Some(root)
    }
}

// Implement save/load operations for `Log`
impl Persistent for Log {
    /// Save log to disk in JSON format
    ///
    /// # Parameters
    ///
    /// - `path`: path to save to. This path should not include a filename.
    ///
    /// # Issues
    ///
    /// - See [#126](https://github.com/PoorRican/sensd/issues/126) which implements validation of `path`.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    ///
    /// - `Ok`: with `()` when log is not empty, and serialization and write to disk is successful.
    /// - `Err`: with appropriate error when `Log` is empty *OR*
    ///   when an error is returned by[`serde_json::to_writer_pretty()`].
    ///
    /// # See Also
    ///
    /// - [`Log::full_path()`] explains usage of `path` parameter.
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

    /// Load log from JSON file
    ///
    /// # Parameters
    ///
    /// - `path`: path to read and load from. This path should not include a filename.
    ///
    /// # Issues
    ///
    /// - See [#126](https://github.com/PoorRican/sensd/issues/126) which implements validation of `path`.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    ///
    /// - `Ok()`: with `()` when loading from disk and deserialization is successful.
    /// - `Err`: with appropriate error when `Log` is not empty, when path/file is not valid, *OR*
    ///   when an error is returned by[`serde_json::from_reader()`]
    ///
    /// # See Also
    ///
    /// - [`Log::full_path()`] explains usage of `path` parameter.
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
    use crate::io::{Device, Input, IOKind, IdType, RawValue};
    use crate::storage::{Chronicle, Log, Persistent};
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, thread};
    use std::sync::Arc;
    use crate::settings::RootPath;

    fn add_to_log<D>(device: &D, log: &Def<Log>, count: usize)
    where
        D: Device
    {
        for _ in 0..count {
            let event = device.generate_event(RawValue::default());
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
            let device = Input::new(String::from(SENSOR_NAME), ID, IOKind::Flow)
                .set_command(COMMAND)
                .init_log();
            let log = device.log().unwrap();

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
            let device = Input::new(SENSOR_NAME, ID, IOKind::Flow)
                .set_command(COMMAND)
                .init_log();
            let log = device.log().unwrap();

            let mut _log = log.lock().unwrap();
            _log.load(&None).unwrap();

            // check count of `IOEvent`
            assert_eq!(COUNT, _log.iter().count() as usize);
        };

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn set_root_path() {
        let mut log = Log::default();

        assert!(log.root_path().is_none());

        let root: RootPath = Arc::new(String::new());
        log.set_root(root);

        assert!(log.root_path().is_some())
    }
}
