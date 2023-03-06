use chrono::Duration;
use sensd::action::IOCommand;
use sensd::io::{IODirection, IOKind, IOType};
use sensd::settings::Settings;
use sensd::storage::PollGroup;
use std::sync::Arc;

#[test]
fn test_add_device() {
    let mut poller: PollGroup = PollGroup::new("main", None);

    let command = IOCommand::Input(move || IOType::default());
    let config = vec![
        (
            "test name",
            0,
            IOKind::PH,
            IODirection::Input,
            command.clone(),
        ),
        (
            "second sensor",
            1,
            IOKind::EC,
            IODirection::Input,
            command.clone(),
        ),
    ];
    poller.add_devices(&config).unwrap();

    assert_eq!(poller.inputs.iter().count(), 2)
}

#[test]
fn test_add_to_log() {
    let mut settings = Settings::default();
    settings.interval = Duration::nanoseconds(1);
    let mut poller: PollGroup = PollGroup::new("main", Some(Arc::new(settings)));

    let command = IOCommand::Input(move || IOType::default());
    let config = vec![
        (
            "test name",
            0,
            IOKind::Temperature,
            IODirection::Input,
            command.clone(),
        ),
        (
            "second sensor",
            1,
            IOKind::Color,
            IODirection::Input,
            command.clone(),
        ),
    ];
    poller.add_devices(&config).unwrap();

    // check that all logs are empty
    const COUNT: usize = 15;
    for iteration in 0..COUNT {
        for log in poller.logs.iter() {
            assert_eq!(log.lock().unwrap().iter().count(), iteration);
        }

        poller.poll().unwrap();

        std::thread::sleep(std::time::Duration::from_nanos(
            poller._interval().num_nanoseconds().unwrap() as u64,
        ));
    }

    for log in poller.logs.iter() {
        assert_eq!(COUNT, log.lock().unwrap().iter().count())
    }
}
