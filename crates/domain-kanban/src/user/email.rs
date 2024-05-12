use domain_util::InvariantError;

use domain_util::InvariantResult;
use invariant_sheild::{invariant_sheild, InvariantSheild};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Email(String);

#[invariant_sheild(InvariantError)]
impl Email {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).satisfy_sheilds()
    }
    pub fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    #[sheild]
    pub(crate) fn email_contains_atmark(&self) -> InvariantResult<()> {
        if self.0.contains("@") {
            Ok(())
        } else {
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_string(),
            ))
        }
    }
}

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_with_atmark_is_ok() -> InvariantResult<()> {
        let email = Email::new("hoge@example.com".to_owned())?;
        let result = email.satisfy_sheilds_ref();
        assert_eq!(result, Ok(&email));
        Ok(())
    }

    #[test]
    fn email_without_atmark_is_ng() -> InvariantResult<()> {
        let email = Email::new("hoge_example.com".to_owned());

        assert_eq!(
            email,
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_owned()
            ))
        );
        Ok(())
    }
}
