//! Type aliases for functions and closures to assist `ActionBuilder`.
//! These aliases allow for strongly structuring the dynamic initialization of subscriber/command instances.
use crate::action::{Command, Comparison, PublisherInstance, SubscriberType, CommandType};
use crate::helpers::Deferred;
use crate::io::IOType;

// Command Factories
pub type BaseCommandFactory = fn(IOType, IOType) -> CommandType;

// **********************
// Subscriber Factories *
// **********************

/// Type alias for a function or closure that returns a `ThresholdNotifier` instance
pub type ThresholdNotifierFactory = fn(String, IOType, Comparison, BaseCommandFactory) -> Deferred<SubscriberType>;
