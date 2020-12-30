mod constant;
mod schema;

use crate::constant::ENV;
use crate::schema::{State, init_schema, handle_graphql, handle_playground};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::start();

    let mut app = tide::with_state(State(init_schema().await));

    app.at("/")
        .get(tide::Redirect::new(ENV.get("GRAPHIQL_PATH").unwrap()));
    // app.at("/graphql").post(async_graphql_tide::endpoint(schema));
    app.at(ENV.get("GRAPHQL_PATH").unwrap())
        .post(handle_graphql);
    app.at(ENV.get("GRAPHIQL_PATH").unwrap())
        .get(handle_playground);

    app.listen(format!(
        "{}:{}",
        ENV.get("GRAPHQL_ADDRESS").unwrap(),
        ENV.get("GRAPHQL_PORT").unwrap()
    ))
    .await?;

    Ok(())
}
