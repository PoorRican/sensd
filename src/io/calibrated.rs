/// Defines an interface for an input device that needs to be calibrated
pub trait Calibrated {
    /// Initiate the calibration procedures for a specific device instance.
    fn calibrate(&self) -> bool;
}
