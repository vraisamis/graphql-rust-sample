use anyhow::{anyhow, Result};
use async_trait::async_trait;
use query_resolver::{UserView, UsersQuery};

struct UsersQueryImpl;
impl UsersQueryImpl {
    fn to_view(u: &sample::User) -> UserView {
        UserView {
            id: u.id.value.to_string(),
        }
    }
}

#[async_trait]
impl UsersQuery for UsersQueryImpl {
    async fn find_by_id(&self, id: &str) -> Result<UserView> {
        let data = sample::data();
        let result = data
            .users
            .iter()
            .filter(|u| &u.id.value == id)
            .map(Self::to_view)
            .next()
            .ok_or(anyhow!("Not Found"))?;
        Ok(result)
    }
    async fn list_by_ids(&self, ids: &[String]) -> Result<Vec<UserView>> {
        let data = sample::data();
        let result = data
            .users
            .iter()
            .filter(|u| ids.contains(&u.id.value))
            .map(Self::to_view)
            .collect();
        Ok(result)
    }
}

mod sample {
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::sync::OnceLock;

    pub static DATA: OnceLock<Data> = OnceLock::new();
    pub fn data<'a>() -> &'a Data {
        DATA.get_or_init(Data::new)
    }

    // DataloaderするのにIDの型が個別のほうが嬉しい
    #[derive(Debug, Clone)]
    pub struct Id<T> {
        pub value: String,
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

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: Id<User>,
        pub name: String,
        pub email: String,
        // #[graphql(skip)]
        pub owned_board_ids: Vec<Id<Board>>,
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
    // TODO
    #[derive(Debug, Clone)]
    pub struct Board;
    // pub struct Column;

    pub struct Data {
        pub users: Vec<User>,
        // boards: Vec<Board>,
        // columns: Vec<Column>,
    }
    impl Data {
        pub fn new() -> Self {
            let users = vec![
                User::new("u0", "aaa", "aaa@example.com", vec!["b0", "b2"]),
                User::new("u1", "bbb", "bbb@example.com", vec!["b1"]),
                User::new("u2", "ccc", "ccc@example.com", Vec::<String>::new()),
            ];
            // let columns = vec![
            //     Column::new(
            //         "o0",
            //         "TODO",
            //         vec![
            //             Card::new("c0", "掃除", ""),
            //             Card::new("c1", "洗濯", ""),
            //             Card::new("c2", "食事", ""),
            //         ],
            //     ),
            //     Column::new("o1", "doing", vec![Card::new("c3", "ゴミ出し", "")]),
            //     Column::new(
            //         "o2",
            //         "DONE",
            //         vec![
            //             Card::new("c14", "買い物", ""),
            //             Card::new("c15", "洗い物", ""),
            //         ],
            //     ),
            //     Column::new(
            //         "o3",
            //         "wish",
            //         vec![
            //             Card::new("c4", "ランタン", ""),
            //             Card::new("c5", "本棚", "いろいろ"),
            //         ],
            //     ),
            //     Column::new(
            //         "o4",
            //         "bought",
            //         vec![
            //             Card::new("c13", "石鹸", ""),
            //             Card::new("c12", "常備薬", ""),
            //             Card::new("c11", "米", ""),
            //             Card::new("c9", "センサーライト", ""),
            //             Card::new("c10", "飲み物", ""),
            //         ],
            //     ),
            //     Column::new("o5", "pending", vec![]),
            //     Column::new(
            //         "o6",
            //         "challenge",
            //         vec![
            //             Card::new("c8", "ゴハッチュウ", ""),
            //             Card::new("c7", "ダソッキー", ""),
            //             Card::new("c6", "床下三兄弟", ""),
            //         ],
            //     ),
            //     Column::new("o7", "got", vec![]),
            // ];
            // let boards = vec![
            //     Board::new("b0", "yarukoto", "u0", vec!["o0", "o1", "o2"]),
            //     Board::new("b1", "wishlist", "u1", vec!["o3", "o4", "o5"]),
            //     Board::new("b2", "monster", "u0", vec!["o6", "o7"]),
            // ];

            Data {
                users,
                // boards,
                // columns,
            }
        }
    }
}
