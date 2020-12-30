use dotenv::dotenv;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ENV: HashMap<&'static str, String> = {
        dotenv().ok();

        let mut map = HashMap::new();

        map.insert("GRAPHQL_ADDRESS", dotenv::var("GRAPHQL_ADDRESS").unwrap());
        map.insert("GRAPHQL_PORT", dotenv::var("GRAPHQL_PORT").unwrap());
        map.insert("GRAPHQL_PATH", dotenv::var("GRAPHQL_PATH").unwrap());
        map.insert("GRAPHIQL_PATH", dotenv::var("GRAPHIQL_PATH").unwrap());

        map
    };
}
