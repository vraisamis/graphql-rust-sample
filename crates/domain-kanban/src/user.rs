#![allow(unused)]
use domain_util::{Entity, Identifier};

#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
    email: Email,
}

impl User {
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

impl Entity for User {
    fn entity_type() -> &'static str {
        "user"
    }
}

pub type UserId = Identifier<User>;

#[derive(Debug)]
pub struct UserName(String);

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug)]
pub struct Email(String);

impl From<String> for Email {
    fn from(value: String) -> Self {
        let v = Self(value);
        // v.stisfy_invariants().unwrap()
        v
    }
}

// TODO
// impl ModelInvariants for Email {
//     fn invariants() -> Vec<&'static Invariant<Self>> {
//         vec![&invariant::length_not_zero]
//     }
// }
//
// mod invariant {
//     use crate::InvariantResult;
//
//     use super::Email;
//
//     pub fn length_not_zero(email: Email) -> InvariantResult<Email> {
//         Ok(email)
//     }
// }

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
}
