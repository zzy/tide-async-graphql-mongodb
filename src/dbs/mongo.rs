use crate::constant::ENV;

use mongodb::{Client, Database};

pub struct DataSource {
    pub client: Client,
    pub db_budshome: Database,
}

impl DataSource {
    pub async fn new() -> DataSource {
        let client = Client::with_uri_str(ENV.get("MONGODB_URI").unwrap())
            .await
            .expect("Failed to initialize database!");
        let db_budshome = client.database(ENV.get("DB_BUDSHOME").unwrap());

        DataSource { client: client, db_budshome: db_budshome }
    }
}
