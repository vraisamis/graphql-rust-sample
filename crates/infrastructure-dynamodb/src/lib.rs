mod repository;

use aws_config::{BehaviorVersion, SdkConfig as AwsSdkConfig};
use aws_sdk_dynamodb::Client as DynamoDbClient;
use shaku::{Component, Interface};
use std::fmt::Debug;

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

