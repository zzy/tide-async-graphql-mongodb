use serde::{Serialize, Deserialize};
use mongodb::bson::oid::ObjectId;

use crate::dbs::mongo::DataSource;
use crate::projects::models::Project;
use crate::projects::services::all_projects_by_user;

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub email: String,
    pub username: String,
}

#[async_graphql::Object]
impl User {
    pub async fn id(&self) -> ObjectId {
        self._id.clone()
    }

    pub async fn email(&self) -> &str {
        self.email.as_str()
    }

    pub async fn username(&self) -> &str {
        self.username.as_str()
    }

    pub async fn projects(&self, ctx: &async_graphql::Context<'_>) -> Vec<Project> {
        let db = ctx.data_unchecked::<DataSource>().db_budshome.clone();
        all_projects_by_user(db, self._id.clone()).await
    }
}
#[derive(Serialize, Deserialize, async_graphql::InputObject)]
pub struct NewUser {
    pub email: String,
    pub username: String,
}
