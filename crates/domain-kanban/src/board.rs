use crate::{column::ColumnId, user::UserId};
use domain_util::{Entity, Identifier, InvariantError, InvariantResult};
use invariant_sheild::{invariant_sheild, InvariantSheild};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct Board {
    id: BoardId,
    title: BoardTitle,
    owner: UserId,
    members: Vec<UserId>,
    column_ids: Vec<ColumnId>,
}

impl Entity for Board {
    fn entity_type() -> &'static str {
        "board"
    }
}

#[invariant_sheild(InvariantError)]
impl Board {
    pub fn new(
        id: BoardId,
        title: BoardTitle,
        owner: UserId,
        members: Vec<UserId>,
        column_ids: Vec<ColumnId>,
    ) -> InvariantResult<Self> {
        let result = Self {
            id,
            title,
            owner,
            members,
            column_ids,
        };
        result.satisfy_sheilds()
    }

    const MAX_COLUMN_COUNT: usize = 10;
    #[sheild]
    fn column_count_lower_than_max(&self) -> InvariantResult<()> {
        if self.column_ids.len() > Self::MAX_COLUMN_COUNT {
            return Err(InvariantError::ViolationError(
                "カラムは10個までです".to_owned(),
            ));
        }
        Ok(())
    }
}

pub type BoardId = Identifier<Board>;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BoardTitle(String);

#[invariant_sheild(InvariantError)]
impl BoardTitle {
    pub fn new(title: String) -> InvariantResult<Self> {
        let result = Self(title);
        result.satisfy_sheilds()
    }

    const MAX_LENGTH: usize = 100;
    #[sheild]
    fn title_lower_than_max(&self) -> InvariantResult<()> {
        if self.0.len() > Self::MAX_LENGTH {
            return Err(InvariantError::ViolationError(
                "タイトルは100文字以内にしてください".to_owned(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new() -> InvariantResult<()> {
        let id = BoardId::gen();
        let title = BoardTitle::new("title".to_owned())?;
        let owner = UserId::gen();
        let members = vec![UserId::gen()];
        let column_ids: Vec<_> = (0..10).into_iter().map(|_| ColumnId::gen()).collect();
        let board = Board::new(
            id.clone(),
            title.clone(),
            owner.clone(),
            members.clone(),
            column_ids.clone(),
        )?;
        assert_eq!(board.id, id);
        assert_eq!(board.title, title);
        assert_eq!(board.owner, owner);
        assert_eq!(board.members, members);
        assert_eq!(board.column_ids, column_ids);
        Ok(())
    }
    #[test]
    fn test_board_new_with_error() -> InvariantResult<()> {
        let id = BoardId::gen();
        let title = BoardTitle::new("title".to_owned())?;
        let owner = UserId::gen();
        let members = vec![UserId::gen()];
        let column_ids: Vec<_> = (0..11).into_iter().map(|_| ColumnId::gen()).collect();
        let board_result = Board::new(
            id.clone(),
            title.clone(),
            owner.clone(),
            members.clone(),
            column_ids.clone(),
        );
        assert!(board_result.is_err());
        let board_error = board_result.unwrap_err();
        assert_eq!(
            board_error,
            InvariantError::ViolationError("カラムは10個までです".to_owned())
        );

        Ok(())
    }

    #[test]
    fn test_board_title_new() -> InvariantResult<()> {
        let title = BoardTitle::new("title".to_owned())?;
        assert_eq!(title.0, "title".to_owned());
        Ok(())
    }

    #[test]
    fn test_board_title_new_with_error() -> InvariantResult<()> {
        let title_result = BoardTitle::new((0..101).into_iter().map(|_| 'a').collect());
        assert!(title_result.is_err());
        let title_error = title_result.unwrap_err();
        assert_eq!(
            title_error,
            InvariantError::ViolationError("タイトルは100文字以内にしてください".to_owned())
        );
        Ok(())
    }
}
