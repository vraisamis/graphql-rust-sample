use std::str::FromStr;
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

#[derive(Debug, Clone)]
pub struct Board {
    pub id: BoardId,
    pub title: String,
    pub owner_id: UserId,
    pub column_ids: Vec<ColumnId>,
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

#[derive(Debug, Clone)]
pub struct Column {
    pub id: ColumnId,
    pub title: String,
    pub cards: Vec<Card>,
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

#[derive(Debug, Clone)]
pub struct Card {
    pub id: String,
    pub title: String,
    pub description: String,
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
    pub boards: Vec<Board>,
    pub columns: Vec<Column>,
}
impl Data {
    pub fn new() -> Self {
        let user_ids = vec![
            UserId::from_str("user-01HBCCGK3MG5HA7GJG25BGV6PJ").unwrap(),
            UserId::from_str("user-01HBCCGK3MH7XKWBDHXSWCAPWA").unwrap(),
            UserId::from_str("user-01HBCCGK3MS53D8NM6EYZ0KZEH").unwrap(),
        ];
        let board_ids = vec![
            BoardId::from_str("board-01HBCCGK3MH83RJ4Y8AVECQ5W9").unwrap(),
            BoardId::from_str("board-01HBCCGK3M3039H2QQEYD94TMS").unwrap(),
            BoardId::from_str("board-01HBCCGK3M18C851FA0067MRPF").unwrap(),
        ];
        let column_ids = vec![
            ColumnId::from_str("column-01HBCCGK3MAWDZKS74M1DEJQ54").unwrap(),
            ColumnId::from_str("column-01HBCCGK3MR41MEZWGJERC5PHD").unwrap(),
            ColumnId::from_str("column-01HBCCGK3MDQRSF7X7EGKBMAY8").unwrap(),
            ColumnId::from_str("column-01HBCCGK3MDD8M1T47N4MDB6AA").unwrap(),
            ColumnId::from_str("column-01HBCCGK3MMEFTBS3SJ73CK96K").unwrap(),
            ColumnId::from_str("column-01HBCCGK3M9BMDD7Z16JQNX3QC").unwrap(),
            ColumnId::from_str("column-01HBCCGK3MTAFEVEFAQMFE2W43").unwrap(),
            ColumnId::from_str("column-01HBCCGK3M3SA44D9SCJVR5D8X").unwrap(),
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
        println!("######## users ########");
        println!("{:?}", user_ids);
        println!("######## boards ########");
        println!("{:?}", board_ids);
        println!("######## columns ########");
        println!("{:?}", column_ids);

        Data {
            users,
            boards,
            columns,
        }
    }
}
