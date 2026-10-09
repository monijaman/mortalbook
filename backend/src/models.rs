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
    pub occupation: Option<String>,
    pub birth_place: Option<String>,
    pub death_place: Option<String>,
    /// 'day' | 'month' | 'year' | 'decade' | 'century'
    pub birth_precision: String,
    pub death_precision: String,
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

#[derive(Serialize, FromRow)]
pub struct PendingPerson {
    pub id: Uuid,
    pub wikidata_id: String,
    pub name: String,
    pub birth_date: Option<NaiveDate>,
    pub death_date: NaiveDate,
    pub bio: String,
    pub occupation: Option<String>,
    pub birth_place: Option<String>,
    pub death_place: Option<String>,
    pub wikidata_url: String,
    pub wikipedia_url: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
