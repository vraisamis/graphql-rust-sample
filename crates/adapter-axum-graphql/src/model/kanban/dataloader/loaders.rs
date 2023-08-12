use super::{Board, Column, Data, Id, User};
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

#[async_trait]
impl Loader<Id<Board>> for Data {
    type Value = Board;
    type Error = String;

    async fn load(
        &self,
        keys: &[Id<Board>],
    ) -> Result<HashMap<Id<Board>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<Board> -> Board: {:?}",
            keys
        );
        let result: HashMap<Id<Board>, Board> = self
            .boards
            .iter()
            .filter_map(|b| {
                if keys.contains(&b.id) {
                    Some((b.id.clone(), b.clone()))
                } else {
                    None
                }
            })
            .collect();
        Ok(result)
    }
}

#[async_trait]
impl Loader<Id<Column>> for Data {
    type Value = Column;
    type Error = String;

    async fn load(
        &self,
        keys: &[Id<Column>],
    ) -> Result<HashMap<Id<Column>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<Column> -> Column: {:?}",
            keys
        );
        let result: HashMap<Id<Column>, Column> = self
            .columns
            .iter()
            .filter_map(|c| {
                if keys.contains(&c.id) {
                    Some((c.id.clone(), c.clone()))
                } else {
                    None
                }
            })
            .collect();
        Ok(result)
    }
}
