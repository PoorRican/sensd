mod action;

use std::ops::DerefMut;
pub use action::*;

use std::sync::{Arc, Mutex, Weak};
use crate::action::{BaseCommandFactory, Comparison, ThresholdNotifier, Publisher,
                    PublisherInstance};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{DeferredDevice, Device, DeviceType, GenericInput, IdType, IOKind, IOType};
use crate::settings::Settings;
use crate::storage::OwnedLog;

/// Init input and `OwnedLog`, then set owner on log. Return deferred log and deferred input.
pub fn input_log_builder(
    name: &str,
    id: &IdType,
    kind: &Option<IOKind>,
    settings: Option<Arc<Settings>>,
) -> (Deferred<OwnedLog>, Deferred<DeviceType>) {
    let log = Arc::new(Mutex::new(OwnedLog::new(*id, settings)));
    let input = GenericInput::new(name.to_string(), *id, *kind, log.clone());

    let wrapped = input.deferred();
    let downgraded: Weak<Mutex<DeviceType>> = Arc::downgrade(&wrapped.clone());
    log.lock().unwrap().set_owner(downgraded);

    (log, wrapped)
}

pub fn pubsub_builder(input: DeferredDevice, name: String, threshold: IOType, trigger: Comparison,
                      factory: BaseCommandFactory) {
    let binding = PublisherInstance::default();
    let publisher = binding.deferred();

    // attempt to add publisher. Existing publisher is not overwritten.
    if let DeviceType::Input(inner) = input.lock().unwrap().deref_mut() {
        let _ = inner.add_publisher(publisher.clone());
    }

    let notifier = ThresholdNotifier::new(
        name.clone(),
        threshold,
        trigger,
        factory,
    );
    let deferred = notifier.deferred();
    let mut binding = publisher.try_lock().unwrap();
    binding.subscribe(deferred);

    println!("Initialized and setup up subscriber: {}", name);
}