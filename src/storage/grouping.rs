use crate::action::{IOCommand, PublisherInstance};
use crate::builders::DeviceLogBuilder;
use crate::errors::ErrorType;
use crate::helpers::{check_results, Def};
use crate::io::{DeviceContainer, DeviceType, IODirection, IOEvent, IOKind, IdType};
use crate::settings::Settings;
use crate::storage::{LogContainer, Persistent};
use chrono::{DateTime, Duration, Utc};
use std::ops::DerefMut;
use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::Arc;

/// High-level container to manage multiple [`Device`] objects, logging, and actions.
///
/// [`Group::poll()`] and [`Group::attempt_routines()`] are the primary callables for function. Both functions are
/// called on different intervals. The execution of `[poll()`] is dictated by the interval stored in
/// runtime settings. Conversely, [`Group::attempt_routines()`] should be executed as often as possible to
/// maintain timing accuracy.
///
/// Both [`Group::poll()`] and [`Group::attempt_routines()`] are high-level functions whose returned values
/// can mainly be ignored. Future revisions will add failure log functionality in the event of failure or
/// misconfiguration.
pub struct Group {
    /// Name used to identify this specific device grouping.
    ///
    /// This is mainly used for sub-directory labeling
    name: String,
    /// Buffer to store time of the last successful poll.
    last_execution: DateTime<Utc>,

    /// Non-mutable storage of runtime settings
    settings: Arc<Settings>,

    // internal containers
    pub logs: LogContainer,

    pub inputs: DeviceContainer<IdType>,
    pub outputs: DeviceContainer<IdType>,

    /// Container for `PublisherInstances`
    pub publishers: Vec<Def<PublisherInstance>>,
}

impl Group {
    /// Primary callable to iterate through input device container once.
    ///
    /// [`GenericInput::read()`] is called on each input device at the frequency dictated by
    /// [`Group::interval()`]. Generated [`IOEvent`] instances are handled by [`crate::io::GenericInput::read()`].
    /// Failure does not halt execution. Instead, failed calls to [`GenericInput::read()`] are returned as an
    /// array of [`Result`] objects. [`check_results()`] should be used to catch and handle any errors
    ///
    /// # Returns
    /// [`Ok`] when poll has successfully executed. The wrapped value is a vector of [`Result`]
    /// values. Otherwise, [`Err`] is returned when function has been called out of sync with
    /// interval.
    // TODO: custom `ErrorType` for failed read. Should include device metadata.
    pub fn poll(&mut self) -> Result<Vec<Result<IOEvent, ErrorType>>, ()> {
        let mut results: Vec<Result<IOEvent, ErrorType>> = Vec::new();
        let next_execution = self.last_execution + self.interval();

        if next_execution <= Utc::now() {
            for (_, input) in self.inputs.iter_mut() {
                let mut device = input.try_lock().unwrap();
                if let DeviceType::Input(inner) = device.deref_mut() {
                    let result = inner.read();
                    results.push(result);
                }
                // TODO: throw failure if device is not input
            }
            self.last_execution = next_execution;
            Ok(results)
        } else {
            Err(())
        }
    }

    /// Initialized empty containers.
    ///
    /// Builder and setter functions should be used to populate containers.
    pub fn new(name: &str, settings: Option<Arc<Settings>>) -> Self {
        let settings = settings.unwrap_or_else(|| Arc::new(Settings::default()));
        let last_execution = Utc::now() - settings.interval;

        let inputs = <DeviceContainer<IdType>>::default();
        let outputs = <DeviceContainer<IdType>>::default();
        let logs = Vec::default();
        let publishers = Vec::default();

        Self {
            name: String::from(name),
            settings,
            last_execution,
            logs,
            inputs,
            outputs,
            publishers,
        }
    }

    /// Build device and log and locally store both.
    ///
    /// # Errors
    /// Panics error if [`IOCommand`] and [`IODirection`] are mismatched
    /// (according to [`builder::check_alignment()`])
    pub fn build_device(
        &mut self,
        name: &str,
        id: &IdType,
        kind: &Option<IOKind>,
        direction: &IODirection,
        command: &IOCommand,
    ) -> Result<Def<DeviceType>, ErrorType> {
        // variable allowed to go out-of-scope because `poller` owns reference
        let settings = Some(self.settings.clone());

        let builder = DeviceLogBuilder::new(name, id, kind, direction, command, settings);
        builder.setup_command();

        let (device, log) = builder.get();

        self.logs.push(log);

        match direction {
            IODirection::Input => self.inputs.insert(*id, device.clone()),
            IODirection::Output => self.outputs.insert(*id, device.clone()),
        };

        Ok(device)
    }

    /// Wrapper for [`Group::build_device()`] for building multiple device/log abstractions
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
    pub fn interval(&self) -> Duration {
        self.settings.interval
    }

    /// Load all device logs
    ///
    /// # Errors
    /// Returns an error if any single load fails. However, failure does not prevent loading of
    /// other device logs.
    fn load_logs(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.try_lock().unwrap().load(path);
            results.push(result);
        }
        check_results(&results)
    }

    /// Dedicated directory for [`Group`]
    ///
    /// The dedicated directory for a [`Group`] is simply a sub-directory in the global path.
    pub fn dir(&self) -> PathBuf {
        let path = Path::new(self.settings.data_root.as_str());
        path.join(self.name.as_str())
    }

    /// Save all device logs
    ///
    /// # Errors
    /// Returns an error if any single save fails. However, failure does not prevent saving of
    /// other device logs.
    fn save_logs(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let mut results = Vec::new();
        for log in self.logs.iter() {
            let result = log.try_lock().unwrap().save(path);
            results.push(result);
        }
        check_results(&results)
    }

    /// Attempt to create root data directory
    pub fn setup_dir(&self) {
        let path = self.dir();
        match path.exists() {
            true => (),
            false => {
                create_dir_all(path).expect("Could not create root data directory");
            },
        }
    }

    pub fn attempt_routines(&self) {
        for _publisher in self.publishers.iter() {
            let mut publisher = _publisher.try_lock().unwrap();
            publisher.attempt_routines();
        }
    }
}

/// Only save and load log data since [`Group`] is statically initialized
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
    use std::path::Path;
    use std::sync::Arc;

    use crate::settings::Settings;
    use crate::storage::Group;

    use std::fs::remove_dir_all;

    /// Test [`Group::dir()`]
    #[test]
    fn test_dir() {
        const DIR_NAME: &str = "test_root";
        const GROUP_NAME: &str = "main";

        // init `Group` and settings
        let dir_name = String::from(DIR_NAME);
        let mut _settings = Settings::default();
        _settings.set_root(dir_name.clone());

        let expected = Path::new(DIR_NAME).join(GROUP_NAME);
        let group = Group::new(GROUP_NAME, Some(Arc::new(_settings)));

        // assert directory path is correct
        assert_eq!(expected.to_str().unwrap(), group.dir().to_str().unwrap());
    }

    /// Test [`Group::setup_dir()`]
    #[test]
    fn test_setup_dir() {
        const DIR_NAME: &str = "test_root";
        const GROUP_NAME: &str = "main";

        // init `Group` and settings
        let dir_name = String::from(DIR_NAME);
        let mut _settings = Settings::default();
        _settings.set_root(dir_name.clone());

        let group = Group::new(GROUP_NAME, Some(Arc::new(_settings)));

        // assert `setup_dir()` works as expected exists
        group.setup_dir();
        assert!(group.dir().exists());

        remove_dir_all(group.dir().parent().unwrap())
            .unwrap();
    }
}
