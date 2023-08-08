use std::any::Any;

use async_graphql::{ObjectType, Schema, SubscriptionType};

pub mod simple;
pub mod validate;
// TODO
// pub mod federation;
// pub mod dataloader;

pub trait SchemaWithStaticData<T, Query, Mutation, Subscription>
where
    T: Any + Send + Sync,
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    fn query() -> Query;
    fn mutation() -> Mutation;
    fn subscription() -> Subscription;
    fn data() -> T;
    fn schema_with_static_data() -> Schema<Query, Mutation, Subscription> {
        Schema::build(Self::query(), Self::mutation(), Self::subscription())
            .data(Self::data())
            .finish()
    }
}
