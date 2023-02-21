mod action;

pub use action::*;

use std::sync::{Arc, Mutex, Weak};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Device, DeviceType, GenericInput, IdType, IOKind};
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