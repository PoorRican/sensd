use chrono::{DateTime, Duration, Utc};
use std::sync::Arc;

use crate::errors::Result;
use crate::helpers::{check_results, input_log_builder};
use crate::io::{IdType, InputContainer, IOKind};
use crate::settings::Settings;
use crate::storage::{MappedCollection, Persistent, LogContainer};

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

    /// Non-mutable storage of runtime settings
    /// Ownership of settings should be given to `PollGroup`
    settings: Arc<Settings>,

    // internal containers
    pub logs: LogContainer,
    pub inputs: InputContainer<IdType>,
}

impl PollGroup {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) -> std::result::Result<Vec<Result<()>>, ()> {
        let mut results: Vec<Result<()>> = Vec::new();
        let next_execution = self.last_execution + self.settings.interval;

        if next_execution <= Utc::now() {
            for (_, sensor) in self.inputs.iter_mut() {
                let result = sensor.lock().unwrap().read(next_execution);
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
    pub fn new(name: &str, settings: Option<Arc<Settings>>) -> Self {
        let settings = settings.unwrap_or_else(|| Arc::new(Settings::default()));
        let last_execution = Utc::now() - settings.interval;

        let sensors = <InputContainer<IdType>>::default();
        let logs = Vec::default();

        Self {
            name: String::from(name),
            settings,
            last_execution,
            logs,
            inputs: sensors,
        }
    }

    pub fn build_input(&mut self, name: &str, id: &IdType, kind: &Option<IOKind>) -> Result<()> {
        // variable allowed to go out-of-scope because `poller` owns reference
        let settings = Some(self.settings.clone());

        let (log, input) = input_log_builder(name, id, kind, settings);
        self.logs.push(log);

        let id = input.lock().unwrap().id();
        self.inputs.push(id, input)
    }

    /// Builds multiple input objects and respective `OwnedLog` containers.
    /// # Args:
    /// Single array should be any sequence of tuples containing a name literal, an `IdType`, and an `IOKind`
    pub fn add_inputs(&mut self, arr: &[(&str, IdType, IOKind)]) -> Result<()> {
        let mut results = Vec::new();
        for (name, id, kind) in arr.into_iter() {
            let result = self.build_input(name, id, &Some(*kind));
            results.push(result);
        }
        check_results(&results)
    }

    /// Facade to return operating frequency
    pub fn _interval(&self) -> Duration {
        self.settings.interval
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

/// Only save and load log data since PollGroup is statically initialized
/// If `&None` is given to either methods, then current directory is used.
impl Persistent for PollGroup {
    fn save(&self, path: &Option<String>) -> Result<()> {
        let results = &[self.save_logs(path)];
        check_results(results)
    }

    fn load(&mut self, path: &Option<String>) -> Result<()> {
        let results = &[self.load_logs(path)];
        check_results(results)
    }
}
