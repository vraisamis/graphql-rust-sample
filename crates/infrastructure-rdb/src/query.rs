mod board;
mod column;
mod user;

shaku::module! {
    pub Module {
        components = [],
        providers = [user::UsersQueryImpl]
    }
}
