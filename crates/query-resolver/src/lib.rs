use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait UsersQuery {
    async fn find_by_id(&self, id: &str) -> Result<UserView>;
    async fn list_by_ids(&self, ids: &[String]) -> Result<Vec<UserView>>;
}

pub struct UserView {
    pub id: String,
}
