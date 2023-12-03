use std::sync::Arc;

use async_trait::async_trait;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::types::AttributeValue;
use domain_kanban::user::{User, UserId, UserName, UserRepository};
use shaku::Provider;

use crate::Client;

/// UserRepositoryの実装
// #[derive(Debug, Clone, Provider)]
// #[shaku(interface = UserRepository)]
struct UserRepositoryImpl {
    client: Arc<dyn Client>,
}

// TODO: anyhow
#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> Result<(), String> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = aws_sdk_dynamodb::Client::new(&config);
        // clientを使ってuserをusersテーブルに上書き保存する。なければ新規に追加する。
        let request = client
            .put_item()
            .table_name("users")
            .item("id", user.user_id().to_string().to_attribute_value())
            // TODO
            .item("name", user.user_id().to_string().to_attribute_value())
            .item("email", user.user_id().to_string().to_attribute_value());
        request.send().await.map(|_| ()).map_err(|e| e.to_string())
    }
    async fn find_by_id(&self, id: &UserId) -> Result<User, String> {
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = aws_sdk_dynamodb::Client::new(&config);
        // clientを使ってidに一致するuserをusersテーブルから取得する。
        let request = client
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
