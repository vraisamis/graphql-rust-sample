use async_graphql::{CustomValidator, InputValueError};

use crate::scalar::Id;

pub struct IdValidator {
    type_name: String,
    start: String,
}

impl IdValidator {
    pub fn new(type_name: impl Into<String>, start: impl Into<String>) -> Self {
        Self {
            type_name: type_name.into(),
            start: start.into(),
        }
    }
}

impl<T> CustomValidator<Id<T>> for IdValidator
where
    T: Send + Sync,
{
    fn check(&self, value: &Id<T>) -> Result<(), InputValueError<Id<T>>> {
        value
            .value()
            .starts_with(&self.start)
            .then_some(())
            .ok_or(InputValueError::custom(format!(
                "{}のIDは{}で始まらなければなりません",
                self.type_name, self.start
            )))
    }
}
