use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use query_resolver::{CardView, CardsQuery};
use shaku::Provider;
use sqlx::query;

use crate::Pool;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = CardsQuery)]
pub struct CardsQueryImpl {
    #[shaku(inject)]
    pool: Arc<dyn Pool>,
}

impl CardsQueryImpl {}

#[async_trait]
impl CardsQuery for CardsQueryImpl {
    async fn find_by_order(&self, column_id: &ColumnId, order: &usize) -> Result<CardView> {
        let pool = self.pool.pool();
        let executor = pool;

        let column_id_string = column_id.to_string();
        let card = query!(
            r#"
            select c.id, c.title, c.description
            from cards c
            where c.column_id = $1
            -- 暫定sort
            order by id
            limit 1
            offset $2
            "#,
            &column_id_string,
            i64::try_from(*order)?
        )
        .fetch_one(executor)
        .await?;

        let result = CardView {
            id: card.id,
            title: card.title,
            description: card.description.unwrap_or_else(|| "".into()),
        };
        Ok(result)
    }

    async fn list_by_orders(
        &self,
        column_id: &ColumnId,
        orders: &[usize],
    ) -> Result<HashMap<usize, CardView>> {
        let pool = self.pool.pool();
        let executor = pool;

        let column_id_string = column_id.to_string();
        let orders_i64: Result<Vec<_>, _> = orders.iter().map(|o| i64::try_from(*o)).collect();
        let orders_i64 = orders_i64?;
        let min_order = orders_i64.iter().min().unwrap_or(&0);
        let max_order = orders_i64.iter().max().unwrap_or(&0);
        let length = 1 + max_order - min_order;
        let cards = query!(
            r#"
            select c.id, c.title, c.description
            from cards c
            where c.column_id = $1
            -- 暫定sort
            order by id
            limit $3
            offset $2
            "#,
            &column_id_string,
            min_order,
            length,
        )
        .fetch_all(executor)
        .await?;

        let result = cards
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                (
                    i + (*min_order as usize),
                    to_view(c.id, c.title, c.description),
                )
            })
            .filter(|(k, _)| orders.contains(k))
            .collect();
        Ok(result)
    }
}
fn to_view(id: String, title: String, description: Option<String>) -> CardView {
    CardView {
        id,
        title,
        description: description.unwrap_or_else(|| "".into()),
    }
}
