use crate::action::{Command, IOCommand};
use crate::io::{DeferredDevice, IOType, DeviceTraits, IODirection};
use crate::errors::{Error, ErrorKind, ErrorType};

pub struct GPIOCommand {
    func: IOCommand,
}

impl GPIOCommand {
    pub fn new(func: IOCommand, device: Option<DeferredDevice>) -> Self {
        if let Some(device) = device {
            check_alignment(&func, device.clone()).unwrap();
        }

        Self { func }
    }

    pub fn direction(&self) -> IODirection {
        match self.func {
            IOCommand::Input(_) => IODirection::Input,
            IOCommand::Output(_) => IODirection::Output,
        }
    }
}

impl Command<IOType> for GPIOCommand {
    /// Execute internally stored function.
    ///
    /// # Returns
    /// If internal function is `IOCommand::Input`, then the value that is read from device is returned.
    /// Otherwise, if `IOCommand::Output`, then `None` is returned.
    fn execute(&self, value: Option<IOType>) -> Result<Option<IOType>, ErrorType> {
        match self.func {
            IOCommand::Input(inner) => {
                // throw warning for unused value
                if let Some(_) = value { unused_value() }

                let read_value = inner();

                Ok(Some(read_value))

            },
            IOCommand::Output(inner) => {
                let unwrapped_value = value.expect("No value was passed to write...");
                let _ = inner(unwrapped_value);       // TODO: handle bad result

                Ok(None)
            },
        }
    }
}

/// Panic if command and device are not aligned
pub fn check_alignment(command: &IOCommand, device: DeferredDevice) -> Result<(), ErrorType> {
    let aligned = command.direction() == device.direction();
    match aligned {
        true => Ok(()),
        false => Err(misconfigured_error())
    }
}

/// Generate an error for when command type does not match device type
pub fn misconfigured_error() -> ErrorType {
    Error::new(ErrorKind::CommandError, "Misconfigured device! Device and command type do not match.")
}

/// Print a warning on console stderr
fn unused_value() {
    const MSG: &str = "Unused value passed when reading input...";
    eprintln!("{}", MSG);
}

#[cfg(test)]
mod tests {
    use crate::action::{IOCommand, check_alignment, GPIOCommand, Command};
    use crate::helpers::Deferrable;
    use crate::io::{DeferredDevice, Device, DeviceType, GenericOutput, IdType, IODirection, GenericInput, IOType};
    use crate::storage::OwnedLog;

    const REGISTER_DEFAULT: IOType = IOType::PosInt8(255);
    static mut REGISTER: IOType = REGISTER_DEFAULT;

    unsafe fn reset_register() {
        REGISTER = REGISTER_DEFAULT;
    }

    unsafe fn set_register(val: IOType) {
        REGISTER = val;
    }

    fn make_device(direction: &IODirection) -> DeferredDevice {
        let name = "";
        let id = IdType::default();
        let log = OwnedLog::new(id, None).deferred();

        let device = match direction {
            IODirection::Input => {
                DeviceType::Input(GenericInput::new(String::from(name), id, None, log))
            }
            IODirection::Output => {
                DeviceType::Output(GenericOutput::new(String::from(name), id, None, log))
            }
        };
        device.deferred()
    }

    #[test]
    fn test_check_alignment() {
        {
            let direction = IODirection::Input;
            let device = make_device(&direction);

            let command = IOCommand::Input(move || IOType::Float(0.0));

            let result = check_alignment(&command, device);
            match result {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }
        {
            let direction = IODirection::Output;
            let device = make_device(&direction);

            let command = IOCommand::Output(move |_| Ok(()));

            let result = check_alignment(&command, device);
            match result {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }


        {
            let direction = IODirection::Output;
            let device = make_device(&direction);

            let command = IOCommand::Input(move || IOType::Float(0.0));

            let result = check_alignment(&command, device);
            match result {
                Ok(_) => assert!(false),
                Err(_) => assert!(true)
            }
        }
        {
            let direction = IODirection::Input;
            let device = make_device(&direction);

            let command = IOCommand::Output(move |_| Ok(()));

            let result = check_alignment(&command, device);
            match result {
                Ok(_) => assert!(false),
                Err(_) => assert!(true)
            }
        }
    }

    #[test]
    #[should_panic]
    /// Assert that program panics when device and IOCommand are misaligned
    /// This test case specifically uses an input device and an output command.
    fn test_alignment_i_o() {
        let direction = IODirection::Input;
        let device = make_device(&direction);

        let command = IOCommand::Output(move |_| Ok(()));

        GPIOCommand::new(command, Some(device));
    }

    #[test]
    #[should_panic]
    /// Assert that program panics when device and IOCommand are misaligned
    /// This test case specifically uses an output device and an input command.
    fn test_alignment_o_i() {
        let direction = IODirection::Output;
        let device = make_device(&direction);

        let command = IOCommand::Input(move || IOType::default());

        GPIOCommand::new(command, Some(device));
    }

    #[test]
    fn test_execute() {
        {
            unsafe { reset_register(); }

            let func = IOCommand::Input(move || unsafe {
                REGISTER
            });
            let command = GPIOCommand::new(func, None);

            match command.execute(None) {
                Ok(tentative) => unsafe {
                    match tentative {
                        Some(inner) => assert_eq!(REGISTER, inner),
                        None => assert!(false)
                    }
                },
                Err(_) => assert!(false)
            }

        }
        {
            unsafe { reset_register(); }

            let func = IOCommand::Output(move |val| unsafe {
                set_register(val);
                Ok(())
            });
            let command = GPIOCommand::new(func, None);
            let value = IOType::Binary(true);

            unsafe { assert_ne!(REGISTER, value); }

            match command.execute(Some(value)) {
                Ok(tentative) => {
                    match tentative {
                        Some(_) => assert!(false),
                        None => ()
                    }
                },
                Err(_) => assert!(false)
            }

            unsafe { assert_eq!(REGISTER, value); }
        }
    }
}
