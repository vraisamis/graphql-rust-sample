use std::collections::HashMap;

use crate::sample;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use query_resolver::{ColumnView, ColumnsQuery};
use shaku::Provider;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = ColumnsQuery)]
pub struct ColumnsQueryImpl;

fn to_view(u: sample::Column) -> ColumnView {
    ColumnView {
        id: u.id.to_string(),
        title: u.title,
        card_cnt: u.cards.len(),
    }
}

#[async_trait]
impl ColumnsQuery for ColumnsQueryImpl {
    async fn find_by_id(&self, id: &ColumnId) -> Result<ColumnView> {
        let data = sample::data();
        let result = data
            .columns
            .clone()
            .into_iter()
            .filter(|u| &u.id == id)
            .map(to_view)
            .next()
            .ok_or(anyhow!("Not Found"))?;
        Ok(result)
    }
    async fn list_by_ids(&self, ids: &[ColumnId]) -> Result<HashMap<ColumnId, ColumnView>> {
        let data = sample::data();
        let result = data
            .columns
            .clone()
            .into_iter()
            .filter_map(|u| ids.contains(&u.id).then(|| (u.id.clone(), to_view(u))))
            .collect();
        Ok(result)
    }
}
