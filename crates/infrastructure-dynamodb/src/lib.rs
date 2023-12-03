mod user;

use aws_config::{SdkConfig as AwsSdkConfig, BehaviorVersion};
use aws_sdk_dynamodb::Client as DynamoDbClient;
use shaku::{Component, Interface};
use std::{fmt::Debug, sync::Arc};

pub trait Config: Interface + Debug {
    fn config(&self) -> &AwsSdkConfig;
}

#[derive(Debug, Clone, Component)]
#[shaku(interface = Config)]
pub struct SdkConfigImpl {
    config: AwsSdkConfig,
}

impl SdkConfigImpl {
    pub async fn new() -> Self {
        Self {
            config: aws_config::load_defaults(BehaviorVersion::latest()).await
        }
    }
}

impl Config for SdkConfigImpl {
    fn config(&self) -> &AwsSdkConfig {
        &self.config
    }
}

trait Client: Interface + Debug {
    fn client(&self) -> &DynamoDbClient;
}

#[derive(Debug, Clone)]
// #[derive(Debug, Clone, Component)]
// #[shaku(interface = Client)]
struct ClientImpl {
    // #[shaku(inject)]
    config: Arc<dyn Config>,
}