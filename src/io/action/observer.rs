use std::sync::{Arc, Mutex};
use crate::helpers::{Deferrable, Deferred};
use crate::io::IOEvent;
/// Implement observer design pattern to implement control system based off of polling of `Input` objects

pub trait NamedRoutine {
    fn name(&self) -> String;
}

/// Trait to implement on Input objects
pub trait Publisher: Deferrable {
    fn subscribers(&self) -> &[Deferred<SubscriberType>];
    fn subscribe(&mut self, subscriber: Deferred<SubscriberType>);

    fn notify(&mut self, data: &IOEvent);
}

#[derive(Default, Clone)]
pub struct PublisherInstance {
    subscribers: Vec<Deferred<SubscriberType>>
}

impl Publisher for PublisherInstance {
    fn subscribers(&self) -> &[Deferred<SubscriberType>] { &self.subscribers }

    fn subscribe(&mut self, subscriber: Deferred<SubscriberType>) {
        self.subscribers.push(subscriber)
    }

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

pub type SubscriberType = Box<dyn SubscriberStrategy>;

/// Subscriber to Publisher which enacts a dynamic strategy
pub trait SubscriberStrategy: NamedRoutine {
    /// Primary method to evaluate incoming data
    /// Returned IOEvent should be logged
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent>;
    fn publisher(&self) -> Deferred<PublisherInstance>;
}
