use std::path::PathBuf;

use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use chrono::{Datelike, NaiveDate, Utc};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Media, Person, PersonDetail},
    translate::{self, TranslateRequest, TranslateResponse},
    AppState,
};

const PERSON_COLS: &str = "id, name, birth_date, death_date, bio, lang, photo_url, occupation, birth_place, death_place, created_at";

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[derive(Deserialize)]
pub struct TodayQuery {
    month: Option<u32>,
    day: Option<u32>,
}

/// People who died on a given calendar day (defaults to today, UTC).
/// The client passes its own local month/day so "today" matches the visitor's timezone.
pub async fn today(
    State(s): State<AppState>,
    Query(q): Query<TodayQuery>,
) -> Result<Json<Vec<Person>>, AppError> {
    let now = Utc::now().date_naive();
    let month = q.month.unwrap_or(now.month());
    let day = q.day.unwrap_or(now.day());
    if NaiveDate::from_ymd_opt(2000, month, day).is_none() {
        return Err(AppError::BadRequest("invalid month/day".into()));
    }
    // In non-leap years, show Feb 29 anniversaries on Feb 28.
    let leap_year = NaiveDate::from_ymd_opt(now.year(), 2, 29).is_some();
    let feb29_fallback = month == 2 && day == 28 && !leap_year;

    let people = sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people
         WHERE (EXTRACT(MONTH FROM death_date)::int = $1 AND EXTRACT(DAY FROM death_date)::int = $2)
            OR ($3 AND EXTRACT(MONTH FROM death_date)::int = 2 AND EXTRACT(DAY FROM death_date)::int = 29)
         ORDER BY death_date, name"
    ))
    .bind(month as i32)
    .bind(day as i32)
    .bind(feb29_fallback)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(people))
}

#[derive(Deserialize)]
pub struct ListQuery {
    q: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn list(
    State(s): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Value>, AppError> {
    let search = q.q.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    let limit = q.limit.unwrap_or(24).clamp(1, 100);
    let offset = q.offset.unwrap_or(0).max(0);
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM people WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%')",
    )
    .bind(&search)
    .fetch_one(&s.db)
    .await?;
    let people = sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people
         WHERE ($1::text IS NULL OR name ILIKE '%' || $1 || '%')
         ORDER BY name, id LIMIT $2 OFFSET $3"
    ))
    .bind(&search)
    .bind(limit)
    .bind(offset)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(json!({ "items": people, "total": total, "limit": limit, "offset": offset })))
}

pub async fn get(
    State(s): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<PersonDetail>, AppError> {
    let person = sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people WHERE id = $1"
    ))
    .bind(id)
    .fetch_optional(&s.db)
    .await?
    .ok_or(AppError::NotFound)?;
    let media = sqlx::query_as::<_, Media>(
        "SELECT id, kind, url FROM media WHERE person_id = $1 ORDER BY created_at, id",
    )
    .bind(id)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(PersonDetail { person, media }))
}

struct Saved {
    kind: &'static str,
    url: String,
    path: Option<PathBuf>,
    is_photo: bool,
}

fn ext_for(content_type: &str) -> Option<(&'static str, &'static str)> {
    Some(match content_type {
        "image/jpeg" => ("image", "jpg"),
        "image/png" => ("image", "png"),
        "image/webp" => ("image", "webp"),
        "image/gif" => ("image", "gif"),
        "video/mp4" => ("video", "mp4"),
        "video/webm" => ("video", "webm"),
        "video/quicktime" => ("video", "mov"),
        _ => return None,
    })
}

fn bad(e: impl std::fmt::Display) -> AppError {
    AppError::BadRequest(e.to_string())
}

fn parse_date(label: &str, v: &str) -> Result<Option<NaiveDate>, AppError> {
    let v = v.trim();
    if v.is_empty() {
        return Ok(None);
    }
    NaiveDate::parse_from_str(v, "%Y-%m-%d")
        .map(Some)
        .map_err(|_| AppError::BadRequest(format!("{label} must be YYYY-MM-DD")))
}

/// Public submission: multipart with name, death_date, [birth_date], [bio],
/// [photo], any number of `media` files and `video_url` links.
pub async fn create(
    State(s): State<AppState>,
    mp: Multipart,
) -> Result<Json<Value>, AppError> {
    let mut saved: Vec<Saved> = Vec::new();
    let result = create_inner(&s, mp, &mut saved).await;
    if result.is_err() {
        for f in &saved {
            if let Some(p) = &f.path {
                let _ = fs::remove_file(p).await;
            }
        }
    }
    result
}

async fn create_inner(
    s: &AppState,
    mut mp: Multipart,
    saved: &mut Vec<Saved>,
) -> Result<Json<Value>, AppError> {
    let (mut name, mut birth, mut death, mut bio) =
        (String::new(), String::new(), String::new(), String::new());

    while let Some(mut field) = mp.next_field().await.map_err(bad)? {
        let fname = field.name().unwrap_or("").to_string();
        match fname.as_str() {
            "name" => name = field.text().await.map_err(bad)?,
            "birth_date" => birth = field.text().await.map_err(bad)?,
            "death_date" => death = field.text().await.map_err(bad)?,
            "bio" => bio = field.text().await.map_err(bad)?,
            "video_url" => {
                let u = field.text().await.map_err(bad)?.trim().to_string();
                if u.is_empty() {
                    continue;
                }
                if !(u.starts_with("https://") || u.starts_with("http://")) || u.len() > 500 {
                    return Err(AppError::BadRequest("video link must be an http(s) URL".into()));
                }
                saved.push(Saved { kind: "video", url: u, path: None, is_photo: false });
            }
            "photo" | "media" => {
                let ct = field.content_type().unwrap_or("").to_string();
                if field.file_name().map_or(true, |f| f.is_empty()) {
                    continue; // empty file input
                }
                let (kind, ext) = ext_for(&ct)
                    .ok_or_else(|| AppError::BadRequest(format!("unsupported file type: {ct}")))?;
                let is_photo = fname == "photo";
                if is_photo && kind != "image" {
                    return Err(AppError::BadRequest("photo must be an image".into()));
                }
                let file_name = format!("{}.{ext}", Uuid::new_v4());
                let path = s.upload_dir.join(&file_name);
                let mut file = fs::File::create(&path).await?;
                saved.push(Saved {
                    kind,
                    url: format!("/uploads/{file_name}"),
                    path: Some(path),
                    is_photo,
                });
                while let Some(chunk) = field.chunk().await.map_err(bad)? {
                    file.write_all(&chunk).await?;
                }
                file.flush().await?;
            }
            _ => {}
        }
    }

    let name = name.trim().to_string();
    let bio = bio.trim().to_string();
    if name.is_empty() || name.chars().count() > 200 {
        return Err(AppError::BadRequest("name is required (max 200 chars)".into()));
    }
    if bio.chars().count() > 10_000 {
        return Err(AppError::BadRequest("description is too long (max 10000 chars)".into()));
    }
    let death_date = parse_date("death date", &death)?
        .ok_or_else(|| AppError::BadRequest("death date is required".into()))?;
    let birth_date = parse_date("birth date", &birth)?;
    if death_date > Utc::now().date_naive() + chrono::Duration::days(1) {
        return Err(AppError::BadRequest("death date cannot be in the future".into()));
    }
    if birth_date.is_some_and(|b| b > death_date) {
        return Err(AppError::BadRequest("birth date must be before death date".into()));
    }

    let lang = if bio.is_empty() { None } else { translate::detect(s, &bio).await };
    let photo_url = saved
        .iter()
        .find(|f| f.is_photo)
        .or_else(|| saved.iter().find(|f| f.kind == "image"))
        .map(|f| f.url.clone());

    let id = Uuid::new_v4();
    let mut tx = s.db.begin().await?;
    sqlx::query(
        "INSERT INTO people (id, name, birth_date, death_date, bio, lang, photo_url)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(id)
    .bind(&name)
    .bind(birth_date)
    .bind(death_date)
    .bind(&bio)
    .bind(&lang)
    .bind(&photo_url)
    .execute(&mut *tx)
    .await?;
    for f in saved.iter() {
        sqlx::query("INSERT INTO media (id, person_id, kind, url) VALUES ($1, $2, $3, $4)")
            .bind(Uuid::new_v4())
            .bind(id)
            .bind(f.kind)
            .bind(&f.url)
            .execute(&mut *tx)
            .await?;
    }
    tx.commit().await?;
    Ok(Json(json!({ "id": id })))
}

pub async fn translate_texts(
    State(s): State<AppState>,
    Json(req): Json<TranslateRequest>,
) -> Result<Json<TranslateResponse>, AppError> {
    let translations = translate::translate(&s, req).await?;
    Ok(Json(TranslateResponse { translations }))
}

pub async fn languages(State(s): State<AppState>) -> Result<Json<Value>, AppError> {
    Ok(Json(translate::languages(&s).await?))
}
