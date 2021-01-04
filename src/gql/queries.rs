use async_graphql::Context;

use crate::dbs::mongo::DataSource;
use crate::users::{self, models::User};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    /// Get all Users,
    async fn all_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::all_users(db).await
    }
}
