use tide::Request;

mod util;
mod dbs;
mod gql;

mod users;
mod projects;

use crate::util::{constant::CFG, common::Tpl};

//  Tide application scope state.
#[derive(Clone)]
pub struct State(
    pub  async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
);

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::with_state(State(gql::build_schema().await));

    //environment variables defined in .env file
    app.at("/").get(index);
    app.at("users").get(user_index);
    app.at("projects").get(user_index);

    // app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(async_graphql_tide::endpoint(schema));
    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(gql::graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(gql::graphiql);

    app.listen(format!("{}:{}", CFG.get("ADDRESS").unwrap(), CFG.get("PORT").unwrap())).await?;

    Ok(())
}

pub async fn index(_req: Request<State>) -> tide::Result {
    let index: Tpl = Tpl::new("index").await;

    // make data and render it
    let data = json!({"app_name": "tide-handlebars-graphql-mongodb", "author": "zzy"});

    index.render(&data).await
}

pub async fn user_index(_req: Request<State>) -> tide::Result {
    let mut user_index: Tpl = Tpl::new("user/index").await;

    // register some custom helpers
    user_index.reg.register_helper("format", Box::new(format_helper));
    user_index.reg.register_helper("ranking_label", Box::new(rank_helper));

    // make data and render it
    let data = make_data();

    // index.render(&data).await
    user_index.render(&data).await
}

use serde_json::{
    json,
    value::{Map, Value as Json},
};

use handlebars::{to_json, Context, Handlebars, Helper, JsonRender, Output, RenderContext, RenderError};

// define a custom helper
fn format_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    // get parameter from helper or throw an error
    let param = h.param(0).ok_or(RenderError::new("Param 0 is required for format helper."))?;
    let rendered = format!("{} pts", param.value().render());
    out.write(rendered.as_ref())?;
    Ok(())
}

// another custom helper
fn rank_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let rank = h
        .param(0)
        .and_then(|v| v.value().as_u64())
        .ok_or(RenderError::new("Param 0 with u64 type is required for rank helper."))?
        as usize;
    let total = h
        .param(1)
        .as_ref()
        .and_then(|v| v.value().as_array())
        .map(|arr| arr.len())
        .ok_or(RenderError::new("Param 1 with array type is required for rank helper"))?;
    if rank == 0 {
        out.write("champion")?;
    } else if rank >= total - 2 {
        out.write("relegation")?;
    } else if rank <= 2 {
        out.write("acl")?;
    }
    Ok(())
}

static TYPES: &'static str = "serde_json";

use serde::Serialize;

// define some data
#[derive(Serialize)]
pub struct Team {
    name: String,
    pts: u16,
}

// produce some data
pub fn make_data() -> Map<String, Json> {
    let mut data = Map::new();

    data.insert("year".to_string(), to_json("2015"));

    let teams = vec![
        Team { name: "Jiangsu Suning".to_string(), pts: 43u16 },
        Team { name: "Shanghai SIPG".to_string(), pts: 39u16 },
        Team { name: "Hebei CFFC".to_string(), pts: 27u16 },
        Team { name: "Guangzhou Evergrand".to_string(), pts: 22u16 },
        Team { name: "Shandong Luneng".to_string(), pts: 12u16 },
        Team { name: "Beijing Guoan".to_string(), pts: 7u16 },
        Team { name: "Hangzhou Greentown".to_string(), pts: 7u16 },
        Team { name: "Shanghai Shenhua".to_string(), pts: 4u16 },
    ];

    data.insert("teams".to_string(), to_json(&teams));
    data.insert("engine".to_string(), to_json(TYPES));
    data
}
