mod user;

shaku::module! {
    pub Module {
        components = [super::ClientImpl],
        providers = [
            user::UserRepositoryImpl,
        ]
    }
}
