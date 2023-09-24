use std::collections::HashMap;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use domain_kanban::user::UserId;
use query_resolver::{UserView, UsersQuery};
use shaku::Provider;

shaku::module! {
    pub QueryModule {
        components = [],
        providers = [UsersQueryImpl]
    }
}

#[derive(Debug, Clone, Provider)]
#[shaku(interface = UsersQuery)]
pub struct UsersQueryImpl;
impl UsersQueryImpl {
    fn to_view(u: &sample::User) -> UserView {
        UserView {
            id: u.id.to_string(),
        }
    }
}

#[async_trait]
impl UsersQuery for UsersQueryImpl {
    async fn find_by_id(&self, id: &UserId) -> Result<UserView> {
        let data = sample::data();
        let result = data
            .users
            .iter()
            .filter(|u| &u.id == id)
            .map(Self::to_view)
            .next()
            .ok_or(anyhow!("Not Found"))?;
        Ok(result)
    }
    async fn list_by_ids(&self, ids: &[UserId]) -> Result<HashMap<UserId, UserView>> {
        let data = sample::data();
        let result = data
            .users
            .iter()
            .filter_map(|u| {
                ids.contains(&u.id)
                    .then(|| (u.id.clone(), Self::to_view(u)))
            })
            .collect();
        Ok(result)
    }
    async fn all(&self) -> Result<Vec<UserView>> {
        let data = sample::data();
        let result = data.users.iter().map(Self::to_view).collect();
        Ok(result)
    }
}

mod sample {
    use std::hash::Hash;
    use std::marker::PhantomData;
    use std::sync::OnceLock;

    use domain_kanban::board::BoardId;
    use domain_kanban::user::UserId;

    pub static DATA: OnceLock<Data> = OnceLock::new();
    pub fn data<'a>() -> &'a Data {
        DATA.get_or_init(Data::new)
    }

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: UserId,
        pub name: String,
        pub email: String,
        // #[graphql(skip)]
        pub owned_board_ids: Vec<BoardId>,
    }
    impl User {
        fn new(
            id: impl Into<UserId>,
            name: impl Into<String>,
            email: impl Into<String>,
            owned_board_ids: Vec<impl Into<BoardId>>,
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
            let board_ids = vec![BoardId::gen(), BoardId::gen(), BoardId::gen()];
            let users = vec![
                User::new(
                    UserId::gen(),
                    "aaa",
                    "aaa@example.com",
                    vec![board_ids[0].clone(), board_ids[2].clone()],
                ),
                User::new(
                    UserId::gen(),
                    "bbb",
                    "bbb@example.com",
                    vec![board_ids[1].clone()],
                ),
                User::new(
                    UserId::gen(),
                    "ccc",
                    "ccc@example.com",
                    Vec::<BoardId>::new(),
                ),
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
