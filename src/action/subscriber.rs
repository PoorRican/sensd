use crate::action::PublisherInstance;
use crate::helpers::Deferred;
use crate::io::{IOEvent, DeferredDevice};

pub type SubscriberType = Box<dyn Subscriber>;

/// Subscriber to Publisher which enacts a dynamic strategy
///
/// The relationship between publisher and subscriber is dually-linked as
/// publisher has a reference to subscriber via `subscribers` and subscriber
/// has a reference via `publisher()`.
///
/// During the build process (handled by `ActionBuilder`), a publisher is not
/// associated with the initialized subscriber. In this state, it is considered an
/// "orphan" and can be checked via `orphan()`. During the build state, `Self::add_publisher()`
/// creates the reverse association.
///
/// Subscriber should have a strong reference to Output, so that `GenericCommand` may be built.
pub trait Subscriber {
    fn name(&self) -> String;
    /// Primary method to evaluate incoming data
    /// Returned IOEvent should be logged
    fn evaluate(&mut self, data: &IOEvent);

    fn publisher(&self) -> &Option<Deferred<PublisherInstance>>;
    fn add_publisher(&mut self, publisher: Deferred<PublisherInstance>);
    fn orphan(&self) -> bool {
        match self.publisher() {
            Some(_) => true,
            None => false,
        }
    }

    /// Getter function for `output` field.
    fn output(&self) -> Option<DeferredDevice> {
        unimplemented!()
    }
    /// Setter function for output device field
    ///
    /// Should print warning to `stderr` if field is not `None`. Method should not panic.
    fn set_output(&mut self, _device: DeferredDevice) {
        unimplemented!()
    }

    /// Print notification to stdout.
    ///
    /// This should be controlled by an internal option flag.
    fn notify(&self, msg: &str) {
        println!("{}", msg);
    }
}
