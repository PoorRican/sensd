use crate::action::{Action, BoxedAction};
use crate::helpers::Def;
use crate::io::{DeferredDevice, DeviceType, DeviceWrapper, IOEvent, RawValue};

/// Subscriber abstracting a PID controller
pub struct PIDMonitor {
    name: String,
    _threshold: RawValue,

    // TODO: check that device is output
    output: Def<DeviceType>,
}

impl Action for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
    }

    fn set_output(mut self, device: DeferredDevice) -> Self where Self: Sized {
        if device.is_output() {
            self.output = device;
            self
        } else {
            panic!("device is not output!")
        }
    }

    fn output(&self) -> Option<DeferredDevice> {
        Some(self.output.clone())
    }

    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
