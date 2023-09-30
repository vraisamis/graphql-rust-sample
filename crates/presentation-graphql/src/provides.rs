use std::any::type_name;

use async_graphql::{dataloader::DataLoader, Context, Error as GqlError};
use query_resolver::{BoardQuery, CardsQuery, ColumnsQuery, UsersQuery};
use shaku::HasProvider;

pub trait QueryProvider
where
    Self: HasProvider<dyn UsersQuery>,
    Self: HasProvider<dyn BoardQuery>,
    Self: HasProvider<dyn ColumnsQuery>,
    Self: HasProvider<dyn CardsQuery>,
{
}
impl<T> QueryProvider for T
where
    Self: HasProvider<dyn UsersQuery>,
    Self: HasProvider<dyn BoardQuery>,
    Self: HasProvider<dyn ColumnsQuery>,
    Self: HasProvider<dyn CardsQuery>,
{
}

pub struct Modules {
    pub(crate) m: Box<dyn QueryProvider + Send + Sync>,
}

impl Modules {
    pub fn new(m: Box<dyn QueryProvider + Send + Sync>) -> Self {
        Self { m }
    }

    pub fn m(&self) -> &dyn QueryProvider {
        self.m.as_ref()
    }
}

// ContextにDataLoader, Modulesを取得するメソッドを作成する
pub trait ContextExt {
    fn data_loader(&self) -> Result<&DataLoader<Modules>, GqlError>;
    fn modules(&self) -> Result<&Modules, GqlError> {
        Ok(self.data_loader()?.loader())
    }
}

impl<'ctx> ContextExt for Context<'ctx> {
    fn data_loader(&self) -> Result<&DataLoader<Modules>, GqlError> {
        self.data()
    }
}

// shakuのErrorをasync-graphqlにあわせる
pub trait HasProviderGql<I: ?Sized>: HasProvider<I> {
    fn provide_gql_result(&self) -> Result<Box<I>, GqlError>;
}

impl<T, I: ?Sized> HasProviderGql<I> for T
where
    T: HasProvider<I> + ?Sized,
{
    fn provide_gql_result(&self) -> Result<Box<I>, GqlError> {
        self.provide().map_err(|e| {
            GqlError::new(format!(
                "providing <{}> failed: {}",
                type_name::<I>(),
                e.to_string()
            ))
        })
    }
}
