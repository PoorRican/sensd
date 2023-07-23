/// Interface for a named object
///
/// [`String`] has been chosen to be used because using `&str` involves a headache of passing lifetimes.
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
    fn set_name<S>(self, name: S) -> Self
        where
            Self: Sized,
            S: Into<String>;
}