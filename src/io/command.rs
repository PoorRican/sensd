use std::fmt::{Debug, Formatter};
use std::ops::{Deref, Sub};
use std::sync::{Arc, Mutex, TryLockResult};
use chrono::Utc;
use crate::errors::{Result};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, IOType, IOData, SubscriberStrategy, Publisher, InputType, Input, DeviceType, Device};

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
    threshold: IOType,
    publisher: Deferred<InputType>,

    output: Deferred<DeviceType>        // this should really `OutputType`
}
impl Debug for PIDMonitor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info())
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

    fn publisher(&self) -> Deferred<InputType> {
        self.publisher.clone()
    }
}


#[derive(Debug, Clone)]
/// Enum used by `ThresholdMonitor` logic
/// Controls when comparison of external value and threshold returns `true`.
pub enum Direction {
    Above,
    Below
}


/// Notify if threshold is exceeded
#[derive(Clone)]
pub struct ThresholdNotifier {
    threshold: IOType,
    publisher: Deferred<InputType>,

    direction: Direction,
}
impl ThresholdNotifier {
    pub fn new(threshold: IOType, publisher: Deferred<InputType>, direction: Direction) -> Self {
        Self { threshold, publisher, direction }
    }
}
impl ThresholdMonitor for ThresholdNotifier {
    fn threshold(&self) -> IOType {
        self.threshold
    }
}
impl Debug for ThresholdNotifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.info())
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

    fn publisher(&self) -> Deferred<InputType> {
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
        return Arc::new(Mutex::new(Box::new(self)))
    }
}