use mongodb::{Client, options::ClientOptions, bson::doc};

#[async_std::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await.expect("msg");
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options).expect("msg");

    for db_name in client.list_database_names(None, None).await {
        println!("{:?}", db_name);
    }

    let db = client.database("budshome");

    // Get a handle to a collection in the database.
    let collection = db.collection("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "budshome.books" collection.
    collection.insert_many(docs, None).await.expect("msg");
}
