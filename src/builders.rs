use std::sync::{Arc, Mutex};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Comparison, Device, GenericInput, IdType, InputType, IOKind, IOType, BaseCommandFactory, SimpleNotifier, ThresholdNotifier};
use crate::io::{Publisher, PublisherInstance};
use crate::settings::Settings;
use crate::storage::OwnedLog;

/// Init input and `OwnedLog`, then set owner on log. Return deferred log and deferred input.
pub fn input_log_builder(
    name: &str,
    id: &IdType,
    kind: &Option<IOKind>,
    settings: Option<Arc<Settings>>,
) -> (Deferred<OwnedLog>, Deferred<InputType>) {
    let log = Arc::new(Mutex::new(OwnedLog::new(*id, settings)));
    let input = GenericInput::new(name.to_string(), *id, *kind, log.clone());

    let wrapped = input.deferred();
    log.lock().unwrap().set_owner(wrapped.clone());

    (log, wrapped)
}

pub fn pubsub_builder(input: Deferred<InputType>, name: String, threshold: IOType, trigger: Comparison, factory: BaseCommandFactory<IOType, IOType>) {
    let binding = PublisherInstance::default();
    let publisher = binding.deferred();

    // attempt to add publisher. Existing publisher is not overwritten.
    let _ = input.try_lock().unwrap().add_publisher(publisher.clone());

    let notifier = ThresholdNotifier::new(
        name.clone(),
        threshold,
        publisher.clone(),
        trigger,
        factory,
    );
    let deferred = notifier.deferred();
    let mut binding = publisher.try_lock().unwrap();
    binding.subscribe(deferred);

    println!("Initialized and setup up subscriber: {}", name);
}