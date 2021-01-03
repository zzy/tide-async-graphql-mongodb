use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    // environment variables defined in .env file
    pub static ref ENV: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert(
            "WEB_ADDRESS",
            dotenv::var("WEB_ADDRESS").expect("Expected WEB_ADDRESS to be set in env!"),
        );
        map.insert(
            "WEB_PORT",
            dotenv::var("WEB_PORT").expect("Expected WEB_PORT to be set in env!"),
        );

        map.insert(
            "GRAPHQL_ADDRESS",
            dotenv::var("GRAPHQL_ADDRESS").expect("Expected GRAPHQL_ADDRESS to be set in env!"),
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

        map.insert(
            "MONGODB_URI",
            dotenv::var("MONGODB_URI").expect("Expected MONGODB_URI to be set in env!"),
        );
        map.insert(
            "MONGODB_BUDSHOME",
            dotenv::var("MONGODB_BUDSHOME").expect("Expected MONGODB_BUDSHOME to be set in env!"),
        );
        map.insert(
            "MONGODB_YAZHIJIA",
            dotenv::var("MONGODB_YAZHIJIA").expect("Expected MONGODB_YAZHIJIA to be set in env!"),
        );

        map
    };
}
