use anyhow::Result;
use presentation_axum::App;
use tokio::spawn;

#[tokio::main]
async fn main() -> Result<()> {
    logger_init();
    App::new()?.run(spawn).await?;
    Ok(())
}

fn logger_init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
}
