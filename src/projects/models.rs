use chrono::prelude::{Utc, DateTime, NaiveDateTime};
#[derive(Clone)]
pub struct Project {
    pub _id: String,
    pub user_id: String,
    pub subject: String,
    pub website: String,
}

#[async_graphql::Object]
impl Project {
    pub fn id(&self) -> &str {
        self._id
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn subject(&self) -> &str {
        self.subject.as_str()
    }

    pub fn website(&self) -> &str {
        self.website.as_str()
    }

    pub fn source_code(&self) -> &str {
        self.source_code.as_str()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.created_at, Utc)
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_utc(self.updated_at, Utc)
    }

    pub fn published(&self) -> bool {
        self.published
    }
}

#[derive(async_graphql::InputObject)]
pub struct NewProject {
    pub user_id: String,
    pub subject: String,
    pub website: String,
}
