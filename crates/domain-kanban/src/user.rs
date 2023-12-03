use async_trait::async_trait;
use shaku::Interface;
use domain_util::{Entity, Identifier, InvariantError, InvariantResult};
use invariant_sheild::{invariant_sheild, InvariantSheild};

#[allow(unused)]
#[derive(Debug)]
pub struct User {
    user_id: UserId,
    name: UserName,
    email: Email,
}

#[invariant_sheild(InvariantError)]
impl User {
    pub fn new_check(
        name: InvariantResult<UserName>,
        email: InvariantResult<Email>,
    ) -> InvariantResult<Self> {
        Self::new(name?, email?).satisfy_sheilds()
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

impl Entity for User {
    fn entity_type() -> &'static str {
        "user"
    }
}

pub type UserId = Identifier<User>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

#[invariant_sheild(InvariantError)]
impl UserName {
    pub fn new(value: String) -> InvariantResult<Self> {
        Self::new_unchecked(value).satisfy_sheilds()
    }

    pub fn new_unchecked(value: String) -> Self {
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

impl From<String> for UserName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

    #[test]
    fn user_name_less_than_21_is_ok() {
        let name = UserName::from("12345678901234567890".to_owned());
        assert_eq!(name.satisfy_sheilds_ref(), Ok(&name));
    }

    #[test]
    fn email_with_atmark_is_ok() {
        let email = Email::from("hoge@example.com".to_owned());
        let result = email.satisfy_sheilds_ref();
        assert_eq!(result, Ok(&email));
    }

    #[test]
    fn email_without_atmark_is_ng() {
        let email = Email::from("hoge_example.com".to_owned());

        let result = email.satisfy_sheilds_ref();
        assert_eq!(
            result,
            Err(InvariantError::ViolationError(
                "メールアドレスに「@」が含まれていません".to_owned()
            ))
        );
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