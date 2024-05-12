mod repository;
#[cfg(test)]
mod test_util;

use aws_config::{BehaviorVersion, SdkConfig as AwsSdkConfig};
use aws_sdk_dynamodb::{types::AttributeValue, Client as DynamoDbClient};
use serde::{Deserialize, Serialize};
use serde_dynamo::{aws_sdk_dynamodb_1::from_item, to_item};
use shaku::{Component, Interface};
use std::{collections::HashMap, fmt::Debug};

pub use repository::Module as RepositoryModule;

pub async fn default_sdk_config() -> AwsSdkConfig {
    aws_config::load_defaults(BehaviorVersion::latest()).await
}

pub fn dynamo_db_client(config: &AwsSdkConfig) -> DynamoDbClient {
    DynamoDbClient::new(config)
}

pub trait Client: Interface + Debug {
    fn client(&self) -> &DynamoDbClient;
}

#[derive(Debug, Clone, Component)]
#[shaku(interface = Client)]
pub struct ClientImpl {
    client: DynamoDbClient,
}

impl Client for ClientImpl {
    fn client(&self) -> &DynamoDbClient {
        &self.client
    }
}

async fn save_to(
    client: &DynamoDbClient,
    table_name: impl Into<String>,
    value: impl Serialize,
) -> Result<(), String> {
    // TODO
    let item = to_item(value).map_err(|e| e.to_string())?;
    let save_request = client
        .put_item()
        .table_name(table_name)
        .set_item(Some(item));
    // TODO
    save_request.send().await.map_err(|e| e.to_string())?;
    Ok(())
}

async fn get_from<
    'a,
    T: Into<String>,
    K: Into<HashMap<String, AttributeValue>>,
    R: Deserialize<'a>,
>(
    client: &DynamoDbClient,
    table_name: T,
    keys: K,
) -> Result<R, String> {
    let get_request = client
        .get_item()
        .table_name(table_name)
        .set_key(Some(keys.into()));
    // TODO
    let item = get_request.send().await.map_err(|e| e.to_string())?;
    dbg!(&item);
    let item = item.item.ok_or_else(|| "e".to_string())?;
    let result = from_item(item).map_err(|e| e.to_string())?;
    // TODO
    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::test_util::{async_client_init, create_table, ID_ONLY_TABLE};
    use serde_dynamo::to_attribute_value;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    struct Test {
        id: String,
        name: Name,
        count: i32,
        age: u32,
        age_value: AgeValue,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    struct Name {
        first: String,
        last: String,
        middle: Option<String>,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
    #[serde(transparent)]
    struct AgeValue(u32);

    #[tokio::test]
    async fn test_save_and_get() {
        // Arrange
        let (_c, client) = async_client_init().await;
        let table_name = "test_table";
        create_table(&client, table_name, &ID_ONLY_TABLE)
            .await
            .unwrap();

        let test = Test {
            id: "test_id".to_string(),
            name: Name {
                first: "first".to_string(),
                last: "last".to_string(),
                middle: Some("middle".to_string()),
                // middle: None,
            },
            count: 1,
            age: 18,
            age_value: AgeValue(18),
        };

        // Act
        save_to(&client, table_name, test.clone()).await.unwrap();
        let search_key = to_attribute_value("test_id").unwrap();
        let result: Test = get_from(&client, table_name, [(String::from("id"), search_key)])
            .await
            .unwrap();

        // Assert
        assert_eq!(result, test);
    }
}
