use chrono::Utc;
use sensd::builders::DeviceLogBuilder;
use sensd::helpers::Deferred;
use sensd::io::{Device, IOKind, IdType, DeviceType, IODirection};
use sensd::storage::{MappedCollection, OwnedLog, Persistent};
use std::path::Path;
use std::time::Duration;
use std::{fs, thread};
use std::ops::Deref;

fn add_to_log(device: &Deferred<DeviceType>, log: &Deferred<OwnedLog>, count: usize) {
    for _ in 0..count {
        let binding = device.lock().unwrap();
        let event = match binding.deref() {
            DeviceType::Input(inner) => inner.generate_event(Utc::now(), None),
            DeviceType::Output(inner) => inner.generate_event(Utc::now(), None),
        };
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
        let builder = DeviceLogBuilder::new(SENSOR_NAME, &ID, &Some(IOKind::Flow),
                                            &IODirection::Input, None);
        let (device, log) = builder.get();
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
        let builder = DeviceLogBuilder::new(SENSOR_NAME, &ID, &Some(IOKind::Flow),
                                            &IODirection::Input, None);
        let (_device, log) = builder.get();
        let mut _log = log.lock().unwrap();
        _log.load(&None).unwrap();

        // check count of `IOEvent`
        assert_eq!(COUNT, _log.length() as usize);
    };

    fs::remove_file(filename).unwrap();
}
