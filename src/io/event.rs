use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::io::{IdTraits, RawValue};

/// Dedicated object for storing a single record at a specific point in time.
///
/// # Getting Started
///
/// The easiest way to create an [`IOEvent`] is by using the `new` constructor:
///
/// ```
/// use sensd::io::{IOEvent, RawValue};
///
/// let value = RawValue::default();
///
/// let event = IOEvent::new(value);
///
/// assert_eq!(value, event.value);
/// ```
///
/// However, if a specific `timestamp` is desired, the [`IOEvent::with_timestamp()`]
/// allows `timestamp` to be passed as a parameter.
///
/// # See Also
///
/// A collection of multiple [`IOEvent`] objects is handled by [`crate::storage::EventCollection`].
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IOEvent {
    pub timestamp: DateTime<Utc>,
    pub value: RawValue,
}

impl IOEvent {
    /// Alternate constructor for [`IOEvent`] that accepts a timestamp parameter
    ///
    /// # Arguments
    ///
    /// - `timestamp`: timestamp of event
    /// - `value`: value to include in record
    ///
    /// # Returns
    ///
    /// `IOEvent` based on timestamp and value
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Utc;
    /// use sensd::io::{IOEvent, RawValue};
    ///
    /// let now = Utc::now();
    /// let value = RawValue::default();
    ///
    /// let event = IOEvent::with_timestamp(now, value);
    ///
    /// assert_eq!(now, event.timestamp);
    /// assert_eq!(value, event.value);
    /// ```
    pub fn with_timestamp(timestamp: DateTime<Utc>, value: RawValue) -> Self {
        IOEvent {
            timestamp,
            value,
        }
    }

    /// Constructor for [`IOEvent`]
    ///
    /// # Parameters
    ///
    /// - `value`: value to include in record
    ///
    /// # Returns
    ///
    /// [`IOEvent`] with internally generated `timestamp` and given `value`.
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::{IOEvent, RawValue};
    ///
    /// let value = RawValue::default();
    ///
    /// let event = IOEvent::new(value);
    ///
    /// assert_eq!(value, event.value);
    /// ```
    pub fn new(value: RawValue) -> Self {
        let timestamp = Utc::now();
        IOEvent::with_timestamp(timestamp, value)
    }
}

impl IdTraits for DateTime<Utc> {}
