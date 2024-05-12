use anyhow::Result;

use crate::{Configuration, PgPoolImpl, Pool};

mod board;
mod card;
mod column;
mod user;

shaku::module! {
    pub Module {
        components = [super::PgPoolImpl],
        providers = [
            board::BoardQueryImpl,
            card::CardsQueryImpl,
            column::ColumnsQueryImpl,
            user::UsersQueryImpl,
        ]
    }
}

impl Module {
    // PgPoolを受け取ってOverrideしたModuleを返すメソッド
    pub fn new_with_pool(pool: Box<dyn Pool>) -> Self {
        Self::builder().with_component_override(pool).build()
    }

    // Configurationを受け取ってOverrideしたModuleを返すメソッド
    // TODO: エラー処理
    pub async fn new_with_config(config: Configuration) -> Result<Self> {
        let pool = PgPoolImpl::from_configuration(config).await?;
        let ret = Self::builder()
            .with_component_override(Box::new(pool))
            .build();

        Ok(ret)
    }
}
