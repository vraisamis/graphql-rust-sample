use domain_util::{Entity, Identifier};

use crate::todo_invariants::{Invariant, InvariantError, InvariantResult, ModelInvariants};

#[allow(unused)]
#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
    email: Email,
}

impl User {
    pub fn new_check(
        name: InvariantResult<UserName>,
        email: InvariantResult<Email>,
    ) -> InvariantResult<Self> {
        Self::new(name?, email?).stisfy_invariants()
    }
    pub fn new(name: UserName, email: Email) -> Self {
        let user_id = UserId::gen();
        Self {
            user_id,
            name,
            email,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }
}

impl ModelInvariants for User {
    fn invariants() -> Vec<&'static Invariant<Self>> {
        vec![]
    }
}

impl Entity for User {
    fn entity_type() -> &'static str {
        "user"
    }
}

pub type UserId = Identifier<User>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl UserName {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).stisfy_invariants()
    }
    pub fn new_unchecked(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

fn user_name_length_less_than_20(name: &UserName) -> InvariantResult<()> {
    if name.0.len() <= 20 {
        Ok(())
    } else {
        Err(InvariantError::ViolationError(
            "名前が長すぎます。20文字以内にしてください".to_owned(),
        ))
    }
}

impl ModelInvariants for UserName {
    fn invariants() -> Vec<&'static Invariant<Self>> {
        vec![&user_name_length_less_than_20]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).stisfy_invariants()
    }
    pub fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    fn email_contains_atmark(&self) -> InvariantResult<()> {
        if self.0.contains("@") {
            Ok(())
        } else {
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_string(),
            ))
        }
    }
}

impl From<String> for Email {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl ModelInvariants for Email {
    fn invariants() -> Vec<&'static Invariant<Self>> {
        vec![&Self::email_contains_atmark]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_test() {
        let name = UserName::from("Foo".to_owned());
        let email = Email::from("hoge@example.com".to_owned());
        let user = User::new(name, email);
        let user_id = user.user_id();

        println!("{:?}, id: {:?}", user, user_id);
    }

    #[test]
    fn email_ok() {
        let email = Email::from("hoge@example.com".to_owned());
        let result = email.clone().stisfy_invariants();
        assert_eq!(result, Ok(email.clone()));
        let result = email.stisfy_invariants_ref();
        assert_eq!(result, Ok(&email));
    }

    #[test]
    fn email_ng() {
        let email = Email::from("hoge_example.com".to_owned());
        let result = email.clone().stisfy_invariants();
        assert_eq!(
            result,
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_owned()
            ))
        );

        let result = email.stisfy_invariants_ref();
        assert_eq!(
            result,
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_owned()
            ))
        );
    }
}
