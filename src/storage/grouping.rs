use chrono::{DateTime, Utc};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::errors::Result;
use crate::io::{Input, InputType, LogType};
use crate::settings::Settings;
use crate::storage::{Container, Containerized};

/// Mediator to periodically poll sensors of various types, and store the resulting `IOEvent` objects in a `Container`.
///
/// `poll()` is the primary callable and iterates through the `Sensor` container to call `get_event()` on each sensor.
/// Resulting `IOEvent` objects are then added to the `log` container.
///
/// The `interval` field indicates the duration between each poll and the `last_execution` field indicates the last time the poll method was executed
///
/// TODO: multithreaded polling. Implement `RwLock` or `Mutex` to synchronize access to the sensors and
///       log containers in order to make the poll() function thread-safe.
pub struct PollGroup<K: Eq + Hash> {
    name: String,
    last_execution: DateTime<Utc>,
    settings: Arc<Settings>,

    // internal containers
    pub logs: Vec<Arc<Mutex<LogType>>>,
    pub sensors: Container<InputType, K>,
}

impl<K: Eq + Hash> PollGroup<K> {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) -> std::result::Result<Vec<Result<()>>, ()>{
        let mut results: Vec<Result<()>> = Vec::new();
        let next_execution = self.last_execution + self.settings.interval;

        if next_execution <= Utc::now() {
            for (_, sensor) in self.sensors.iter_mut() {
                results.push(sensor.poll(next_execution));
            }
            self.last_execution = next_execution;
            Ok(results)
        } else {
            Err(())
        }
    }

    /// Constructor for `Poller` struct.
    /// Initialized empty containers.
    pub fn new( name: &str, settings: Arc<Settings> ) -> Self {
        let last_execution = Utc::now() - settings.interval;

        let sensors: Container<InputType, K> = <dyn Input>::container();
        let logs: Vec<Arc<Mutex<LogType>>> = Vec::new();

        Self { name: String::from(name), settings, last_execution, logs, sensors }
    }
}