pub trait DeviceWrapper {
    fn is_input(&self) -> bool;
    fn is_output(&self) -> bool;
}
