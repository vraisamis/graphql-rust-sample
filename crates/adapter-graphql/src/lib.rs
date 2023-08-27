mod model;

use async_graphql::{
    http::GraphiQLSource, EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use model::QueryRoot as Query;

#[derive(Clone)]
pub struct GraphQL {
    schema: Schema<Query, EmptyMutation, EmptySubscription>,
}

impl GraphQL {
    pub fn new() -> Self {
        let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

        Self { schema }
    }

    pub fn graphiql(endpoint: &str) -> String {
        GraphiQLSource::build().endpoint(endpoint).finish()
    }

    pub async fn execute(&self, request: Request) -> Response {
        self.schema.execute(request).await
    }
}
