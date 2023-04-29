//! Implement observer design pattern to implement control system based off of polling of `Input` objects
//!
//! # Description
//! The goal of a dedicated `Publisher` implementation being stored as a field is to add a layer of indirection
//! between `Input` and `Output` to serve as a bridge. Both input and output should be unaware of the other.
//! However, events generated by Input::read() are routed to Publisher::notify() which is propagated to
//! Subscriber implementations and therefore events are passed to outputs.
//!
//! `Publisher` objects should be stored a struct which implements `Input`. When `Input::read()` is called,
//! `Input::publisher().notify()` should also be called as well. `notify()` should thereby call
//! `Subscriber::evaluate()` on any listeners.

use crate::action::{SubscriberType, SchedRoutineHandler};
use crate::helpers::{Deferrable, Deferred};
use crate::io::IOEvent;
use std::sync::{Arc, Mutex};

pub trait NamedRoutine {
    fn name(&self) -> String;
}

/// Trait to implement on Input objects
pub trait Publisher: Deferrable {
    fn subscribers(&self) -> &[Deferred<SubscriberType>];
    fn subscribe(&mut self, subscriber: Deferred<SubscriberType>);

    fn notify(&mut self, data: &IOEvent);
}

/// Concrete instance of publisher object
#[derive(Default)]
pub struct PublisherInstance {
    subscribers: Vec<Deferred<SubscriberType>>,
    scheduled: SchedRoutineHandler,
}

impl PublisherInstance {
    /// Attempt to run scheduled `Routine` structs
    pub fn attempt_scheduled(&mut self) {
        self.scheduled.attempt()
    }
}

impl Publisher for PublisherInstance {
    fn subscribers(&self) -> &[Deferred<SubscriberType>] {
        &self.subscribers
    }

    fn subscribe(&mut self, subscriber: Deferred<SubscriberType>) {
        self.subscribers.push(subscriber)
    }

    /// Call `Subscriber::evaluate()` on all associated `Subscriber` implementations.
    fn notify(&mut self, data: &IOEvent) {
        for subscriber in self.subscribers.iter_mut() {
            // TODO: `IOEvent` shall be sent to `OutputDevice` and shall be logged
            // TODO: results should be aggregated
            subscriber.lock().unwrap().evaluate(data);
        }
    }
}

impl Deferrable for PublisherInstance {
    type Inner = PublisherInstance;

    fn deferred(self) -> Deferred<Self::Inner> {
        Arc::new(Mutex::new(self))
    }
}
