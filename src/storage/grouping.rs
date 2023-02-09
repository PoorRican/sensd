use chrono::{DateTime, Utc};
use std::sync::{Arc, Mutex};

use crate::errors::{Error, Result};
use crate::io::{Device, Input, InputType, IOEvent, MockPhSensor, IdType, InputContainer};
use crate::settings::Settings;
use crate::storage::{Container, Containerized, MappedCollection, Persistent};
use crate::storage::{LogType, LogContainer};
use crate::helpers::{check_results, Deferred, input_log_builder};


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
    pub logs: Vec<Deferred<LogType>>,
    pub sensors: InputContainer<IdType>,
}

impl PollGroup {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) -> std::result::Result<Vec<Result<()>>, ()> {
        let mut results: Vec<Result<()>> = Vec::new();
        let next_execution = self.last_execution + self.settings.interval;

        if next_execution <= Utc::now() {
            for (_, sensor) in self.sensors.iter_mut() {
                let result = sensor.lock().unwrap().poll(next_execution);
                results.push(result);
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

        let sensors = <InputContainer<IdType>>::default();
        let logs = Vec::default();

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
        let (log, sensor) = input_log_builder(name, id, self.settings.clone());
        self.logs.push(log);
        let id = sensor.lock().unwrap().id();
        self.sensors.push(id, sensor)
    }

    pub fn add_sensors(&mut self, arr: &[(&str, i32)]) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for (name, id) in arr {
            let result = self.add_sensor(name, *id as IdType);
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

    /// Load each individual log
    /// # Notes
    /// This works because each log container should have it's own name upon initialization
    /// from hardcoded sensors.
    fn load_logs(&self, path: &Option<String>) -> Result<()> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.lock().unwrap().load(path);
            results.push(result);
        }
        check_results(&results)
    }

    /// Save each individual log
    /// # Notes
    /// This works because each log container should have it's own name upon initialization
    /// from hardcoded sensors.
    fn save_logs(&self, path: &Option<String>) -> Result<()> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.lock().unwrap().save(path);
            results.push(result);
        }
        check_results(&results)
    }
}

impl Persistent for PollGroup {
    fn save(&self, path: &Option<String>) -> Result<()> {
        let results = [self.save_logs(path)];
        // check_results(&results).as_ref()
        // hack to run code
        Ok(())
    }

    fn load(&mut self, path: &Option<String>) -> Result<()> {
        let results = [self.load_logs(path)];
        // check_results(&results)
        // hack to run code
        Ok(())
    }
}
