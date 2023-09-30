use crate::scalar::Id;
use crate::Modules;
use crate::{model::Column, provides::HasProviderGql};
use async_graphql::{dataloader::Loader, Error as GqlError};
use async_trait::async_trait;
use query_resolver::ColumnsQuery;
use std::collections::HashMap;

#[async_trait]
impl Loader<Id<Column>> for Modules {
    type Value = Column;
    type Error = GqlError;

    async fn load(
        &self,
        keys: &[Id<Column>],
    ) -> Result<HashMap<Id<Column>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<Column> -> Column: {:?}",
            keys
        );
        let ids: Vec<_> = keys.iter().map(|i| i.clone().into()).collect();
        let column_query: Box<dyn ColumnsQuery> = self.m().provide_gql_result()?;
        let result = column_query.list_by_ids(&ids).await.expect("query error");
        Ok(result
            .into_iter()
            .map(|(k, v)| (k.to_string().into(), v.into()))
            .collect())
    }
}
