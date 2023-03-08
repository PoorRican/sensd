use crate::action::{
    ThresholdFactory, Comparison, Publisher, PublisherInstance, ThresholdAction,
};
use crate::errors::{Error, ErrorKind, ErrorType};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{DeferredDevice, DeviceType, DeviceWrapper, IOType};
use std::ops::DerefMut;

/// Assist the user in dynamically initializing a single publisher for a single input.
/// Since an abstract input only uses a single publisher, helper functions help build
/// subscribers by dynamically building commands.
///
/// # Notes
/// Return types should be checked here, if anywhere.
pub struct ActionBuilder {
    input: DeferredDevice,
    publisher: Deferred<PublisherInstance>,
    // TODO: add reference to `PollGroup`
}
impl ActionBuilder {
    /// Create a new builder for a given device
    /// This starts the process of adding pubs/subscribers
    /// `Err` is returned if passed device is not input.
    /// # Args
    /// - device: Device to add pub/subs. Should be Input
    pub fn new(device: DeferredDevice) -> Result<Self, ErrorType> {
        if device.is_output() {
            return Err(Error::new(
                ErrorKind::DeviceError,
                "Passed device is output. Expected input.",
            ));
        }
        let publisher = Self::build_publisher();
        Ok(Self {
            input: device,
            publisher,
        })
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
        threshold: IOType,
        trigger: Comparison,
        factory: ThresholdFactory,
    ) {
        // TODO: raise an error if device type is not numeric (ie: IOType::Boolean)
        self.check_publisher();

        let _subscriber = ThresholdAction::new(name.to_string(), threshold, trigger, factory);
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
