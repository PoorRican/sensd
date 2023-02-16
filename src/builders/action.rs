use crate::action::{BaseCommandFactory, Comparison, Publisher, PublisherInstance, SubscriberType,
                    ThresholdNotifier, ThresholdNotifierFactory};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOType, InputType};

/// Assist the user in dynamically initializing a single publisher for a single input.
/// Since an abstract input only uses a single publisher, helper functions help build
/// subscribers by dynamically building commands.
///
/// # Notes
/// Return types should be checked here, if anywhere.
pub struct ActionBuilder {
    input: Deferred<InputType>,
    publisher: Deferred<PublisherInstance>,
    // TODO: add reference to `PollGroup`
}
impl ActionBuilder {
    pub fn new(input: Deferred<InputType>) -> Self {
        let publisher = Self::build_publisher();
        Self { input, publisher }
    }

    /// Initialize and return a deferred `PublisherInstance`
    fn build_publisher() -> Deferred<PublisherInstance> {
        let binding = PublisherInstance::default();
        binding.deferred()
        // TODO: add publisher to `PollGroup`
    }

    /// Silently add to publisher.
    /// Existing publisher is not overwritten, however any returned error is muted.
    /// Future updates will return a reference to the existing publisher. However, this shouldn't be
    /// necessary for instances built with `ActionBuilder`.
    fn check_publisher(&mut self) {
        let mut binding = self.input.lock().unwrap();
        if binding.has_publisher() == false {
            let publisher: Deferred<PublisherInstance> = self.publisher.clone();

            binding.add_publisher(publisher).unwrap()
        }
    }

    /// Associate a `ThresholdMonitor` to input
    pub fn add_threshold(
        &mut self,
        name: &str,
        threshold: IOType,
        trigger: Comparison,
        factory: BaseCommandFactory,
    ) {
        self.check_publisher();

        let subscriber = ThresholdNotifier::new(name.to_string(), threshold, trigger, factory);
        let deferred = subscriber.deferred();

        // add subscriber to publisher
        self.publisher.lock().unwrap().subscribe(deferred.clone());

        // add reverse reference to publisher from subscriber
        deferred
            .lock()
            .unwrap()
            .add_publisher(self.publisher.clone());

        println!("Initialized and setup up subscriber: {}", name);
    }
}
