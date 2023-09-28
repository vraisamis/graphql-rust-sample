use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use shaku::Interface;

#[async_trait]
pub trait ColumnsQuery: Interface {
    async fn find_by_id(&self, id: &ColumnId) -> Result<ColumnView>;
    async fn list_by_ids(&self, ids: &[ColumnId]) -> Result<HashMap<ColumnId, ColumnView>>;
}

pub struct ColumnView {
    pub id: String,
    pub title: String,
    pub card_cnt: usize,
}
