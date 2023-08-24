mod loaders;

use super::SchemaWithStaticData;
use async_graphql::{
    dataloader::DataLoader, extensions, ComplexObject, Context, EmptyMutation, EmptySubscription,
    Error as GqlError, InputValueError, Object, Result as GqlResult, Scalar, ScalarType, Schema,
    SimpleObject, Value,
};
use loaders::{load_many, load_one};
use std::{hash::Hash, marker::PhantomData};

pub type KanbanSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

impl SchemaWithStaticData<Data, QueryRoot, EmptyMutation, EmptySubscription> for KanbanSchema {
    fn query() -> QueryRoot {
        QueryRoot
    }

    fn mutation() -> EmptyMutation {
        EmptyMutation
    }

    fn subscription() -> EmptySubscription {
        EmptySubscription
    }

    fn data() -> Data {
        let users = vec![
            User::new("u0", "aaa", "aaa@example.com", vec!["b0", "b2"]),
            User::new("u1", "bbb", "bbb@example.com", vec!["b1"]),
            User::new("u2", "ccc", "ccc@example.com", Vec::<String>::new()),
        ];
        let columns = vec![
            Column::new(
                "o0",
                "TODO",
                vec![
                    Card::new("c0", "掃除", ""),
                    Card::new("c1", "洗濯", ""),
                    Card::new("c2", "食事", ""),
                ],
            ),
            Column::new("o1", "doing", vec![Card::new("c3", "ゴミ出し", "")]),
            Column::new(
                "o2",
                "DONE",
                vec![
                    Card::new("c14", "買い物", ""),
                    Card::new("c15", "洗い物", ""),
                ],
            ),
            Column::new(
                "o3",
                "wish",
                vec![
                    Card::new("c4", "ランタン", ""),
                    Card::new("c5", "本棚", "いろいろ"),
                ],
            ),
            Column::new(
                "o4",
                "bought",
                vec![
                    Card::new("c13", "石鹸", ""),
                    Card::new("c12", "常備薬", ""),
                    Card::new("c11", "米", ""),
                    Card::new("c9", "センサーライト", ""),
                    Card::new("c10", "飲み物", ""),
                ],
            ),
            Column::new("o5", "pending", vec![]),
            Column::new(
                "o6",
                "challenge",
                vec![
                    Card::new("c8", "ゴハッチュウ", ""),
                    Card::new("c7", "ダソッキー", ""),
                    Card::new("c6", "床下三兄弟", ""),
                ],
            ),
            Column::new("o7", "got", vec![]),
        ];
        let boards = vec![
            Board::new("b0", "yarukoto", "u0", vec!["o0", "o1", "o2"]),
            Board::new("b1", "wishlist", "u1", vec!["o3", "o4", "o5"]),
            Board::new("b2", "monster", "u0", vec!["o6", "o7"]),
        ];

        Data {
            users,
            boards,
            columns,
        }
    }

    fn schema_with_static_data() -> Schema<QueryRoot, EmptyMutation, EmptySubscription> {
        Schema::build(Self::query(), Self::mutation(), Self::subscription())
            .data(Self::data())
            .data(DataLoader::new(Self::data(), tokio::spawn))
            .extension(extensions::Logger)
            .finish()
    }
}

pub struct Data {
    users: Vec<User>,
    boards: Vec<Board>,
    columns: Vec<Column>,
}

pub struct QueryRoot;
// re-export
pub use async_graphql::EmptyMutation as MutationRoot;
pub use async_graphql::EmptySubscription as SubscriptionRoot;

#[Object]
impl QueryRoot {
    async fn get_user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::UserIdValidator"))] id: Id<User>,
    ) -> GqlResult<Option<User>> {
        let result = load_one(ctx, id).await?;
        Ok(result)
    }

    async fn get_board<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new(\"Board\", \"b\")"))] id: Id<
            Board,
        >,
    ) -> GqlResult<Option<Board>> {
        let result = load_one(ctx, id).await?;
        Ok(result)
    }

    async fn get_column<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new(\"Column\", \"o\")"))] id: Id<
            Column,
        >,
    ) -> GqlResult<Option<Column>> {
        let result = load_one(ctx, id).await?;
        Ok(result)
    }

    async fn get_card<'a>(&self, ctx: &Context<'a>, id: String) -> GqlResult<Option<Card>> {
        let result = ctx
            .data::<Data>()?
            .columns
            .iter()
            .map(|c| &c.cards)
            .flatten()
            .filter(|c| c.id == id)
            .next();
        Ok(result.cloned())
    }
}

// DataloaderするのにIDの型が個別のほうが嬉しい
#[derive(Debug, Clone)]
pub struct Id<T> {
    value: String,
    _phantom: PhantomData<T>,
}
impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self._phantom == other._phantom
    }
}
impl<T> Eq for Id<T> {}
impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self._phantom.hash(state);
    }
}

#[Scalar]
impl<T: Send + Sync> ScalarType for Id<T> {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(Self {
                value,
                _phantom: PhantomData,
            })
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        let v: &str = &self.value;
        Value::from(v)
    }
}

impl<T, U> From<U> for Id<T>
where
    U: Into<String>,
{
    fn from(value: U) -> Self {
        Self {
            value: value.into(),
            _phantom: PhantomData,
        }
    }
}

// SimpleObjectならフィールドがそのままgraphqlにでていく
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
    fn new(
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
        // TODO: remove clone
        let map = load_many(ctx, self.owned_board_ids.clone()).await?;
        let result = Vec::from_iter(map.into_values().into_iter());
        Ok(result)
    }
}

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
    fn new(
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
    async fn owner<'a>(&self, ctx: &Context<'a>) -> GqlResult<User> {
        println!("CALLED Resolver: Board.owner(): load_one");
        let result = load_one(ctx, self.owner_id.clone()).await?;
        result.ok_or(GqlError::new("user not found"))
    }

    async fn columns<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<Column>> {
        println!("CALLED Resolver: Board.columns(): load_many");
        // TODO: remove clone
        let map = load_many(ctx, self.column_ids.clone()).await?;
        let result = Vec::from_iter(map.into_values().into_iter());
        Ok(result)
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Column {
    id: Id<Column>,
    title: String,
    cards: Vec<Card>,
}

impl Column {
    fn new(id: impl Into<Id<Column>>, title: impl Into<String>, cards: Vec<Card>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            cards,
        }
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

mod validator {
    use super::*;
    use async_graphql::{CustomValidator, InputValueError};

    pub struct UserIdValidator;
    impl CustomValidator<Id<User>> for UserIdValidator {
        fn check(&self, value: &Id<User>) -> Result<(), async_graphql::InputValueError<Id<User>>> {
            value
                .value
                .starts_with("u")
                .then_some(())
                .ok_or(InputValueError::custom(
                    "UserのIDはuで始まらなければなりません",
                ))
        }
    }

    pub struct IdValidator {
        type_name: String,
        start: String,
    }
    impl IdValidator {
        pub fn new(type_name: impl Into<String>, start: impl Into<String>) -> Self {
            Self {
                type_name: type_name.into(),
                start: start.into(),
            }
        }
    }

    impl<T> CustomValidator<Id<T>> for IdValidator
    where
        T: Send + Sync,
    {
        fn check(&self, value: &Id<T>) -> Result<(), async_graphql::InputValueError<Id<T>>> {
            value
                .value
                .starts_with(&self.start)
                .then_some(())
                .ok_or(InputValueError::custom(format!(
                    "{}のIDは{}で始まらなければなりません",
                    self.type_name, self.start
                )))
        }
    }
}
