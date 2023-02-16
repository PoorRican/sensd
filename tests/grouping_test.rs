use chrono::Duration;
use sensd::io::GenericSensor;
use sensd::io::{Device, IOKind};
use sensd::settings::Settings;
use sensd::storage::{LogType, MappedCollection, PollGroup};
use std::sync::Arc;
use std::sync::Mutex;

#[test]
fn test_add_device() {
    let mut poller: PollGroup = PollGroup::new("main", None);

    let config = vec![
        ("test name", 0, IOKind::PH),
        ("second sensor", 1, IOKind::EC),
    ];
    poller.add_inputs(&config).unwrap();

    assert_eq!(poller.inputs.iter().count(), 2)
}

#[test]
fn test_add_to_log() {
    let mut settings = Settings::default();
    settings.interval = Duration::nanoseconds(1);
    let mut poller: PollGroup = PollGroup::new("main", Some(Arc::new(settings)));

    let config = vec![
        ("test name", 0, IOKind::AmbientTemperature),
        ("second sensor", 1, IOKind::Color),
    ];
    poller.add_inputs(&config).unwrap();

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
