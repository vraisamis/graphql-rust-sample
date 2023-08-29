mod user;

use async_graphql::{dataloader::DataLoader, Context, Object, Result as GqlResult};

use crate::{scalar::Id, Injections};

use self::user::User;
use crate::validator;

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
}
