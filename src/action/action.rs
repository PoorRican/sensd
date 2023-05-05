use crate::action::BoxedAction;
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

    /// Builder function for setting `output` field.
    ///
    /// # Parameters
    /// - `device`: `DeferredDevice` to set as output
    ///
    /// # Panics
    /// Panic is raised if device is not [`crate::io::DeviceType::Output`]
    ///
    /// # Returns
    /// - `&mut self`: enables builder pattern
    fn set_output(self, device: DeferredDevice) -> Self
    where Self: Sized;

    fn output(&self) -> Option<DeferredDevice>;

    /// Print notification to stdout.
    ///
    /// This should be controlled by an internal option flag.
    fn notify(&self, msg: &str) {
        println!("{}", msg);
    }

    fn into_boxed(self) -> BoxedAction;
}

