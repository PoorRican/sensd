//! Implement observer design pattern to implement control system based off of polling of `Input` objects
//!
//! # Operation
//! `Publisher` objects should be stored a struct which implements `Input`. When `Input::read()` is called,
//! `Input::publisher().notify()` should also be called as well. `notify()` should thereby call
//! `Subscriber::evaluate()` on any listeners.
//!
//! The goal of a dedicated `Publisher` implementation being stored as a field is to add a layer of indirection
//! between `Input` and `Subscriber` to serve as a mediator.

use std::sync::{Arc, Mutex};
use crate::helpers::{Deferrable, Deferred};
use crate::io::{IOEvent, SubscriberType};

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
#[derive(Default, Clone)]
pub struct PublisherInstance {
    subscribers: Vec<Deferred<SubscriberType>>
}

impl Publisher for PublisherInstance {
    fn subscribers(&self) -> &[Deferred<SubscriberType>] { &self.subscribers }

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
