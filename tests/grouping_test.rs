// TODO: these tests need to be added to "src/storage/grouping.rs"
use chrono::Duration;
use sensd::action::IOCommand;
use sensd::io::{IOKind, RawValue};
use sensd::settings::Settings;
use sensd::storage::Group;
use std::sync::Arc;

#[test]
fn test_add_device() {
    let mut poller: Group = Group::new("main", None);

    let command = IOCommand::Input(move || RawValue::default());
    let config = vec![
        (
            "test name",
            0,
            IOKind::PH,
            command.clone(),
        ),
        (
            "second sensor",
            1,
            IOKind::EC,
            command.clone(),
        ),
    ];
    poller.build_inputs(&config).unwrap();

    assert_eq!(poller.inputs.iter().count(), 2)
}

#[test]
fn test_add_to_log() {
    let mut settings = Settings::default();
    settings.interval = Duration::nanoseconds(1);
    let mut poller: Group = Group::new("main", Some(Arc::new(settings)));

    let command = IOCommand::Input(move || RawValue::default());
    let config = vec![
        (
            "test name",
            0,
            IOKind::Temperature,
            command.clone(),
        ),
        (
            "second sensor",
            1,
            IOKind::Color,
            command.clone(),
        ),
    ];
    poller.build_inputs(&config).unwrap();

    // check that all logs are empty
    const COUNT: usize = 15;
    for iteration in 0..COUNT {
        for log in poller.logs.iter() {
            assert_eq!(log.lock().unwrap().iter().count(), iteration);
        }

        poller.poll().unwrap();

        std::thread::sleep(std::time::Duration::from_nanos(
            poller.interval().num_nanoseconds().unwrap() as u64,
        ));
    }

    for log in poller.logs.iter() {
        assert_eq!(COUNT, log.lock().unwrap().iter().count())
    }
}
