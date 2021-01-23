use serde::{Serialize, Deserialize};
use jsonwebtoken::{decode, TokenData, Algorithm, DecodingKey, Validation, errors::Error};

use crate::util::constant::CFG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub email: String,
    pub username: String,
    pub exp: usize,
}

pub async fn token_data(token: &str) -> Result<TokenData<Claims>, Error> {
    let site_key = CFG.get("SITE_KEY").unwrap().as_bytes();

    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(site_key),
        &Validation::new(Algorithm::HS512),
    );

    data
}
