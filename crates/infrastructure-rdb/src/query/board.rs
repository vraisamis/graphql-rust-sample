use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::board::BoardId;
use query_resolver::{BoardQuery, BoardView};
use shaku::Provider;

use crate::sample;

#[derive(Debug, Clone, Provider)]
#[shaku(interface=BoardQuery)]
pub struct BoardQueryImpl;

#[async_trait]
impl BoardQuery for BoardQueryImpl {
    async fn find_by_id(&self, id: &BoardId) -> Result<BoardView> {
        let data = sample::data();
        let result = data
            .boards
            .clone()
            .into_iter()
            .filter(|b| &b.id == id)
            .map(to_view)
            .next()
            .ok_or(anyhow!("Not Found"))?;
        Ok(result)
    }

    async fn list_by_ids(&self, ids: &[BoardId]) -> Result<HashMap<BoardId, BoardView>> {
        let data = sample::data();
        let result = data
            .boards
            .clone()
            .into_iter()
            .filter_map(|b| ids.contains(&b.id).then(|| (b.id.clone(), to_view(b))))
            .collect();
        Ok(result)
    }

    async fn all(&self) -> Result<Vec<BoardView>> {
        let data = sample::data();
        let result = data.boards.clone().into_iter().map(to_view).collect();
        Ok(result)
    }
}

fn to_view(b: sample::Board) -> BoardView {
    BoardView {
        id: b.id.to_string(),
        title: b.title,
        owner_id: b.owner_id.to_string(),
        column_ids: b.column_ids.into_iter().map(|i| i.to_string()).collect(),
    }
}
