#![allow(unused)]
use crate::{column::ColumnId, user::UserId, Identifier};

pub struct Board {
    id: BoardId,
    title: BoardTitle,
    owner: UserId,
    // members: Vec<UserId>,
    // TODO
    column_ids: Vec<ColumnId>,
}

pub type BoardId = Identifier<Board>;

pub struct BoardTitle(String);

impl BoardTitle {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}
