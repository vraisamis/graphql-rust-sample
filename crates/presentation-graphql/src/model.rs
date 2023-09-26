mod user;

pub use self::user::*;
use crate::provides::{ContextExt, HasProviderGql};
use crate::validator;
use crate::{provides::Modules, scalar::Id};
use async_graphql::{dataloader::DataLoader, Context, Object, Result as GqlResult};
use query_resolver::UsersQuery;
use shaku::HasProvider;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new()"))] id: Id<User>,
    ) -> GqlResult<Option<User>> {
        let loader = ctx.data_loader()?;
        let r: Option<_> = loader.load_one(id).await?;
        Ok(r)
    }
    async fn users<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<User>> {
        let modules: &Modules = ctx.modules()?;
        let user_query = modules.m().provide_gql_result()?;
        let result = user_query
            .all()
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(result)
    }
}
