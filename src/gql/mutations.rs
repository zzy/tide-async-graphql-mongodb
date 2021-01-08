use async_graphql::Context;

use crate::dbs::mongo::DataSource;
use crate::constant::GqlResult;
use crate::users::{
    self,
    models::{User, NewUser},
};
use crate::projects::{
    self,
    models::{Project, NewProject},
};

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    // Add new user
    async fn user_register(&self, ctx: &Context<'_>, new_user: NewUser) -> GqlResult<User> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        users::services::user_register(db, new_user).await
    }

    // Add new project
    async fn add_project(&self, ctx: &Context<'_>, new_project: NewProject) -> Project {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        projects::services::add_project(db, new_project).await
    }
}
