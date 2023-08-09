use async_graphql::{
    extensions, ComplexObject, Context, EmptyMutation, EmptySubscription, Error as GqlError,
    Object, Pos, Result as GqlResult, Schema, SimpleObject,
};

use super::SchemaWithStaticData;

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
            .extension(extensions::Logger)
            .finish()
    }
}
// pub fn schema() -> KanbanSchema {
//     Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
// }

pub struct Data {
    users: Vec<User>,
    boards: Vec<Board>,
    columns: Vec<Column>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_user<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::UserIdValidator"))] id: String,
    ) -> GqlResult<Option<User>> {
        let result = ctx
            .data::<Data>()?
            .users
            .iter()
            .filter(|e| e.id == id)
            .next();
        Ok(result.cloned())
        // NOTE: 以下でエラーを返せるが、クエリ自体は成功する。
        // ctx.add_error(ServerError::new("foobar", Some((1, 1).into())));
        // Ok(u1)
        // NOTE: Errorを返すと（全ての）クエリが失敗になる
        // Err(GqlError::new("hogehoge"))
    }

    async fn get_board<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new(\"Board\", \"b\")"))] id: String,
    ) -> GqlResult<Option<Board>> {
        let result = ctx
            .data::<Data>()?
            .boards
            .iter()
            .filter(|e| e.id == id)
            .next();
        Ok(result.cloned())
    }

    async fn get_column<'a>(
        &self,
        ctx: &Context<'a>,
        #[graphql(validator(custom = "validator::IdValidator::new(\"Column\", \"o\")"))] id: String,
    ) -> GqlResult<Option<Column>> {
        let result = ctx
            .data::<Data>()?
            .columns
            .iter()
            .filter(|c| c.id == id)
            .next();
        Ok(result.cloned())
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

// SimpleObjectならフィールドがそのままgraphqlにでていく
#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: String,
    name: String,
    email: String,
    #[graphql(skip)]
    owned_board_ids: Vec<String>,
}

impl User {
    fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        email: impl Into<String>,
        owned_board_ids: Vec<impl Into<String>>,
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
        println!("CALLED Resolver: User.owned_boards()");
        let ids = &self.owned_board_ids;
        let result = ctx
            .data::<Data>()?
            .boards
            .iter()
            .filter_map(|b| {
                if ids.contains(&b.id) {
                    Some(b.clone())
                } else {
                    None
                }
            })
            .collect();
        Ok(result)
    }
}

#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct Board {
    id: String,
    title: String,
    #[graphql(skip)]
    owner_id: String,
    #[graphql(skip)]
    column_ids: Vec<String>,
}

impl Board {
    fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        owner_id: impl Into<String>,
        column_ids: Vec<impl Into<String>>,
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
        println!("CALLED Resolver: Board.owner()");
        let result = ctx
            .data::<Data>()?
            .users
            .iter()
            .filter(|u| u.id == self.owner_id)
            .map(Clone::clone)
            .next();
        result.ok_or(GqlError::new("user not found"))
    }

    async fn columns<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<Column>> {
        println!("CALLED Resolver: Board.columns()");
        let result = ctx
            .data::<Data>()?
            .columns
            .iter()
            .filter(|c| self.column_ids.contains(&c.id))
            .map(Clone::clone)
            .collect();
        Ok(result)
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Column {
    id: String,
    title: String,
    cards: Vec<Card>,
}

impl Column {
    fn new(id: impl Into<String>, title: impl Into<String>, cards: Vec<Card>) -> Self {
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
    use async_graphql::{CustomValidator, InputValueError};

    pub struct UserIdValidator;
    impl CustomValidator<String> for UserIdValidator {
        fn check(&self, value: &String) -> Result<(), async_graphql::InputValueError<String>> {
            value
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

    impl CustomValidator<String> for IdValidator {
        fn check(&self, value: &String) -> Result<(), async_graphql::InputValueError<String>> {
            value
                .starts_with(&self.start)
                .then_some(())
                .ok_or(InputValueError::custom(format!(
                    "{}のIDは{}で始まらなければなりません",
                    self.type_name, self.start
                )))
        }
    }
}
