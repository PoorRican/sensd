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

    let config = [("test name", 0), ("second sensor", 1)];
    for (name, id) in config {
        // variable allowed to go out-of-scope because `poller` owns reference
        let log = Arc::new(Mutex::new(LogType::new()));
        poller.logs.push(log.clone());

        let sensor = MockPhSensor::new(name.to_string(), id, log.clone());
        poller.sensors.add(sensor.id(), sensor.boxed()).unwrap();
    }

    assert_eq!(poller.sensors.iter().count(), 2)
}

#[test]
fn test_add_to_log() {
    let settings: Arc<Settings> = Arc::new(Settings::initialize());
    let mut poller: PollGroup<i32> = PollGroup::new("main", settings);

    let config = [("test name", 0), ("second sensor", 1)];
    for (name, id) in config {
        // variable allowed to go out-of-scope because `poller` owns reference
        let log = Arc::new(Mutex::new(LogType::new()));
        poller.logs.push(log.clone());

        let sensor = MockPhSensor::new(name.to_string(), id, log.clone());
        poller.sensors.add(sensor.id(), sensor.boxed()).unwrap();
    }

    // check that all logs are empty
    for iteration in 0..2 {
        for log in poller.logs.iter() {
            assert_eq!(log.lock().unwrap().length(), iteration)
        }

        poller.poll().unwrap();

        std::thread::sleep(std::time::Duration::from_secs(4));
    }
}
