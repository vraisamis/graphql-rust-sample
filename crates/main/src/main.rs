use anyhow::Result;
use infrastructure_dynamodb::{
    default_sdk_config, dynamo_db_client, ClientImpl, ClientImplParameters, RepositoryModule,
};
use infrastructure_rdb::{Configuration, PgPoolImpl, PgPoolImplParameters, QueryModule};
use presentation_axum::{App, Modules};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<()> {
    logger_init();
    let m = module().await?;
    App::new()?.run(spawn, m).await?;
    Ok(())
}

fn logger_init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
}

async fn module() -> Result<Modules> {
    let query_module = query_module().await?;
    let repository_module = repository_module().await?;

    let module = Modules::new(query_module, repository_module);

    Ok(module)
}

async fn query_module() -> Result<Box<QueryModule>> {
    let parameters = PgPoolImplParameters {
        pool: Configuration::default().connect().await?,
    };
    let query_module: QueryModule = QueryModule::builder()
        .with_component_parameters::<PgPoolImpl>(parameters)
        .build();
    let query_module = Box::new(query_module);
    Ok(query_module)
}

async fn repository_module() -> Result<Box<RepositoryModule>> {
    let parameters = ClientImplParameters {
        client: dynamo_db_client(&default_sdk_config().await),
    };
    let repository_module = RepositoryModule::builder()
        .with_component_parameters::<ClientImpl>(parameters)
        .build();
    let repository_module = Box::new(repository_module);
    Ok(repository_module)
}
