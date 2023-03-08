use sensd::action::{ThresholdFactory, Comparison, IOCommand, SimpleNotifier};
use sensd::builders::{ActionBuilder, DeviceLogBuilder};
use sensd::helpers::*;
use sensd::io::{DeviceTraits, DeviceType, GenericInput, IODirection, IOKind, IOType, IdType};
use std::ops::Deref;

#[test]
fn test_action_builder() {
    let _input = GenericInput::default();
    assert!(!(_input.has_publisher()));
    let input = _input.deferred();

    let mut builder = ActionBuilder::new(input.clone()).unwrap();

    let name = "Subscriber for Input";
    let threshold = IOType::Float(1.0);
    let trigger = Comparison::GT;
    let factory: ThresholdFactory =
        |value, threshold| SimpleNotifier::command(format!("{} exceeded {}", value, threshold));
    builder.add_threshold(&name, threshold, trigger, factory);

    let binding = input.lock().unwrap();
    let device = binding.deref();
    if let DeviceType::Input(inner) = device {
        assert!(inner.has_publisher());
    }
}


#[test]
fn test_device_log_builder() {
    const NAME: &str = "device name";
    const ID: IdType = 0;
    const DIRECTION: IODirection = IODirection::Input;
    const KIND: IOKind = IOKind::Unassigned;

    let command = IOCommand::Input(move || IOType::default());
    let builder = DeviceLogBuilder::new(NAME, &ID, &Some(KIND), &DIRECTION, &command, None);
    let (device, log) = builder.get();

    assert_eq!(false, log.lock().unwrap().orphan());
    assert!(log
        .lock()
        .unwrap()
        .filename()
        .contains(&device.lock().unwrap().name()));
}
