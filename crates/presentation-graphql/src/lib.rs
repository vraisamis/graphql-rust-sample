mod model;
mod provides;
mod scalar;
mod validator;

use async_graphql::{
    dataloader::DataLoader, extensions::Logger, http::GraphiQLSource, EmptyMutation,
    EmptySubscription, Request, Response, Schema,
};
use futures_util::future::BoxFuture;
use infrastructure_rdb::UsersQueryImpl;
use model::QueryRoot as Query;
pub use provides::Modules;

#[derive(Clone)]
pub struct GraphQL {
    schema: Schema<Query, EmptyMutation, EmptySubscription>,
}

// Spawnerは利用するライブラリに依存しないよう、traitで受ける
// trait aliasが書けないので、依存させたtrait定義＋全称実装
pub trait Spawner<R>: Fn(BoxFuture<'static, ()>) -> R + Send + Sync + 'static {}
// trait implementation
impl<T, R> Spawner<R> for T where T: Fn(BoxFuture<'static, ()>) -> R + Send + Sync + 'static {}

impl GraphQL {
    pub fn new<S, R>(spawner: S, m: Modules) -> Self
    where
        S: Spawner<R>,
    {
        let i = Injections::new();
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
            // .data(DataLoader::new(i.clone(), spawner))
            // // TODO: dataloaderのみにする
            // .data(i)
            // TODO: dataloader
            // NOTE: Modulesをdataに持っていることはContextからは見られないけど、諦めた方がよさそう
            .data(m)
            .extension(Logger)
            .finish();

        Self { schema }
    }

    pub fn graphiql(endpoint: &str) -> String {
        GraphiQLSource::build().endpoint(endpoint).finish()
    }

    pub async fn execute(&self, request: Request) -> Response {
        self.schema.execute(request).await
    }
}

// TODO: あとでinfrastructureを除去
#[derive(Debug, Clone)]
pub struct Injections {
    user_query: UsersQueryImpl,
}

impl Injections {
    pub fn new() -> Self {
        Self {
            user_query: UsersQueryImpl,
        }
    }
}
