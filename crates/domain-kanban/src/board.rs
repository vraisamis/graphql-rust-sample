#![allow(unused)]
use crate::{column::ColumnId, user::UserId};
use domain_util::{Entity, Identifier};

pub struct Board {
    id: BoardId,
    title: BoardTitle,
    owner: UserId,
    members: Vec<UserId>,
    // TODO
    column_ids: Vec<ColumnId>,
}

impl Entity for Board {
    fn entity_type() -> &'static str {
        "board"
    }
}

pub type BoardId = Identifier<Board>;

pub struct BoardTitle(String);

impl BoardTitle {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}
