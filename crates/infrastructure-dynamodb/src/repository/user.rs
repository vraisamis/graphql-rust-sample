use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_dynamodb::{config::IntoShared, types::AttributeValue};
use domain_kanban::user::{User, UserId, UserRepository};
use shaku::Provider;

use crate::Client;

/// UserRepositoryの実装
#[derive(Debug, Clone, Provider)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    #[shaku(inject)]
    client: Arc<dyn Client>,
}

// TODO: anyhow
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> Result<(), String> {
        // clientを使ってuserをusersテーブルに上書き保存する。なければ新規に追加する。
        let request = self.client.client()
            .put_item()
            .table_name("users")
            .item("id", user.user_id().to_string().to_attribute_value())
            // TODO
            .item("name", user.user_id().to_string().to_attribute_value())
            .item("email", user.user_id().to_string().to_attribute_value());
        request.send().await.map(|_| ()).map_err(|e| e.to_string())
    }
    async fn find_by_id(&self, id: &UserId) -> Result<User, String> {
        // clientを使ってidに一致するuserをusersテーブルから取得する。
        let request = self.client.client()
            .get_item()
            .table_name("users")
            .key("id", id.to_string().to_attribute_value());
        let response = request.send().await.map_err(|e| e.to_string())?;
        let items = response.item().ok_or_else(|| "".to_string())?;
        let email = items.get("email").ok_or_else(|| "".to_string())?;
        let email = email.as_s().map_err(|_| "".to_string())?;
        // let user = User::new(UserName::from(value));
        Ok(todo!())
    }
}

/// それぞれの型からDynamoDbのAttributeValueに変換するtrait
trait ToAttributeValue {
    fn to_attribute_value(self) -> AttributeValue;
}

impl ToAttributeValue for String {
    fn to_attribute_value(self) -> AttributeValue {
        AttributeValue::S(self)
    }
}
