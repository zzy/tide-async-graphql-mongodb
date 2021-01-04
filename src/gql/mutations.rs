use async_graphql::Context;

use crate::users::{
    self,
    models::{User, NewUser},
};
use crate::dbs::mongo::DataSource;

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    /// Add new user
    async fn add_user(&self, ctx: &Context<'_>, new_user: NewUser) -> User {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::add_user(db, new_user).await
    }
}
