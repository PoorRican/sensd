use std::sync::{Arc, Mutex};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{Command, CommandType, IOEvent};

/// Simple command for printing a message to stdout
pub struct SimpleNotifier {
    msg: String
}

impl SimpleNotifier {
    pub fn new(msg: String) -> Self {
        Self { msg }
    }
    pub fn command(msg: String) -> CommandType {
        Box::new(Self::new(msg))
    }
}

impl Command for SimpleNotifier {
    fn execute(&self) -> Option<IOEvent> {
        println!("{}", self.msg);
        None
    }
}

impl Deferrable for SimpleNotifier {
    type Inner = CommandType;
    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(Box::new(self)))
    }
}
