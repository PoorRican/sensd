/// Interface for a named object
pub trait Name {
    /// Getter for name field
    ///
    /// # Returns
    ///
    /// Reference to internal `name`
    fn name(&self) -> &String;

    /// Setter method for name field
    ///
    /// # Parameters
    ///
    /// - `name`: Desired name
    fn set_name<S>(&mut self, name: S)
        where
            S: Into<String>;
}