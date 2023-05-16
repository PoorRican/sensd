use chrono::Duration;
use ext_pid::Pid;
use crate::action::{Action, BoxedAction, SchedRoutineHandler};
use crate::helpers::Def;
use crate::io::{Output, IOEvent, RawValue};

/// Action implementing a PID controller to control a single output
///
/// This is a wrapper for [`Pid`] that conforms to Rust API guidelines and attaches an [`Output`].
/// Output should be a device which can be controlled in a binary fashion (eg: pump, valve, etc).
pub struct PID {
    name: String,
    pid: Pid<f32>,

    output: Option<Def<Output>>,
    handler: Option<Def<SchedRoutineHandler>>,
}

impl PID {
    /// Constructor for [`PID`]
    ///
    /// # Parameters
    ///
    /// - `name`: Name of controller
    /// - `setpoint`: Setpoint of process variable
    /// - `output_limit`: Hard of output
    ///
    /// # Returns
    ///
    /// A newly initialized [`PID`] action without an `output` and PID gains set to 0.
    /// Use of [`Action::set_output()`], and setters for PID gains are required to be called
    /// after initialization.
    pub fn new<N, V>(name: N, setpoint: V, output_limit: V) -> Self
        where
            N: Into<String>,
            V: Into<f32> + Copy
    {
        Self {
            name: name.into(),
            pid: Pid::new(setpoint.into(),
                          output_limit.into()),
            output: None,
            handler: None,
        }
    }

    /// Getter for *P* gain
    ///
    /// # Returns
    ///
    /// Current "proportional" gain
    pub fn p(&self) -> f32 {
        self.pid.kp
    }

    /// Getter for limit of *P* gain
    ///
    /// # Returns
    ///
    /// Current limit on "proportional" gain value
    pub fn p_limit(&self) -> f32 {
        self.pid.p_limit
    }

    /// Builder method for setting "proportional" gain and limit
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *P*
    /// - `limit`: Desired limit for *P*
    ///
    /// # Returns
    ///
    /// Ownership of `Self` is returned, with adjusted *P* gain and limit. As per Rust API guidelines,
    /// building by method chaining is encouraged.
    pub fn set_p<V>(mut self, gain: V, limit: V) -> Self
    where
        V: Into<f32> + Copy
    {
        self.pid.p(gain, limit);
        self
    }

    /// Setter for "proportional" gain and limit by reference
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *P*
    /// - `limit`: Desired limit for *P*
    ///
    /// # Returns
    ///
    /// Reference of `Self` is returned, with adjusted *P* gain and limit. Calling this method in a singular
    /// fashion is enabled by this function.
    pub fn set_p_ref<V>(&mut self, gain: V, limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.p(gain, limit);
        self
    }

    /// Getter for *I* gain
    ///
    /// # Returns
    ///
    /// Current "integral" gain
    pub fn i(&self) -> f32 {
        self.pid.ki
    }

    /// Getter for limit of *I* gain
    ///
    /// # Returns
    ///
    /// Current limit on "integral" gain value
    pub fn i_limit(&self) -> f32 {
        self.pid.i_limit
    }

    /// Builder method for setting "integral" gain and limit
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *I*
    /// - `limit`: Desired limit for *I*
    ///
    /// # Returns
    ///
    /// Ownership of `Self` is returned, with adjusted *I* gain and limit. As per Rust API guidelines,
    /// building by method chaining is encouraged.
    pub fn set_i<V>(&mut self, gain: V, limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.i(gain, limit);
        self
    }

    /// Setter for "integral" gain and limit by reference
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *I*
    /// - `limit`: Desired limit for *I*
    ///
    /// # Returns
    ///
    /// Reference of `Self` is returned, with adjusted *I* gain and limit. Calling this method in a singular
    /// fashion is enabled by this function.
    pub fn set_i_ref<V>(&mut self, gain: V, limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.i(gain, limit);
        self
    }

    /// Getter for *D* gain
    ///
    /// # Returns
    ///
    /// Current "derivative" gain
    pub fn d(&self) -> f32 {
        self.pid.kd
    }

    /// Getter for limit of *D* gain
    ///
    /// # Returns
    ///
    /// Current limit on "derivative" gain value
    pub fn d_limit(&self) -> f32 {
        self.pid.d_limit
    }

    /// Builder method for setting "derivative" gain and limit
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *D*
    /// - `limit`: Desired limit for *D*
    ///
    /// # Returns
    ///
    /// Ownership of `Self` is returned, with adjusted *D* gain and limit. As per Rust API guidelines,
    /// building by method chaining is encouraged.
    pub fn set_d<V>(&mut self, gain: V, limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.d(gain.into(),
                   limit.into());
        self
    }

    /// Setter for "derivative" gain and limit by reference
    ///
    /// # Parameters
    ///
    /// - `gain`: Desired gain for *d*
    /// - `limit`: Desired limit for *D*
    ///
    /// # Returns
    ///
    /// Reference of `Self` is returned, with adjusted *D* gain and limit.
    /// Calling this method in a singular fashion is enabled by this function.
    pub fn set_d_ref<V>(&mut self, gain: V, limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.i(gain, limit);
        self
    }

    /// Getter for setpoint of process variable
    ///
    /// # Returns
    ///
    /// Internal value of setpoint to achieve
    pub fn setpoint(&self) -> f32 {
        self.pid.setpoint
    }

    /// Setter for setpoint
    ///
    /// # Parameters
    ///
    /// - `setpoint`: Desired setpoint
    ///
    /// # Returns
    ///
    /// Mutable reference to `self` to allow method chaining.
    pub fn set_setpoint<V>(&mut self, setpoint: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.setpoint(setpoint.into());
        self
    }

    /// Getter for output limit
    ///
    /// # Returns
    ///
    /// Current value of output limit
    pub fn output_limit(&self) -> f32 {
        self.pid.output_limit
    }

    /// Setter for output limit
    ///
    /// # Parameters
    ///
    /// - `output_limit`: Desired output limit
    ///
    /// # Returns
    ///
    /// Reference to `self` with updated output limit to allow for
    /// method chaining.
    pub fn set_output_limit<V>(&mut self, output_limit: V) -> &mut Self
    where
        V: Into<f32> + Copy
    {
        self.pid.output_limit = output_limit.into();
        self
    }

    /// Calculate duration of output signal from sensor data
    ///
    /// # Parameters
    ///
    /// - `measurement`: Sensor data from input
    ///
    /// # Returns
    ///
    /// [`Duration`] for which to keep `output` activated. Float value is
    /// divided between seconds and milliseconds to allow the PID algorithm
    /// to handle a wide range of values without the need for other parameters
    /// or generics.
    fn calculate<V>(&mut self, measurement: V) -> Duration
    where
        V: Into<f32> + Copy
    {
        let measurement = measurement.into();
        let output = self.pid.next_control_output(
            measurement.into()).output;


        Duration::seconds(output.trunc() as i64) +
        Duration::milliseconds(output.fract() as i64)

    }

    /// Builder function to set `handler` parameter
    pub fn set_handler(mut self, handler: Def<SchedRoutineHandler>) -> Self {
        self.handler = Some(handler);
        self
    }
}

impl Action for PID {
    fn name(&self) -> &String {
        &self.name
    }

    fn evaluate(&mut self, data: &IOEvent) {
        let measurement = data.value;
        if let RawValue::Float(value) = measurement {

            let duration =
                self.calculate(value);

            if duration > Duration::milliseconds(0) {
                if self.handler.is_none() {
                    panic!("Handler has not been set!");
                }

                self.write(RawValue::Binary(true));

                let output = self.output.as_ref()
                    .expect("Output has not been set!")
                    .try_lock().unwrap();
                let routine = output.create_routine(
                    RawValue::Binary(false),
                    duration);
                self.handler.as_ref().unwrap().try_lock().unwrap().push(routine);
            }
        }
    }

    fn set_output(mut self, device: Def<Output>) -> Self
    where
        Self: Sized,
    {
        self.output = Some(device);
        self
    }

    fn output(&self) -> Option<Def<Output>> {
        self.output.clone()
    }

    fn into_boxed(self) -> BoxedAction {
        Box::new(self)
    }
}
