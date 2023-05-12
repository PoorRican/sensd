# sensd - _The Ultimate Sensor Logging and Control System_


## █▓▒░ Purpose

`sensd` aims to be a multipurpose robust framework for sensor logging and a control system for mission-critical
and high reliability environments. Originally designed for the needs of an aquaponics system, `sensd` is being made
highly adaptable by implementing generic architectural design patterns. The goal is to create a library suitable for a
wide range of applications, from wild-life tracking and bioreactors to access control and beyond using pre-packaged
generic I/O devices and triggers. With a focus on reliability, safety, and ease of use, `sensd` aims to empower scientists, 
engineers, and makers to build monitoring and control systems by reducing boilerplate code.


## █▓▒░ Features

- Straight-forward, simple initialization and configuration.
- Data logging capabilities to store and retrieve information for later analysis.
- Generic I/O devices for classification of various device types such as switches, motors, manifold valves, etc.
- Robust error handling for safe and reliable operation.
- Hardware agnostic.


## █▓▒░ Hardware

The library is built with version 1.66.1 of the Rust programming language, but should be reverse compatible 
with recent versions. A Unix based host is required (ie: Raspberry Pi). While the library has been designed
to decrease low-overhead, `no_std` has not implemented (yet) for compiling on bare-metal MCU's such as Atmel chips.


## █▓▒░ Getting Started

Since this library is still in early development, it has not yet been uploaded to [crates.io](https://crates.io).

The binaries in the `/examples` directory are designed to be comprehensive to demonstrate potential use cases and are
highly documented. For further documentation, please refer to the docs.


## █▓▒░ Contributing

The goal of this project is to become a standard for scientific and industrial applications. Therefore, contributions
to the library are more than welcomed.

Remember that contributions can be in the form of bug reports, feature requests, and not just code contributions. Bug
reports and feature requests add fortitude and reliability to the library as whole and benefits the community.
To get started, simply fork the repository and make your changes. I simply ask, that before submitting a pull request,
please make sure to run the test suite and update the documentation as necessary.

Remember, communication is essential for any collaborative project. Let's bring the most loved programming language
to the scientific community!


## █▓▒░ Features Forthcoming

The following features are planned for the next major releases (in order of priority):
- PID controller
- Partitioning of logs (both by size and date)
- Incorporate `no_std` feature flag
- Async operation
