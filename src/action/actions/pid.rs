use crate::action::{Action, BoxedAction};
use crate::helpers::Def;
use crate::io::{DeviceType, IOEvent, RawValue};

/// Subscriber abstracting a PID controller
pub struct PIDMonitor {
    name: String,
    _threshold: RawValue,

    // TODO: check that device is output
    _output: Def<DeviceType>,
}

impl Action for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
    }

    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
