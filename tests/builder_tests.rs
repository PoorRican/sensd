use std::ops::Deref;
use sensd::action::{BaseCommandFactory, Comparison, SimpleNotifier};
use sensd::helpers::*;
use sensd::builders::ActionBuilder;
use sensd::io::{DeviceType, GenericInput, IOType};

#[test]
fn test_action_builder() {
    let _input = GenericInput::default();
    assert!( !(_input.has_publisher()) );
    let input = _input.deferred();


    let mut builder = ActionBuilder::new(input.clone()).unwrap();

    let name = "Subscriber for Input";
    let threshold = IOType::Float(1.0);
    let trigger = Comparison::GT;
    let factory: BaseCommandFactory =
        |value, threshold| SimpleNotifier::command(format!("{} exceeded {}", value, threshold));
    builder.add_threshold(&name, threshold, trigger, factory);

    let binding = input.lock().unwrap();
    let device = binding.deref();
    if let DeviceType::Input(inner) = device {
        assert!(inner.has_publisher());
    }
}

#[test]
fn test_pub_sub_builder() {
    unimplemented!()
}