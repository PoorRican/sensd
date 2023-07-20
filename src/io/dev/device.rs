//! Defines common interfaces for interacting with GPIO devices.
//!
//! There are two device types: [`crate::io::Input`] and [`crate::io::Output`].
//! Both share the same constructors and builder methods, and share the
//! super-traits [`DeviceGetters`] and [`DeviceSetters`].
//!
//! # See Also
//!
//! - [`DeviceMetadata`] for user defined metadata and field descriptions

use std::path::{Path};
use crate::action::IOCommand;
use crate::helpers::Def;
use crate::io::{DeviceMetadata, IODirection, IOKind, IdType, Datum};
use crate::storage::Document;
use crate::storage::{Chronicle, Log, Persistent};
use crate::errors::ErrorType;
use crate::name::Name;

/// Common constructors and builder methods for all device types
pub trait Device: Name + Chronicle + DeviceGetters + DeviceSetters + Persistent {
    /// Creates a new instance of the device with the given parameters.
    ///
    /// # Parameters
    ///
    /// - `name`: name of device.
    /// - `id`: device ID.
    /// - `kind`: kind of I/O device. Optional argument.
    fn new<N, K>(name: N, id: IdType, kind: K) -> Self
    where
        Self: Sized,
        N: Into<String>,
        K: Into<Option<IOKind>>;

    /// Setter for `command` field as builder method
    ///
    /// # Notes
    ///
    /// Since this function is a builder command, and is meant to be used with method chaining,
    /// it is not included in [`DeviceSetters`]
    ///
    /// # Returns
    ///
    /// Passes ownership of `self`
    fn set_command(self, command: IOCommand) -> Self
    where
        Self: Sized;

    /// Initialize, set, and return log.
    fn init_log(mut self) -> Self
    where
        Self: Sized,
    {
        let log = Def::new(Log::with_metadata(&self.metadata()));
        self.set_log(log);
        self
    }

    fn into_deferred(self) -> Def<Self>
    where
        Self: Sized
    {
        Def::new(self)
    }
}

/// Common getter methods shared by all device types
pub trait DeviceGetters {
    /// Reference to device metadata
    ///
    /// Information such as `name`, `id`, `kind`, and `direction` are taken from metadata.
    ///
    /// # Returns
    ///
    /// An immutable reference to internal device metadata
    ///
    /// # See Also
    ///
    /// - [`DeviceMetadata`]
    fn metadata(&self) -> &DeviceMetadata;

    /// Returns the ID of the device.
    fn id(&self) -> IdType {
        self.metadata().id
    }

    /// Returns the I/O direction of the device.
    fn direction(&self) -> IODirection {
        self.metadata().direction
    }

    /// Returns the type of device as `IOKind`.
    fn kind(&self) -> IOKind {
        self.metadata().kind
    }

    /// Immutable reference to cached state
    ///
    /// # Returns
    ///
    /// An `Option` that is:
    /// - `None` upon initialization since device has not been read from or written to.
    /// - `Datum` after first read or write, and represents last known state.
    fn state(&self) -> &Option<Datum>;
}

/// Command setter methods share by all device types
pub trait DeviceSetters {
    fn set_id(&mut self, id: IdType);

    /// Setter for `log` field
    fn set_log(&mut self, log: Def<Log>);
}

impl<T: Device> Persistent for T {
    fn save(&self) -> Result<(), ErrorType> {
        match self.log() {
            Some(log) => log.try_lock().unwrap().save(),
            None => Ok(())
        }
    }

    fn load(&mut self) -> Result<(), ErrorType> {
        match self.log() {
            Some(log) => log.try_lock().unwrap().load(),
            None => Ok(())
        }
    }
}

/// Helper for setting log directory
pub fn set_log_dir<S>(log: Option<Def<Log>>, path: S)
    where
        S: AsRef<Path>
{
    match log {
        Some(inner) => {
            let mut log =
                inner.try_lock()
                    .expect("Log is poisoned");
            log.set_dir_ref(path);
        },
        None => ()
    }
}