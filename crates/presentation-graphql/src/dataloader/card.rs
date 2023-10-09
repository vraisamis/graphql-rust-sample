use crate::model::Column;
use crate::scalar::Id;
use crate::Modules;
use crate::{model::Card, provides::HasProviderGql};
use anyhow::Result;
use async_graphql::{dataloader::Loader, Error as GqlError};
use async_trait::async_trait;
use futures_util::future::{join_all, JoinAll};
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
        let card_query: Arc<dyn CardsQuery> = self.query().provide_arc_gql_result()?;
        let futures_iterator: Vec<_> = idmap
            .into_iter()
            .map(|(cid, us)| list_by_orders(Arc::clone(&card_query), cid, us))
            .collect();
        let v = JoinAll::from_iter(futures_iterator)
            .await
            .into_iter()
            .collect::<Result<Vec<_>>>()?;

        Ok(merge_all(v))
    }
}

// id群からクエリを呼ぶ部分を抽出
// inlineで書くと `async move` ブロックになる
async fn list_by_orders(
    card_query: Arc<dyn CardsQuery>,
    id: Id<Column>,
    indices: Vec<usize>,
) -> Result<HashMap<(Id<Column>, usize), Card>> {
    let column_id = id.clone().into();
    let hash_map = card_query.list_by_orders(&column_id, &indices).await?;
    let hash_map = hash_map
        .into_iter()
        .map(|(u, v)| ((id.clone(), u), v.into()))
        .collect::<HashMap<_, _>>();
    Ok(hash_map)
}

// hashMapをマージする
fn merge_all<K: Eq + std::hash::Hash, V>(
    hash_maps: impl IntoIterator<Item = HashMap<K, V>>,
) -> HashMap<K, V> {
    hash_maps.into_iter().fold(HashMap::new(), |mut acc, h| {
        acc.extend(h);
        acc
    })
}
