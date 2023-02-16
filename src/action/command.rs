use crate::action::{BaseCommandFactory, SubscriberStrategy, CommandType, NamedRoutine, PublisherInstance};
use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, OutputType, IOType};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self) -> Option<IOEvent>;
}
