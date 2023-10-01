mod query;
mod sample;

pub use query::Module as QueryModule;

use anyhow::Result;
use shaku::{Component, Interface};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct Configuration {
    max_connections: u32,
    uri: String,
}

impl Configuration {
    pub async fn connect(self) -> Result<PgPool> {
        let result = PgPoolOptions::new()
            .max_connections(self.max_connections)
            .connect(&self.uri)
            .await?;
        Ok(result)
    }
}

// TODO: injectionにする
impl Default for Configuration {
    fn default() -> Self {
        Self {
            max_connections: 5,
            uri: "postgres://gql:postgres@localhost:60003/gql_sample".to_owned(),
        }
    }
}

pub trait Pool: Interface {
    fn pool(&self) -> &PgPool;
}

#[derive(Component)]
#[shaku(interface = Pool)]
pub struct PgPoolImpl {
    pool: PgPool,
}

impl PgPoolImpl {
    pub async fn from_configuration(conf: Configuration) -> Result<Self> {
        let slf = Self {
            pool: conf.connect().await?,
        };
        Ok(slf)
    }
}

impl Pool for PgPoolImpl {
    fn pool(&self) -> &PgPool {
        &self.pool
    }
}
