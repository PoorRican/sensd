use crate::action::{
    EvaluationFunction, Comparison, Publisher, PublisherInstance, ThresholdAction,
};
use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{DeferredDevice, DeviceType, DeviceWrapper, RawValue, IdType, IODirection};
use crate::storage::{PollGroup, MappedCollection};
use std::ops::DerefMut;

/// Assist the user in dynamically initializing a single publisher for a single input.
/// Since an input device has a one-to-one relationship with a `PublisherInstance`, helper functions
/// help build subscribers by dynamically building commands.
///
/// # Notes
/// Return types should be checked here, if anywhere.
pub struct ActionBuilder {
    input: DeferredDevice,
    output: Option<DeferredDevice>,

    publisher: Deferred<PublisherInstance>,
}

impl ActionBuilder {
    /// Create a new builder for the given input device
    ///
    /// This starts the process of adding pubs/subscribers
    /// `Err` is returned if passed device is not input.
    ///
    /// # Notes
    /// This constructor method may be used directly, or alternatively
    /// `ActionBuilder::from_group()` may be used for indirectly constructing builder by only
    /// supplying a reference to `PollGroup` and device id.
    ///
    /// # Args
    /// - device: Device to add pub/subs. Should be Input
    pub fn new(input: DeferredDevice) -> Result<Self, ErrorType> {
        if input.is_output() {
            return Err(Error::new(
                ErrorKind::DeviceError,
                "Passed device is output. Expected input.",
            ));
        }
        let publisher = Self::build_publisher();
        Ok(Self {
            input,
            output: None,
            publisher,
        })
    }

    /// Helper function that extracts deferred device from `PollGroup`
    ///
    /// Device type container is determined by `direction`: inputs are taken from
    /// `PollGroup::inputs`, and outputs are taken from `PollGroup::outputs`.
    ///
    /// Extracting devices from respective `PollGroup` containers grants indirection between device
    /// ininitialization and pub/sub building.
    ///
    /// # Args
    /// poller: Reference to `PollGroup`
    /// direction: IODirection which determines which container is used to retrieve deferred device
    /// id: id of device
    ///
    /// # Returns
    /// Result with deferred device or `ErrorType`.
    ///
    /// If id doesn't exist in `PollGroup::inputs`, then an error with `DeviceKind::ContainerError`
    /// and the appropriate message is returned.
    fn extract_device(poller: &PollGroup,
                      direction: IODirection,
                      id: IdType) -> Result<DeferredDevice, ErrorType> {

        let result;
        match direction {
            IODirection::Output => {
                result = poller.inputs.get(id);
            }
            IODirection::Input => {
                result = poller.outputs.get(id);
            }
        };

        if let Some(device) = result {
            Ok(device.clone())
        } else {
            Err(Error::new(ErrorKind::ContainerError,
                           "Incorrect id passed to `ActionBuilder::extract_device()`"))
        }

    }

    /// Indirect constructor for `ActionBuilder`.
    ///
    /// Allows input device to be constructed and added to `PollGroup::inputs` seperately.
    /// Therefore, deferred device does not need to remain in scope and passed to
    /// `::new()`. This allows device building to be handled by an external function. Only the
    /// device id needs remain in scope.
    ///
    /// # Args
    /// poller: Reference to `PollGroup`
    /// id: id of device
    ///
    /// # Returns
    /// Result containing `ActionBuilder` or `ErrorType`.
    ///
    /// If id doesn't exist in `PollGroup::inputs`, then an error with `DeviceKind::ContainerError`
    /// and the appropriate message is returned.
    pub fn from_group(poller: &PollGroup, id: IdType) -> Result<Self, ErrorType> {
        let input = Self::extract_device(poller, IODirection::Input, id)?;
        Self::new(input)
    }

    /// Associate a deferred output to builder.
    pub fn attach_output(&mut self, poller: &PollGroup, id: IdType) -> Result<(), ErrorType> {
        let output = Self::extract_device(poller, IODirection::Output, id)?;
        self.output = Some(output);
        Ok(())
    }

    /// Initialize and return a deferred `PublisherInstance`
    fn build_publisher() -> Deferred<PublisherInstance> {
        let binding = PublisherInstance::default();
        binding.deferred()
        // TODO: add publisher to `PollGroup`
    }

    /// Silently add publisher to input device.
    /// Existing publisher is not overwritten as any returned error is ignored.
    /// Future updates will return a reference to the existing publisher. However, this shouldn't be
    /// necessary for instances built with `ActionBuilder`.
    fn check_publisher(&self) {
        let mut binding = self.input.lock().unwrap();
        if let DeviceType::Input(inner) = binding.deref_mut() {
            if inner.has_publisher() == false {
                let publisher: Deferred<PublisherInstance> = self.publisher.clone();

                inner.add_publisher(publisher).unwrap()
            }
        }
    }

    /// Associate a `ThresholdMonitor` to input
    pub fn add_threshold(
        &mut self,
        name: &str,
        threshold: RawValue,
        trigger: Comparison,
        evaluator: EvaluationFunction,
        output: Option<DeferredDevice>,
    ) {
        // TODO: raise an error if device type is not numeric (ie: RawValue::Boolean)
        // TODO: check that `evaluator` is `EvaluationFunction::Threshold`
        self.check_publisher();

        let command;
        // construct simple command that writes to output device
        if let Some(output) = output {
            let _command = move |val: RawValue| {
                let mut binding = output.try_lock().unwrap();
                let device = binding.deref_mut();
                if let DeviceType::Output(inner) = device {
                    inner.write(val)
                } else {
                    Err(Error::new(ErrorKind::DeviceError,
                                   "Command encountered error. Expected Output device."))
                }
            };
            command = Some(_command);
        } else {
            command = None;
        }

        // construct subscriber
        let _subscriber = ThresholdAction::new(name.to_string(), threshold, trigger, command, evaluator);
        let subscriber = _subscriber.deferred();

        // add subscriber to publisher
        self.publisher.lock().unwrap().subscribe(subscriber.clone());

        // add reverse reference to publisher from subscriber
        subscriber
            .lock()
            .unwrap()
            .add_publisher(self.publisher.clone());

        println!("Initialized and setup up subscriber: {}", name);
    }
}
