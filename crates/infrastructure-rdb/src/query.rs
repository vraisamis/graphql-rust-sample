mod board;
mod card;
mod column;
mod user;

shaku::module! {
    pub Module {
        components = [super::PgPoolImpl],
        providers = [
            board::BoardQueryImpl,
            card::CardsQueryImpl,
            column::ColumnsQueryImpl,
            user::UsersQueryImpl,
        ]
    }
}
