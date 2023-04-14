use crate::action::IOCommand;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{
    DeferredDevice, Device, DeviceType, GenericInput, GenericOutput, IODirection, IOKind, IdType,
};
use crate::settings::Settings;
use crate::storage::Log;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex, Weak};

pub struct DeviceLogBuilder {
    device: DeferredDevice,
    log: Deferred<Log>,
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

        let log: Deferred<Log>;

        let device = match direction {
            IODirection::Output => {
                let mut output = GenericOutput::new(name.to_string(), *id, *kind, None);
                log = output.init_log(settings);
                DeviceType::Output(output)
            }
            IODirection::Input => {
                let mut input = GenericInput::new(name.to_string(), *id, *kind, None);
                log = input.init_log(settings);
                DeviceType::Input(input)
            }
        };

        // wrap device
        let wrapped = device.deferred();

        // set log owner
        let downgraded: Weak<Mutex<DeviceType>> = Arc::downgrade(&wrapped.clone());
        log.lock().unwrap().set_owner(downgraded);

        Self {
            device: wrapped,
            log,
            command: command.clone(),
        }
    }

    pub fn get(&self) -> (DeferredDevice, Deferred<Log>) {
        (self.device.clone(), self.log.clone())
    }

    /// Create a `GPIOCommand` from `command` field.
    ///
    /// # Notes
    /// Alignment of command and device type is checked in `::new()` by `check_command_alignment()`
    pub fn setup_command(&self) {
        let command = self.command.clone();

        let mut binding = self.device.lock().unwrap();
        let device = binding.deref_mut();

        match device {
            DeviceType::Input(inner) => inner.add_command(command),
            DeviceType::Output(inner) => inner.add_command(command),
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
