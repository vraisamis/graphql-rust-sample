mod model;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use model::kanban::{
    dataloader::KanbanSchema as DataLoaderSchema, simple::KanbanSchema as SimpleSchema,
    storage::KanbanSchema as StorageSchema, validate::KanbanSchema as ValidateSchema,
    SchemaWithStaticData,
};

async fn kanban_graphql_handler(
    schema: Extension<SimpleSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn kanban_validate_graphql_handler(
    schema: Extension<ValidateSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn kanban_storage_graphql_handler(
    schema: Extension<StorageSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn kanban_dataloader_graphql_handler(
    schema: Extension<DataLoaderSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql(endpoint: &str) -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint(endpoint).finish())
}

#[tokio::main]
async fn main() {
    logger_init();
    let kanban_schema = SimpleSchema::schema_with_static_data();
    let validate_schema = ValidateSchema::schema_with_static_data();
    let storage_schema = StorageSchema::schema_with_static_data();
    let datalader_schema = DataLoaderSchema::schema_with_static_data();

    let app = Router::new()
        .route("/", get(|| graphiql("/")).post(kanban_graphql_handler))
        .route(
            "/v",
            get(|| graphiql("/v")).post(kanban_validate_graphql_handler),
        )
        .route(
            "/s",
            get(|| graphiql("/s")).post(kanban_storage_graphql_handler),
        )
        .route(
            "/d",
            get(|| graphiql("/d")).post(kanban_dataloader_graphql_handler),
        )
        .layer(Extension(kanban_schema))
        .layer(Extension(validate_schema))
        .layer(Extension(storage_schema))
        .layer(Extension(datalader_schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn logger_init() {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();
}
