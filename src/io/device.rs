/// Provide Low-level Device Functionality
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

use crate::errors::Result;
use crate::io;
use crate::io::input::Input;
use crate::io::metadata::DeviceMetadata;
use crate::storage::{Container, Containerized};

/// Basic interface for GPIO device metadata
pub trait Device {
    fn get_metadata(&self) -> &DeviceMetadata;
    fn name(&self) -> String;
    fn id(&self) -> i32;
}

impl std::fmt::Debug for dyn Input {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Sensor {{ name: {}, id: {}, kind: {}",
            self.name(),
            self.id(),
            self.get_metadata().kind
        )
    }
}

/// Defines an interface for an input device that needs to be calibrated
pub trait Calibrated {
    /// Initiate the calibration procedures for a specific device instance.
    fn calibrate(&self) -> bool;
}
