use tide::Request;
use serde_json::json;

pub mod users;
pub mod projects;

use crate::util::common::Tpl;

pub async fn index(_req: Request<()>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data =
        json!({"app_name": "tide-handlebars-graphql-mongodb", "author": "zzy"});

    index.render(&data).await
}
