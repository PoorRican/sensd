use crate::action::{Comparison, Publisher, PublisherInstance, Subscriber, ThresholdAction};
use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::Def;
use crate::io::{DeferredDevice, DeviceType, DeviceWrapper, RawValue, IdType, IODirection};
use crate::storage::{Group, MappedCollection};
use std::ops::DerefMut;

/// Builder class that builds and attaches subscribers for inputs.
///
/// A `PublisherInstance` is built and attached to input. Then subsequent methods are used to
/// build subscribers and associate with inputs and outputs.
///
/// TODO: Check that `RawValue` variants are compatible
pub struct ActionBuilder {
    input: DeferredDevice,
    output: Option<DeferredDevice>,

    publisher: Def<PublisherInstance>,
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
    /// supplying a reference to `Group` and device id.
    ///
    /// # Args
    /// - input: Device to add pub/subs
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

    /// Helper function that extracts deferred device from `Group`
    ///
    /// Device type container is determined by `direction`: inputs are taken from
    /// `Group::inputs`, and outputs are taken from `Group::outputs`.
    ///
    /// Extracting devices from respective `Group` containers grants indirection between device
    /// ininitialization and pub/sub building.
    ///
    /// # Args
    /// group: Reference to `Group`
    /// direction: IODirection which determines which container is used to retrieve deferred device
    /// id: id of device
    ///
    /// # Returns
    /// Result with deferred device or `ErrorType`.
    ///
    /// If id doesn't exist in `Group::inputs`, then an error with `DeviceKind::ContainerError`
    /// and the appropriate message is returned.
    fn extract_device(group: &Group,
                      direction: IODirection,
                      id: IdType) -> Result<DeferredDevice, ErrorType> {

        let result;
        match direction {
            IODirection::Output => {
                result = group.inputs.get(id);
            }
            IODirection::Input => {
                result = group.outputs.get(id);
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
    /// Allows input device to be constructed and added to `Group::inputs` seperately.
    /// Therefore, deferred device does not need to remain in scope and passed to
    /// `::new()`. This allows device building to be handled by an external function. Only the
    /// device id needs remain in scope.
    ///
    /// # Args
    /// group: Reference to `Group`
    /// id: id of device
    ///
    /// # Returns
    /// Result containing `ActionBuilder` or `ErrorType`.
    ///
    /// If id doesn't exist in `Group::inputs`, then an error with `DeviceKind::ContainerError`
    /// and the appropriate message is returned.
    pub fn from_group(group: &Group, id: IdType) -> Result<Self, ErrorType> {
        let input = Self::extract_device(group, IODirection::Input, id)?;
        Self::new(input)
    }

    /// Associate a deferred output to builder.
    pub fn attach_output(&mut self, group: &Group, id: IdType) -> Result<(), ErrorType> {
        let output = Self::extract_device(group, IODirection::Output, id)?;
        self.output = Some(output);
        Ok(())
    }

    /// Initialize and return a deferred `PublisherInstance`
    fn build_publisher() -> Def<PublisherInstance> {
        let binding = PublisherInstance::default();
        Def::new(binding)
        // TODO: add publisher to `Group`
    }

    /// Silently add publisher to input device.
    /// Existing publisher is not overwritten as any returned error is ignored.
    /// Future updates will return a reference to the existing publisher. However, this shouldn't be
    /// necessary for instances built with `ActionBuilder`.
    fn check_publisher(&self) {
        let mut binding = self.input.try_lock().unwrap();
        if let DeviceType::Input(inner) = binding.deref_mut() {
            if inner.has_publisher() == false {
                let publisher: Def<PublisherInstance> = self.publisher.clone();

                inner.add_publisher(publisher).unwrap()
            }
        }
    }

    /// Build and attach a `ThresholdAction` subscriber
    ///
    /// TODO: raise an error if device type is not numeric (ie: RawValue::Boolean)
    /// TODO: check that `evaluator` is `EvaluationFunction::Threshold`
    /// TODO: assert `output` is output-type
    pub fn add_threshold(
        &mut self,
        name: &str,
        threshold: RawValue,
        trigger: Comparison,
        output: Option<DeferredDevice>,
    ) {
        self.check_publisher();

        // construct subscriber
        let action = ThresholdAction::new(name.to_string(), threshold, trigger, output);
        let subscriber = Def::new(action.as_subscriber());

        // add subscriber to publisher
        self.publisher.try_lock().unwrap().subscribe(subscriber.clone());

        // add reverse reference to publisher
        subscriber
            .try_lock()
            .unwrap()
            .add_publisher(self.publisher.clone());

        println!("Initialized and setup up subscriber: {}", name);
    }
}
