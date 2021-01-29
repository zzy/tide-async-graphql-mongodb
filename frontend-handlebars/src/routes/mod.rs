use tide::{self, Server, Request};
use serde_json::json;

pub mod users;
pub mod projects;

use crate::{State, util::common::Tpl};
use crate::routes::{users::user_index, projects::project_index};

pub async fn push_res(mut app: Server<State>) -> Server<State> {
    app.at("/static").serve_dir("./static").unwrap();

    //environment variables defined in .env file
    app.at("/").get(index);
    app.at("users").get(user_index);
    app.at("projects").get(project_index);

    app
}

async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "tide-graphql-mongodb", "author": "zzy"});

    index.render(&data).await
}
