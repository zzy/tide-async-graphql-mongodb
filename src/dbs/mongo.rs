use crate::constant::ENV;

use mongodb::{Client, Database};

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
}

pub async fn ds() -> DataSource {
    let client = Client::with_uri_str(ENV.get("MONGODB_URI").unwrap())
        .await
        .expect("Failed to initialize database!");
    let db_budshome = client.database(ENV.get("MONGODB_BUDSHOME").unwrap());
    let db_yazhijia = client.database(ENV.get("MONGODB_YAZHIJIA").unwrap());

    DataSource { client: client, db_budshome: db_budshome, db_yazhijia: db_yazhijia }
}
