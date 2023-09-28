use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::column::ColumnId;
use shaku::Interface;

#[async_trait]
pub trait CardsQuery: Interface {
    async fn find_by_order(&self, column_id: &ColumnId, order: &usize) -> Result<CardView>;
    async fn list_by_orders(
        &self,
        column_id: &ColumnId,
        orders: &[usize],
    ) -> Result<HashMap<usize, CardView>>;
}

pub struct CardView {
    pub id: String,
    pub title: String,
    pub description: String,
}
