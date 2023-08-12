use super::{Data, Id, User};
use anyhow::Result as AResult;
use async_graphql::{
    dataloader::{DataLoader, Loader},
    Context,
};
use async_trait::async_trait;
use std::{collections::HashMap, fmt::Display};

pub async fn load_one<'ctx, K>(
    ctx: &Context<'ctx>,
    key: K,
) -> Result<Option<<Data as Loader<K>>::Value>, async_graphql::Error>
where
    K: Send + Sync + core::hash::Hash + Eq + Clone + 'static,
    Data: Loader<K>,
    <Data as Loader<K>>::Value: Send + Sync + Clone + 'static,
    <Data as Loader<K>>::Error: Display + Send + Sync + Clone + 'static,
{
    let loader = ctx.data::<DataLoader<Data>>()?;
    let r: Option<_> = loader.load_one::<K>(key).await?;
    Ok(r)
}

pub async fn load_many<'ctx, K, I>(
    ctx: &Context<'ctx>,
    keys: I,
) -> Result<HashMap<K, <Data as Loader<K>>::Value>, async_graphql::Error>
where
    K: Send + Sync + core::hash::Hash + Eq + Clone + 'static,
    I: IntoIterator<Item = K>,
    Data: Loader<K>,
    <Data as Loader<K>>::Value: Send + Sync + Clone + 'static,
    <Data as Loader<K>>::Error: Display + Send + Sync + Clone + 'static,
{
    let loader = ctx.data::<DataLoader<Data>>()?;
    let r = loader.load_many::<K, I>(keys).await?;
    Ok(r)
}

#[async_trait]
impl Loader<Id<User>> for Data {
    type Value = User;
    type Error = String;

    async fn load(&self, keys: &[Id<User>]) -> Result<HashMap<Id<User>, Self::Value>, Self::Error> {
        // 出力されない、メインスレッドではないから？
        println!(
            "[Dataloader] CALLED DataLoader of Id<User> -> User: {:?}",
            keys
        );
        let result: HashMap<Id<User>, User> = self
            .users
            .iter()
            .filter_map(|u| {
                if keys.contains(&u.id) {
                    Some((u.id.clone(), u.clone()))
                } else {
                    None
                }
            })
            .collect();
        Ok(result)
    }
}
