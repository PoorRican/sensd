use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::IOType;
use crate::io::{BaseCommandFactory, CommandType, OutputType};
use crate::io::{IOEvent, SubscriberStrategy};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::io::action::{NamedRoutine, PublisherInstance};

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self) -> Option<IOEvent>;
}
