use crate::io::{IOEvent, Output, Datum};
use std::ops::DerefMut;
use crate::helpers::Def;

pub type BoxedAction = Box<dyn Action>;

/// Trait that enables actions to be performed based on incoming data.
///
/// Actions are designed to activate [`Output`] devices based on data
/// from [`crate::io::Input`] devices. The primary method for processing incoming
/// data is [`Action::evaluate()`]
pub trait Action {
    fn name(&self) -> &String;

    /// Evaluate incoming data and perform action if necessary.
    ///
    /// # Parameters
    ///
    /// - `data`: Raw incoming data from input device.
    fn evaluate(&mut self, data: &IOEvent);

    /// Builder function for setting `output` field.
    ///
    /// # Parameters
    ///
    /// - `device`: `DeferredDevice` to set as output
    ///
    /// # Returns
    ///
    /// Ownership of `self` to allow builder pattern method chaining
    fn set_output(self, device: Def<Output>) -> Self
    where
        Self: Sized;

    /// Getter function for `output` field.
    fn output(&self) -> Option<Def<Output>>;

    /// Setter function for output device field
    ///
    /// # Parameters
    ///
    /// - `value`: Binary value to send to device
    ///
    /// # Panics
    ///
    /// - If error occurs when writing to device
    /// - If output has no associated output
    fn write(&self, value: Datum) {
        let output = self.output()
            .expect("Action has no associated output device");

        let mut binding = output.try_lock().unwrap();
        let device = binding.deref_mut();

        device.write(value)
            .expect("Unexpected error when writing to output device.");
    }

    /// Print notification to stdout.
    ///
    /// This should be controlled by an internal option flag.
    fn notify(&self, msg: &str) {
        println!("{}", msg);
    }

    /// Consume [`Self`] and wrap in a [`Box`] so it can be coerced into an [`Action`] trait object.
    fn into_boxed(self) -> BoxedAction;
}
