use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use query_resolver::{ColumnView, ColumnsQuery};
use shaku::Provider;
use sqlx::query;

use crate::Pool;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = ColumnsQuery)]
pub struct ColumnsQueryImpl {
    #[shaku(inject)]
    pool: Arc<dyn Pool>,
}

#[async_trait]
impl ColumnsQuery for ColumnsQueryImpl {
    async fn find_by_id(&self, id: &ColumnId) -> Result<ColumnView> {
        let pool = self.pool.pool();
        let executor = pool;

        let id_string = id.to_string();
        let column = query!(
            r#"
            select c.id, c.title, count(distinct ca.id) as "card_cnt"
            from columns c
                left outer join cards ca on c.id = ca.column_id
            where c.id = $1
            group by 1, 2
            "#,
            &id_string
        )
        .fetch_one(executor)
        .await?;

        let result = to_view(column.id, column.title, column.card_cnt)?;
        Ok(result)
    }

    async fn list_by_ids(&self, ids: &[ColumnId]) -> Result<HashMap<ColumnId, ColumnView>> {
        let pool = self.pool.pool();
        let executor = pool;
        let ids_string: Vec<_> = ids.iter().map(ToString::to_string).collect();

        let columns = query!(
            r#"
            -- NOTE: `count(...) as "card_cnt!: usize"` は、usizeにdecodeできないので不可。i64で帰ってきたのをTryFromするしかない
            select c.id, c.title, count(distinct ca.id) as "card_cnt"
            from columns c
                left outer join cards ca on c.id = ca.column_id
            where c.id = any($1)
            group by 1, 2
            "#,
            &ids_string
        )
        .fetch_all(executor)
        .await?;

        let result: Result<Vec<_>> = columns
            .into_iter()
            .map(|column| to_view_kv(column.id, column.title, column.card_cnt))
            .collect();
        Ok(HashMap::from_iter(result?.into_iter()))
    }
}

fn to_view(id: String, title: String, card_cnt: Option<i64>) -> Result<ColumnView> {
    let card_cnt = card_cnt.unwrap_or(0).try_into()?;
    let result = ColumnView {
        id,
        title,
        card_cnt,
    };
    Ok(result)
}

fn to_view_kv(id: String, title: String, card_cnt: Option<i64>) -> Result<(ColumnId, ColumnView)> {
    let key = FromStr::from_str(&id).unwrap();
    let result = (key, to_view(id, title, card_cnt)?);
    Ok(result)
}
