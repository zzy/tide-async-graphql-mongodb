use async_std::sync::{Arc, RwLock};

use crate::users::models::User;

#[derive(Default)]
pub struct Users(pub Arc<RwLock<Vec<User>>>);
