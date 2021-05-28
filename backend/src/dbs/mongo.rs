use crate::util::constant::CFG;

use mongodb::{Client, options::ClientOptions, Database};

pub struct DataSource {
    client: Client,
    pub db_budshome: Database,
}

#[allow(dead_code)]
impl DataSource {
    pub async fn client(&self) -> Client {
        self.client.clone()
    }

    pub async fn init() -> DataSource {
        // Parse a connection string into an options struct.
        // environment variables defined in .env file
        let mut client_options =
            ClientOptions::parse(CFG.get("MONGODB_URI").unwrap())
                .await
                .expect("Failed to parse options!");
        // Manually set an option.
        client_options.app_name =
            Some("tide-async-graphql-mongodb".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)
            .expect("Failed to initialize database!");

        // Get a handle to a database.
        let db_budshome = client.database(CFG.get("MONGODB_BUDSHOME").unwrap());

        // return mongodb datasource.
        DataSource { client: client, db_budshome: db_budshome }
    }
}
