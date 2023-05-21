use crate::action::Command;
use crate::errors::DeviceError;
use crate::io::{IODirection, RawValue};

/// Command design pattern for storing low-level I/O code
///
/// Should be used as an interface for HAL code and otherwise perform no other logic.
#[derive(Clone, PartialEq)]
pub enum IOCommand {
    /// Low-level code to read HW input
    Input(fn() -> RawValue),
    /// Low-level code to write to HW output
    ///
    /// # Returns
    /// `Err` is returned if `RawValue` variant is incorrect. Otherwise, `Ok` is returned by
    /// default.
    Output(fn(RawValue) -> Result<(), ()>),
}

impl IOCommand {
    pub fn is_output(&self) -> bool {
        match self {
            Self::Input(_) => false,
            Self::Output(_) => true,
        }
    }

    pub fn is_input(&self) -> bool {
        match self {
            Self::Input(_) => true,
            Self::Output(_) => false,
        }
    }

    /// Get direction of `IOCommand` instance.
    ///
    /// Used to verify device type aligns with function intention: input with input, vice versa.
    pub fn direction(&self) -> IODirection {
        match self {
            IOCommand::Input(_) => IODirection::In,
            IOCommand::Output(_) => IODirection::Out,
        }
    }

    /// Validation to check agreement between command and external [`IODirection`]
    ///
    /// # Parameters
    ///
    /// - `direction`: external direction to check against internal variant
    ///
    /// # Returns
    ///
    /// A `Result` that is:
    /// - `Ok` if internal variant agrees with external direction
    /// - `Err` if internal variant disagrees with external direction
    pub fn agrees(&self, direction: IODirection) -> Result<(), ()> {
        match direction == self.direction() {
            true => Ok(()),
            false => Err(())
        }
    }
}

impl Default for IOCommand {
    fn default() -> Self {
        IOCommand::Output(|_| Ok(()))
    }
}

impl Command<RawValue, DeviceError> for IOCommand {
    /// Execute internally stored function.
    ///
    /// In summary, input command returns a value, output command accepts a value.
    ///
    /// # Parameters
    ///
    /// - `value`: Arbitrary value to pass to output. If passed to an input, a warning is printed.
    ///
    /// # Returns
    ///
    /// A `Result` containing:
    ///
    /// - `Ok` containing [`RawValue`] if internal function is [`IOCommand::Input`]. Otherwise, `None`
    ///   since internal function is [`IOCommand::Output`].
    ///
    /// Currently, there is no scenario that returns `Err`. It is set as the return type to match
    /// [`Input::read()`] and [`Output::write()`].
    ///
    /// # Panics
    ///
    /// A panic is thrown if no value is passed to [`IOCommand::Output`]
    fn execute<V>(&self, value: V) -> Result<Option<RawValue>, DeviceError>
    where
        V: Into<Option<RawValue>>
    {
        let value = value.into();
        match self {
            Self::Input(inner) => {
                // throw warning for unused value
                value.is_some().then(unused_value);

                let read_value = inner();

                Ok(Some(read_value))
            }
            Self::Output(inner) => {
                let unwrapped_value = value.expect("No value was passed to write...");
                let _ = inner(unwrapped_value); // TODO: handle bad result

                Ok(None)
            }
        }
    }
}

/// Print a warning on console stderr
fn unused_value() {
    const MSG: &str = "Unused value passed when reading input...";
    eprintln!("{}", MSG);
}

#[cfg(test)]
mod tests {
    use crate::action::{Command, IOCommand};
    use crate::io::{IODirection, RawValue};

    #[test]
    #[should_panic]
    fn test_output_fails_wo_value() {
        let command = IOCommand::Output(|_| Ok(()));
        command.execute(None).unwrap();
    }

    #[test]
    fn test_default() {
        let command = IOCommand::default();
        assert_eq!(command.direction(), IODirection::Out);
        assert_eq!(None, command.execute(Some(RawValue::Binary(true))).unwrap());
    }

    #[test]
    fn test_agrees() {
        let mut command = IOCommand::Output(|_| Ok(()));
        assert_eq!((),
                   command.agrees(IODirection::Out)
                       .unwrap());
        assert_eq!((),
                   command.agrees(IODirection::In)
                       .err()
                       .unwrap());

        command = IOCommand::Input(|| RawValue::default());
        assert_eq!((),
                   command.agrees(IODirection::In)
                       .unwrap());
        assert_eq!((),
                   command.agrees(IODirection::Out)
                       .err()
                       .unwrap());
    }
}
