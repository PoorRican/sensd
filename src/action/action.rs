use crate::errors::{Error, ErrorKind, ErrorType};
use crate::io::{GenericOutput, IOEvent, RawValue};
use std::ops::DerefMut;
use crate::helpers::Def;

pub type BoxedAction = Box<dyn Action>;

/// Trait that enables enables actions to be performed based on incoming data.
pub trait Action {
    fn name(&self) -> &String;

    /// Evaluate incoming data and perform action if necessary.
    ///
    /// # Parameters
    /// - `data`: Raw incoming data from input device.
    fn evaluate(&mut self, data: &IOEvent);

    /// Builder function for setting `output` field.
    ///
    /// # Parameters
    /// - `device`: `DeferredDevice` to set as output
    ///
    /// # Panics
    /// Panic is raised if device is not [`DeviceType::Output`]
    ///
    /// # Returns
    /// - `&mut self`: enables builder pattern
    fn set_output(self, device: Def<GenericOutput>) -> Self
    where
        Self: Sized;

    /// Getter function for `output` field.
    fn output(&self) -> Option<Def<GenericOutput>>;

    /// Setter function for output device field
    ///
    /// # Parameters
    /// - `value`: Binary value to send to device
    ///
    /// # Returns
    /// - `Ok(IOEvent)`: when I/O operation completes successfully.
    /// - `Err(ErrorType)`: when an error occurs during I/O operation
    fn write(&self, value: RawValue) -> Result<IOEvent, ErrorType> {
        if let Some(inner) = self.output() {
            let mut binding = inner.try_lock().unwrap();
            let device = binding.deref_mut();
            device.write(value)
        } else {
            Err(Error::new(
                ErrorKind::DeviceError,
                "ThresholdAction has no device associated as output.",
            ))
        }
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
