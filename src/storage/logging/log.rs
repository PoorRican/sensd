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
use crate::storage::{EventCollection, Persistent, FILETYPE, Document};


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
    dir: Option<PathBuf>,

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

/// - See [#126](https://github.com/PoorRican/sensd/issues/126) which implements validation of `path`.
impl Document for Log {
    fn dir(&self) -> Option<&PathBuf> {
        self.dir.as_ref()
    }

    fn set_dir_ref<P>(&mut self, path: P) -> &mut Self
        where Self: Sized,
              P: AsRef<Path>
    {
        self.dir = Some(PathBuf::from(path.as_ref()));

        self
    }

    /// Generate generic filename based on settings, owner, and id
    ///
    /// # Returns
    ///
    /// A formatted filename as [`String`] with JSON filetype prefix.
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
}

// Testing
#[cfg(test)]
mod tests {
    use crate::io::{IOKind, RawValue, IOEvent, DeviceMetadata, IODirection};
    use crate::storage::{Document, Log, Persistent};
    use std::path::Path;
    use std::time::Duration;
    use std::{fs, thread};

    fn generate_log<'meta, M>(count: usize, metadata: M) -> Log
        where
            M: Into<Option<&'meta DeviceMetadata>>
    {
        let mut log;
        match metadata.into() {
            Some(meta) => log = Log::with_metadata(meta),
            None => log = Log::default(),
        }

        for _ in 0..count {
            let event = IOEvent::new(RawValue::default());
            log.push(event).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }

        log
    }

    #[test]
    fn test_load_save() {
        const COUNT: usize = 10;
        const TMP_DIR: &str = "/tmp/device/";

        let metadata = DeviceMetadata::new(
            "test",
            32,
            IOKind::Unassigned,
            IODirection::In,
        );

        /* NOTE: More complex `IOEvent` objects *could* be checked, but we are trusting `serde`.
        These tests only count the number of `IOEvent`'s added. */

        let filename;
        // test save
        {
            let log =
                generate_log(COUNT, &metadata)
                    .set_dir(TMP_DIR);

            log.save().unwrap();

            // save filename for later
            filename = log.full_path();
            // check that file exists
            assert!(Path::new(&filename).exists());
        };

        // test load
        // build back up then load
        {
            let mut log = Log::with_metadata(&metadata)
                .set_dir(TMP_DIR);

            log.load().unwrap();

            // check count of `IOEvent`
            assert_eq!(COUNT, log.iter().count() as usize);
        };

        fs::remove_file(filename).unwrap();
    }

    #[test]
    fn set_dir() {
        let mut log = Log::default();

        assert!(log.dir().is_none());

        log.set_dir_ref("");

        assert!(log.dir().is_some())
    }

    #[test]
    fn test_extend() {
        let mut orig = generate_log(50, None);
        let mut new = generate_log(50, None);

        assert_eq!(50, orig.iter().count());

        orig.extend(&mut new);

        assert_eq!(100, orig.iter().count())
    }
}
