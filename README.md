# 6SENS
A Multi-purpose Sensor Logger and Control System

## Purpose

Provide an industrial strength library for building sensor monitoring and control systems.
Originally, the system has been designed to monitor the mechanical and biological processes in an aquaponics system,
but the system has been designed with generic types that and is agnostic to context or space.
Possible applications include wild-life tracking, greenhouse management, access control, or anything imaginable.

## Future Intentions

### Minor Updates

- Implement [dimensioned](https://docs.rs/crate/dimensioned/0.8.0) crate for type checked SI units.

### Major Releases

The following features are planned for the next major releases (in order of priority):
    - A control system for operating switches, motors, and other output devices.
    - 3D (or 2D), location awareness. Nodes can be aware of neighbors.
    - Port to an embedded system (to run on "bare-metal"). Which means a hardware UI.
    - Secure, wireless network for receiving data and sending commands to nodes.