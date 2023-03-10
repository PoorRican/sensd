use crate::action::{Command, CommandType};
use crate::errors::ErrorType;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, IOType};
use std::sync::{Arc, Mutex};

/// Simple command for printing a message to stdout
pub struct SimpleNotifier {
    msg: String,
}

impl SimpleNotifier {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
    pub fn command(msg: String) -> CommandType<IOEvent> {
        Box::new(Self::new(msg))
    }
}

impl Command<IOEvent> for SimpleNotifier {
    fn execute(&self, _value: Option<IOType>) -> Result<Option<IOEvent>, ErrorType> {
        println!("{}", self.msg);
        Ok(None)
    }
}

impl Deferrable for SimpleNotifier {
    type Inner = CommandType<IOEvent>;
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(Box::new(self)))
    }
}
