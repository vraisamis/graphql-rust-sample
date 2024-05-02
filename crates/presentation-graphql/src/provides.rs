use std::{any::type_name, sync::Arc};

use async_graphql::{dataloader::DataLoader, Context, Error as GqlError};
use domain_kanban::user::UserRepository;
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

pub trait RepositoryProvider
where
    Self: HasProvider<dyn UserRepository>,
{
}

impl<T> RepositoryProvider for T where Self: HasProvider<dyn UserRepository> {}

pub struct Modules {
    pub query_providers: Box<dyn QueryProvider + Send + Sync>,
    pub repository_providers: Box<dyn RepositoryProvider + Send + Sync>,
}

impl Modules {
    pub fn new(
        query_providers: Box<dyn QueryProvider + Send + Sync>,
        repository_providers: Box<dyn RepositoryProvider + Send + Sync>,
    ) -> Self {
        Self {
            query_providers,
            repository_providers,
        }
    }

    pub fn query(&self) -> &dyn QueryProvider {
        self.query_providers.as_ref()
    }
    pub fn repository(&self) -> &dyn RepositoryProvider {
        self.repository_providers.as_ref()
    }   
}

// ContextにDataLoader, Modulesを取得するメソッドを作成する
// dataはAnyで型消去しているけど、このプロジェクト内ではSchemaを作っている箇所で必ず設定しているはずなので気にしないことにする
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
    fn provide_arc_gql_result(&self) -> Result<Arc<I>, GqlError> {
        self.provide_gql_result().map(Arc::from)
    }
}

impl<T, I: ?Sized> HasProviderGql<I> for T
where
    T: HasProvider<I> + ?Sized,
{
    fn provide_gql_result(&self) -> Result<Box<I>, GqlError> {
        self.provide()
            .map_err(|e| GqlError::new(format!("providing <{}> failed: {}", type_name::<I>(), e)))
    }
}
