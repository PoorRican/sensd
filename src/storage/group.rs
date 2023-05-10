use crate::errors::ErrorType;
use crate::helpers::check_results;
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

    /// Immutable storage of runtime settings
    settings: Arc<Settings>,

    interval: Duration,

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
        let next_execution = self.last_execution + *self.interval();

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

    /// Primary constructor.
    ///
    /// Since `settings` represents individual runtime settings, default is used. Setter functions may be
    /// used to manipulate settings and populate containers.
    ///
    /// # Parameters
    ///
    /// - `name`: Name of group used for directory/file naming.
    ///
    /// # Returns
    ///
    /// Initialized `Group` with `name, default settings, and empty containers.
    ///
    /// # See Also
    ///
    /// [`Settings] for runtime settings options.
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>
    {
        let interval = Duration::seconds(5);
        let settings = Arc::new(Settings::default());
        let last_execution = Utc::now() - interval;

        let inputs = <DeviceContainer<IdType, Input>>::default();
        let outputs = <DeviceContainer<IdType, Output>>::default();

        Self {
            name: name.into(),
            interval,
            settings,
            last_execution,
            inputs,
            outputs,
        }
    }

    /// Alternate constructor with `settings` parameter
    ///
    /// Setter methods should be used to populate containers and manipulate `settings`.
    ///
    /// # Parameters
    ///
    /// - `name`: Name of group used for directory/file naming.
    /// - `settings`: Pre-existing runtime settings
    ///
    /// # Returns
    ///
    /// Initialized `Group` using given `name` and `settings`, with empty containers
    ///
    /// # See Also
    ///
    /// [`Settings] for runtime settings options.
    pub fn with_settings<N, S>(name: N, settings: S) -> Self
        where
            N: Into<String>,
            S: Into<Option<Arc<Settings>>>
    {
        let mut group = Self::new(name.into());
        if let Some(inner) = settings.into() {
            group.set_settings(inner)
        }
        group
    }

    pub fn with_interval<N>(name: N, interval: Duration) -> Self
        where
            N: Into<String>,
    {
        let mut group = Self::new(name.into());
        group.set_interval(interval);

        group
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
    pub fn push_input(&mut self, device: Input) -> &mut Self {
        let id = device.id();

        device.set_settings(self.settings.clone());

        self.inputs.insert(id, device.into_deferred())
            .unwrap();

        self
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

        device.set_settings(self.settings.clone());

        self.outputs.insert(id, device.into_deferred())
            .unwrap();

        self
    }

    /// Dedicated directory for [`Group`]
    ///
    /// The dedicated directory for a [`Group`] is simply a sub-directory in the global path.
    pub fn dir(&self) -> PathBuf {
        let root = self.settings.root_path();
        let path = Path::new(root.as_str());
        path.join(self.name.as_str())
    }

    /// Attempt to create root data directory
    pub fn init_root(self) -> Self {
        let path = self.dir();
        match path.exists() {
            true => (),
            false => {
                create_dir_all(path).expect("Could not create root data directory");
            }
        };
        self
    }

    pub fn attempt_routines(&self) {
        for device in self.inputs.values() {
            let mut binding = device.try_lock().unwrap();
            if let Some(publisher) = binding.publisher_mut() {
                publisher.attempt_routines()
            }
        }
    }

    //
    // Getters

    /// Getter for `name`
    ///
    /// # Returns
    ///
    /// Immutable reference to `name`
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    /// Getter for `interval`
    ///
    /// # Returns
    ///
    /// Immutable reference to `interval`
    pub fn interval(&self) -> &Duration {
        &self.interval
    }

    /// Setter for `interval`
    ///
    /// # Parameters
    ///
    /// - `interval`: any value that can be coerced into `Interval`
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval
    }

    /// Getter for `settings`
    ///
    /// # Returns
    ///
    /// An `Arc` reference to `Settings`
    pub fn settings(&self) -> Arc<Settings> {
        self.settings.clone()
    }

    //
    // Setters

    /// Setter for `name`
    ///
    /// # Parameters
    ///
    /// - `name`: new name for group. Uses `Into<_>` to coerce into `String`.
    pub fn set_name<N>(&mut self, name: N)
        where
            N: Into<String>
    {
        self.name = name.into();
    }

    /// Setter for settings
    ///
    /// # Parameters
    ///
    /// - `settings`: `Arc` reference to new settings.
    pub fn set_settings(&mut self, settings: Arc<Settings>) {
        self.settings = settings
    }
}

/// Only save and load log data since [`Group`] is statically initialized
/// If `&None` is given to either methods, then current directory is used.
impl Persistent for Group {
    /// Save all device logs
    ///
    /// # Errors
    ///
    /// Returns an error if any single save fails. However, failure does not prevent saving of
    /// other device logs.
    fn save(&self, path: &Option<String>) -> Result<(), ErrorType> {
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

    /// Load all device logs
    ///
    /// # Errors
    /// Returns an error if any single load fails. However, failure does not prevent loading of
    /// other device logs.
    fn load(&mut self, path: &Option<String>) -> Result<(), ErrorType> {
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
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::Arc;

    use crate::settings::Settings;
    use crate::storage::Group;

    use std::fs::remove_dir_all;
    use chrono::Duration;
    use crate::io::{Device, Input, Output};

    #[test]
    /// Test that constructor accepts `name` as `&str` or `String`
    fn new_name_parameter() {
        Group::new("as &str");
        Group::new(String::from("as String"));
    }

    #[test]
    /// Test that alternate constructor sets settings
    fn with_settings() {
        let mut settings = Settings::default();
        settings.version = "blah".into();
        let settings = Arc::new(settings);

        let group = Group::with_settings(
            "",
            settings.clone());
        assert_eq!(settings, group.settings());
    }

    #[test]
    fn with_interval() {
        let interval = Duration::nanoseconds(30);

        let group = Group::with_interval(
            "",
            interval);
        assert!(interval.eq(group.interval()))
    }

    #[test]
    fn push_input() {
        let mut group = Group::new("name");

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
        let mut group = Group::new("name");
        group.push_input(Input::new("", 0, None));
        group.push_input(Input::new("", 0, None));
    }

    #[test]
    fn push_output() {
        let mut group = Group::new("name");

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
        let mut group = Group::new("name");
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
        let group = Group::with_settings(GROUP_NAME, Arc::new(_settings));

        // assert directory path is correct
        assert_eq!(expected.to_str().unwrap(), group.dir().to_str().unwrap());
    }

    /// Test [`Group::init_root()`]
    #[test]
    fn test_init_root() {
        const DIR_NAME: &str = "test_root";
        const GROUP_NAME: &str = "main";

        // init `Group` and settings
        let dir_name = String::from(DIR_NAME);
        let mut _settings = Settings::default();
        _settings.set_root(dir_name.clone());

        let group = Group::new(GROUP_NAME)
            .init_root();
        assert!(group.dir().exists());

        remove_dir_all(group.dir().parent().unwrap()).unwrap();
    }
}
