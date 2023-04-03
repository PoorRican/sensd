use crate::action::{IOCommand, PublisherInstance, Routine};
use crate::builders::DeviceLogBuilder;
use crate::errors::ErrorType;
use crate::helpers::{check_results, Deferred};
use crate::io::{DeviceContainer, DeviceType, IODirection, IOEvent, IOKind, IdType};
use crate::settings::Settings;
use crate::storage::{LogContainer, MappedCollection, Persistent};
use chrono::{DateTime, Duration, Utc};
use std::ops::DerefMut;
use std::fs::create_dir_all;
use std::path::Path;
use std::sync::Arc;

/// Mediator to periodically poll input devices of various types, and store the resulting `IOEvent` objects in a `Container`.
///
/// `poll()` is the primary callable and iterates through `InputContainers` to call `read()` on each input device.
/// Resulting `IOEvent` objects are then added to the `log` container.
///
/// `interval` dictates the duration between each poll,
/// and `last_execution` field is working memory to store the time of the last successful poll.
pub struct Group {
    _name: String,
    last_execution: DateTime<Utc>,

    /// Non-mutable storage of runtime settings
    /// Ownership of settings should be given to `Group`
    settings: Arc<Settings>,

    // internal containers
    pub logs: LogContainer,

    pub inputs: DeviceContainer<IdType>,
    pub outputs: DeviceContainer<IdType>,

    pub publishers: Vec<Deferred<PublisherInstance>>,
    pub scheduled: Vec<Routine>,
}

impl Group {
    /// Iterate through container once. Call `get_event()` on each value.
    /// Update according to the lowest rate.
    pub fn poll(&mut self) -> Result<Vec<Result<IOEvent, ErrorType>>, ()> {
        let mut results: Vec<Result<IOEvent, ErrorType>> = Vec::new();
        let next_execution = self.last_execution + self.settings.interval;

        if next_execution <= Utc::now() {
            for (_, input) in self.inputs.iter_mut() {
                let mut device = input.try_lock().unwrap();
                if let DeviceType::Input(inner) = device.deref_mut() {
                    let result = inner.read();
                    results.push(result);
                }
            }
            self.last_execution = next_execution;
            Ok(results)
        } else {
            Err(())
        }
    }

    pub fn check_scheduled(&mut self) {
        let mut executed = Vec::default();
        for (index, routine) in self.scheduled.iter().enumerate() {
            if routine.attempt() {
                executed.push(index);
            }
        }
        // remove completed
        for index in executed {
            self.scheduled.remove(index);
        }
    }

    /// Constructor for `Poller` struct.
    /// Initialized empty containers.
    pub fn new(name: &str, settings: Option<Arc<Settings>>) -> Self {
        let settings = settings.unwrap_or_else(|| Arc::new(Settings::default()));
        let last_execution = Utc::now() - settings.interval;

        let inputs = <DeviceContainer<IdType>>::default();
        let outputs = <DeviceContainer<IdType>>::default();
        let logs = Vec::default();
        let publishers = Vec::default();
        let scheduled = Vec::default();

        Self {
            _name: String::from(name),
            settings,
            last_execution,
            logs,
            inputs,
            outputs,
            publishers,
            scheduled,
        }
    }

    /// Build device interface and log.
    ///
    /// Add device to store
    pub fn build_device(
        &mut self,
        name: &str,
        id: &IdType,
        kind: &Option<IOKind>,
        direction: &IODirection,
        command: &IOCommand,
    ) -> Result<Deferred<DeviceType>, ErrorType> {
        // variable allowed to go out-of-scope because `poller` owns reference
        let settings = Some(self.settings.clone());

        let builder = DeviceLogBuilder::new(name, id, kind, direction, command, settings);
        builder.setup_command();

        let (device, log) = builder.get();

        self.logs.push(log);

        match direction {
            IODirection::Input => {
                match self.inputs.push(*id, device.clone()) {
                    Err(error) => eprintln!("{}", error.to_string()),
                    _ => (),
                }
            },
            IODirection::Output => {
                match self.outputs.push(*id, device.clone()) {
                    Err(error) => eprintln!("{}", error.to_string()),
                    _ => (),
                }
            },
        }
        Ok(device)
    }

    /// Builds multiple input objects and their respective `OwnedLog` containers.
    ///
    /// # Args:
    /// Single array should be any sequence of tuples containing a name literal, an `IdType`, and an `IOKind`
    pub fn add_devices(
        &mut self,
        arr: &[(&str, IdType, IOKind, IODirection, IOCommand)],
    ) -> Result<(), ErrorType> {
        let mut results = Vec::default();
        for (name, id, kind, direction, command) in arr.iter().to_owned() {
            let result = self.build_device(name, id, &Some(*kind), direction, command);
            results.push(result);
        }
        check_results(&results)
    }

    /// Facade to return operating frequency
    pub fn _interval(&self) -> Duration {
        self.settings.interval
    }

    /// Load each individual log
    ///
    /// # Notes
    /// This works because each log container should have it's own name upon initialization
    /// from hardcoded input devices.
    fn load_logs(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.lock().unwrap().load(path);
            results.push(result);
        }
        check_results(&results)
    }

    /// Dedicated directory for `Group`
    ///
    /// TODO: append group name to path to isolate data
    pub fn dir(&self) -> &Path {
        Path::new(self.settings.data_root.as_str())
    }

    /// Save each individual log
    ///
    /// # Notes
    /// This works because each log container should have it's own name upon initialization
    /// from hardcoded input devices.
    fn save_logs(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.lock().unwrap().save(path);
            results.push(result);
        }
        check_results(&results)
    }

    /// Attempt to setup root data directory
    pub fn setup_dir(&self) {
        let path = self.dir();
        match path.exists() {
            true => (),
            false => {
                create_dir_all(path).expect("Could not create root data directory");
            },
        }
    }
}

/// Only save and load log data since Group is statically initialized
/// If `&None` is given to either methods, then current directory is used.
impl Persistent for Group {
    fn save(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let results = &[self.save_logs(path)];
        check_results(results)
    }

    fn load(&mut self, path: &Option<String>) -> Result<(), ErrorType> {
        let results = &[self.load_logs(path)];
        check_results(results)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::settings::Settings;
    use crate::storage::Group;

    use std::fs::remove_dir_all;

    #[test]
    fn test_setup_dir() {
        // init `Group` and settings
        let dir_name = String::from("test_root");
        let mut _settings = Settings::default();
        _settings.set_root(dir_name.clone());

        let group = Group::new("main", Some(Arc::new(_settings)));

        // assert directory path is correct
        assert_eq!(dir_name.as_str(), group.dir().to_str().unwrap());

        group.setup_dir();

        // assert directory exists
        assert!(group.dir().exists());

        remove_dir_all(group.dir()).unwrap();
    }
}
