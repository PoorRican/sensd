use crate::errors::Result;
use crate::helpers::{Deferrable, Deferred};
use crate::io::types::{DeviceType, IOType, InputType};
use crate::io::{NamedRoutine, OutputType, PublisherInstance};
use crate::io::{IOEvent, SubscriberStrategy};
use std::fmt::{Debug, Formatter};
use std::sync::{Arc, Mutex};

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
pub enum Direction {
    Above,
    Below,
}

/// Notify if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdNotifier {
    name: String,
    threshold: IOType,
    publisher: Deferred<PublisherInstance>,

    direction: Direction,
}
impl ThresholdNotifier {
    pub fn new(
        name: String,
        threshold: IOType,
        publisher: Deferred<PublisherInstance>,
        direction: Direction,
    ) -> Self {
        Self {
            name,
            threshold,
            publisher,
            direction,
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
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent> {
        let exceed = match &self.direction {
            &Direction::Above => data.data.data >= self.threshold,
            &Direction::Below => data.data.data <= self.threshold,
        };
        if exceed {
            let msg = String::from("Value exceeded");
            self.send_notification(msg).unwrap();
            Some(data.invert(1.0))
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
