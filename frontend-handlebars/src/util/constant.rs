use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // CFG variables defined in .env file
    pub static ref CFG: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "ADDRESS",
            dotenv::var("ADDRESS").expect("Expected ADDRESS to be set in env!"),
        );
        map.insert(
            "PORT",
            dotenv::var("PORT").expect("Expected PORT to be set in env!"),
        );

        map.insert(
            "GRAPHQL_PORT",
            dotenv::var("GRAPHQL_PORT").expect("Expected GRAPHQL_PORT to be set in env!"),
        );
        map.insert(
            "GRAPHQL_PATH",
            dotenv::var("GRAPHQL_PATH").expect("Expected GRAPHQL_PATH to be set in env!"),
        );
        map.insert(
            "GRAPHIQL_PATH",
            dotenv::var("GRAPHIQL_PATH").expect("Expected GRAPHIQL_PATH to be set in env!"),
        );

        map
    };
}
