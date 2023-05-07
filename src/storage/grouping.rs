use std::collections::hash_map::Entry;
use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::{check_results, Def};
use crate::io::{Device, DeviceContainer, Input, Output, IOEvent, IdType, DeviceGetters};
use crate::settings::Settings;
use crate::storage::{Chronicle, Persistent};
use chrono::{DateTime, Duration, Utc};
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

    pub inputs: DeviceContainer<IdType, Input>,
    pub outputs: DeviceContainer<IdType, Output>,
}

impl Group {
    /// Primary callable to iterate through input device container once.
    ///
    /// [`Input::read()`] is called on each input device at the frequency dictated by
    /// [`Group::interval()`]. Generated [`IOEvent`] instances are handled by [`Input::read()`].
    /// Failure does not halt execution. Instead, failed calls to [`Input::read()`] are returned as an
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
            for input in self.inputs.values_mut() {
                let mut binding = input.try_lock().unwrap();
                results.push(binding.read());
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
    pub fn new<N>(name: N, settings: Option<Arc<Settings>>) -> Self
    where
        N: Into<String>
    {
        let settings = settings.unwrap_or_else(|| Arc::new(Settings::default()));
        let last_execution = Utc::now() - settings.interval;

        let inputs = <DeviceContainer<IdType, Input>>::default();
        let outputs = <DeviceContainer<IdType, Output>>::default();

        Self {
            name: name.into(),
            settings,
            last_execution,
            inputs,
            outputs,
        }
    }

    pub fn insert_input(&mut self, id: IdType, input: Def<Input>) -> Result<Def<Input>, ErrorType> {
        match self.inputs.entry(id) {
            Entry::Occupied(_) => Err(Error::new(ErrorKind::ContainerError, "Input already exists")),
            Entry::Vacant(entry) => Ok(entry.insert(input).clone()),
        }
    }

    /// Builder method to store [`Input`] in internal collection
    ///
    /// # Parameters
    ///
    /// - `device`: [`Input`] device to be added
    ///
    /// # Returns
    ///
    /// Mutable reference to `self`
    pub fn push_input(&mut self, input: Input) -> &mut Self {
        let id = input.id();

        self.insert_input(id, input.into_deferred())
            .unwrap();

        self
    }

    pub fn insert_output(&mut self, id: IdType, output: Def<Output>) -> Result<Def<Output>, ErrorType> {
        match self.outputs.entry(id) {
            Entry::Occupied(_) => Err(Error::new(ErrorKind::ContainerError, "Output already exists")),
            Entry::Vacant(entry) => Ok(entry.insert(output).clone()),
        }
    }

    /// Store [`Output`] in internal collection
    ///
    /// # Parameters
    ///
    /// - `device`: [`Output`] device guarded by [`Def`]
    ///
    /// # Panics
    ///
    /// Panic is raised if `device` can't be locked.
    pub fn push_output(&mut self, device: Output) -> &mut Self {
        let id = device.id();

        self.insert_output(id, device.into_deferred())
            .unwrap();

        self
    }

    /// Facade to return operating frequency
    pub fn interval(&self) -> Duration {
        self.settings.interval
    }

    pub fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    /// Load all device logs
    ///
    /// # Errors
    /// Returns an error if any single load fails. However, failure does not prevent loading of
    /// other device logs.
    fn load_logs(&self, path: &Option<String>) -> Result<(), ErrorType> {
        let mut results = Vec::new();

        for device in self.outputs.values() {
            let binding = device.try_lock().unwrap();
            if binding.has_log() {
                let result = binding.log().unwrap()
                    .try_lock().unwrap()
                    .load(path);
                results.push(result);
            }
        }

        for device in self.inputs.values() {
            let binding = device.try_lock().unwrap();
            if binding.has_log() {
                let result = binding.log().unwrap()
                    .try_lock().unwrap()
                    .load(path);
                results.push(result);
            }
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

        for device in self.inputs.values() {
            let binding = device.try_lock().unwrap();
            if binding.has_log() {
                let result = binding.log().unwrap()
                    .try_lock().unwrap()
                    .save(path);
                results.push(result);
            }
        }

        for device in self.outputs.values() {
            let binding = device.try_lock().unwrap();
            if binding.has_log() {
                let result = binding.log().unwrap()
                    .try_lock().unwrap()
                    .save(path);
                results.push(result);
            }
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
            }
        }
    }

    pub fn attempt_routines(&self) {
        for device in self.inputs.values() {
            let mut binding = device.try_lock().unwrap();
            if let Some(publisher) = binding.publisher_mut() {
                publisher.attempt_routines()
            }
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
    use crate::io::{Device, IdType, Input, Output};

    #[test]
    /// Test that constructor accepts `name` as `&str` or `String`
    fn new_name_parameter() {
        Group::new("as &str", None);
        Group::new(String::from("as String"), None);
    }

    #[test]
    fn insert_input() {
        const ITERATIONS: u32 = 15;
        let mut group = Group::new("name", None);

        assert_eq!(0, group.inputs.len());

        for id in 0..ITERATIONS {
            let input = Input::new("", id, None).into_deferred();

            assert!(
                group.insert_input(id, input)
                    .is_ok()
            );
            assert_eq!(
                (id + 1) as usize,
                group.inputs.len()
            );
        }

        for id in 0..ITERATIONS {
            let input = Input::new("", id, None).into_deferred();

            assert!(
                group.insert_input(id, input)
                    .is_err()
            );
            assert_eq!(
                ITERATIONS as usize,
                group.inputs.len()
            );
        }
    }

    #[test]
    fn insert_output() {
        const ITERATIONS: u32 = 15;

        let mut group = Group::new("name", None);

        assert_eq!(0, group.outputs.len());

        for id in 0..ITERATIONS {
            let output = Output::new("", id, None).into_deferred();

            assert!(
                group.insert_output(id as IdType, output)
                    .is_ok()
            );
            assert_eq!(
                (id + 1) as usize,
                group.outputs.len()
            );
        }

        // check adding duplicates
        for id in 0..ITERATIONS {
            let output = Output::new("", id, None).into_deferred();

            assert!(
                group.insert_output(id as IdType, output)
                    .is_err()
            );
            assert_eq!(
                ITERATIONS as usize,
                group.outputs.len()
            );
        }
    }

    #[test]
    fn push_input() {
        let mut group = Group::new("name", None);

        assert_eq!(0, group.inputs.len());

        for id in 0..15 {
            group.push_input(Input::new("", id, None));

            assert_eq!(
                (id + 1) as usize,
                group.inputs.len()
            );
        }
    }

    #[test]
    #[should_panic]
    fn push_input_panics() {
        let mut group = Group::new("name", None);
        group.push_input(Input::new("", 0, None));
        group.push_input(Input::new("", 0, None));
    }

    #[test]
    fn push_output() {
        let mut group = Group::new("name", None);

        assert_eq!(0, group.outputs.len());

        for id in 0..15 {
            group.push_output(Output::new("", id, None));

            assert_eq!(
                (id + 1) as usize,
                group.outputs.len()
            );
        }
    }

    #[test]
    #[should_panic]
    fn push_output_panics() {
        let mut group = Group::new("name", None);
        group.push_output(Output::new("", 0, None));
        group.push_output(Output::new("", 0, None));
    }

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

        remove_dir_all(group.dir().parent().unwrap()).unwrap();
    }
}
