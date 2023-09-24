use anyhow::Result;
use infrastructure_rdb::QueryModule;
use presentation_axum::{App, Modules};
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<()> {
    logger_init();
    let m = Modules::new(Box::new(QueryModule::builder().build()));
    App::new()?.run(spawn, m).await?;
    Ok(())
}

fn logger_init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
}
