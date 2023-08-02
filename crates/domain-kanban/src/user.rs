use std::fmt::Debug as DebugTrait;
use std::str::FromStr;

use ulid::Ulid;

use crate::Identifier;

#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
    email: Email,
}

impl User {
    pub fn new(name: UserName, email: Email) -> Self {
        let user_id = UserId::new();
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

type UserId = Identifier<User>;
// pub struct UserId(Ulid);
//
// impl UserId {
//     pub fn new() -> Self {
//         Self(Ulid::new())
//     }
// }
//
// impl From<Ulid> for UserId {
//     fn from(value: Ulid) -> Self {
//         Self(value)
//     }
// }
//
// impl DebugTrait for UserId {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("UserId").field(&self.0.to_string()).finish()
//     }
// }
//
// impl FromStr for UserId {
//     type Err = <Ulid as FromStr>::Err;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         let value = Ulid::from_str(s)?;
//         Ok(UserId(value))
//     }
// }

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
        Self(value)
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
}
