use async_graphql::Context;
use bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;

use crate::users::{self, models::User};
use crate::projects::{self, models::Project};

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    // Get all Users,
    async fn all_users(&self, ctx: &Context<'_>) -> Vec<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::all_users(db).await
    }

    // Get all Projects
    async fn all_projects(&self, ctx: &Context<'_>) -> Vec<Project> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        projects::services::all_projects(db).await
    }

    // Get all Projects of one User
    async fn all_projects_by_user(&self, ctx: &Context<'_>, user_id: ObjectId) -> Vec<Project> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        projects::services::all_projects_by_user(db, user_id).await
    }
}
