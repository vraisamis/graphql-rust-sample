mod user;

use async_graphql::{dataloader::DataLoader, Context, Object, Result as GqlResult};
use query_resolver::UsersQuery;

use crate::{provides::Modules, scalar::Id, Injections};

use self::user::User;
use crate::validator;
use shaku::HasProvider;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new()"))] id: Id<User>,
    ) -> GqlResult<Option<User>> {
        let loader: &DataLoader<Injections> = ctx.data()?;
        let r: Option<_> = loader.load_one(id).await?;
        Ok(r)
    }
    async fn users<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<User>> {
        let modules: &Modules = ctx.data()?;
        // TODO remove unwrap
        let user_query = HasProvider::<dyn UsersQuery>::provide(modules.m.as_ref()).unwrap();
        // let i: &Injections = ctx.data()?;
        // let result = i
        let result = user_query
            .all()
            .await?
            .into_iter()
            .map(Into::into)
            .collect();
        Ok(result)
    }
}
