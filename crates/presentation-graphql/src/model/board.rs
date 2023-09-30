use crate::{provides::ContextExt, scalar::Id};
use async_graphql::{ComplexObject, Context, Result as GqlResult, SimpleObject};
use query_resolver::BoardView;

use super::{Column, User};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Board {
    id: Id<Board>,
    title: String,
    #[graphql(skip)]
    owner_id: Id<User>,
    #[graphql(skip)]
    column_ids: Vec<Id<Column>>,
}

impl Board {
    pub fn new(
        id: impl Into<Id<Board>>,
        title: impl Into<String>,
        owner_id: impl Into<Id<User>>,
        column_ids: Vec<impl Into<Id<Column>>>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            owner_id: owner_id.into(),
            column_ids: column_ids.into_iter().map(Into::into).collect(),
        }
    }
}

#[ComplexObject]
impl Board {
    async fn owner<'ctx>(&self, ctx: &Context<'ctx>) -> GqlResult<Option<User>> {
        let loader = ctx.data_loader()?;
        let result = loader.load_one(self.owner_id.clone()).await?;
        Ok(result)
    }

    async fn columns<'ctx>(&self, ctx: &Context<'ctx>) -> GqlResult<Vec<Column>> {
        let loader = ctx.data_loader()?;
        let map = loader.load_many(self.column_ids.clone()).await?;
        let result = map.values().cloned().collect();
        Ok(result)
    }
}

impl From<BoardView> for Board {
    fn from(value: BoardView) -> Self {
        Self::new(value.id, value.title, value.owner_id, value.column_ids)
    }
}
