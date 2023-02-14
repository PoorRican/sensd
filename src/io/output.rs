use std::fmt::Formatter;
use crate::helpers::Deferred;
use crate::io::{IdTraits, IOEvent};
use crate::storage::Containerized;
use crate::io::{Device, OutputType};
use crate::storage::Container;

/// Interface defining an output device
/// Implementing output devices can be done through structs and can be stored in a container via
/// `Containerized` trait. Any structs that implement this trait may be accessed by `OutputType`
///
/// # Functions
/// - `tx() -> IOType`: Low-level function for interacting with device.
/// - `write() -> Result<()>`: Main interface function output device.
///
/// # Notes:
/// Since `Containerized` is implemented for the `Output` trait, types that implement the `Output` trait
/// can be stored in a container returned by the `Containerized::container()` method. This way, multiple instances of
/// differing types may be stored in the same `Container`.
///
/// ```
/// let mut container = Containerized::<Box<dyn crate::Output<f32>>, i32>::container();
/// container.insert(1, Box::new(HeatingPad::new(String::from("Temperature Output"), 1)));
/// container.insert(2, Box::new(Humidifier::new(String::from("Humidity Output"), 2)));
/// ```
/// > Note how two different output device types were stored in `container`.
pub trait Output: Device {
    fn tx(&self, event: &IOEvent) -> IOEvent;
    fn write(&self, event: &IOEvent) -> IOEvent;
}

impl<K> Containerized<Deferred<OutputType>, K> for OutputType
where
    K: IdTraits
{
    fn container() -> Container<Deferred<OutputType>, K> {
        Container::<Deferred<OutputType>, K>::new()
    }
}

impl std::fmt::Debug for OutputType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Output {{ name: {}, id: {}, kind: {}",
            self.name(),
            self.id(),
            self.metadata().kind
        )
    }
}
