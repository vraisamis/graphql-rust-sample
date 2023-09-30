use crate::model::Column;
use crate::scalar::Id;
use crate::Modules;
use crate::{model::Card, provides::HasProviderGql};
use anyhow::Result;
use async_graphql::{dataloader::Loader, Error as GqlError};
use async_trait::async_trait;
use futures_util::future::join_all;
use itertools::Itertools;
use query_resolver::CardsQuery;
use std::collections::HashMap;
use std::sync::Arc;

#[async_trait]
impl Loader<(Id<Column>, usize)> for Modules {
    type Value = Card;
    type Error = GqlError;

    async fn load(
        &self,
        keys: &[(Id<Column>, usize)],
    ) -> Result<HashMap<(Id<Column>, usize), Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<Card> -> Card: {:?}",
            keys
        );
        let idmap: HashMap<_, _> = Vec::from(keys).into_iter().into_group_map();
        let card_query: Box<dyn CardsQuery> = self.m().provide_gql_result()?;
        let card_query: Arc<dyn CardsQuery> = Arc::from(card_query);
        let result_future: Vec<_> = idmap
            .into_iter()
            .map(|(cid, us)| {
                let card_query_clone = Arc::clone(&card_query);
                async move {
                    let column_id = cid.clone().into();
                    let hash_map = card_query_clone.list_by_orders(&column_id, &us).await?;
                    let hash_map = hash_map
                        .into_iter()
                        .map(|(u, v)| ((cid.clone(), u), v.into()))
                        .collect::<HashMap<_, _>>();
                    Result::<_>::Ok(hash_map)
                }
            })
            .collect();
        let result = join_all(result_future).await;

        let result = result.into_iter().collect::<Result<Vec<_>>>()?;
        let result = result.into_iter().fold(HashMap::new(), |mut acc, h| {
            acc.extend(h);
            acc
        });
        Ok(result)
    }
}
