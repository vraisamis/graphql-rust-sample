use std::any::{type_name, TypeId};

use async_graphql::{dataloader::DataLoader, Context, Error as GqlError};
use query_resolver::UsersQuery;
use shaku::HasProvider;

pub trait QueryProvider
where
    Self: HasProvider<dyn UsersQuery>,
{
}
impl<T> QueryProvider for T where T: HasProvider<dyn UsersQuery> {}

pub struct Modules {
    pub(crate) m: Box<dyn QueryProvider + Send + Sync>,
}

impl Modules {
    pub fn new(m: Box<dyn QueryProvider + Send + Sync>) -> Self {
        Self { m }
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
