use aws_config::{meta::region::RegionProviderChain, BehaviorVersion, SdkConfig};
use aws_sdk_dynamodb::{
    error::SdkError,
    operation::create_table::{CreateTableError, CreateTableOutput},
    types::{AttributeDefinition, KeySchemaElement, KeyType, ScalarAttributeType},
    Client,
};
use testcontainers_modules::{
    localstack::LocalStack,
    testcontainers::{runners::AsyncRunner, ContainerAsync, RunnableImage},
};

use crate::dynamo_db_client;

pub async fn async_client_init() -> (ContainerAsync<LocalStack>, Client) {
    // create docker instance
    let image = RunnableImage::from(LocalStack);
    let container = image.start().await;

    // get localstack endpoint
    let host_ip = container.get_host().await;
    let host_port = container.get_host_port_ipv4(4566).await;
    let endpoint_url = format!("http://{host_ip}:{host_port}");

    // create dynamo db client
    let config = aws_skd_config_localstack(endpoint_url).await;
    let client = dynamo_db_client(&config);

    (container, client)
}

async fn aws_skd_config_localstack(localstack_endpoint_url: String) -> SdkConfig {
    let region = RegionProviderChain::default_provider().or_else("ap-northeast-1");
    aws_config::defaults(BehaviorVersion::latest())
        .region(region)
        .endpoint_url(localstack_endpoint_url)
        .test_credentials()
        .load()
        .await
}

pub async fn create_table(
    client: &Client,
    table_name: &str,
    keys: &[(&str, KeyType, ScalarAttributeType)],
) -> Result<CreateTableOutput, SdkError<CreateTableError>> {
    let (key_schemas, attribute_definitions): (Vec<_>, Vec<_>) = keys
        .into_iter()
        .map(|(key_name, key_type, attribute_type)| {
            (
                KeySchemaElement::builder()
                    .attribute_name(*key_name)
                    .key_type(key_type.clone())
                    .build()
                    .unwrap(),
                AttributeDefinition::builder()
                    .attribute_name(*key_name)
                    .attribute_type(attribute_type.clone())
                    .build()
                    .unwrap(),
            )
        })
        .unzip();

    client
        .create_table()
        .billing_mode(aws_sdk_dynamodb::types::BillingMode::PayPerRequest)
        .table_name(table_name)
        .set_key_schema(Some(key_schemas))
        .set_attribute_definitions(Some(attribute_definitions))
        .send()
        .await
}

pub const ID_ONLY_TABLE: [(&'static str, KeyType, ScalarAttributeType); 1] =
    [("id", KeyType::Hash, ScalarAttributeType::S)];
