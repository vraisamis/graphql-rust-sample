use anyhow::Result;
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
    let parameters = PgPoolImplParameters {
        pool: Configuration::default().connect().await?,
    };
    let query_module: QueryModule = QueryModule::builder()
        .with_component_parameters::<PgPoolImpl>(parameters)
        .build();
    let query_module = Box::new(query_module);

    let module = Modules::new(query_module);

    Ok(module)
}
