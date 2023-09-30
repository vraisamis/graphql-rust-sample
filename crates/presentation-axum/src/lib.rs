use anyhow::Result;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, response::Html, routing::get, Router, Server};

pub use presentation_graphql::Modules;
use presentation_graphql::{GraphQL, Spawner};

pub struct App;

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub async fn run<S, R>(&self, spawner: S, m: Modules) -> Result<()>
    where
        S: Spawner<R>,
    {
        let gql = GraphQL::new(spawner, m);

        let app = Router::new()
            .route(
                "/",
                get(|| async { Html(GraphQL::graphiql("/")) }).post(graphql_handler),
            )
            .layer(Extension(gql));

        println!("GraphiQL IDE: http://localhost:8000");
        Server::bind(&"127.0.0.1:8000".parse().unwrap())
            .serve(app.into_make_service())
            .await?;
        Ok(())
    }
}

async fn graphql_handler(gql: Extension<GraphQL>, req: GraphQLRequest) -> GraphQLResponse {
    gql.execute(req.into_inner()).await.into()
}
