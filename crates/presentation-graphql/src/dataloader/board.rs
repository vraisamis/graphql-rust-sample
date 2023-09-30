use crate::scalar::Id;
use crate::Modules;
use crate::{model::Board, provides::HasProviderGql};
use async_graphql::{dataloader::Loader, Error as GqlError};
use async_trait::async_trait;
use query_resolver::BoardQuery;
use std::collections::HashMap;

#[async_trait]
impl Loader<Id<Board>> for Modules {
    type Value = Board;
    type Error = GqlError;

    async fn load(
        &self,
        keys: &[Id<Board>],
    ) -> Result<HashMap<Id<Board>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<Board> -> Board: {:?}",
            keys
        );
        let ids: Vec<_> = keys.iter().map(|i| i.clone().into()).collect();
        let board_query: Box<dyn BoardQuery> = self.m().provide_gql_result()?;
        let result = board_query.list_by_ids(&ids).await.expect("query error");
        Ok(result
            .into_iter()
            .map(|(k, v)| (k.to_string().into(), v.into()))
            .collect())
    }
}
