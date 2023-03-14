use crate::action::{Command, BoxedCommand};
use crate::errors::ErrorType;
use crate::io::{IOEvent, RawValue};

// Simple command for printing a message to stdout
pub struct SimpleNotifier {
    msg: String,
}

impl SimpleNotifier {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
    pub fn boxed(msg: String) -> BoxedCommand<IOEvent> {
        Box::new(Self::new(msg))
    }
}

impl Command<IOEvent> for SimpleNotifier {
    fn execute(&self, _value: Option<RawValue>) -> Result<Option<IOEvent>, ErrorType> {
        println!("{}", self.msg);
        Ok(None)
    }
}
