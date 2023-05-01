use crate::action::PublisherInstance;
use crate::helpers::Def;
use crate::io::{IOEvent, DeferredDevice};

/// Subscriber design pattern for performing actions based on inputs
///
/// The relationship between `Publisher` and `Subscriber` is dually-linked as
/// `Publisher` has a reference to subscriber via the `subscribers` field and subscriber
/// has a reference via `publisher()`.
///
///
/// Subscriber should have a strong reference to Output, so that `Command` may be built.
pub trait Action {
    fn name(&self) -> String;
    /// Primary method to handle incoming data
    ///
    /// `data` argument should be raw input data.
    fn evaluate(&mut self, data: &IOEvent);

    /// Reference to `PublisherInstance`
    fn publisher(&self) -> &Option<Def<PublisherInstance>>;
    /// Set publisher field
    ///
    /// This interface function is used by `ActionBuilder`
    fn add_publisher(&mut self, publisher: Def<PublisherInstance>);
    /// Get boolean if a publisher is assigned or not.
    ///
    /// During the build process (handled by `ActionBuilder`), a publisher is not
    /// associated with the initialized subscriber. In this state, it is considered an
    /// "orphan" and can be checked via `orphan()`. During the build state, `Self::add_publisher()`
    /// creates the reverse association.
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

    fn into_action(self) -> Box<dyn Action>;
}

