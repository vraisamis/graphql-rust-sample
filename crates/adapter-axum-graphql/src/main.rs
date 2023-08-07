mod model;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use model::{
    kanban::{simple::KanbanSchema as SimpleSchema, SchemaWithStaticData},
    starwars::{QueryRoot, StarWars, StarWarsSchema},
};

async fn starwars_graphql_handler(
    schema: Extension<StarWarsSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn kanban_graphql_handler(
    schema: Extension<SimpleSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql(endpoint: &str) -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint(endpoint).finish())
}

#[tokio::main]
async fn main() {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(StarWars::new())
        .finish();
    let kanban_schema = SimpleSchema::schema_with_static_data();

    let app = Router::new()
        .route(
            "/starwars",
            get(|| graphiql("/starwars")).post(starwars_graphql_handler),
        )
        .route("/", get(|| graphiql("/")).post(kanban_graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(kanban_schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
