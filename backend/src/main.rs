mod util;
mod dbs;
mod gql;

mod users;
mod projects;

use crate::util::constant::CFG;
use crate::gql::{build_schema, graphql, graphiql};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    //environment variables defined in .env file
    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(graphiql);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}
