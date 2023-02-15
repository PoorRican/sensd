use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::IOType;
use crate::io::OutputType;
use crate::io::{IOEvent, SubscriberStrategy};
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::io::action::{NamedRoutine, PublisherInstance};

/// Generic command that monitors a threshold
pub trait ThresholdMonitor: SubscriberStrategy {
    fn threshold(&self) -> IOType;
}

/// Interface for sending a notification
pub trait Notifier: SubscriberStrategy {
    fn send_notification(&mut self, msg: String) -> Result<()>;
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
}
impl ThresholdNotifier {
    pub fn new(
        name: String,
        threshold: IOType,
        publisher: Deferred<PublisherInstance>,
        direction: Comparison,
    ) -> Self {
        Self {
            name,
            threshold,
            publisher,
            trigger: direction,
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
        let exceed = match &self.trigger {
            &Comparison::GT => event.data.value >= self.threshold,
            &Comparison::LT => event.data.value <= self.threshold,
        };
        if exceed {
            let msg = String::from("Value exceeded");
            self.send_notification(msg).unwrap();
            Some(event.invert(1.0))
        } else {
            None
        }
    }

    fn publisher(&self) -> Deferred<PublisherInstance> {
        self.publisher.clone()
    }
}
impl Notifier for ThresholdNotifier {
    fn send_notification(&mut self, msg: String) -> Result<()> {
        println!("{}", msg);
        Ok(())
    }
}
impl Deferrable for ThresholdNotifier {
    type Inner = Box<dyn SubscriberStrategy>;

    fn deferred(self) -> Deferred<Self::Inner> {
        return Arc::new(Mutex::new(Box::new(self)));
    }
}
