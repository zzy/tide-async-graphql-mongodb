use crate::constant::ENV;

use mongodb::{Client, options::ClientOptions, Database};

pub struct DataSource {
    client: Client,
    pub db_budshome: Database,
    pub db_yazhijia: Database,
}

#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        let mut client_options = ClientOptions::parse(ENV.get("MONGODB_URI").unwrap())
            .await
            .expect("Failed to parse options!");
        client_options.app_name = Some("tide-async-graphql-mongodb".to_string());

        let client = Client::with_options(client_options).expect("Failed to initialize database!");

        let db_budshome = client.database(ENV.get("MONGODB_BUDSHOME").unwrap());
        let db_yazhijia = client.database(ENV.get("MONGODB_YAZHIJIA").unwrap());

        DataSource { client: client, db_budshome: db_budshome, db_yazhijia: db_yazhijia }
    }
}
