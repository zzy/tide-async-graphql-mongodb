use futures::stream::StreamExt;
use mongodb::Database;
use async_graphql::ErrorExtensions;

use crate::constant::GqlResult;
use crate::users::models::{User, NewUser};

use super::cred;

pub async fn user_register(db: Database, mut new_user: NewUser) -> GqlResult<User> {
    let coll = db.collection("users");

    let exist_document = coll
        .find_one(
            bson::doc! {"$or": [{"email": &new_user.email}, {"username": &new_user.username}]},
            None,
        )
        .await
        .unwrap();

    if exist_document.is_none() {
        new_user.password = cred::cred_encode(&new_user.username, &new_user.password).await;
        println!("{}", &new_user.password.len());
        let new_user_bson = bson::to_bson(&new_user).unwrap();

        if let bson::Bson::Document(document) = new_user_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        };

        let user_document = coll
            .find_one(bson::doc! {"email": &new_user.email}, None)
            .await
            .expect("Document not found")
            .unwrap();

        let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
        Ok(user)
    } else {
        Err(async_graphql::Error::new("MyMessage")
            .extend_with(|_, e| e.set("details", "CAN_NOT_FETCH")))
    }
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
