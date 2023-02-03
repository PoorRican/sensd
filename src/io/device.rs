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
