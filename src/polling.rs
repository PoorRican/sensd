use chrono::{DateTime, Duration, Utc};
use std::hash::Hash;

use crate::container::{Collection, Container, Containerized};
use crate::device::Sensor;
use crate::io::IOEvent;

/// Mediator to periodically poll sensors of various types, and store the resulting `IOEvent` objects in a `Container`.
///
/// `poll()` is the primary callable and iterates through the `Sensor` container to call `get_event()` on each sensor.
/// Resulting `IOEvent` objects are then added to the `log` container.
///
/// The `interval` field indicates the duration between each poll and the `last_execution` field indicates the last time the poll method was executed
///
/// TODO: multithreaded polling. Implement `RwLock` or `Mutex` to synchronize access to the sensors and
///       log containers in order to make the poll() function thread-safe.
pub struct PollGroup<T, K: Eq + Hash> {
    interval: Duration,
    last_execution: DateTime<Utc>,

    // internal containers
    pub sensors: Container<Box<dyn Sensor<T>>, K>,
    pub log: Container<IOEvent<T>, DateTime<Utc>>,
}

impl<T: std::fmt::Debug, K: Eq + Hash> PollGroup<T, K> {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) {
        let next_execution = self.last_execution + self.interval;

        if next_execution <= Utc::now() {
            for (_, sensor) in self.sensors.iter() {
                self.last_execution = next_execution;
                let event = sensor.get_event(next_execution);
                self.log.add(next_execution, event);
                dbg!(sensor);
            }
        }
    }

    /// Constructor for `Poller` struct.
    /// Internal containers are instantiated as empty.
    pub fn new( interval: Duration, last_execution: DateTime<Utc> ) -> Self {
        let sensors: Container<Box<dyn Sensor<T>>, K> = <dyn Sensor<T>>::container();
        let log: Container<IOEvent<T>, DateTime<Utc>> = <IOEvent<T>>::container();
        Self { interval, last_execution, sensors, log }
    }
}
