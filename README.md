# sensd - _The Ultimate Sensor Logging and Control System_

## Purpose

`sensd` is a multipurpose robust framework for sensor logging and a control system for mission-critical 
or high reliability environments. It was originally designed for the needs of an aquaponics system, but
its generic design and agnostic approach make it suitable for a wide range of applications,
from wild-life tracking and bioreactors to access control and beyond using pre-packaged generic I/O devices.
With a focus on reliability, safety, and ease of use, `sensd` aims to empower both engineers and makers
to build sophisticated monitoring and control systems with ease.

### Key Features

- User-friendly API for easy integration into existing systems.
- Packaged generic I/O devices (e.g: various measurement types, switches, motors, manifold valves, etc)
- Robust error handling for safe and reliable operation.
- Data logging capabilities to store and retrieve information for later analysis.
- Dynamic configuration options for customized and adaptable solutions.
- Support for multiple sensor types and data inputs.

## Hardware
The library is built with version 1.45.0 of the Rust programming language, for ensuring a high level of performance
and security, while introducing extremely low-overhead. This allows `sensd` to be deployed on any modern system or
embedded devices.

# Getting Started
To use the library, simply add the following to your Cargo.toml file:

```toml
[dependencies]
sensd = "0.0.1-alpha"
```
And include the following in your main file:

```rust
use sensd;
```

For further documentation and examples on how to use the library, please refer to the docs.

# Contributing
We welcome all contributions to the library, whether it's in the form of bug reports, feature requests, or code contributions. To get started, simply fork the repository and make your changes. Before submitting a pull request, please make sure to run the test suite and update the documentation as necessary.