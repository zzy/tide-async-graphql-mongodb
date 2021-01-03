mod constant;
mod schema;
mod dbs;

use crate::constant::ENV;
use crate::schema::{State, build_schema, graphql, graphiql};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::with_state(State(build_schema().await));

    //environment variables defined in .env file
    app.at("/").get(tide::Redirect::new(ENV.get("GRAPHIQL_PATH").unwrap()));
    // app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(async_graphql_tide::endpoint(schema));
    app.at(ENV.get("GRAPHQL_PATH").unwrap()).post(graphql);
    app.at(ENV.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

    app.listen(format!(
        "{}:{}",
        ENV.get("GRAPHQL_ADDRESS").unwrap(),
        ENV.get("GRAPHQL_PORT").unwrap()
    ))
    .await?;

    Ok(())
}
