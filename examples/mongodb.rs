// use crate::constant::ENV;
use mongodb::{Client, options::ClientOptions};

#[async_std::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("msg");
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).expect("msg");

    for db_name in client.list_database_names(None, None).await {
        println!("{:?}", db_name);
    }
}
