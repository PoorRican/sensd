use crate::action::{Action, BoxedAction};
use crate::helpers::Def;
use crate::io::{Output, IOEvent, RawValue};

/// Subscriber abstracting a PID controller
pub struct PID {
    name: String,
    _setpoint: RawValue,

    // TODO: check that device is output
    output: Def<Output>,
}

impl Action for PID {
    fn name(&self) -> &String {
        &self.name
    }
    fn evaluate(&mut self, _data: &IOEvent) {
        todo!()
        // maintain PID
    }

    fn set_output(mut self, device: Def<Output>) -> Self
    where
        Self: Sized,
    {
        self.output = device;
        self
    }

    fn output(&self) -> Option<Def<Output>> {
        Some(self.output.clone())
    }

    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
