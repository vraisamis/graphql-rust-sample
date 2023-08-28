use anyhow::Result;
use presentation_axum::App;

#[tokio::main]
async fn main() -> Result<()> {
    logger_init();
    App::new()?.run().await?;
    Ok(())
}

fn logger_init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
}
