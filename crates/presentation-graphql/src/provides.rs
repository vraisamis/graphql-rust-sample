use async_graphql::Context;
use query_resolver::UsersQuery;
use shaku::HasProvider;

pub trait QueryProvider
where
    Self: HasProvider<dyn UsersQuery>,
{
}
impl<T> QueryProvider for T where T: HasProvider<dyn UsersQuery> {}

pub struct Modules {
    pub(crate) m: Box<dyn QueryProvider + Send + Sync>,
}

impl Modules {
    pub fn new(m: Box<dyn QueryProvider + Send + Sync>) -> Self {
        Self { m }
    }
}

// impl<T> HasProvider<T> for Modules {
//     fn provide(&self) -> Result<Box<I>, Box<dyn std::error::Error>> {
//         self.m.
//     }
// }
