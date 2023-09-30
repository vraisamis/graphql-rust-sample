mod board;
mod column;
mod user;

pub use self::board::*;
pub use self::column::*;
pub use self::user::*;
use crate::provides::{ContextExt, HasProviderGql};
use crate::validator;
use crate::{provides::Modules, scalar::Id};
use async_graphql::{Context, Object, Result as GqlResult};
use query_resolver::UsersQuery;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = r#"validator::IdValidator::new("User", "user")"#))] id: Id<
            User,
        >,
    ) -> GqlResult<Option<User>> {
        let loader = ctx.data_loader()?;
        let r: Option<_> = loader.load_one(id).await?;
        Ok(r)
    }
    async fn users_all<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<User>> {
        let modules: &Modules = ctx.modules()?;
        let user_query: Box<dyn UsersQuery> = modules.query().provide_gql_result()?;
        let result = user_query
            .all()
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(result)
    }
}
