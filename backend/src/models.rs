use chrono::{DateTime, NaiveDate, Utc};
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct Person {
    pub id: Uuid,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub death_date: NaiveDate,
    pub bio: String,
    pub lang: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct Media {
    pub id: Uuid,
    pub kind: String,
    pub url: String,
}

#[derive(Serialize)]
pub struct PersonDetail {
    #[serde(flatten)]
    pub person: Person,
    pub media: Vec<Media>,
}
