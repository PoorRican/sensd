use std::fmt::{Display, Formatter};
use crate::io::RawValue;

#[derive(Debug, Clone)]
/// Discrete variants that abstract comparison of external and threshold values.
///
/// Used by [`crate::action::Action::evaluate()`]
pub enum Trigger {
    GT,
    LT,
    GTE,
    LTE,
}

impl Trigger {
    #[inline]
    /// Check to see if external value as exceeded threshold value
    ///
    /// # Parameters
    /// - `value`: Value returned to compare against threshold
    /// - `threshold`: Threshold value not to exceed
    ///
    /// # Returns
    ///
    /// A `bool` if threshold is exceeded or not in relation to variant of `self`
    pub fn exceeded(&self, value: RawValue, threshold: RawValue) -> bool {
        match &self {
            &Trigger::GT => value > threshold,
            &Trigger::GTE => value >= threshold,
            &Trigger::LT => value < threshold,
            &Trigger::LTE => value <= threshold,
        }
    }
}

impl Display for Trigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Trigger::GT => ">",
            Trigger::LT => "<",
            Trigger::GTE => "≥",
            Trigger::LTE => "≤",
        };
        write!(f, "{}", name)
    }
}

#[cfg(test)]
mod tests {
    use crate::action::Trigger;
    use crate::io::RawValue;


    #[test]
    fn lte() {
        let trigger = Trigger::LTE;
        let (bigger, smaller) = (
            RawValue::Int(2),
            RawValue::Int(1),
        );

        assert_eq!(true,
                   trigger.exceeded(smaller,  bigger)
        );

        assert_eq!(false,
                   trigger.exceeded(bigger, smaller)
        );

        assert!(trigger.exceeded(bigger, bigger));
        assert!(trigger.exceeded(smaller, smaller));
    }

    #[test]
    fn gte() {
        let trigger = Trigger::GTE;
        let (bigger, smaller) = (
            RawValue::Int(2),
            RawValue::Int(1),
        );



        assert_eq!(true,
                   trigger.exceeded(bigger, smaller)
        );

        assert_eq!(false,
                   trigger.exceeded(smaller,  bigger, )
        );

        assert!(trigger.exceeded(bigger, bigger));
        assert!(trigger.exceeded(smaller, smaller));
    }
    #[test]
    fn lt() {
        let trigger = Trigger::LT;
        let (bigger, smaller) = (
            RawValue::Int(2),
            RawValue::Int(1),
        );

        assert_eq!(true,
            trigger.exceeded(smaller,  bigger)
        );

        assert_eq!(false,
                   trigger.exceeded(bigger, smaller)
        );
    }

    #[test]
    fn gt() {
        let trigger = Trigger::GT;
        let (bigger, smaller) = (
            RawValue::Int(2),
            RawValue::Int(1),
        );



        assert_eq!(true,
                   trigger.exceeded(bigger, smaller)
        );

        assert_eq!(false,
                   trigger.exceeded(smaller,  bigger, )
        );
    }
}
