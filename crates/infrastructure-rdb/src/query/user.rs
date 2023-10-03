use std::{collections::HashMap, sync::Arc};

use crate::{sample, Pool};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::user::UserId;
use query_resolver::{UserView, UsersQuery};
use shaku::Provider;
use sqlx::query_as;

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
        let (user_id, name, email): (String, String, String) = query_as(
            r#"
            select
                u.id
                , u.name
                , u.email
            from users u
            where u.id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_one(executor)
        .await?;
        let owned_board_ids: Vec<(String,)> = query_as(
            r#"
            select board_id
            from user_board_relations
            where user_id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_all(executor)
        .await?;

        let result = UserView {
            id: user_id,
            name,
            email,
            owned_board_ids: owned_board_ids.into_iter().map(|(s,)| s).collect(),
        };

        Ok(result)
    }
    async fn list_by_ids(&self, ids: &[UserId]) -> Result<HashMap<UserId, UserView>> {
        let data = sample::data();
        let result = data
            .users
            .clone()
            .into_iter()
            .filter_map(|u| ids.contains(&u.id).then(|| (u.id.clone(), to_view(u))))
            .collect();
        Ok(result)
    }
    async fn all(&self) -> Result<Vec<UserView>> {
        let data = sample::data();
        let result = data.users.clone().into_iter().map(to_view).collect();
        Ok(result)
    }
}
