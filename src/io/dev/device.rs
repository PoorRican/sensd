//! Defines common interfaces for interacting with GPIO devices.
//!
//! There are two device types: [`crate::io::Input`] and [`crate::io::Output`].
//! Both share the same constructors and builder methods, and share the
//! super-traits [`DeviceGetters`] and [`DeviceSetters`].
//!
//! # See Also
//!
//! - [`DeviceMetadata`] for user defined metadata and field descriptions

use crate::action::IOCommand;
use crate::helpers::Def;
use crate::io::{DeviceMetadata, IODirection, IOEvent, IOKind, IdType, RawValue};
use crate::storage::RootPath;
use crate::storage::{Chronicle, Log, Persistent};
use chrono::Utc;
use crate::errors::ErrorType;

/// Common constructors and builder methods for all device types
pub trait Device: Chronicle + DeviceGetters + DeviceSetters + Persistent {
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

    #[deprecated]
    /// Generate an `IOEvent` instance from provided value
    ///
    /// This is used by internal `command` for building events from given data.
    /// Input devices pass read value; output devices pass write value.
    ///
    /// # Notes
    /// Utc time is generated within this function. This allows each call to be more accurately
    /// recorded instead of using a single time when polling. Accurate record keeping is more
    /// valuable than a slight hit to performance.
    ///
    /// Additionally, internally generating timestamp adds a layer of separation between
    /// device trait objects and any of it's owners (i.e.: `PollGroup`).
    fn generate_event(&self, value: RawValue) -> IOEvent {
        let timestamp = Utc::now();
        IOEvent::with_timestamp(timestamp, value)
    }

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

    /// Setter for root
    ///
    /// Updates any internal field that needs a root path (ie: [`Log`])
    ///
    /// # Parameters
    ///
    /// - `root`: New [`RootPath`] to store
    fn set_root(&self, root: RootPath) {
        if self.has_log() {
            let binding = self.log().unwrap();
            let mut log = binding.try_lock().unwrap();
            log.set_root_ref(root)
        }
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

    /// Returns the name of the device.
    fn name(&self) -> String {
        self.metadata().name.clone()
    }

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
    /// - `RawValue` after first read or write, and represents last known state.
    fn state(&self) -> &Option<RawValue>;
}

/// Command setter methods share by all device types
pub trait DeviceSetters {
    fn set_name<N>(&mut self, name: N)
        where
            N: Into<String>;

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