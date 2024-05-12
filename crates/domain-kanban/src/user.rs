mod email;
mod user_name;
pub use email::*;
pub use user_name::*;

use async_trait::async_trait;
use domain_util::{Entity, Identifier, InvariantError, InvariantResult};
use invariant_sheild::{invariant_sheild, InvariantSheild};
use serde::{Deserialize, Serialize};
use shaku::Interface;

pub type UserId = Identifier<User>;

#[allow(unused)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename = "id")]
    user_id: UserId,
    name: UserName,
    email: Email,
}

#[invariant_sheild(InvariantError)]
impl User {
    pub fn new(name: UserName, email: Email) -> InvariantResult<Self> {
        Self::new_unchecked(name, email).satisfy_sheilds()
    }
    fn new_unchecked(name: UserName, email: Email) -> Self {
        let user_id = UserId::gen();
        Self {
            user_id,
            name,
            email,
        }
    }

    /// UserId, UserName, EmailからUserモデルを作成
    pub fn new_with_id(user_id: UserId, name: UserName, email: Email) -> Self {
        Self {
            user_id,
            name,
            email,
        }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn user_name(&self) -> &UserName {
        &self.name
    }

    pub fn email(&self) -> &Email {
        &self.email
    }
}

impl Entity for User {
    fn entity_type() -> &'static str {
        "user"
    }
}

/// Userモデルを保存するリポジトリのインターフェース
#[async_trait]
pub trait UserRepository: Interface {
    /// Userを保存する
    async fn save(&self, user: User) -> Result<(), String>;
    /// UserをIDで検索する
    async fn find_by_id(&self, id: &UserId) -> Result<User, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn user_test() -> InvariantResult<()> {
        let name = UserName::new("Foo".to_owned())?;
        let email = Email::new("hoge@example.com".to_owned())?;
        let user = User::new(name, email)?;
        let user_id = user.user_id();

        println!("{:?}, id: {:?}", user, user_id);
        Ok(())
    }
}
