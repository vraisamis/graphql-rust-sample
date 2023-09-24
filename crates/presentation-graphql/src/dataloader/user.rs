use crate::scalar::Id;
use crate::Modules;
use crate::{model::User, provides::HasProviderGql};
use async_graphql::{dataloader::Loader, Error as GqlError};
use async_trait::async_trait;
use query_resolver::UsersQuery;
use std::collections::HashMap;

#[async_trait]
impl Loader<Id<User>> for Modules {
    type Value = User;
    // TODO: error type
    type Error = GqlError;

    async fn load(&self, keys: &[Id<User>]) -> Result<HashMap<Id<User>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<User> -> User: {:?}",
            keys
        );
        let ids: Vec<_> = keys.iter().map(|i| i.clone().into()).collect();
        // TODO: remove unwrap
        let user_query: Box<dyn UsersQuery> = self.m.as_ref().provide_gql_result()?;
        let result = user_query.list_by_ids(&ids).await.expect("query error");
        Ok(result
            .into_iter()
            .map(|(k, v)| (k.to_string().into(), v.into()))
            .collect())
    }
}
