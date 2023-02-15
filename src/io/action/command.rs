use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::IOType;
use crate::io::{CommandType, OutputType, BaseCommandFactory};
use crate::io::{IOEvent, SubscriberStrategy};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::io::action::{NamedRoutine, PublisherInstance};

/// Abstraction for single atomic output operation
pub trait Command {
    fn execute(&self) -> Option<IOEvent>;
}

/// Simple command for printing a message to stdout
pub struct SimpleNotifier {
    msg: String
}
impl SimpleNotifier {
    fn new(msg: String) -> Self {
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

/// Generic command that monitors a threshold
pub trait ThresholdMonitor: SubscriberStrategy {
    fn threshold(&self) -> IOType;
}

/// Subscriber routine to actively maintain an arbitrary threshold using PID
pub struct PIDMonitor {
    name: String,
    threshold: IOType,
    publisher: Deferred<PublisherInstance>,

    output: Deferred<OutputType>, // this should really `OutputType`
}
impl NamedRoutine for PIDMonitor {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl ThresholdMonitor for PIDMonitor {
    fn threshold(&self) -> IOType {
        self.threshold
    }
}
impl SubscriberStrategy for PIDMonitor {
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent> {
        todo!()
        // maintain PID
    }

    fn publisher(&self) -> Deferred<PublisherInstance> {
        self.publisher.clone()
    }
}

#[derive(Debug, Clone)]
/// Enum used by `ThresholdMonitor` logic
/// Controls when comparison of external value and threshold returns `true`.
pub enum Comparison {
    GT,
    LT,
}

/// Notify if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdNotifier {
    name: String,
    threshold: IOType,
    publisher: Deferred<PublisherInstance>,

    trigger: Comparison,
    factory: BaseCommandFactory<IOType, IOType>,
}
impl ThresholdNotifier {
    pub fn new(
        name: String,
        threshold: IOType,
        publisher: Deferred<PublisherInstance>,
        trigger: Comparison,
        factory: BaseCommandFactory<IOType, IOType>
    ) -> Self {
        Self {
            name,
            threshold,
            publisher,
            trigger,
            factory,
        }
    }
}
impl NamedRoutine for ThresholdNotifier {
    fn name(&self) -> String {
        self.name.clone()
    }
}
impl ThresholdMonitor for ThresholdNotifier {
    fn threshold(&self) -> IOType {
        self.threshold
    }
}
impl SubscriberStrategy for ThresholdNotifier {
    fn evaluate(&mut self, event: &IOEvent) -> Option<IOEvent> {
        let value = event.data.value;
        let exceed = match &self.trigger {
            &Comparison::GT => value >= self.threshold,
            &Comparison::LT => value <= self.threshold,
        };
        if exceed {
            // insert command here
            let msg = format!("{} exceeded {}", value, self.threshold);
            let command = SimpleNotifier::new(msg);
            // Some(event.invert(1.0))  // re-enable this when dynamic IOTypes have been implemented
            command.execute()
        } else {
            None
        }
    }

    fn publisher(&self) -> Deferred<PublisherInstance> {
        self.publisher.clone()
    }
}
impl Deferrable for ThresholdNotifier {
    type Inner = Box<dyn SubscriberStrategy>;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
