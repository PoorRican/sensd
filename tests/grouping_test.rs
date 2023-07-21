use std::path::PathBuf;
// TODO: these tests need to be added to "src/storage/grouping.rs"
use chrono::Duration;
use sensd::action::IOCommand;
use sensd::io::{Device, Input, Output, Datum};
use sensd::storage::{Chronicle, Group, Persistent, RootDirectory};

#[test]
/// Test builder pattern for adding devices
fn test_builder_pattern() {
    let command = IOCommand::Input(move || Datum::default());

    let mut group = Group::new("main");
    group
        .push_input(
            Input::new(
                "test name",
                0,
            ).set_command(command.clone()))
        .push_input(
            Input::new(
                "second sensor",
                1,
            ).set_command(command.clone()))
        .push_output(
            Output::new(
                "output device",
                2,
            ).set_command(IOCommand::Output(|_| Ok(())))
        );

    assert_eq!(group.inputs.len(), 2);
    assert_eq!(group.outputs.len(), 1);
}

#[test]
fn test_poll() {
    let command = IOCommand::Input(move || Datum::default());

    let mut group = Group::with_interval("main", Duration::nanoseconds(1));
    group
        .push_input(

            Input::new(
                "test name",
                0,
            ).set_command(
                command.clone()
            ).init_log()

        ).push_input(

            Input::new(
                "second sensor",
                1,
            ).set_command(
                command.clone()
            ).init_log()

        );

    // check that all logs are empty
    const COUNT: usize = 15;
    for iteration in 0..COUNT {
        for input in group.inputs.values() {
            let binding = input.try_lock().unwrap();
            let log = binding.log().unwrap();
            assert_eq!(log.lock().unwrap().iter().count(), iteration);
        }

        group.poll().unwrap();

        std::thread::sleep(std::time::Duration::from_nanos(
            group.interval().num_nanoseconds().unwrap() as u64,
        ));
    }

    for input in group.inputs.values() {
        let binding = input.try_lock().unwrap();
        let log = binding.log().unwrap();

        assert_eq!(COUNT, log.lock().unwrap().iter().count());
    }
}

#[test]
fn test_directory_hierarchy() {
    const TMP_DIR: &str = "/tmp/sensd/hierarchy_testing";
    const INTERVAL: i64 = 1;

    let in_command = IOCommand::Input(move || Datum::default());

    let input1 =
        Input::new(
            "i1",
            0,
        ).set_command(
            in_command.clone()
        ).init_log();
    let input2 =
        Input::new(
            "i2",
            1,
        ).set_command(
            in_command.clone()
        ).init_log();

    let out_command = IOCommand::Output(|_| Ok(()));

    let output1 =
        Output::new(
            "o1",
            0,
        ).set_command(
            out_command.clone()
        ).init_log();

    let output2 =
        Output::new(
            "o2",
            1,
        ).set_command(
            out_command.clone()
        ).init_log();

    // Build `Group` and create directories
    let mut group =
        Group::with_interval(
            "group",
            Duration::nanoseconds(INTERVAL));

    group.set_root_ref(TMP_DIR);

    group
        .push_input(input1)
        .push_input(input2);
    group
        .push_output(output1)
        .push_output(output2);

    group.init_dir_ref();

    // Ensure that log exists
    for _ in 0..15 {
        group.poll().unwrap();
        std::thread::sleep(
            std::time::Duration::from_nanos(INTERVAL as u64));
    }
    group.save().expect("Could not save `Group`");

    // Manually check directories
    let group_dir = PathBuf::from(TMP_DIR).join("group");
    assert!(group_dir.exists());

    let dirs = ["i1", "i2", "o1", "o2"];
    for device in dirs {
        let path = group_dir.join(device);
        assert!(path.exists());
        assert_eq!(1, path.read_dir().unwrap().count())
    }
}
