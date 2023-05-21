use std::error::Error as _Error;
use std::fmt;

use custom_error::custom_error;

use crate::io::{DeviceMetadata, IODirection};

pub type ErrorType = Box<dyn _Error>;

custom_error! { pub ContainerError
    MiscError{name: String, msg: String} = "Unknown container error from \"{name}\": {msg}",
    ContainerEmpty = "Container is empty",
    ContainerNotEmpty = "Container is not empty",
    KeyExists{key: String} = "Device entry {key} exists",
}

custom_error! { pub DeviceError
    HWFault{metadata: DeviceMetadata} = "HW fault from {metadata}",
    NoCommand{metadata: DeviceMetadata} = "No associated command for {metadata}",
    ValueExpected{metadata: DeviceMetadata} = "Value expected from {metadata}",
}

custom_error! { pub FilesystemError
    SerializationError{msg: String} = "Error during serialization: {msg}",
    PermissionError{path: String} = "Incorrect permissions for {path}",
}
