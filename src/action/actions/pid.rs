use crate::action::{Action, BoxedAction};
use crate::helpers::Def;
use crate::io::{GenericOutput, IOEvent, RawValue};

/// Subscriber abstracting a PID controller
pub struct PIDMonitor {
    name: String,
    _setpoint: RawValue,

    // TODO: check that device is output
    output: Def<GenericOutput>,
}

impl Action for PIDMonitor {
    fn name(&self) -> &String {
        &self.name
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
    }

    fn set_output(mut self, device: Def<GenericOutput>) -> Self
    where
        Self: Sized,
    {
        self.output = device;
        self
    }

    fn output(&self) -> Option<Def<GenericOutput>> {
        Some(self.output.clone())
    }

    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
