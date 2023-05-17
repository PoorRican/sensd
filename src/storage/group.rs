use crate::errors::ErrorType;
use crate::helpers::check_results;
use crate::io::{Device, DeviceContainer, DeviceGetters, IdType, Input, Output};
use crate::settings::DATA_ROOT;
use crate::storage::{Directory, Persistent};

use chrono::{DateTime, Duration, Utc};
use std::fs::create_dir_all;
use std::path::PathBuf;
use crate::storage::directory::RootPath;

/// High-level container to manage multiple [`Device`] objects, logging, and
/// actions.
///
/// # Getting Started
///
/// ## Initialization
///
/// To establish a root directory for storing logs and other data, the
/// builder method [`Group::set_root()`] is used to set path to directory.
/// Then, [`Group::init_dir()`] ensures that directory exists and is valid:
///
/// ```
/// use std::path::PathBuf;
/// use std::sync::Arc;
/// use sensd::storage::{Directory, Group};
///
/// let root_dir = Arc::new(PathBuf::from("/tmp/root_dir/"));
/// let group =
///     Group::new("")
///         .set_root(root_dir.clone())
///         .init_dir();
///
/// assert_eq!(root_dir, group.root_dir());
/// ```
///
/// Similarly, the [`Group::with_root()`] alternate constructor allows
/// [`RootPath`] to be passed as
/// a parameter. However, the builder method [`Group::init_dir()`] still
/// needs to be explicitly chained.
///
/// ## Adding Devices
///
/// Using and adding devices to [`Group`] is most easily accomplished
/// via the builder pattern:
///
/// ```
/// use sensd::io::{Input, Output};
/// use sensd::storage::Group;
///
/// let input = Input::default();
/// let output = Output::default();
///
/// let mut group = Group::new("");
/// group.push_input(input);
/// group.push_output(output);
/// ```
///
/// ## Main Operation / Polling
///
/// [`Group::poll()`] and [`Group::attempt_routines()`] are the primary callables for function. Both functions are
/// called on different intervals. The execution of [`Group::poll()`] is dictated by the interval stored in
/// runtime settings. Conversely, [`Group::attempt_routines()`] should be executed as often as possible to
/// maintain timing accuracy.
///
/// Both [`Group::poll()`] and [`Group::attempt_routines()`] are high-level functions whose returned values
/// can mainly be ignored. Future revisions will add failure log functionality in the event of failure or
/// misconfiguration.
///
/// In order to set `interval`, either the alternate constructor [`Group::with_interval()`] can be utilized,
/// or the builder method [`Group::set_interval()`] both result in user configured `interval`:
///
pub struct Group {
    /// Name used to identify this specific device grouping.
    ///
    /// This is mainly used for sub-directory labeling
    name: String,
    /// Buffer to store time of the last successful poll.
    last_execution: DateTime<Utc>,

    /// Immutable storage of runtime settings
    root: RootPath,

    interval: Duration,

    pub inputs: DeviceContainer<IdType, Input>,
    pub outputs: DeviceContainer<IdType, Output>,
}

impl Group {
    /// Primary callable to iterate through input device container once.
    ///
    /// [`Input::read()`] is called once on each input device at a frequency of
    /// [`Group::interval()`]. Generated [`crate::io::IOEvent`] instances are
    /// handled by [`Input::read()`].
    ///
    /// Failure of any individual read does not halt execution. Instead, errors
    /// from [`Input::read()`] are returned as a [`Vec`].
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    ///
    /// - `Ok` when poll has been executed. `Ok` value will contain any errors
    ///   that arose.
    /// - `Err` when poll was not executed
    pub fn poll(&mut self) -> Result<Vec<ErrorType>, ()> {
        let mut errors = Vec::new();
        let next_execution = self.last_execution + *self.interval();

        if next_execution <= Utc::now() {
            for input in self.inputs.values_mut() {
                let mut binding = input.try_lock().unwrap();
                let result = binding.read();

                // Add errors to array
                if result.is_err() {
                    errors.push(result.err().unwrap());
                }
            }
            self.last_execution = next_execution;
            Ok(errors)
        } else {
            Err(())
        }
    }

    /// Primary constructor.
    ///
    /// [`Group::set_root()`] or [`Group::set_root_ref()`] should be used to set root path
    ///
    /// # Parameters
    ///
    /// - `name`: Name of group used for directory/file naming.
    ///
    /// # Returns
    ///
    /// Initialized [`Group`] with `name`, default root directory, and empty containers.
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::storage::Group;
    ///
    /// let name = "name";
    /// let group = Group::new(name);
    ///
    /// assert_eq!(name, group.name());
    /// ```
    pub fn new<N>(name: N) -> Self
    where
        N: Into<String>
    {
        let interval = Duration::seconds(5);
        let last_execution = Utc::now() - interval;

        let inputs = <DeviceContainer<IdType, Input>>::default();
        let outputs = <DeviceContainer<IdType, Output>>::default();

        let root = PathBuf::from(DATA_ROOT).into();

        Self {
            name: name.into(),
            interval,
            root,
            last_execution,
            inputs,
            outputs,
        }
    }

    /// Alternate constructor with `root` parameter
    ///
    /// # Parameters
    ///
    /// - `name`: Name of group used for directory/file naming.
    /// - `root`: Desired root path to override default
    ///
    /// # Returns
    ///
    /// Initialized [`Group`] with `name`, a `root` directory, and empty containers
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use std::sync::Arc;
    /// use sensd::storage::{Group, Directory};
    ///
    /// let root_dir = Arc::new(PathBuf::from("/tmp/root_dir/"));
    /// let group =
    ///     Group::with_root("", root_dir.clone());
    ///
    /// assert_eq!(root_dir, group.root_dir());
    /// ```
    pub fn with_root<N>(name: N, root: RootPath) -> Self
        where
            N: Into<String>,
    {
        let mut group = Self::new(name.into());

        group.set_root_ref(root);

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
    /// [`Device::set_root()`] is called to pass settings to device.
    ///
    /// # Parameters
    ///
    /// - `device`: [`Input`] device to be added
    ///
    /// # Returns
    ///
    /// Mutable reference to `self`
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::Input;
    /// use sensd::storage::Group;
    ///
    /// let input = Input::default();
    ///
    /// let mut group = Group::new("");
    /// group.push_input(input);
    ///
    /// assert_eq!(group.inputs.len(), 1);
    /// assert_eq!(group.outputs.len(), 0);
    /// ```
    pub fn push_input(&mut self, device: Input) -> &mut Self {
        let id = device.id();

        device.set_root(self.root.clone());

        self.inputs.insert(id, device.into_deferred())
            .unwrap();

        self
    }

    /// Store [`Output`] in internal collection
    ///
    /// [`Device::set_root()`] is called to pass settings to device.
    ///
    /// # Parameters
    ///
    /// - `device`: [`Output`] device guarded by [`crate::helpers::Def`]
    ///
    /// # Example
    ///
    /// ```
    /// use sensd::io::Output;
    /// use sensd::storage::Group;
    ///
    /// let output = Output::default();
    ///
    /// let mut group = Group::new("");
    /// group.push_output(output);
    ///
    /// assert_eq!(group.outputs.len(), 1);
    /// assert_eq!(group.inputs.len(), 0);
    /// ```
    pub fn push_output(&mut self, device: Output) -> &mut Self {
        let id = device.id();

        device.set_root(self.root.clone());

        self.outputs.insert(id, device.into_deferred())
            .unwrap();

        self
    }

    /// Attempt to create dedicated directory for this `Group`
    ///
    /// If directory already exists, then this method silently fails.
    ///
    /// # Panics
    ///
    /// This method panics if an error occurs when creating directory (other than directory
    /// already existing). This could happen if write permissions are misconfigured.
    pub fn init_dir(self) -> Self {
        let path = self.full_path();
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
    /// # Notes
    ///
    /// Since this is frequently used in iterators and polling, this
    /// method is marked inline to avoiding jumping in memory.
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
    /// - `interval`: any value that can be coerced into [`Duration`]
    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval
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

    /// Setter for `root_path` that can be used as a builder function.
    ///
    /// # Parameters
    ///
    /// - `root`: New path to global root dir
    ///
    /// # Returns
    ///
    /// Ownership of `Self`. This is to be used as a builder function using method chaining.
    pub fn set_root(mut self, root: RootPath) -> Self {
        self.set_root_ref(root);

        self
    }

    /// Setter for `root_path`
    ///
    /// This does not take ownership of `self`, unlike [`Group::set_root()`].
    ///
    /// Propagates changes to internal device containers using [`DeviceContainer::set_root()`]
    ///
    /// # Parameters
    ///
    /// - `root`: New path to global root dir
    pub fn set_root_ref(&mut self, root: RootPath) {
        self.root = root.clone();

        self.inputs.set_root(root.clone());
        self.outputs.set_root(root.clone());
    }
}

/// Only save and load log data since [`Group`] is statically initialized
/// If `&None` is given to either methods, then current directory is used.
impl Persistent for Group {
    /// Save all device logs
    ///
    /// # Errors
    ///
    /// Returns an error if any single save fails. However, failure is silent and
    /// does not prevent saving other device logs.
    ///
    /// # Panics
    ///
    /// Panics when any single input or output device cannot be locked.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing:
    ///
    /// - `Ok` that is empty when saving occurred without error.
    /// - `Err` containing the first error stored. There may be more errors that were
    ///   not returned. An error occurring does not halt saving other logs.
    fn save(&self) -> Result<(), ErrorType> {
        let mut results = Vec::new();

        for device in self.inputs.values() {
            let binding = device.try_lock().unwrap();
            results.push(
                binding.save());
        }

        for device in self.outputs.values() {
            let binding = device.try_lock().unwrap();
            results.push(
                binding.save());
        }

        check_results(&results)
    }

    /// Load all device logs
    ///
    /// # Errors
    ///
    /// Returns an error if any single load fails. However, failure is silent and does not prevent
    /// loading other device logs.
    ///
    /// # Panics
    ///
    /// Panics when any single input or output device cannot be locked.
    ///
    /// # Returns
    ///
    /// A [`Result`] containing:
    ///
    /// - `Ok` that is empty when loading occurred without error.
    /// - `Err` containing the first error stored. There may be more errors that were
    ///   not returned. An error occurring does not halt loading other logs.
    fn load(&mut self) -> Result<(), ErrorType> {
        let mut results = Vec::new();

        for device in self.outputs.values() {
            let mut binding = device.try_lock().unwrap();
            results.push(
                binding.load());
        }

        for device in self.inputs.values() {
            let mut binding = device.try_lock().unwrap();
            results.push(
                binding.load());
        }

        check_results(&results)
    }
}

impl Directory for Group {
    /// Getter for `root_path`
    ///
    /// This field represents the top-most directory and is where all dedicated directories
    /// for [`Group`]'s are located. For retrieving a path to save or retrieve data,
    /// use [`Group::full_path()`].
    ///
    /// # Returns
    ///
    /// `Option` of [`RootPath`] representing root data path of [`Group`] if set.
    fn root_dir(&self) -> RootPath {
        self.root.clone()
    }

    fn dir_name(&self) -> &String {
        self.name()
    }

    fn full_path(&self) -> PathBuf {
        self.root_dir()
            .join(self.dir_name())
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::sync::Arc;

    use crate::settings::Settings;
    use crate::storage::{Directory, Group};

    use std::fs::remove_dir_all;
    use chrono::Duration;
    use crate::io::{Device, Input, Output};
    use crate::storage::directory::RootPath;

    #[test]
    /// Test that constructor accepts `name` as `&str` or `String`
    fn new_name_parameter() {
        Group::new("as &str");
        Group::new(String::from("as String"));
    }

    #[test]
    /// Test that alternate constructor sets settings
    fn with_root() {
        let settings = Settings::default();

        let group = Group::with_root(
            "",
            settings.root_path());
        assert_eq!(settings.root_path(), group.root_dir());
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

    /// Test [`Group::full_path()`]
    #[test]
    fn test_dir() {
        const DIR_NAME: &str = "test_root";
        const GROUP_NAME: &str = "main";

        // init `Group` and settings
        let dir_name = Arc::from(PathBuf::from(DIR_NAME));

        let expected = Path::new(DIR_NAME).join(GROUP_NAME);
        let group = Group::with_root(GROUP_NAME, dir_name);

        // assert directory path is correct
        assert_eq!(expected.to_str().unwrap(), group.full_path().to_str().unwrap());
    }

    /// Test [`Group::init_dir()`]
    #[test]
    fn test_init_root() {
        const DIR_NAME: &str = "test_root";
        const GROUP_NAME: &str = "main";

        // init `Group` and settings
        let dir_name: RootPath = Arc::new(PathBuf::from(DIR_NAME));

        let group = Group::new(GROUP_NAME)
            .set_root(dir_name)
            .init_dir();

        assert!(group.full_path().exists());

        remove_dir_all(group.full_path().parent().unwrap()).unwrap();
    }
}
