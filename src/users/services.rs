use futures::stream::StreamExt;
use mongodb::Database;

use crate::users::models::{User, NewUser};

pub async fn add_user(db: Database, new_user: NewUser) -> User {
    let coll = db.collection("users");

    let exist_document =
        coll.find_one(Some(bson::doc! { "email": &new_user.email }), None).await.unwrap();
    if let Some(_document) = exist_document {
        println!("MongoDB document is exist!");
    } else {
        let new_user_bson = bson::to_bson(&new_user).unwrap();

        if let bson::Bson::Document(document) = new_user_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        };
    }

    let user_document = coll
        .find_one(Some(bson::doc! { "email": &new_user.email }), None)
        .await
        .expect("Document not found")
        .unwrap();

    let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
    user
}

pub async fn all_users(db: Database) -> Vec<User> {
    let coll = db.collection("users");

    let mut users: Vec<User> = vec![];

    // Query all documents in the collection.
    let mut cursor = coll.find(None, None).await.unwrap();

    // Iterate over the results of the cursor.
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let user = bson::from_bson(bson::Bson::Document(document)).unwrap();
                users.push(user);
            }
            Err(error) => {
                println!("Error to find doc: {}", error);
            }
        }
    }

    users
}
