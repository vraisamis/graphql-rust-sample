use anyhow::{Context, Result};
use infrastructure_rdb::Configuration;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let database_uri = env::var("DATABASE_URL")
        .context("migrate先として環境変数 `DATABASE_URL` を定義してください")?;
    let pool = Configuration::new(1, database_uri).connect().await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(())
}
