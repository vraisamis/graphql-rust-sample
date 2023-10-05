use std::{collections::HashMap, str::FromStr, sync::Arc};

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::board::BoardId;
use itertools::Itertools;
use query_resolver::{BoardQuery, BoardView};
use shaku::Provider;
use sqlx::query;

use crate::Pool;

#[derive(Debug, Clone, Provider)]
#[shaku(interface=BoardQuery)]
pub struct BoardQueryImpl {
    #[shaku(inject)]
    pool: Arc<dyn Pool>,
}

#[async_trait]
impl BoardQuery for BoardQueryImpl {
    async fn find_by_id(&self, id: &BoardId) -> Result<BoardView> {
        let pool = self.pool.pool();
        let executor = pool;

        let id_string = id.to_string();
        let board = query!(
            r#"
            select b.id, b.title, ubr.user_id as owner_id
            from boards b
                inner join user_board_relations ubr on b.id = ubr.board_id
            where b.id = $1
            "#,
            &id_string
        )
        .fetch_one(executor)
        .await?;

        let column_ids: Vec<_> = query!(
            r#"
                select column_id
                from board_column_relations
                where board_id = $1
            "#,
            &id_string
        )
        .fetch_all(executor)
        .await?;
        let column_ids = column_ids.into_iter().map(|r| r.column_id).collect();

        let result = BoardView {
            id: board.id,
            title: board.title,
            owner_id: board.owner_id,
            column_ids,
        };

        Ok(result)
    }

    async fn list_by_ids(&self, ids: &[BoardId]) -> Result<HashMap<BoardId, BoardView>> {
        let pool = self.pool.pool();
        let executor = pool;
        let ids_string: Vec<_> = ids.iter().map(ToString::to_string).collect();

        let boards = query!(
            r#"
            select b.id, b.title, ubr.user_id as owner_id
            from boards b
                inner join user_board_relations ubr on b.id = ubr.board_id
            where b.id = any($1)
            "#,
            &ids_string
        )
        .fetch_all(executor)
        .await?;

        let column_ids: Vec<_> = query!(
            r#"
                select board_id, column_id
                from board_column_relations
                where board_id = any($1)
            "#,
            &ids_string
        )
        .fetch_all(executor)
        .await?;

        let mut column_id_map: HashMap<_, _> = column_ids
            .into_iter()
            .map(|r| (r.board_id, r.column_id))
            .into_group_map();

        let result = boards
            .into_iter()
            .map(|b| to_view_kv(b.id, b.title, b.owner_id, &mut column_id_map))
            .collect();
        Ok(result)
    }

    async fn all(&self) -> Result<Vec<BoardView>> {
        let pool = self.pool.pool();
        let executor = pool;

        let boards = query!(
            r#"
            select b.id, b.title, ubr.user_id as owner_id
            from boards b
                inner join user_board_relations ubr on b.id = ubr.board_id
            "#,
        )
        .fetch_all(executor)
        .await?;

        let column_ids: Vec<_> = query!(
            r#"
                select board_id, column_id
                from board_column_relations
            "#,
        )
        .fetch_all(executor)
        .await?;

        let mut column_id_map: HashMap<_, _> = column_ids
            .into_iter()
            .map(|r| (r.board_id, r.column_id))
            .into_group_map();

        let result = boards
            .into_iter()
            .map(|b| to_view(b.id, b.title, b.owner_id, &mut column_id_map))
            .collect();
        Ok(result)
    }
}

fn to_view(
    id: String,
    title: String,
    owner_id: String,
    column_id_map: &mut HashMap<String, Vec<String>>,
) -> BoardView {
    let column_ids = column_id_map.remove(&id).unwrap_or_else(|| vec![]);
    BoardView {
        id,
        title,
        owner_id,
        column_ids,
    }
}

fn to_view_kv(
    id: String,
    title: String,
    owner_id: String,
    column_id_map: &mut HashMap<String, Vec<String>>,
) -> (BoardId, BoardView) {
    let key = FromStr::from_str(&id).unwrap();
    (key, to_view(id, title, owner_id, column_id_map))
}
