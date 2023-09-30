use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::board::BoardId;
use shaku::Interface;

#[async_trait]
pub trait BoardQuery: Interface {
    async fn find_by_id(&self, id: &BoardId) -> Result<BoardView>;
    async fn list_by_ids(&self, ids: &[BoardId]) -> Result<HashMap<BoardId, BoardView>>;
    async fn all(&self) -> Result<Vec<BoardView>>;
}

pub struct BoardView {
    pub id: String,
    pub title: String,
    pub owner_id: String,
    pub column_ids: Vec<String>,
}
