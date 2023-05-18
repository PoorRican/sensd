use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::{Entry, Iter};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::path::{Path, PathBuf};

use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::writable_or_create;
use crate::io::{DeviceMetadata, IdType, IOEvent};
use crate::settings;
use crate::storage::directory::RootPath;
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
    /// Retain a copy of source metadata for verification and recovery
    metadata: Option<DeviceMetadata>,
    #[serde(skip)]
    /// Store a reference to local root
    ///
    /// This field is not serialized
    root_path: Option<RootPath>,

    /// Collection of `IOEvent` objects
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
    pub fn with_metadata(metadata: &DeviceMetadata) -> Self
    {
        Self::default()
            .set_metadata(metadata.clone())
    }

    /// Getter for device metadata
    ///
    /// # Returns
    ///
    /// An `Option` with:
    /// - `None` if no device is associated
    /// - `Some` containing a reference to internal device metadata
    pub fn metadata(&self) -> Option<&DeviceMetadata> {
        self.metadata.as_ref()
    }

    /// Getter for `name`
    ///
    /// # Returns
    ///
    /// Reference to `String` in local copy of `DeviceMetadata`
    ///
    /// # Panics
    ///
    /// If there is no associated device, a panic is thrown.
    pub fn name(&self) -> &String {
        &self.metadata()
            .expect("No associated device metadata")
            .name
    }

    /// Getter for `id`
    ///
    /// # Returns
    ///
    /// Reference to `IdType` in local copy of `DeviceMetadata`
    ///
    /// # Panics
    ///
    /// If there is no associated device, a panic is thrown.
    pub fn id(&self) -> &IdType {
        &self.metadata()
            .expect("No associated device metadata")
            .id
    }

    /// Setter for `metadata`
    ///
    /// # Parameters
    ///
    /// - `metadata`: Device metadata to store internally
    ///
    /// # Returns
    ///
    /// Ownership of `self` with updated metadata. This is meant to be used by method
    pub fn set_metadata(mut self, metadata: DeviceMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Helper method which returns internal `root_path` or default
    fn root(&self) -> PathBuf {
        if self.root_path.is_some() {
            self.root_path.as_ref().unwrap()
                .deref().clone()
        } else {
            PathBuf::from(settings::DATA_ROOT)
        }
    }

    /// Full path to log file
    ///
    /// # Issues
    ///
    /// - See [#126](https://github.com/PoorRican/sensd/issues/126) which implements validation of `path`.
    ///
    /// # Returns
    ///
    /// `String` of full path *including* filename
    pub fn full_path(&self) -> PathBuf {
        let root = self.root();
        let dir = Path::new(&root);

        dir.join(self.filename())
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
            self.name(),
            self.id().to_string().as_str(),
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

    pub fn set_root(mut self, root: RootPath) -> Self {
        self.set_root_ref(root);
        self
    }

    pub fn set_root_ref(&mut self, root: RootPath) {
        self.root_path = Some(root)
    }

    /// Extend current [`Log`] with [`EventCollection`] from another [`Log`]
    ///
    /// This is used for loading archived logs into memory.
    ///
    /// # Parameters
    ///
    /// - `other`: [`Log`] to pull [`EventCollection`] from
    ///
    /// # Panics
    ///
    /// If both `metadata` fields do not match, then program panics.
    pub fn extend(&mut self, other: &mut Log) {
        if self.metadata != other.metadata {
            panic!("Metadata does not match. Cannot extend");
        }

        self.log.extend(other.log.clone());
    }
}

// Implement save/load operations for `Log`
impl Persistent for Log {
    /// Save log to disk in JSON format
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
    fn save(&self) -> Result<(), ErrorType> {
        if self.log.is_empty() {
            Err(Error::new(
                ErrorKind::ContainerEmpty,
                format!("Log for '{}'. Nothing to save.", self.name()).as_str(),
            ))
        } else {
            let file = writable_or_create(self.full_path());
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
    fn load(&mut self) -> Result<(), ErrorType> {
        if self.log.is_empty() {
            let file = File::open(self.full_path().deref())?;
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
    use crate::io::{Device, Input, IOKind, IdType, RawValue, IOEvent};
    use crate::storage::{Chronicle, Log, Persistent};
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, thread};
    use crate::storage::directory::RootPath;

    fn generate_log(count: usize) -> Log {
        let mut log = Log::default();

        for _ in 0..count {
            let event = IOEvent::new(RawValue::default());
            log.push(event).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }

        log
    }

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
            _log.save().unwrap();

            // save filename for later
            filename = _log.full_path();
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
            _log.load().unwrap();

            // check count of `IOEvent`
            assert_eq!(COUNT, _log.iter().count() as usize);
        };

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn set_root_path() {
        let mut log = Log::default();

        assert!(log.root_path().is_none());

        let root: RootPath = RootPath::new();
        log.set_root_ref(root);

        assert!(log.root_path().is_some())
    }

    #[test]
    fn test_extend() {
        let mut orig = generate_log(50);
        let mut new = generate_log(50);

        assert_eq!(50, orig.iter().count());

        orig.extend(&mut new);

        assert_eq!(100, orig.iter().count())
    }
}
