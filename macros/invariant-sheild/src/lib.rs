// re-export only
pub use invariant_sheild_macros::*;
pub use invariant_sheild_types::*;

#[cfg(test)]
mod tests {
    use std::{error::Error, fmt::Display};

    use super::*;

    #[derive(Debug)]
    struct MyError(String);

    impl Display for MyError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Error for MyError {}

    impl From<&str> for MyError {
        fn from(value: &str) -> Self {
            MyError(value.to_owned())
        }
    }

    type SheildResult<T> = Result<T, MyError>;

    struct Value(String);

    #[invariant_sheild(MyError)]
    impl Value {
        pub fn new(s: impl Into<String>) -> Self {
            Self(s.into())
        }
        pub fn try_new(s: impl Into<String>) -> SheildResult<Self> {
            Self(s.into()).satisfy_sheilds()
        }

        #[sheild]
        fn starts_a(&self) -> SheildResult<()> {
            if self.0.starts_with("a") {
                Ok(())
            } else {
                Err(MyError::from("please start from 'a'."))
            }
        }

        #[sheild]
        fn contains_x(&self) -> SheildResult<()> {
            if self.0.contains("x") {
                Ok(())
            } else {
                Err(MyError::from("please contain 'x'."))
            }
        }
    }

    #[test]
    fn length_of_sheilds_is_2() {
        assert_eq!(Value::sheilds().len(), 2);
    }

    #[test]
    fn test_value_creation_success() {
        let value = Value::try_new("ax").expect("Failed to create Value");
        assert_eq!(value.0, "ax");

        let result = Value::sheilds().iter().all(|sheild| sheild(&value).is_ok());
        assert!(result);
    }

    #[test]
    fn test_value_creation_failure() {
        assert!(Value::try_new("bx").is_err());

        let raw_value = Value::new("bx");
        let result = Value::sheilds()
            .iter()
            .any(|sheild| sheild(&raw_value).is_err());
        assert!(result);
    }

    #[test]
    fn test_starts_a_success() {
        let value = Value::new("ay");
        assert!(value.starts_a().is_ok());
    }

    #[test]
    fn test_starts_a_failure() {
        let value = Value::new("by");
        assert!(value.starts_a().is_err());
    }

    #[test]
    fn test_contains_x_success() {
        let value = Value::new("bx");
        assert!(value.contains_x().is_ok());
    }

    #[test]
    fn test_contains_x_failure() {
        let value = Value::new("by");
        assert!(value.contains_x().is_err());
    }
}
