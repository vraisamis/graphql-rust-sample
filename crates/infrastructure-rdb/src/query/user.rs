use std::{collections::HashMap, str::FromStr, sync::Arc};

use crate::{sample, Pool};
use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::user::UserId;
use itertools::Itertools;
use query_resolver::{UserView, UsersQuery};
use shaku::Provider;
use sqlx::query;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = UsersQuery)]
pub struct UsersQueryImpl {
    #[shaku(inject)]
    pool: Arc<dyn Pool>,
}

#[async_trait]
impl UsersQuery for UsersQueryImpl {
    async fn find_by_id(&self, id: &UserId) -> Result<UserView> {
        let pool = self.pool.pool();
        let executor = pool;

        let id_string = id.to_string();
        let user = query!(
            r#"
            select u.id, u.name, u.email
            from users u
            where u.id = $1
            "#,
            &id_string
        )
        .fetch_one(executor)
        .await?;

        let owned_board_ids: Vec<_> = query!(
            r#"
                select board_id
                from user_board_relations
                where user_id = $1
            "#,
            &id_string
        )
        .fetch_all(executor)
        .await?;
        let owned_board_ids = owned_board_ids.into_iter().map(|r| r.board_id).collect();

        let result = UserView {
            id: user.id,
            name: user.name,
            email: user.email,
            owned_board_ids,
        };

        Ok(result)
    }
    async fn list_by_ids(&self, ids: &[UserId]) -> Result<HashMap<UserId, UserView>> {
        let pool = self.pool.pool();
        let executor = pool;
        let ids_string: Vec<_> = ids.iter().map(ToString::to_string).collect();

        let users = query!(
            r#"
            select u.id, u.name, u.email
            from users u
            where u.id = any($1)
            "#,
            &ids_string
        )
        .fetch_all(executor)
        .await?;

        let owned_board_ids: Vec<_> = query!(
            r#"
                select user_id, board_id
                from user_board_relations
                where user_id = any($1)
            "#,
            &ids_string
        )
        .fetch_all(executor)
        .await?;

        let mut owned_board_map: HashMap<_, _> = owned_board_ids
            .into_iter()
            .map(|r| (r.user_id, r.board_id))
            .into_group_map();

        let result = users
            .into_iter()
            .map(|u| to_view_kv(u.id, u.name, u.email, &mut owned_board_map))
            .collect();
        Ok(result)
    }
    async fn all(&self) -> Result<Vec<UserView>> {
        let pool = self.pool.pool();
        let executor = pool;

        let users = query!(
            r#"
            select u.id, u.name, u.email
            from users u
            "#
        )
        .fetch_all(executor)
        .await?;

        let owned_board_ids: Vec<_> = query!(
            r#"
                select user_id, board_id
                from user_board_relations
            "#
        )
        .fetch_all(executor)
        .await?;

        let mut owned_board_map: HashMap<_, _> = owned_board_ids
            .into_iter()
            .map(|r| (r.user_id, r.board_id))
            .into_group_map();

        let result = users
            .into_iter()
            .map(|u| to_view(u.id, u.name, u.email, &mut owned_board_map))
            .collect();
        Ok(result)
    }
}

fn to_view(
    id: String,
    name: String,
    email: String,
    owned_board_map: &mut HashMap<String, Vec<String>>,
) -> UserView {
    let owned_board_ids = owned_board_map.remove(&id).unwrap_or_else(|| vec![]);
    UserView {
        id,
        name,
        email,
        owned_board_ids,
    }
}

fn to_view_kv(
    id: String,
    name: String,
    email: String,
    owned_board_map: &mut HashMap<String, Vec<String>>,
) -> (UserId, UserView) {
    let key = UserId::from_str(&id).unwrap();
    (key, to_view(id, name, email, owned_board_map))
}
