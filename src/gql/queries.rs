use async_graphql::Context;

use crate::dbs::mongo;
use crate::users::models::User;
use crate::users::services::Users;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    /// Get all Users,
    async fn all_users1(&self) -> Vec<User> {
        let user1 = User { id: Some(12), first_name: "Alice".to_string() };
        let user2 = User { id: Some(22), first_name: "Jack".to_string() };
        let user3 = User { id: Some(32), first_name: "Tom".to_string() };

        vec![user1, user2, user3]
    }

    /// Get all Users
    async fn all_users2(&self, ctx: &Context<'_>) -> Vec<User> {
        let test = &ctx.data_unchecked::<mongo::DataSource>().db_budshome;
        println!("{:?}", test.name());

        for collection_name in test.list_collection_names(None).await {
            println!("{:?}", collection_name);
        }

        let users = ctx.data_unchecked::<Users>().0.read().await;

        users.iter().cloned().collect()
    }
}
