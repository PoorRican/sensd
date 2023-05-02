use sensd::action::Comparison;
use sensd::builders::ActionBuilder;
use sensd::helpers::*;
use sensd::io::{DeviceType, GenericInput, RawValue};
use std::ops::Deref;

/// Check that action builder sets correct values
/// At the moment, functionality is not checked here
#[test]
fn test_action_builder() {
    let _input = GenericInput::default();
    assert!(!(_input.has_publisher()));
    let input = Def::new(DeviceType::Input(_input));

    let mut builder = ActionBuilder::new(input.clone()).unwrap();

    let name = "Subscriber for Input";
    let threshold = RawValue::Float(1.0);
    let trigger = Comparison::GT;
    builder.add_threshold(&name, threshold, trigger, None);

    // perform assertions
    let binding = input.lock().unwrap();
    let device = binding.deref();
    if let DeviceType::Input(inner) = device {
        assert!(inner.has_publisher());
    }
}