use futures::stream::StreamExt;
use mongodb::Database;
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::constant::GqlResult;
use crate::users::models::{User, NewUser};

// get user info by email
pub async fn get_user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let exist_document = coll.find_one(bson::doc! {"email": email}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
            Ok(user)
        } else {
            Err(Error::new("2-email").extend_with(|_, e| {
                e.set("details", "Error converting the BSON object into a MongoDB document")
            }))
        }
    } else {
        Err(Error::new("1-email").extend_with(|_, e| e.set("details", "Document not found")))
    }
}

// get user info by username
pub async fn get_user_by_username(db: Database, username: &str) -> GqlResult<User> {
    let coll = db.collection("users");

    let exist_document = coll.find_one(bson::doc! {"username": username}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = bson::from_bson(bson::Bson::Document(user_document)).unwrap();
            Ok(user)
        } else {
            Err(Error::new("4-username").extend_with(|_, e| {
                e.set("details", "Error converting the BSON object into a MongoDB document")
            }))
        }
    } else {
        Err(Error::new("3-username").extend_with(|_, e| e.set("details", "Document not found")))
    }
}

pub async fn user_register(db: Database, mut new_user: NewUser) -> GqlResult<User> {
    let coll = db.collection("users");

    new_user.email = new_user.email.to_lowercase();
    new_user.username = new_user.username.to_lowercase();

    if self::get_user_by_email(db.clone(), &new_user.email).await.is_ok() {
        Err(Error::new("email exists").extend_with(|_, e| e.set("details", "1_EMAIL_EXIStS")))
    } else if self::get_user_by_username(db.clone(), &new_user.username).await.is_ok() {
        Err(Error::new("username exists").extend_with(|_, e| e.set("details", "2_USERNAME_EXISTS")))
    } else {
        new_user.password = super::cred::cred_encode(&new_user.username, &new_user.password).await;
        let new_user_bson = bson::to_bson(&new_user).unwrap();

        if let bson::Bson::Document(document) = new_user_bson {
            // Insert into a MongoDB collection
            coll.insert_one(document, None)
                .await
                .expect("Failed to insert into a MongoDB collection!");

            self::get_user_by_email(db.clone(), &new_user.email).await
        } else {
            Err(Error::new("5-register").extend_with(|_, e| {
                e.set("details", "Error converting the BSON object into a MongoDB document")
            }))
        }
    }
}

pub async fn user_sign_in(db: Database, user_account: NewUser) -> GqlResult<User> {
    user_account.email.to_lowercase();
    user_account.username.to_lowercase();

    let user;
    match regex::Regex::new(r"(@)").unwrap().is_match(&user_account.email) {
        true => {
            user = self::get_user_by_email(db.clone(), &user_account.email).await.unwrap();
        }
        false => {
            user = self::get_user_by_username(db.clone(), &user_account.username).await.unwrap();
        }
    }

    if super::cred::cred_verify(&user.username, &user_account.password, &user.credential).await {
        Ok(user)
    } else {
        Err(Error::new("user_sign_in").extend_with(|_, e| e.set("details", "Invalid credential")))
    }
}

pub async fn all_users(db: Database) -> GqlResult<Vec<User>> {
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

    if users.len() > 0 {
        Ok(users)
    } else {
        Err(Error::new("6-all").extend_with(|_, e| e.set("details", "No records")))
    }
}
