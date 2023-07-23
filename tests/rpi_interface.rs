#[cfg(feature = "rppal")]
#[cfg(test)]
mod tests {

    /// This is an integrated test use of the [`rppal`] library with both [`Input`] and [`Output`]
    ///
    /// These tests are normally ignored as they're meant to executed on the RPi platform since many
    /// tests directly make use of the [`rppal`] crate to comprehensively test functionality.
    use rppal::gpio::Gpio;
    use sensd::action::IOCommand;
    use sensd::io::{Datum, Device, Input, Output};

    /// Common GPIO pin to use throughout tests
    ///
    /// There is no particular significance with this pin.
    const GPIO_PIN: u8 = 23;

    #[test]
    #[ignore]
    /// Ensure no build errors with [`Input`] builder API and
    fn test_input_api() {
        let pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_input();

        let mut input =
            Input::new("output", 0).set_command(IOCommand::Input(|| Datum::uint8(pin.read())));

        println!("{:?}", input.read());
    }

    #[test]
    #[ignore]
    /// Ensure no build errors with [`Output`] builder API
    fn test_output_api() {
        let mut pin = Gpio::new().unwrap().get(GPIO_PIN).unwrap().into_output();

        let mut output = Output::new("output", 0)
            .set_command(IOCommand::Output(|val| Ok(pin.write(val.into()))));

        output.write(32.into()).unwrap();
    }
}
