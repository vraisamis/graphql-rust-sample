use crate::{provides::ContextExt, scalar::Id};
use async_graphql::{ComplexObject, Context, Result as GqlResult, SimpleObject};
use query_resolver::{CardView, ColumnView};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Column {
    id: Id<Column>,
    title: String,
    #[graphql(skip)]
    cards_cnt: usize,
}

impl Column {
    fn new(
        id: impl Into<Id<Column>>,
        title: impl Into<String>,
        cards_cnt: impl Into<usize>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            cards_cnt: cards_cnt.into(),
        }
    }
}

#[ComplexObject]
impl Column {
    async fn cards<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<Card>> {
        let loader = ctx.data_loader()?;
        let ids: Vec<_> = (0..self.cards_cnt)
            .into_iter()
            .map(|i| (self.id.clone(), i))
            .collect();
        let map = loader.load_many(ids).await?;
        let result = Vec::from_iter(map.into_values().into_iter());
        Ok(result)
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Card {
    id: String,
    title: String,
    description: String,
}

impl Card {
    fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: description.into(),
        }
    }
}

impl From<ColumnView> for Column {
    fn from(value: ColumnView) -> Self {
        Self::new(value.id, value.title, value.card_cnt)
    }
}

impl From<CardView> for Card {
    fn from(value: CardView) -> Self {
        Self::new(value.id, value.title, value.description)
    }
}
