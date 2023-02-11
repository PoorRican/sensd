use chrono::Utc;
use sensd::helpers::{input_log_builder, Deferred};
use sensd::io::{Device, DeviceType, IOEvent, IdType, Input, InputType, GenericSensor, IOKind};
use sensd::storage::{LogType, MappedCollection, OwnedLog, Persistent};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

fn add_to_log(device: &Deferred<InputType>, log: &Deferred<OwnedLog>, count: usize) {
    for _ in 0..count {
        let event = device.lock().unwrap().get_event(Utc::now());
        log.lock().unwrap().push(event.timestamp, event).unwrap();
        thread::sleep(Duration::from_nanos(1)); // add delay so that we don't finish too quickly
    }
}

#[test]
fn test_load_save() {
    const SENSOR_NAME: &str = "test";
    const ID: IdType = 32;
    const COUNT: usize = 10;

    /* NOTE: More complex `IOEvent` objects *could* be checked, but we are trusting `serde`.
    These tests only count the number of `IOEvent`'s added. */

    let filename;
    // test save
    {
        let (log, device) = input_log_builder(SENSOR_NAME, &ID, &Some(IOKind::Flow), None);
        add_to_log(&device, &log, COUNT);
        let _log = log.lock().unwrap();
        _log.save(&None).unwrap();

        // save filename for later
        filename = _log.filename();
        // check that file exists
        assert!(Path::new(&filename).exists());
    };

    // test load
    // build back up then load
    {
        let (log, device) = input_log_builder(SENSOR_NAME, &ID, &Some(IOKind::Flow), None);
        let mut _log = log.lock().unwrap();
        _log.load(&None).unwrap();

        // check count of `IOEvent`
        assert_eq!(COUNT, _log.length() as usize);
    };

    fs::remove_file(filename).unwrap();
}
