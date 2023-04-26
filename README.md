# sensd - _The Ultimate Sensor Logging and Control System_


## █▓▒░ Features

- Straight-forward, simple initialization and configuration.
- Generic I/O devices for classification of various device types types such as switches, motors, manifold valves, etc.
- Robust error handling for safe and reliable operation.
- Data logging capabilities to store and retrieve information for later analysis.
- Support for multiple sensor types and data inputs.


## █▓▒░ Purpose

`sensd` is a multipurpose robust framework for sensor logging and a control system for mission-critical 
or high reliability environments. Originally designed for the needs of an aquaponics system, `sensd` has been made
highly adaptable with generic design and agnostic approach which make it suitable for a wide range of applications,
from wild-life tracking and bioreactors to access control and beyond using pre-packaged generic I/O devices.
With a focus on reliability, safety, and ease of use, `sensd` aims to empower both engineers and makers
to build sophisticated monitoring and control systems with ease.


## █▓▒░ Hardware

The library is built with version 1.66.0 of the Rust programming language, but should should be reverse compatable 
with recent versions. While the library has been designed to decrease low-overhead, `no_std` is not implemented and the
library on `std` implementations at the moment. Therefore, for embedded applications, a POSIX based host is required
(such as the Raspberry Pi).


## █▓▒░ Getting Started

To use the library, simply add the following to your Cargo.toml file:

```toml
[dependencies]
sensd = "0.1.0"
```

Then remember to include the following:

```rust
extern crate sensd;

use sensd;
```

The binaries in the `/examples` directory are designed to be comprensive to demonstrate potential use cases and are
highly documented. For further documentation, please refer to the docs.


## █▓▒░ Contributing

The goal of this project is to become a standard for scientific and industrial applications. Therefore, contributions
to the library are more than welcomed.

Remember that contributions can be in the form of bug reports, feature requests, and not just jcode contributions. Bug
reports and feature requests add fortitude and reliability to the library as whole and benefits the community.
To get started, simply fork the repository and make your changes. I simply ask, that before submitting a pull request,
please make sure to run the test suite and update the documentation as necessary.

Remember, communication is essential for any collaborative probject. Let's bring the most loved programming language
to the scientific community!


## █▓▒░ Features Forthcoming

The following features are planned for the next major releases (in order of priority):
    - Incorporate `no_std` feature flag
    - Partitioning of logs (both by size and date)
