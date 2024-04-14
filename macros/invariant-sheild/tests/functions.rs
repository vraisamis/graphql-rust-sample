use std::{error::Error, fmt::Display};

use invariant_sheild::{invariant_sheild, InvariantSheild};

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

struct Value(i32);

#[invariant_sheild(MyError)]
impl Value {
    fn new(i: impl Into<i32>) -> Self {
        Value(i.into())
    }

    #[sheild]
    fn check_2(&self) -> SheildResult<()> {
        if self.0 % 2 == 0 {
            Ok(())
        } else {
            Err("value is odd: ".into())
        }
    }

    #[sheild]
    fn check_3(&self) -> SheildResult<()> {
        if self.0 % 3 == 0 {
            Ok(())
        } else {
            Err("value is cannot divide 3. value: ".into())
        }
    }
}

#[test]
fn value_has_6_is_ok() {
    let v = Value::new(6);
    assert_eq!(v.0, 6);

    assert!(v.satisfy_sheilds_ref().is_ok());
}

#[test]
fn value_has_1_is_err() {
    let v = Value::new(1);
    assert_eq!(v.0, 1);

    assert!(v.satisfy_sheilds_ref().is_err());
}
