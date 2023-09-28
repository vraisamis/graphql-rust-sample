use std::collections::HashMap;

use crate::sample;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::user::UserId;
use query_resolver::{UserView, UsersQuery};
use shaku::Provider;

#[derive(Debug, Clone, Provider)]
#[shaku(interface = UsersQuery)]
pub struct UsersQueryImpl;

fn to_view(u: sample::User) -> UserView {
    UserView {
        id: u.id.to_string(),
        name: u.name,
        email: u.email,
    }
}

#[async_trait]
impl UsersQuery for UsersQueryImpl {
    async fn find_by_id(&self, id: &UserId) -> Result<UserView> {
        let data = sample::data();
        let result = data
            .users
            .clone()
            .into_iter()
            .filter(|u| &u.id == id)
            .map(to_view)
            .next()
            .ok_or(anyhow!("Not Found"))?;
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
