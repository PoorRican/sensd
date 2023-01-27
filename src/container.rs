use std::collections::HashMap;

use crate::io;
use crate::device;


// TODO: store and retrieve from storage

pub trait Collection<T> {
    fn add(&mut self, data: T) -> bool;
    fn is_empty(&self) -> bool;
    fn length(&self) -> usize;
}

/// Store `IOEvent` object in a polars dataframe
pub struct DeviceContainer {
    inner: HashMap<i32, device::DeviceInfo<f64>>
}

impl DeviceContainer {
    fn find(&self, device_id: i32) -> &device::DeviceInfo<f64> {
        &self.inner[&device_id]
    }
    fn new() -> Self {
        let inner: HashMap<i32, device::DeviceInfo<f64>> = Default::default();

        DeviceContainer {inner}
    }
}

impl Collection<device::DeviceInfo<f64>> for DeviceContainer {
    fn add(&mut self, data: device::DeviceInfo<f64>) -> bool {
        if self.inner.contains_key(&data.sensor_id) {
            true
        } else {
            self.inner.insert(data.sensor_id, data);
            false
        }
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn length(&self) -> usize {
        self.inner.len()
    }
}


pub struct SensorLog {
    inner: Vec<io::IOEvent<f64>>
}

impl Collection<io::IOEvent<f64>> for SensorLog {
    fn add(&mut self, data: io::IOEvent<f64>) -> bool {
        self.inner.push(data);
        true
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn length(&self) -> usize {
        self.inner.len()
    }
}