use anyhow::Result;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{extract::Extension, response::Html, routing::get, Router, Server};

use presentation_graphql::{GraphQL, Spawner};

pub struct App;

impl App {
    // TODO: return type
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    // TODO: return type
    pub async fn run<S, R>(&self, spawner: S) -> Result<()>
    where
        S: Spawner<R>,
    {
        let gql = GraphQL::new(spawner);

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
