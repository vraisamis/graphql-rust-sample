mod dataloader;
mod extensions;
mod model;
mod provides;
mod scalar;
mod validator;

use async_graphql::{
    dataloader::DataLoader, extensions::Logger, http::GraphiQLSource, EmptyMutation,
    EmptySubscription, Request, Response, Schema, SchemaBuilder,
};
use extensions::RestrictQueryAliases;
use futures_util::future::BoxFuture;
use model::QueryRoot as Query;
pub use provides::Modules;

type SchemaType = Schema<Query, EmptyMutation, EmptySubscription>;
type SchemaBuilderType = SchemaBuilder<Query, EmptyMutation, EmptySubscription>;

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
        let schema = schema_with(|mut s| {
            if cfg!(not(debug_assertions)) {
                s = s.disable_suggestions().disable_introspection();
            }
            s
                // NOTE: Modulesをdataに持っていることはContextからは見られないけど、諦めた方がよさそう
                .data(DataLoader::new(m, spawner))
                .extension(Logger)
                .extension(RestrictQueryAliases::default())
        });

        Self { schema }
    }

    pub fn sdl() -> String {
        schema().sdl()
    }

    pub fn graphiql(endpoint: &str) -> String {
        GraphiQLSource::build().endpoint(endpoint).finish()
    }

    pub async fn execute(&self, request: Request) -> Response {
        self.schema.execute(request).await
    }
}

fn schema_builder() -> SchemaBuilderType {
    Schema::build(Query, EmptyMutation, EmptySubscription)
}

fn schema() -> SchemaType {
    schema_builder().finish()
}

fn schema_with<F>(f: F) -> SchemaType
where
    F: FnOnce(SchemaBuilderType) -> SchemaBuilderType,
{
    f(schema_builder()).finish()
}
