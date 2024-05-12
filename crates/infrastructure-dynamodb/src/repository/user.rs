use std::sync::Arc;

use async_trait::async_trait;
use aws_sdk_dynamodb::types::AttributeValue;
use domain_kanban::user::{User, UserId, UserRepository};
use serde_dynamo::to_attribute_value;
use shaku::Provider;

use crate::{get_from, save_to, Client};

/// UserRepositoryの実装
#[derive(Debug, Clone, Provider)]
#[shaku(interface = UserRepository)]
pub struct UserRepositoryImpl {
    #[shaku(inject)]
    client: Arc<dyn Client>,
}

const TABLE_NAME: &'static str = "users";

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn save(&self, user: User) -> Result<(), String> {
        // clientを使ってuserをusersテーブルに上書き保存する。なければ新規に追加する。
        save_to(self.client.client(), TABLE_NAME, user).await?;
        Ok(())
    }
    async fn find_by_id(&self, id: &UserId) -> Result<User, String> {
        // TODO
        let id_key: AttributeValue =
            to_attribute_value(id.to_string()).map_err(|e| e.to_string())?;
        dbg!(&id_key);
        let result = get_from(
            self.client.client(),
            TABLE_NAME,
            [(String::from("id"), id_key)],
        )
        .await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use domain_kanban::user::{Email, UserName};
    use testcontainers_modules::{localstack::LocalStack, testcontainers::ContainerAsync};

    use crate::{
        test_util::{async_client_init, create_table, ID_ONLY_TABLE},
        ClientImpl,
    };

    use super::*;

    async fn arrange_repository() -> (ContainerAsync<LocalStack>, UserRepositoryImpl) {
        let (c, dynamodb_client) = async_client_init().await;
        create_table(&dynamodb_client, TABLE_NAME, &ID_ONLY_TABLE)
            .await
            .unwrap();

        let client = Arc::new(ClientImpl {
            client: dynamodb_client,
        });
        (c, UserRepositoryImpl { client })
    }

    #[tokio::test]
    async fn test_save_find() {
        // Arrange
        let (_c, user_repository) = arrange_repository().await;
        let user = User::new(
            UserName::new("test user".to_owned()).unwrap(),
            Email::new("test@example.com".to_owned()).unwrap(),
        )
        .unwrap();
        let user_id = user.user_id().clone();
        assert_eq!(user.user_id(), &user_id);

        // Act
        user_repository.save(user.clone()).await.unwrap();
        // XXX: serdeでserializeしたときは "01HXANXCXB0HSWPV9P0TQCJVSD" になっていて、 user_id.to_string()では "user-01HXANXCXB0HSWPV9P0TQCJVSD" になっているため検索できずエラーになる
        let result = user_repository.find_by_id(&user.user_id()).await.unwrap();

        // Assert
        assert_eq!(result.user_id(), user.user_id());
        assert_eq!(result.user_name(), user.user_name());
        assert_eq!(result.email(), user.email());
    }
}
