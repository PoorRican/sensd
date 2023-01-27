use chrono::{DateTime, Duration, Utc};
use std::hash::Hash;

use crate::container::{Collection, Container};
use crate::device::Sensor;
use crate::io::IOEvent;

/// Mediator that polls a `Container` of `Sensors` and populates another container with `IOEvent` objects.
/// TODO: multithreaded polling
pub struct Poller<T: 'static, K: Eq + Hash + 'static> {
    interval: Duration,
    last_execution: DateTime<Utc>,

    // internal containers
    sensors: &'static Container<Box<dyn Sensor<T>>, K>,
    log: &'static mut Container<IOEvent<T>, DateTime<Utc>>,
}

impl<T, K: Eq + Hash> Poller<T, K> {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    fn poll(&mut self) {
        let next_execution = self.last_execution + self.interval;

        if next_execution <= Utc::now() {
            for sensor in self.sensors._inner().values() {
                self.last_execution = next_execution;
                let event = sensor.get_event(next_execution);
                self.log.add(next_execution, event);
            }
        }
    }
}
