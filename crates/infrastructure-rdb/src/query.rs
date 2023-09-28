mod board;
mod card;
mod column;
mod user;

shaku::module! {
    pub Module {
        components = [],
        providers = [
            board::BoardQueryImpl,
            card::CardsQueryImpl,
            column::ColumnsQueryImpl,
            user::UsersQueryImpl,
        ]
    }
}
