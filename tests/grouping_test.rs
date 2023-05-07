// TODO: these tests need to be added to "src/storage/grouping.rs"
use chrono::Duration;
use sensd::action::IOCommand;
use sensd::io::{Device, Input, IOKind, Output, RawValue};
use sensd::settings::Settings;
use sensd::storage::{Chronicle, Group};
use std::sync::Arc;

#[test]
/// Test builder pattern for adding devices
fn test_builder_pattern() {
    let command = IOCommand::Input(move || RawValue::default());

    let mut group = Group::new("main", None);
    group
        .push_input(
            Input::new(
                "test name",
                0,
                IOKind::PH,
            ).set_command(command.clone()))
        .push_input(
            Input::new(
                "second sensor",
                1,
                IOKind::EC,
            ).set_command(command.clone()))
        .push_output(
            Output::new(
                "output device",
                2,
                IOKind::Flow
            ).set_command(IOCommand::Output(|_| Ok(())))
        );

    assert_eq!(group.inputs.iter().count(), 2);
    assert_eq!(group.outputs.iter().count(), 1);
}

#[test]
fn test_poll() {
    let mut _settings = Settings::default();
    _settings.interval = Duration::nanoseconds(1);
    let settings = Arc::new(_settings);

    let command = IOCommand::Input(move || RawValue::default());

    let mut group = Group::new("main", Some(settings.clone()));
    group
        .push_input(

            Input::new(
                "test name",
                0,
                IOKind::PH,
            ).set_command(
                command.clone()
            ).init_log(settings.clone())

        ).push_input(

            Input::new(
                "second sensor",
                1,
                IOKind::EC,
            ).set_command(
                command.clone()
            ).init_log(
                settings.clone()
            )

        );

    // check that all logs are empty
    const COUNT: usize = 15;
    for iteration in 0..COUNT {
        for input in group.inputs.values() {
            let binding = input.try_lock().unwrap();
            let log = binding.log().unwrap();
            assert_eq!(log.lock().unwrap().iter().count(), iteration);
        }

        group.poll().unwrap();

        std::thread::sleep(std::time::Duration::from_nanos(
            group.interval().num_nanoseconds().unwrap() as u64,
        ));
    }

    for input in group.inputs.values() {
        let binding = input.try_lock().unwrap();
        let log = binding.log().unwrap();

        assert_eq!(COUNT, log.lock().unwrap().iter().count());
    }
}
