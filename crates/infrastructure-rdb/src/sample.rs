use std::sync::OnceLock;

use domain_kanban::board::BoardId;
use domain_kanban::column::ColumnId;
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

// #[derive(Debug, Clone)]
pub struct Board {
    id: BoardId,
    title: String,
    owner_id: UserId,
    column_ids: Vec<ColumnId>,
}

impl Board {
    fn new(
        id: impl Into<BoardId>,
        title: impl Into<String>,
        owner_id: impl Into<UserId>,
        column_ids: Vec<impl Into<ColumnId>>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            owner_id: owner_id.into(),
            column_ids: column_ids.into_iter().map(Into::into).collect(),
        }
    }
}

pub struct Column {
    id: ColumnId,
    title: String,
    cards: Vec<Card>,
}

impl Column {
    fn new(id: impl Into<ColumnId>, title: impl Into<String>, cards: Vec<Card>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            cards,
        }
    }
}

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

pub struct Data {
    pub users: Vec<User>,
    boards: Vec<Board>,
    columns: Vec<Column>,
}
impl Data {
    pub fn new() -> Self {
        let user_ids = vec![UserId::gen(), UserId::gen(), UserId::gen()];
        let board_ids = vec![BoardId::gen(), BoardId::gen(), BoardId::gen()];
        let column_ids = vec![
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
            ColumnId::gen(),
        ];
        let users = vec![
            User::new(
                user_ids[0].clone(),
                "aaa",
                "aaa@example.com",
                vec![board_ids[0].clone(), board_ids[2].clone()],
            ),
            User::new(
                user_ids[1].clone(),
                "bbb",
                "bbb@example.com",
                vec![board_ids[1].clone()],
            ),
            User::new(
                user_ids[2].clone(),
                "ccc",
                "ccc@example.com",
                Vec::<BoardId>::new(),
            ),
        ];
        let columns = vec![
            Column::new(
                column_ids[0].clone(),
                "TODO",
                vec![
                    Card::new("c0", "掃除", ""),
                    Card::new("c1", "洗濯", ""),
                    Card::new("c2", "食事", ""),
                ],
            ),
            Column::new(
                column_ids[1].clone(),
                "doing",
                vec![Card::new("c3", "ゴミ出し", "")],
            ),
            Column::new(
                column_ids[2].clone(),
                "DONE",
                vec![
                    Card::new("c14", "買い物", ""),
                    Card::new("c15", "洗い物", ""),
                ],
            ),
            Column::new(
                column_ids[3].clone(),
                "wish",
                vec![
                    Card::new("c4", "ランタン", ""),
                    Card::new("c5", "本棚", "いろいろ"),
                ],
            ),
            Column::new(
                column_ids[4].clone(),
                "bought",
                vec![
                    Card::new("c13", "石鹸", ""),
                    Card::new("c12", "常備薬", ""),
                    Card::new("c11", "米", ""),
                    Card::new("c9", "センサーライト", ""),
                    Card::new("c10", "飲み物", ""),
                ],
            ),
            Column::new(column_ids[5].clone(), "pending", vec![]),
            Column::new(
                column_ids[6].clone(),
                "challenge",
                vec![
                    Card::new("c8", "ゴハッチュウ", ""),
                    Card::new("c7", "ダソッキー", ""),
                    Card::new("c6", "床下三兄弟", ""),
                ],
            ),
            Column::new(column_ids[7].clone(), "got", vec![]),
        ];
        let boards = vec![
            Board::new(
                board_ids[0].clone(),
                "yarukoto",
                user_ids[0].clone(),
                vec![
                    column_ids[0].clone(),
                    column_ids[1].clone(),
                    column_ids[2].clone(),
                ],
            ),
            Board::new(
                board_ids[1].clone(),
                "wishlist",
                user_ids[1].clone(),
                vec![
                    column_ids[3].clone(),
                    column_ids[4].clone(),
                    column_ids[5].clone(),
                ],
            ),
            Board::new(
                board_ids[2].clone(),
                "monster",
                user_ids[0].clone(),
                vec![column_ids[6].clone(), column_ids[7].clone()],
            ),
        ];

        Data {
            users,
            boards,
            columns,
        }
    }
}
