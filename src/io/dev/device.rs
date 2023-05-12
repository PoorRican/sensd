use crate::action::IOCommand;
use crate::helpers::Def;
use crate::io::{DeviceMetadata, IODirection, IOEvent, IOKind, IdType, RawValue};
use crate::settings::RootPath;
use crate::storage::{Chronicle, Log, Persistent};
use chrono::Utc;
use crate::errors::ErrorType;

/// Defines a minimum interface for interacting with GPIO devices.
///
/// A universal constructor is provided that can be shared between any implementing structs.
/// Additionally, an accessor, `metadata()` is defined to provide for the facade methods to access
/// device name, id, direction, and kind. Therefore, implementing structs shall implement a field
/// `metadata` that is mutably accessed through the reciprocal getter method.
pub trait Device: Chronicle + DeviceGetters + DeviceSetters + Persistent {
    /// Creates a new instance of the device with the given parameters.
    ///
    /// # Parameters
    /// `name`: name of device.
    /// `id`: device ID.
    /// `kind`: kind of I/O device. Optional argument.
    /// `log`: Optional deferred owned log for the device.
    fn new<N, K>(name: N, id: IdType, kind: K) -> Self
    where
        Self: Sized,
        N: Into<String>,
        K: Into<Option<IOKind>>;

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
        let dt = Utc::now();
        IOEvent::new(self.metadata(), dt, value)
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
        let log = Def::new(Log::new(&self.metadata()));
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
            log.set_root(root)
        }
    }

    fn into_deferred(self) -> Def<Self>
    where
        Self: Sized
    {
        Def::new(self)
    }
}

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