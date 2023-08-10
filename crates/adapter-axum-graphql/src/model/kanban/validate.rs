use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object,
    Result as GqlResult, Schema, SimpleObject,
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
            User::new("u0", "aaa", "aaa@example.com"),
            User::new("u1", "bbb", "bbb@example.com"),
            User::new("u2", "ccc", "ccc@example.com"),
        ];
        let boards = vec![
            Board::new(
                "b0",
                "yarukoto",
                users[0].clone(),
                vec![
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
                ],
            ),
            Board::new(
                "b1",
                "wishlist",
                users[1].clone(),
                vec![
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
                ],
            ),
            Board::new(
                "b2",
                "monster",
                users[0].clone(),
                vec![
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
                ],
            ),
        ];

        Data { users, boards }
    }
}
// pub fn schema() -> KanbanSchema {
//     Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish()
// }

pub struct Data {
    users: Vec<User>,
    boards: Vec<Board>,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    // queryの関数になるっぽい
    // ctxから実装を取り出して実行できそう
    async fn test_users(&self) -> Vec<User> {
        // let x = ctx.data();
        let u1 = User {
            id: "foo".into(),
            name: "bar".into(),
            email: "hoge".into(),
        };
        vec![u1]
    }

    async fn test_user<'a>(&self, _ctx: &Context<'a>, _id: String) -> User {
        let u1 = User {
            id: "foo".into(),
            name: "bar".into(),
            email: "hoge".into(),
        };
        u1
    }

    // ここから
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
            .boards
            .iter()
            .map(|b| &b.columns)
            .flatten()
            .filter(|c| c.id == id)
            .next();
        Ok(result.cloned())
    }

    async fn get_card<'a>(&self, ctx: &Context<'a>, id: String) -> GqlResult<Option<Card>> {
        let result = ctx
            .data::<Data>()?
            .boards
            .iter()
            .map(|b| &b.columns)
            .flatten()
            .map(|c| &c.cards)
            .flatten()
            .filter(|c| c.id == id)
            .next();
        Ok(result.cloned())
    }
}

#[derive(Debug, Clone)]
pub struct User2 {
    id: String, // queryを見る限り、ここは好きな型でよさそう
    name: String,
    email: String,
}

#[Object]
impl User2 {
    // ここにfieldがくる
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}

// SimpleObjectならフィールドがそのままgraphqlにでていく
#[derive(Debug, Clone, SimpleObject)]
#[graphql(complex)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

impl User {
    fn new(id: impl Into<String>, name: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
        }
    }
}

#[ComplexObject]
impl User {
    async fn foo(&self) -> usize {
        self.name.len()
    }
}

#[derive(Debug, Clone, SimpleObject)]
pub struct Board {
    id: String,
    title: String,
    owner: User,
    columns: Vec<Column>,
}

impl Board {
    fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        owner: User,
        columns: Vec<Column>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            owner,
            columns,
        }
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
