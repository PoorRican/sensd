use crate::helpers::Deferred;
use crate::action::PublisherInstance;
use crate::io::IOEvent;

pub type SubscriberType = Box<dyn SubscriberStrategy>;

/// Subscriber to Publisher which enacts a dynamic strategy
///
/// The relationship between publisher and subscriber is dually-linked as
/// publisher has a reference to subscriber via `subscribers` and subscriber
/// has a reference via `publisher()`.
///
/// During the build process (handled by `ActionBuilder`), a publisher is not
/// associated with the initialized subscriber. In this state, it is considered an
/// "orphan" and can be checked via `orphan()`. During the build state, `add_publisher()`
/// creates the reverse association.
pub trait SubscriberStrategy {
    fn name(&self) -> String;
    /// Primary method to evaluate incoming data
    /// Returned IOEvent should be logged
    fn evaluate(&mut self, data: &IOEvent) -> Option<IOEvent>;

    fn publisher(&self) -> &Option<Deferred<PublisherInstance>>;
    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>);
    fn orphan(&self) -> bool {
        match self.publisher() {
            Some(_) => true,
            None => false
        }
    }
}
