use chrono::{DateTime, Duration, Utc};
use std::hash::Hash;

use crate::container::{Collection, Container, Containerized};
use crate::device::Sensor;
use crate::io::IOEvent;

/// Mediator that polls a `Container` of `Sensors` and populates another container with `IOEvent` objects.
/// TODO: multithreaded polling
pub struct Poller<T, K: Eq + Hash> {
    interval: Duration,
    last_execution: DateTime<Utc>,

    // internal containers
    pub sensors: Container<Box<dyn Sensor<T>>, K>,
    pub log: Container<IOEvent<T>, DateTime<Utc>>,
}

impl<T: std::fmt::Debug, K: Eq + Hash> Poller<T, K> {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) {
        let next_execution = self.last_execution + self.interval;

        if next_execution <= Utc::now() {
            for (_, sensor) in self.sensors._inner() {
                self.last_execution = next_execution;
                let event = sensor.get_event(next_execution);
                self.log.add(next_execution, event);
                dbg!(sensor);
            }
        }
    }

    pub fn new( interval: Duration, last_execution: DateTime<Utc> ) -> Self {
        let sensors: Container<Box<dyn Sensor<T>>, K> = <dyn Sensor<T>>::container();
        let log: Container<IOEvent<T>, DateTime<Utc>> = <IOEvent<T>>::container();
        Self { interval, last_execution, sensors, log }
    }
}
