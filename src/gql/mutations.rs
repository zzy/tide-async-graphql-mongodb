use async_graphql::Context;

use crate::users::models::{User, NewUser};
use crate::users::services::Users;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    /// Add new user
    async fn add_user(&self, ctx: &Context<'_>, user: NewUser) -> User {
        let mut users = ctx.data_unchecked::<Users>().0.write().await;
        let mut user = user.into_internal();

        user.id = Some((users.len() + 1) as u16);
        users.push(user.clone());

        user
    }
}
