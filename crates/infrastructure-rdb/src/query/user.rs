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

fn to_view(u: sample::User) -> UserView {
    UserView {
        id: u.id.to_string(),
        name: u.name,
        email: u.email,
        owned_board_ids: u
            .owned_board_ids
            .into_iter()
            .map(|i| i.to_string())
            .collect(),
    }
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
            .map(|u| {
                // TODO
                let key = UserId::from_str(&u.id).unwrap();
                let owned_board_ids = owned_board_map.remove(&u.id).unwrap_or_else(|| vec![]);
                let value = UserView {
                    id: u.id,
                    name: u.name,
                    email: u.email,
                    owned_board_ids,
                };
                (key, value)
            })
            .collect();
        Ok(result)
    }
    async fn all(&self) -> Result<Vec<UserView>> {
        let data = sample::data();
        let result = data.users.clone().into_iter().map(to_view).collect();
        Ok(result)
    }
}
