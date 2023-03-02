use crate::helpers::{Deferrable, Deferred};
use crate::io::{
    DeferredDevice, Device, DeviceType, GenericInput, GenericOutput, IODirection, IOKind, IdType,
};
use crate::settings::Settings;
use crate::storage::OwnedLog;
use std::sync::{Arc, Mutex, Weak};

pub struct DeviceLogBuilder {
    device: DeferredDevice,
    log: Deferred<OwnedLog>,
}

impl DeviceLogBuilder {
    pub fn new(
        name: &str,
        id: &IdType,
        kind: &Option<IOKind>,
        direction: &IODirection,
        settings: Option<Arc<Settings>>,
    ) -> Self {
        let log: Deferred<OwnedLog> = Arc::new(Mutex::new(OwnedLog::new(*id, settings)));
        let device = match direction {
            IODirection::Output => {
                let output = GenericOutput::new(name.to_string(), *id, *kind, log.clone());
                DeviceType::Output(output)
            }
            IODirection::Input => {
                let input = GenericInput::new(name.to_string(), *id, *kind, log.clone());
                DeviceType::Input(input)
            }
        };

        let wrapped = device.deferred();
        let downgraded: Weak<Mutex<DeviceType>> = Arc::downgrade(&wrapped.clone());
        log.lock().unwrap().set_owner(downgraded);
        Self {
            device: wrapped,
            log,
        }
    }

    pub fn get(&self) -> (DeferredDevice, Deferred<OwnedLog>) {
        (self.device.clone(), self.log.clone())
    }
}
