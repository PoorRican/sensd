use std::ops::{Deref, DerefMut};
use crate::action::{IOCommand, GPIOCommand};
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
    command: IOCommand,
}

impl DeviceLogBuilder {
    pub fn new(
        name: &str,
        id: &IdType,
        kind: &Option<IOKind>,
        direction: &IODirection,
        command: &IOCommand,
        settings: Option<Arc<Settings>>,
    ) -> Self {

        check_command_alignment(command, direction, name);

        let log: Deferred<OwnedLog> = Arc::new(Mutex::new(OwnedLog::new(*id, settings)));

        let device = match direction {
            IODirection::Output => {
                let output = GenericOutput::new(
                    name.to_string(),
                    *id,
                    *kind,
                    log.clone(),
                );
                DeviceType::Output(output)
            },
            IODirection::Input => {
                let input = GenericInput::new(
                    name.to_string(),
                    *id,
                    *kind,
                    log.clone(),
                );
                DeviceType::Input(input)
            },
        };

        // wrap device
        let wrapped = device.deferred();

        // set log owner
        let downgraded: Weak<Mutex<DeviceType>> = Arc::downgrade(&wrapped.clone());
        log.lock().unwrap().set_owner(downgraded);

        Self {
            device: wrapped,
            log,
            command: *command
        }
    }

    pub fn get(&self) -> (DeferredDevice, Deferred<OwnedLog>) {
        (self.device.clone(), self.log.clone())
    }

    /// Create a `GPIOCommand` from `command` field.
    ///
    /// # Notes
    /// Alignment of command and device type should be checked in `::new()` by `check_command_alignment()`
    pub fn setup_command(&self) {
        let gpio = GPIOCommand::new(self.command, self.device.clone());

        let mut binding = self.device.lock().unwrap();
        let device = binding.deref_mut();

        match device {
            DeviceType::Input(inner) => {
                inner.add_command(gpio)
            },
            DeviceType::Output(inner) => {
                inner.add_command(gpio)
            }
        }
    }
}

/// Check that `DeviceType` and `IOCommand` align
///
/// Program panics and dies if direction is misaligned.
fn check_command_alignment(command: &IOCommand, direction: &IODirection, name: &str) {
    if command.direction() != *direction {
        panic!("IOCommand type and `IODirection do not align for {}", name);
    }
}