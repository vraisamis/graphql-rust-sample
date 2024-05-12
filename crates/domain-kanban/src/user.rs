use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use shaku::Interface;
use domain_util::{Entity, Identifier, InvariantError, InvariantResult};
use invariant_sheild::{invariant_sheild, InvariantSheild};

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
    pub fn new(
        name: UserName,
        email: Email,
    ) -> InvariantResult<Self> {
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
    pub fn from(user_id: UserId, name: UserName, email: Email) -> Self {
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

pub type UserId = Identifier<User>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserName(String);

#[invariant_sheild(InvariantError)]
impl UserName {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).satisfy_sheilds()
    }

    fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    #[sheild]
    fn user_name_length_less_than_21(&self) -> InvariantResult<()> {
        if self.0.len() < 21 {
            Ok(())
        } else {
            Err(InvariantError::ViolationError(
                "名前が長すぎます。20文字以内にしてください".to_owned(),
            ))
        }
    }
}

impl TryFrom<String> for UserName {
    type Error = InvariantError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl ToString for UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}   

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

impl ToString for Email {
    fn to_string(&self) -> String {
        self.0.clone()
    }
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

    #[test]
    fn user_name_less_than_21_is_ok() -> InvariantResult<()> {
        let name = UserName::new("12345678901234567890".to_owned())?;
        assert_eq!(name.satisfy_sheilds_ref(), Ok(&name));
        Ok(())
    }

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


/// Userモデルを保存するリポジトリのインターフェース
#[async_trait]
pub trait UserRepository: Interface {
    /// Userを保存する
    async fn save(&self, user: User) -> Result<(), String>;
    /// UserをIDで検索する
    async fn find_by_id(&self, id: &UserId) -> Result<User, String>;
}