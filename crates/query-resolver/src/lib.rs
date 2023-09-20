use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use domain_kanban::user::UserId;

#[async_trait]
pub trait UsersQuery {
    async fn find_by_id(&self, id: &UserId) -> Result<UserView>;
    async fn list_by_ids(&self, ids: &[UserId]) -> Result<HashMap<UserId, UserView>>;
    async fn all(&self) -> Result<Vec<UserView>>;
}

pub struct UserView {
    pub id: String,
}
