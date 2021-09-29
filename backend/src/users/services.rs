use futures::stream::StreamExt;
use mongodb::{
    Database,
    bson::{Document, doc, to_document, from_document},
};
use async_graphql::{Error, ErrorExtensions};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};

use crate::util::{
    constant::{CFG, GqlResult},
    common::{Claims, token_data},
};
use crate::users::models::{User, NewUser, SignInfo};

// Find user info by email
pub async fn get_user_by_email(db: Database, email: &str) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"email": email}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("Email not found").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Find user info by username
pub async fn get_user_by_username(
    db: Database,
    username: &str,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    let exist_document = coll.find_one(doc! {"username": username}, None).await;

    if let Ok(user_document_exist) = exist_document {
        if let Some(user_document) = user_document_exist {
            let user: User = from_document(user_document)?;
            Ok(user)
        } else {
            Err(Error::new("Username not found").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("Error searching mongodb")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Create new user
pub async fn user_register(
    db: Database,
    mut new_user: NewUser,
) -> GqlResult<User> {
    let coll = db.collection::<Document>("users");

    new_user.email = new_user.email.to_lowercase();
    new_user.username = new_user.username.to_lowercase();

    if self::get_user_by_email(db.clone(), &new_user.email).await.is_ok() {
        Err(Error::new("email exists")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    } else if self::get_user_by_username(db.clone(), &new_user.username)
        .await
        .is_ok()
    {
        Err(Error::new("username exists")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    } else {
        new_user.cred =
            super::cred::cred_encode(&new_user.username, &new_user.cred).await;
        let new_user_document = to_document(&new_user)?;

        // Insert into a MongoDB collection
        coll.insert_one(new_user_document, None)
            .await
            .expect("Failed to insert into a MongoDB collection!");

        self::get_user_by_email(db.clone(), &new_user.email).await
    }
}

// User sign in
pub async fn user_sign_in(
    db: Database,
    unknown_user: NewUser,
) -> GqlResult<SignInfo> {
    unknown_user.email.to_lowercase();
    unknown_user.username.to_lowercase();

    let user_res;
    match regex::Regex::new(r"(@)").unwrap().is_match(&unknown_user.email) {
        true => {
            user_res =
                self::get_user_by_email(db.clone(), &unknown_user.email).await;
        }
        false => {
            user_res =
                self::get_user_by_username(db.clone(), &unknown_user.username)
                    .await;
        }
    }

    if let Ok(user) = user_res {
        if super::cred::cred_verify(
            &user.username,
            &unknown_user.cred,
            &user.cred,
        )
        .await
        {
            let mut header = Header::default();
            // header.kid = Some("signing_key".to_owned());
            header.alg = Algorithm::HS512;

            let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();
            let claim_exp =
                CFG.get("CLAIM_EXP").unwrap().parse::<usize>().unwrap();
            let claims = Claims {
                email: user.email.to_owned(),
                username: user.username.to_owned(),
                exp: claim_exp,
            };

            let token = match encode(
                &header,
                &claims,
                &EncodingKey::from_secret(site_key),
            ) {
                Ok(t) => t,
                Err(error) => Err(Error::new(format!(
                    "Error to encode token: {}",
                    error
                ))
                .extend_with(|err, eev| {
                    eev.set("details", err.message.as_str())
                }))?,
            };

            let sign_info = SignInfo {
                email: user.email,
                username: user.username,
                token: token,
            };
            Ok(sign_info)
        } else {
            Err(Error::new("Invalid credential").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new("User not exist")
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Find all users
pub async fn all_users(db: Database, token: &str) -> GqlResult<Vec<User>> {
    let token_data = token_data(token).await;
    if token_data.is_ok() {
        let coll = db.collection::<Document>("users");

        let mut users: Vec<User> = vec![];

        // Query all documents in the collection.
        let mut cursor = coll.find(None, None).await.unwrap();

        // Iterate over the results of the cursor.
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    let user = from_document(document)?;
                    users.push(user);
                }
                Err(error) => {
                    Err(Error::new(format!("Error to find doc: {}", error))
                        .extend_with(|err, eev| {
                            eev.set("details", err.message.as_str())
                        }))?
                }
            }
        }

        Ok(users)
    } else {
        Err(Error::new(format!("{}", token_data.err().unwrap()))
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Change user password
pub async fn user_change_password(
    db: Database,
    cur_password: &str,
    new_password: &str,
    token: &str,
) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::get_user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            if super::cred::cred_verify(
                &user.username,
                cur_password,
                &user.cred,
            )
            .await
            {
                user.cred =
                    super::cred::cred_encode(&user.username, new_password)
                        .await;

                let coll = db.collection::<Document>("users");
                coll.update_one(
                    doc! {"_id": &user._id},
                    doc! {"$set": {"cred": &user.cred}},
                    None,
                )
                .await
                .expect("Failed to update a MongoDB collection!");

                Ok(user)
            } else {
                Err(Error::new("Error verifying current password").extend_with(
                    |err, eev| eev.set("details", err.message.as_str()),
                ))
            }
        } else {
            Err(Error::new("User not exist").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new(format!("{}", token_data.err().unwrap()))
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}

// Update user profile
pub async fn user_update_profile(
    db: Database,
    new_user: NewUser,
    token: &str,
) -> GqlResult<User> {
    let token_data = token_data(token).await;
    if let Ok(data) = token_data {
        let email = data.claims.email;
        let user_res = self::get_user_by_email(db.clone(), &email).await;
        if let Ok(mut user) = user_res {
            let coll = db.collection::<Document>("users");

            user.email = new_user.email.to_lowercase();
            user.username = new_user.username.to_lowercase();

            let user_document = to_document(&user)?;

            coll.find_one_and_replace(
                doc! {"_id": &user._id},
                user_document,
                None,
            )
            .await
            .expect("Failed to replace a MongoDB collection!");

            Ok(user)
        } else {
            Err(Error::new("User not exist").extend_with(|err, eev| {
                eev.set("details", err.message.as_str())
            }))
        }
    } else {
        Err(Error::new(format!("{}", token_data.err().unwrap()))
            .extend_with(|err, eev| eev.set("details", err.message.as_str())))
    }
}
