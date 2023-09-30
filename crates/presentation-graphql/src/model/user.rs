use crate::{provides::ContextExt, scalar::Id};
use async_graphql::{ComplexObject, Context, Result as GqlResult, SimpleObject};
use domain_util::{Entity, Identifier};
use query_resolver::UserView;

use super::Board;

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: Id<User>,
    name: String,
    email: String,
    #[graphql(skip)]
    owned_board_ids: Vec<Id<Board>>,
}

impl User {
    pub fn new(
        id: impl Into<Id<User>>,
        name: impl Into<String>,
        email: impl Into<String>,
        owned_board_ids: Vec<impl Into<Id<Board>>>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
            owned_board_ids: owned_board_ids.into_iter().map(Into::into).collect(),
        }
    }
}

#[ComplexObject]
impl User {
    async fn owned_boards<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<Board>> {
        println!("CALLED Resolver: User.owned_boards(): load_many");
        let loader = ctx.data_loader()?;
        let map = loader.load_many(self.owned_board_ids.clone()).await?;
        let result = Vec::from_iter(map.into_values().into_iter());
        Ok(result)
    }
}

impl From<UserView> for User {
    fn from(value: UserView) -> Self {
        Self::new(value.id, value.name, value.email, value.owned_board_ids)
    }
}

impl<T, U: Entity> Into<Identifier<U>> for Id<T> {
    fn into(self) -> Identifier<U> {
        self.value().parse().unwrap()
    }
}
