#[derive(Clone)]
pub struct User {
    pub id: Option<u16>,
    pub first_name: String,
}

#[async_graphql::Object]
impl User {
    async fn id(&self) -> i32 {
        self.id.unwrap_or(0) as i32
    }

    async fn first_name(&self) -> &str {
        &self.first_name
    }
}

#[derive(async_graphql::InputObject)]
pub struct NewUser {
    first_name: String,
}

impl NewUser {
    pub fn into_internal(self) -> User {
        User { id: None, first_name: self.first_name }
    }
}
