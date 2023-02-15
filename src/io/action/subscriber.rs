use crate::helpers::Deferred;
use crate::io::action::{NamedRoutine, PublisherInstance};
use crate::io::IOEvent;

pub type SubscriberType = Box<dyn SubscriberStrategy>;

/// Subscriber to Publisher which enacts a dynamic strategy
pub trait SubscriberStrategy: NamedRoutine {
    /// Primary method to evaluate incoming data
    /// Returned IOEvent should be logged
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent>;
    fn publisher(&self) -> Deferred<PublisherInstance>;
}
