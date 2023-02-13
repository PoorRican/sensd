use std::fmt::{Debug, Formatter};
use std::ops::Sub;
use std::sync::Arc;
use crate::io::{Input, InputType, IOData, IOEvent};
use crate::errors::Result;
use crate::helpers::{Deferred, check_results};

/// Implement observer design pattern to implement control system based off of polling of `Input` objects

/// Trait to implement on Input objects
pub trait Publisher: Input {
    fn subscribers(&mut self) -> &mut [Deferred<Box<dyn SubscriberStrategy>>];
    fn subscribe(&mut self, subscriber: Deferred<Box<dyn SubscriberStrategy>>);

    fn notify(&mut self, data: &IOEvent) {
        for subscriber in self.subscribers().iter_mut() {
            /// TODO: `IOEvent` shall be sent to `OutputDevice` and shall be logged
            subscriber.lock().unwrap().evaluate(data);
        }
    }
}

/// Subscriber to Publisher which enacts a dynamic strategy
pub trait SubscriberStrategy: Debug {
    /// Primary method to evaluate incoming data
    /// Returned IOEvent should be logged
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent>;
    fn publisher(&self) -> Deferred<InputType>;

    fn info(&self) -> String {
        let binding = self.publisher();
        let mut publisher = binding.lock().unwrap();
        let name = publisher.name();
        let subscribers = publisher.subscribers().iter().count();
        format!("Publisher: {} ({} subscribers)", name, subscribers)
    }
}