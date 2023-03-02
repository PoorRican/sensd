//! Provide Low-level Device Functionality
use std::fmt::Formatter;
use std::sync::Arc;
use chrono::Utc;
use crate::action::GPIOCommand;
use crate::errors::*;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IODirection, IOKind, IdType, IOType, IOEvent, DeviceMetadata};
use crate::settings::Settings;
use crate::storage::{HasLog, OwnedLog};

/// Defines a minimum interface for interacting with GPIO devices.
///
/// A universal constructor is provided that can be shared between any implementing structs.
/// Additionally, an accessor, `metadata()` is defined to provide for the facade methods to access
/// device name, id, direction, and kind. Therefore, implementing structs shall implement a field
/// `metadata` that is mutably accessed through the reciprocal getter method.
pub trait Device: HasLog {

    /// Creates a new instance of the device with the given parameters.
    ///
    /// `name`: name of device.
    /// `id`: device ID.
    /// `kind`: kind of I/O device. Optional argument.
    /// `log`: Optional deferred owned log for the device.
    fn new(name: String, id: IdType, kind: Option<IOKind>, log: Option<Deferred<OwnedLog>>) -> Self
    where
        Self: Sized;

    /// Returns a reference to the device's metadata
    /// from which information such as name, ID, kind, and I/O direction are inferred.
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
    fn generate_event(&self, value: IOType) -> IOEvent {
        let dt = Utc::now();
        IOEvent::generate(self.metadata(), dt, value)
    }

    /// Setter for `command` field
    fn add_command(&mut self, command: GPIOCommand);

    /// Setter for `log` field
    fn add_log(&mut self, log: Deferred<OwnedLog>);

    /// Initialize, set, and return log.
    fn init_log(&mut self, settings: Option<Arc<Settings>>) -> Deferred<OwnedLog> {
        let log = OwnedLog::new(self.id(), settings).deferred();
        self.add_log(log.clone());
        log
    }
}

impl std::fmt::Debug for dyn Device {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Device: {} - {{ name: {}, id: {}, kind: {}}}",
            self.direction(),
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}

pub fn no_internal_closure() -> Box<dyn std::error::Error> {
    Error::new(ErrorKind::CommandError, "Device has no internal closure")
}
