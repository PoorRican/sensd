use std::{fs, thread};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use chrono::Utc;
use sensd::storage::{Persistent, MappedCollection, LogType};
use sensd::io::{IOEvent, MockPhSensor, Input, IdType};

#[test]
fn test_load_save() {
    const PATH: &str = "dummy_log.json";
    const SENSOR_NAME: &str = "test";
    const ID: IdType = 32;
    const COUNT: usize = 10;

    /* NOTE: More complex `IOEvent` objects *could* be checked, but we are trusting `serde`.
             These tests only count the number of `IOEvent`'s added. */

    // test save
    {
        let log = Arc::new(Mutex::new(LogType::new()));
        let device: MockPhSensor = MockPhSensor::new(String::from(SENSOR_NAME), ID, log);
        for _ in 0..COUNT {
            let event = IOEvent::create(&device, Utc::now(), device.read());
            device.log.lock().unwrap().add(event.timestamp, event).unwrap();
            thread::sleep(Duration::from_nanos(1));
        }
        device.log.lock().unwrap().save(Some(String::from(PATH))).unwrap();

        // check that file exists
        assert!(Path::new(PATH).exists());
    };

    // test load
    {
        let log = Arc::new(Mutex::new(LogType::new()));
        let device: MockPhSensor = MockPhSensor::new(String::from(SENSOR_NAME), ID, log);
        device.log.lock().unwrap().load(Some(String::from(PATH))).unwrap();

        // check count of `IOEvent`
        assert_eq!(COUNT, device.log.lock().unwrap().length() as usize);
    };

    fs::remove_file(PATH).unwrap();
}
