mod util;
mod routes;

use crate::util::constant::CFG;
use crate::routes::{index, users::user_index, projects::project_index};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let mut app = tide::new();
    // let mut app = tide::with_state(State(gql::build_schema().await));

    //environment variables defined in .env file
    app.at("/").get(index);
    app.at("users").get(user_index);
    app.at("projects").get(project_index);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDRESS").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

// //  Tide application scope state.
// #[derive(Clone)]
// pub struct State(
//     pub  async_graphql::Schema<
//         gql::queries::QueryRoot,
//         gql::mutations::MutationRoot,
//         async_graphql::EmptySubscription,
//     >,
// );
