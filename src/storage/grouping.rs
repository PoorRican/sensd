use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

use crate::errors::{Error, Result};
use crate::io::MockPhSensor;
use crate::io::{Device, Input, InputType};
use crate::settings::Settings;
use crate::storage::{Container, Containerized, MappedCollection, Persistent};
use crate::io::IdType;
use crate::storage::logging::LogType;


/// Mediator to periodically poll sensors of various types, and store the resulting `IOEvent` objects in a `Container`.
///
/// `poll()` is the primary callable and iterates through the `Sensor` container to call `get_event()` on each sensor.
/// Resulting `IOEvent` objects are then added to the `log` container.
///
/// The `interval` field indicates the duration between each poll and the `last_execution` field indicates the last time the poll method was executed
///
/// TODO: multithreaded polling. Implement `RwLock` or `Mutex` to synchronize access to the sensors and
///       log containers in order to make the poll() function thread-safe.
pub struct PollGroup {
    name: String,
    last_execution: DateTime<Utc>,
    settings: Arc<Settings>,

    // internal containers
    pub logs: Vec<Arc<Mutex<LogType>>>,
    pub sensors: Container<InputType, IdType>,
}

impl PollGroup {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) -> std::result::Result<Vec<Result<()>>, ()> {
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
    pub fn new(name: &str, settings: Arc<Settings>) -> Self {
        let last_execution = Utc::now() - settings.interval;

        let sensors: Container<InputType, IdType> = <dyn Input>::container();
        let logs: Vec<Arc<Mutex<LogType>>> = Vec::new();

        Self {
            name: String::from(name),
            settings,
            last_execution,
            logs,
            sensors,
        }
    }

    pub fn add_sensor(&mut self, name: &str, id: IdType) -> Result<()> {
        // variable allowed to go out-of-scope because `poller` owns reference
        let log = Arc::new(Mutex::new(LogType::new()));
        self.logs.push(log.clone());

        let sensor = MockPhSensor::new(name.to_string(), id, log.clone());
        self.sensors.push(sensor.id(), sensor.boxed())
    }

    pub fn add_sensors(&mut self, arr: Vec<(&str, i32)>) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for (name, id) in arr {
            let result = self.add_sensor(name, id as IdType);
            results.push(result);
        }
        results
    }

    fn log_fn(&self, name: String) -> String {
        let s = String::new();
        s + &self.settings.log_fn_prefix + name.as_str() + ".json"
    }

    fn sensors_fn(&self, name: String) -> String {
        let s = String::new();
        s + &self.settings.sensors_fn_prefix + name.as_str() + ".toml"
    }

    fn save_logs(&self) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for (i, guarded) in self.logs.iter().enumerate() {
            let path = Some(self.log_fn(i.to_string()));
            let result = guarded.lock().unwrap().save(path);
            results.push(result);
        }
        results
    }

    fn load_logs(&self) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for (i, guarded) in self.logs.iter().enumerate() {
            let path = Some(self.log_fn(i.to_string()));
            let result = guarded.lock().unwrap().load(path);
            results.push(result);
        }
        results
    }
}

impl Persistent for PollGroup {
    fn save(&self, _: Option<String>) -> Result<()> {
        let results = self.save_logs();
        for result in results.into_iter() {
            match result {
                Err(e) => return Err(e),
                _ => continue
            };
        }
        Ok(())
    }

    fn load(&mut self, _: Option<String>) -> Result<()> {
        let results = self.load_logs();
        for result in results.into_iter() {
            match result {
                Err(e) => return Err(e),
                _ => continue
            }
        };
        Ok(())
    }
}
