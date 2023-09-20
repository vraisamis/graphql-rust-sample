use std::collections::HashMap;

use async_graphql::{
    dataloader::Loader, ComplexObject, Context, Result as GqlResult, SimpleObject,
};
use async_trait::async_trait;
use domain_util::{Entity, Identifier};
use query_resolver::{UserView, UsersQuery};

use crate::{scalar::Id, Injections};

// TODO
struct Board;

#[derive(Debug, Clone, SimpleObject)]
// #[graphql(complex)]
pub struct User {
    id: Id<User>,
    name: String,
    email: String,
    // #[graphql(skip)]
    // owned_board_ids: Vec<Id<Board>>,
}

impl User {
    pub fn new(
        id: impl Into<Id<User>>,
        name: impl Into<String>,
        email: impl Into<String>,
        // owned_board_ids: Vec<impl Into<Id<Board>>>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            email: email.into(),
            // owned_board_ids: owned_board_ids.into_iter().map(Into::into).collect(),
        }
    }
}

// TODO
// #[ComplexObject]
// impl User {
//     async fn owned_boards<'a>(&self, ctx: &Context<'a>) -> GqlResult<Vec<Board>> {
//         println!("CALLED Resolver: User.owned_boards(): load_many");
//         // TODO: remove clone
//         let map = load_many(ctx, self.owned_board_ids.clone()).await?;
//         let result = Vec::from_iter(map.into_values().into_iter());
//         Ok(result)
//     }
// }

impl From<UserView> for User {
    fn from(value: UserView) -> Self {
        Self::new(value.id, "", "")
    }
}

impl<T, U: Entity> Into<Identifier<U>> for Id<T> {
    fn into(self) -> Identifier<U> {
        self.value().parse().unwrap()
    }
}

#[async_trait]
impl Loader<Id<User>> for Injections {
    type Value = User;
    // TODO
    type Error = String;

    async fn load(&self, keys: &[Id<User>]) -> Result<HashMap<Id<User>, Self::Value>, Self::Error> {
        println!(
            "[Dataloader] CALLED DataLoader of Id<User> -> User: {:?}",
            keys
        );
        // TODO
        let ids: Vec<_> = keys.iter().map(|i| i.clone().into()).collect();
        let result = self
            .user_query
            .list_by_ids(&ids)
            .await
            .expect("query error");
        Ok(result
            .into_iter()
            .map(|(k, v)| (k.to_string().into(), v.into()))
            .collect())
        // let result: HashMap<Id<User>, User> = self
        //     .users
        //     .iter()
        //     .filter_map(|u| {
        //         if keys.contains(&u.id) {
        //             Some((u.id.clone(), u.clone()))
        //         } else {
        //             None
        //         }
        //     })
        //     .collect();
        // Ok(result)
    }
}
