use sensd::io::{LogType, MockPhSensor};
use sensd::storage::{PollGroup, MappedCollection};
use std::sync::Arc;
use std::sync::Mutex;
use sensd::settings::Settings;
use sensd::io::Device;

#[test]
fn test_add_device() {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    let mut poller: PollGroup<i32> = PollGroup::new("main", settings);

    let config = vec![("test name", 0), ("second sensor", 1)];
    poller.add_sensors(config);

    assert_eq!(poller.sensors.iter().count(), 2)
}

#[test]
fn test_add_to_log() {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    let mut poller: PollGroup<i32> = PollGroup::new("main", settings);

    let config = vec![("test name", 0), ("second sensor", 1)];
    poller.add_sensors(config);

    // check that all logs are empty
    for iteration in 0..2 {
        for log in poller.logs.iter() {
            assert_eq!(log.lock().unwrap().iter().count(), iteration);
        }

        poller.poll().unwrap();

        // I do not know why a long length is needed. It should only take 1sec
        std::thread::sleep(std::time::Duration::from_secs(4));
    }
}
