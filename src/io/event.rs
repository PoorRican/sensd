use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::{IdTraits, RawValue};

/// Record value at a specific timestamp
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct IOEvent {
    pub timestamp: DateTime<Utc>,
    pub value: RawValue,
}

impl IOEvent {
    /// Constructor for [`IOEvent`]
    ///
    /// # Arguments
    ///
    /// - `timestamp`: timestamp of event
    /// - `value`: value to include in
    ///
    /// # Returns
    ///
    /// `IOEvent` based on timestamp and value
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn with_timestamp(timestamp: DateTime<Utc>, value: RawValue) -> Self {
        IOEvent {
            timestamp,
            value,
        }
    }

    pub fn generate(value: RawValue) -> Self {
        let timestamp = Utc::now();
        IOEvent::with_timestamp(timestamp, value)
    }
}

impl IdTraits for DateTime<Utc> {}
