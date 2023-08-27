use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // TODO
    async fn foo(&self) -> String {
        "foo".to_owned()
    }
}
