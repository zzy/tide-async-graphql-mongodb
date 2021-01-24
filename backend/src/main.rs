mod util;
mod dbs;
mod gql;

mod users;
mod projects;

use crate::util::constant::CFG;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let app_state = State {};
    let mut app = tide::with_state(app_state);

    //environment variables defined in .env file
    // app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(async_graphql_tide::endpoint(schema));
    // If you don't want to use crate async-graphql-tide
    app.at(CFG.get("GRAPHQL_PATH").unwrap()).post(gql::graphql);
    app.at(CFG.get("GRAPHIQL_PATH").unwrap()).get(gql::graphiql);

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
pub struct State {}

// // If you don't want to use crate async-graphql-tide
// #[derive(Clone)]
// pub struct State {
//     pub schema: async_graphql::Schema<
//         gql::queries::QueryRoot,
//         gql::mutations::MutationRoot,
//         async_graphql::EmptySubscription,
//     >,
// }
