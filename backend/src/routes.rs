use std::{collections::HashMap, path::PathBuf};

use axum::{
    extract::{Multipart, Path, Query, State},
    http::{
        header::{COOKIE, ORIGIN, SET_COOKIE},
        HeaderMap, StatusCode,
    },
    response::{IntoResponse, Response},
    Json,
};
use chrono::{Datelike, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use tokio::{fs, io::AsyncWriteExt};
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{Media, PendingPerson, Person, PersonDetail},
    translate::{self, TranslateRequest, TranslateResponse},
    AppState,
};

const PERSON_COLS: &str = "id, name, birth_date, death_date, bio, lang, photo_url, occupation, birth_place, death_place, birth_precision, death_precision, created_at";
const ADMIN_SESSION_COOKIE: &str = "mortalbook_admin_session";
const ADMIN_SESSION_HOURS: i64 = 12;

pub async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

#[derive(Deserialize)]
pub struct TodayQuery {
    month: Option<u32>,
    day: Option<u32>,
    country: Option<String>,
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
         WHERE death_precision = 'day'
           AND ((EXTRACT(MONTH FROM death_date)::int = $1 AND EXTRACT(DAY FROM death_date)::int = $2)
             OR ($3 AND EXTRACT(MONTH FROM death_date)::int = 2 AND EXTRACT(DAY FROM death_date)::int = 29))
         ORDER BY death_date DESC, name"
    ))
    .bind(month as i32)
    .bind(day as i32)
    .bind(feb29_fallback)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(supplement_by_country(
        people,
        q.country.as_deref(),
        |person| person,
    )))
}

#[derive(Deserialize)]
pub struct WeekQuery {
    year: Option<i32>,
    month: Option<u32>,
    day: Option<u32>,
    /// days looked at on each side of today (default 7)
    span: Option<i64>,
    /// max people returned per side (default 6)
    limit: Option<usize>,
    country: Option<String>,
}

#[derive(sqlx::FromRow)]
struct PersonMd {
    #[sqlx(flatten)]
    person: Person,
    md: i32,
}

#[derive(Serialize)]
struct Anniversary {
    #[serde(flatten)]
    person: Person,
    /// days from today: positive = upcoming, negative = already passed
    days: i64,
}

/// Death anniversaries in the days after (tomorrow..+span) and before (-span..yesterday) a date.
/// Only the nearest `limit` people per side are returned, those with photos first within a day.
pub async fn week(
    State(s): State<AppState>,
    Query(q): Query<WeekQuery>,
) -> Result<Json<Value>, AppError> {
    let now = Utc::now().date_naive();
    let today = NaiveDate::from_ymd_opt(
        q.year.unwrap_or(now.year()),
        q.month.unwrap_or(now.month()),
        q.day.unwrap_or(now.day()),
    )
    .ok_or_else(|| AppError::BadRequest("invalid date".into()))?;
    let span = q.span.unwrap_or(7).clamp(1, 14);
    let limit = q.limit.unwrap_or(6).clamp(1, 24);

    // month*100+day -> offset from today
    let mut keys: HashMap<i32, i64> = HashMap::new();
    for off in (-span..=span).filter(|o| *o != 0) {
        let d = today + Duration::days(off);
        keys.insert((d.month() * 100 + d.day()) as i32, off);
        let leap = NaiveDate::from_ymd_opt(d.year(), 2, 29).is_some();
        if d.month() == 2 && d.day() == 28 && !leap {
            keys.entry(229).or_insert(off); // Feb 29 anniversaries land on Feb 28
        }
    }
    let ids: Vec<i32> = keys.keys().copied().collect();

    let rows = sqlx::query_as::<_, PersonMd>(&format!(
        "SELECT {PERSON_COLS},
                (EXTRACT(MONTH FROM death_date) * 100 + EXTRACT(DAY FROM death_date))::int AS md
         FROM people
         WHERE death_precision = 'day'
           AND (EXTRACT(MONTH FROM death_date) * 100 + EXTRACT(DAY FROM death_date))::int = ANY($1)"
    ))
    .bind(&ids)
    .fetch_all(&s.db)
    .await?;

    let mut all: Vec<Anniversary> = rows
        .into_iter()
        .filter_map(|r| keys.get(&r.md).map(|d| Anniversary { person: r.person, days: *d }))
        .collect();
    all = supplement_by_country(all, q.country.as_deref(), |anniversary| {
        &anniversary.person
    });
    all.sort_by(|a, b| {
        (a.days.abs(), a.person.photo_url.is_none(), &a.person.name)
            .cmp(&(b.days.abs(), b.person.photo_url.is_none(), &b.person.name))
    });
    let upcoming: Vec<&Anniversary> = all.iter().filter(|a| a.days > 0).take(limit).collect();
    let recent: Vec<&Anniversary> = all.iter().filter(|a| a.days < 0).take(limit).collect();
    Ok(Json(json!({ "upcoming": upcoming, "recent": recent })))
}

/// People who died during the two years ending on the supplied date (defaults to today, UTC).
pub async fn recent(
    State(s): State<AppState>,
    Query(q): Query<WeekQuery>,
) -> Result<Json<Vec<Person>>, AppError> {
    let now = Utc::now().date_naive();
    let through = NaiveDate::from_ymd_opt(
        q.year.unwrap_or(now.year()),
        q.month.unwrap_or(now.month()),
        q.day.unwrap_or(now.day()),
    )
    .ok_or_else(|| AppError::BadRequest("invalid date".into()))?;

    let people = sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people
         WHERE death_precision = 'day'
           AND death_date >= $1::date - INTERVAL '2 years'
           AND death_date <= $1
         ORDER BY death_date DESC, photo_url IS NULL, name"
    ))
    .bind(through)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(people))
}

fn country_place_patterns(country: &str) -> Vec<String> {
    let terms: &[&str] = match country.to_ascii_lowercase().as_str() {
        "bangladesh" => &[
            "Bangladesh", "Dhaka", "Chittagong", "Chattogram", "Sylhet", "Khulna", "Barisal",
            "Rajshahi", "Rangpur", "Mymensingh", "Tangail", "Pabna", "Faridpur", "Gazipur",
            "Kishoreganj", "Narayanganj", "Bogra",
        ],
        "india" => &[
            "India", "Kolkata", "Calcutta", "West Bengal", "Bihar", "Murshidabad", "Midnapore",
            "Delhi", "Mumbai", "Bombay", "Chennai", "Madras", "Maharashtra", "Gujarat",
            "Uttar Pradesh", "Allahabad", "Hyderabad",
        ],
        "pakistan" => &[
            "Pakistan", "Lahore", "Karachi", "Punjab", "Islamabad", "Rawalpindi", "Peshawar",
            "Sindh", "Quetta",
        ],
        "united kingdom" => &[
            "United Kingdom", "Great Britain", "Britain", "England", "Scotland", "Wales",
            "London", "Oxford", "Cambridge", "Manchester", "Liverpool",
        ],
        "united states" => &[
            "United States", "USA", "U.S.", "New York", "Washington", "California", "Chicago",
            "Boston",
        ],
        _ => return vec![format!("%{country}%")],
    };
    terms.iter().map(|term| format!("%{term}%")).collect()
}

fn nearby_country_patterns(country: &str) -> Vec<String> {
    let nearby: &[&str] = match country.to_ascii_lowercase().as_str() {
        "bangladesh" => &["India"],
        "india" => &["Bangladesh", "Pakistan"],
        "pakistan" => &["India"],
        "united kingdom" => &["Ireland", "France"],
        "united states" => &["Canada", "Mexico"],
        _ => return Vec::new(),
    };
    nearby
        .iter()
        .flat_map(|country| country_place_patterns(country))
        .collect()
}

fn person_matches_country(person: &Person, patterns: &[String]) -> bool {
    let place = format!(
        "{} {}",
        person.birth_place.as_deref().unwrap_or_default(),
        person.death_place.as_deref().unwrap_or_default()
    )
    .to_lowercase();
    patterns.iter().any(|pattern| {
        let term = pattern.trim_matches('%').to_lowercase();
        !term.is_empty() && place.contains(&term)
    })
}

fn supplement_by_country<T, F>(items: Vec<T>, country: Option<&str>, person: F) -> Vec<T>
where
    F: for<'a> Fn(&'a T) -> &'a Person,
{
    let Some(country) = country.filter(|country| !country.trim().is_empty()) else {
        return items;
    };
    let selected_patterns = country_place_patterns(country);
    let nearby_patterns = nearby_country_patterns(country);
    let mut local = Vec::new();
    let mut other = Vec::new();

    for item in items {
        if person_matches_country(person(&item), &selected_patterns) {
            local.push(item);
        } else {
            other.push(item);
        }
    }

    if local.len() >= 10 {
        return local;
    }

    let mut nearby = Vec::new();
    let mut global = Vec::new();
    for item in other {
        if person_matches_country(person(&item), &nearby_patterns) {
            nearby.push(item);
        } else {
            global.push(item);
        }
    }

    let needed = 10 - local.len();
    let nearby_count = nearby.len().min(needed);
    local.extend(nearby.into_iter().take(nearby_count));
    local.extend(global.into_iter().take(needed - nearby_count));
    local
}

#[derive(Deserialize)]
pub struct ListQuery {
    q: Option<String>,
    country: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

async fn count_people(
    db: &sqlx::PgPool,
    search: &Option<String>,
    patterns: Option<&Vec<String>>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM people
         WHERE ($1::text IS NULL OR (name ILIKE '%' || $1 || '%' OR occupation ILIKE '%' || $1 || '%'))
           AND ($2::text[] IS NULL OR concat_ws(' ', birth_place, death_place) ILIKE ANY($2))",
    )
    .bind(search)
    .bind(patterns)
    .fetch_one(db)
    .await
}

async fn query_people(
    db: &sqlx::PgPool,
    search: &Option<String>,
    patterns: Option<&Vec<String>>,
    excluded_ids: &[Uuid],
    limit: i64,
    offset: i64,
) -> Result<Vec<Person>, sqlx::Error> {
    sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people
         WHERE ($1::text IS NULL OR (name ILIKE '%' || $1 || '%' OR occupation ILIKE '%' || $1 || '%'))
           AND ($2::text[] IS NULL OR concat_ws(' ', birth_place, death_place) ILIKE ANY($2))
           AND id != ALL($3::uuid[])
         ORDER BY death_date DESC, name, id LIMIT $4 OFFSET $5"
    ))
    .bind(search)
    .bind(patterns)
    .bind(excluded_ids.to_vec())
    .bind(limit)
    .bind(offset)
    .fetch_all(db)
    .await
}

pub async fn list(
    State(s): State<AppState>,
    Query(q): Query<ListQuery>,
) -> Result<Json<Value>, AppError> {
    let search = q.q.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    let country = q.country.map(|t| t.trim().to_string()).filter(|t| !t.is_empty());
    let limit = q.limit.unwrap_or(24).clamp(1, 100);
    let offset = q.offset.unwrap_or(0).max(0);

    let (people, total) = if let Some(country) = country.as_deref() {
        let local_patterns = country_place_patterns(country);
        let local_total = count_people(&s.db, &search, Some(&local_patterns)).await?;
        if local_total >= 10 {
            let people = query_people(
                &s.db,
                &search,
                Some(&local_patterns),
                &[],
                limit,
                offset,
            )
            .await?;
            (people, local_total)
        } else {
            let mut combined = query_people(
                &s.db,
                &search,
                Some(&local_patterns),
                &[],
                10,
                0,
            )
            .await?;
            let nearby_patterns = nearby_country_patterns(country);
            let remaining = 10 - combined.len() as i64;
            if remaining > 0 {
                let local_ids: Vec<Uuid> = combined.iter().map(|person| person.id).collect();
                combined.extend(
                    query_people(
                        &s.db,
                        &search,
                        Some(&nearby_patterns),
                        &local_ids,
                        remaining,
                        0,
                    )
                    .await?,
                );
            }

            let remaining = 10 - combined.len() as i64;
            if remaining > 0 {
                let excluded_ids: Vec<Uuid> = combined.iter().map(|person| person.id).collect();
                combined.extend(
                    query_people(
                        &s.db,
                        &search,
                        None,
                        &excluded_ids,
                        remaining,
                        0,
                    )
                    .await?,
                );
            }

            let total = combined.len() as i64;
            let start = (offset as usize).min(combined.len());
            let people = combined
                .into_iter()
                .skip(start)
                .take(limit as usize)
                .collect();
            (people, total)
        }
    } else {
        let total = count_people(&s.db, &search, None).await?;
        let people = query_people(&s.db, &search, None, &[], limit, offset).await?;
        (people, total)
    };
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

/// Admin-only submission: multipart with name, death_date, [birth_date], [bio],
/// [photo], any number of `media` files and `video_url` links.
pub async fn create(
    State(s): State<AppState>,
    headers: HeaderMap,
    mp: Multipart,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
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

#[derive(Deserialize)]
pub struct RecentDeathBatch {
    candidates: Vec<RecentDeathCandidate>,
}

#[derive(Clone, Deserialize)]
pub struct RecentDeathCandidate {
    wikidata_id: String,
    name: String,
    birth_date: Option<NaiveDate>,
    death_date: NaiveDate,
    occupation: Option<String>,
    birth_place: Option<String>,
    death_place: Option<String>,
    wikipedia_url: String,
}

#[derive(Deserialize)]
pub struct PendingPersonUpdate {
    name: String,
    birth_date: Option<NaiveDate>,
    death_date: NaiveDate,
    bio: String,
    occupation: Option<String>,
    birth_place: Option<String>,
    death_place: Option<String>,
}

#[derive(Deserialize)]
pub struct AdminLogin {
    username: String,
    password: String,
}

fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let mut difference = left.len() ^ right.len();
    for index in 0..left.len().max(right.len()) {
        difference |= (left.get(index).copied().unwrap_or(0)
            ^ right.get(index).copied().unwrap_or(0)) as usize;
    }
    difference == 0
}

fn verify_same_origin(headers: &HeaderMap) -> Result<(), AppError> {
    let host = headers
        .get(axum::http::header::HOST)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    let origin = headers
        .get(ORIGIN)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    let authority = origin
        .strip_prefix("https://")
        .or_else(|| origin.strip_prefix("http://"))
        .filter(|value| !value.is_empty() && !value.contains('/') && !value.contains('@'))
        .ok_or(AppError::Unauthorized)?;
    if authority.eq_ignore_ascii_case(host) {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}

fn cookie_session(headers: &HeaderMap) -> Result<String, AppError> {
    let cookie_header = headers
        .get(COOKIE)
        .and_then(|value| value.to_str().ok())
        .ok_or(AppError::Unauthorized)?;
    let session = cookie_header
        .split(';')
        .filter_map(|cookie| cookie.trim().split_once('='))
        .find_map(|(name, value)| {
            (name == ADMIN_SESSION_COOKIE).then_some(value)
        })
        .ok_or(AppError::Unauthorized)?;
    if session.len() != 32 || !session.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(AppError::Unauthorized);
    }
    Ok(session.to_string())
}

fn authorization_token(headers: &HeaderMap) -> Option<String> {
    let header_value = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())?
        .trim();
    let token = header_value
        .strip_prefix("Bearer ")
        .unwrap_or(header_value)
        .trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

async fn require_admin(
    headers: &HeaderMap,
    state: &AppState,
    require_same_origin: bool,
) -> Result<(), AppError> {
    if let Some(token) = authorization_token(headers) {
        if let Some(expected) = state.admin_review_token.as_deref() {
            if constant_time_eq(token.as_bytes(), expected.as_bytes()) {
                return Ok(());
            }
        }
    }
    if require_same_origin {
        verify_same_origin(headers)?;
    }
    let session = cookie_session(headers)?;
    let session_hash = hex::encode(Sha256::digest(session.as_bytes()));
    let valid: bool = sqlx::query_scalar(
        "SELECT EXISTS (
            SELECT 1 FROM admin_sessions
            WHERE session_hash = $1 AND expires_at > now()
         )",
    )
    .bind(session_hash)
    .fetch_one(&state.db)
    .await?;
    if valid {
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}

async fn require_recent_death_ingest(
    headers: &HeaderMap,
    state: &AppState,
) -> Result<(), AppError> {
    if let Some(token) = authorization_token(headers) {
        if let Some(expected) = state.recent_deaths_ingest_token.as_deref() {
            if constant_time_eq(token.as_bytes(), expected.as_bytes()) {
                return Ok(());
            }
        }
    }
    require_admin(headers, state, true).await
}

pub async fn admin_login(
    State(s): State<AppState>,
    headers: HeaderMap,
    Json(credentials): Json<AdminLogin>,
) -> Result<Response, AppError> {
    verify_same_origin(&headers)?;
    let (Some(username), Some(password)) = (s.admin_username.as_deref(), s.admin_password.as_deref()) else {
        return Err(AppError::ServiceUnavailable("admin login is not configured".into()));
    };
    if password.chars().count() < 16
        || !constant_time_eq(credentials.username.as_bytes(), username.as_bytes())
        || !constant_time_eq(credentials.password.as_bytes(), password.as_bytes())
    {
        tokio::time::sleep(std::time::Duration::from_millis(750)).await;
        return Err(AppError::Unauthorized);
    }

    let session = Uuid::new_v4().simple().to_string();
    let session_hash = hex::encode(Sha256::digest(session.as_bytes()));
    sqlx::query("DELETE FROM admin_sessions WHERE expires_at <= now()")
        .execute(&s.db)
        .await?;
    sqlx::query(
        "INSERT INTO admin_sessions (id, session_hash, expires_at)
         VALUES ($1, $2, now() + ($3 || ' hours')::interval)",
    )
    .bind(Uuid::new_v4())
    .bind(session_hash)
    .bind(ADMIN_SESSION_HOURS.to_string())
    .execute(&s.db)
    .await?;

    let cookie = format!(
        "{ADMIN_SESSION_COOKIE}={session}; Path=/api/admin; HttpOnly; Secure; SameSite=Strict; Max-Age={}",
        ADMIN_SESSION_HOURS * 3600
    );
    let cookie = axum::http::HeaderValue::from_str(&cookie)
        .map_err(|error| AppError::Internal(error.into()))?;
    Ok((
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        Json(json!({ "authenticated": true })),
    )
        .into_response())
}

pub async fn admin_logout(
    State(s): State<AppState>,
    headers: HeaderMap,
) -> Result<Response, AppError> {
    verify_same_origin(&headers)?;
    if let Ok(session) = cookie_session(&headers) {
        let session_hash = hex::encode(Sha256::digest(session.as_bytes()));
        sqlx::query("DELETE FROM admin_sessions WHERE session_hash = $1")
            .bind(session_hash)
            .execute(&s.db)
            .await?;
    }
    let cookie = format!(
        "{ADMIN_SESSION_COOKIE}=; Path=/api/admin; HttpOnly; Secure; SameSite=Strict; Max-Age=0"
    );
    let cookie = axum::http::HeaderValue::from_str(&cookie)
        .map_err(|error| AppError::Internal(error.into()))?;
    Ok((
        StatusCode::OK,
        [(SET_COOKIE, cookie)],
        Json(json!({ "authenticated": false })),
    )
        .into_response())
}

fn normalize_optional_text(
    value: &mut Option<String>,
    label: &str,
    max_chars: usize,
) -> Result<(), AppError> {
    if let Some(text) = value.take() {
        let normalized = text.trim().to_string();
        if normalized.is_empty() {
            return Ok(());
        }
        if normalized.chars().count() > max_chars {
            return Err(AppError::BadRequest(format!("{label} is too long")));
        }
        *value = Some(normalized);
    }
    Ok(())
}

fn validate_recent_death_candidate(
    mut candidate: RecentDeathCandidate,
) -> Result<RecentDeathCandidate, AppError> {
    candidate.name = candidate.name.trim().to_string();
    candidate.wikidata_id = candidate.wikidata_id.trim().to_string();
    if candidate.name.is_empty() || candidate.name.chars().count() > 200 {
        return Err(AppError::BadRequest("candidate name must be 1–200 characters".into()));
    }
    if candidate.wikidata_id.len() < 2
        || !candidate.wikidata_id.starts_with('Q')
        || !candidate.wikidata_id[1..].bytes().all(|byte| byte.is_ascii_digit())
    {
        return Err(AppError::BadRequest("invalid Wikidata identifier".into()));
    }
    if candidate.birth_date.is_some_and(|birth| birth > candidate.death_date) {
        return Err(AppError::BadRequest("birth date must not be after death date".into()));
    }
    let today = Utc::now().date_naive();
    if candidate.death_date < today - Duration::days(7) || candidate.death_date > today {
        return Err(AppError::BadRequest("candidate death date must be within the past seven days".into()));
    }
    if !candidate.wikipedia_url.starts_with("https://en.wikipedia.org/wiki/")
        || candidate.wikipedia_url.len() > 1000
    {
        return Err(AppError::BadRequest("candidate must include an English Wikipedia article URL".into()));
    }
    normalize_optional_text(&mut candidate.occupation, "occupation", 500)?;
    normalize_optional_text(&mut candidate.birth_place, "birth place", 300)?;
    normalize_optional_text(&mut candidate.death_place, "death place", 300)?;
    Ok(candidate)
}

fn validate_pending_update(mut update: PendingPersonUpdate) -> Result<PendingPersonUpdate, AppError> {
    update.name = update.name.trim().to_string();
    update.bio = update.bio.trim().to_string();
    if update.name.is_empty() || update.name.chars().count() > 200 {
        return Err(AppError::BadRequest("name is required (max 200 characters)".into()));
    }
    if update.bio.is_empty() || update.bio.chars().count() > 10_000 {
        return Err(AppError::BadRequest("reviewed story is required (max 10000 characters)".into()));
    }
    if update.birth_date.is_some_and(|birth| birth > update.death_date) {
        return Err(AppError::BadRequest("birth date must not be after death date".into()));
    }
    if update.death_date > Utc::now().date_naive() {
        return Err(AppError::BadRequest("death date cannot be in the future".into()));
    }
    normalize_optional_text(&mut update.occupation, "occupation", 500)?;
    normalize_optional_text(&mut update.birth_place, "birth place", 300)?;
    normalize_optional_text(&mut update.death_place, "death place", 300)?;
    Ok(update)
}

pub async fn ingest_recent_deaths(
    State(s): State<AppState>,
    headers: HeaderMap,
    Json(batch): Json<RecentDeathBatch>,
) -> Result<Json<Value>, AppError> {
    require_recent_death_ingest(&headers, &s).await?;
    if batch.candidates.len() > 200 {
        return Err(AppError::BadRequest("candidate batch exceeds 200 records".into()));
    }

    let mut seen = std::collections::HashSet::new();
    let mut candidates = Vec::with_capacity(batch.candidates.len());
    for candidate in batch.candidates {
        let candidate = validate_recent_death_candidate(candidate)?;
        if seen.insert(candidate.wikidata_id.clone()) {
            candidates.push(candidate);
        }
    }

    let mut inserted = 0_u64;
    let mut tx = s.db.begin().await?;
    for candidate in candidates {
        let public_duplicate: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM people
                WHERE lower(name) = lower($1) AND death_date = $2
             )",
        )
        .bind(&candidate.name)
        .bind(candidate.death_date)
        .fetch_one(&mut *tx)
        .await?;
        if public_duplicate {
            continue;
        }
        let pending_duplicate: bool = sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM pending_people
                WHERE lower(name) = lower($1) AND death_date = $2
                  AND wikidata_id <> $3 AND status = 'pending'
             )",
        )
        .bind(&candidate.name)
        .bind(candidate.death_date)
        .bind(&candidate.wikidata_id)
        .fetch_one(&mut *tx)
        .await?;
        if pending_duplicate {
            continue;
        }
        let wikidata_url = format!("https://www.wikidata.org/wiki/{}", candidate.wikidata_id);
        let bio = format!(
            "Review required: verify this candidate's death and biography before approval.\n\nWikidata: {wikidata_url}\nEnglish Wikipedia: {}",
            candidate.wikipedia_url
        );
        let result = sqlx::query(
            "INSERT INTO pending_people (
                id, wikidata_id, name, birth_date, death_date, bio, occupation,
                birth_place, death_place, wikidata_url, wikipedia_url
             ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
             ON CONFLICT (wikidata_id) DO NOTHING",
        )
        .bind(Uuid::new_v4())
        .bind(&candidate.wikidata_id)
        .bind(&candidate.name)
        .bind(candidate.birth_date)
        .bind(candidate.death_date)
        .bind(bio)
        .bind(candidate.occupation)
        .bind(candidate.birth_place)
        .bind(candidate.death_place)
        .bind(wikidata_url)
        .bind(candidate.wikipedia_url)
        .execute(&mut *tx)
        .await?;
        inserted += result.rows_affected();
    }
    tx.commit().await?;
    Ok(Json(json!({ "queued": inserted })))
}

pub async fn admin_me(
    State(s): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, false).await?;
    Ok(Json(json!({ "authenticated": true })))
}

#[derive(Deserialize)]
pub struct AdminPeopleQuery {
    q: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

/// Every person in the catalogue (newest first), optionally filtered by name.
pub async fn admin_list_people(
    State(s): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<AdminPeopleQuery>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, false).await?;
    let limit = q.limit.unwrap_or(25).clamp(1, 100);
    let offset = q.offset.unwrap_or(0).max(0);
    let pattern = q
        .q
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| format!("%{}%", v.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_")));
    let total: i64 = sqlx::query_scalar(
        "SELECT count(*) FROM people WHERE $1::text IS NULL OR name ILIKE $1",
    )
    .bind(&pattern)
    .fetch_one(&s.db)
    .await?;
    let items = sqlx::query_as::<_, Person>(&format!(
        "SELECT {PERSON_COLS} FROM people
         WHERE $1::text IS NULL OR name ILIKE $1
         ORDER BY created_at DESC, name
         LIMIT $2 OFFSET $3"
    ))
    .bind(&pattern)
    .bind(limit)
    .bind(offset)
    .fetch_all(&s.db)
    .await?;
    Ok(Json(json!({ "items": items, "total": total, "limit": limit, "offset": offset })))
}

#[derive(Deserialize)]
pub struct PersonUpdate {
    name: String,
    birth_date: Option<NaiveDate>,
    death_date: NaiveDate,
    bio: String,
    occupation: Option<String>,
    birth_place: Option<String>,
    death_place: Option<String>,
    birth_precision: Option<String>,
    death_precision: Option<String>,
}

const PRECISIONS: [&str; 5] = ["day", "month", "year", "decade", "century"];

pub async fn admin_update_person(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(mut update): Json<PersonUpdate>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    update.name = update.name.trim().to_string();
    update.bio = update.bio.trim().to_string();
    if update.name.is_empty() || update.name.chars().count() > 200 {
        return Err(AppError::BadRequest("name is required (max 200 characters)".into()));
    }
    if update.bio.chars().count() > 10_000 {
        return Err(AppError::BadRequest("story is too long (max 10000 characters)".into()));
    }
    if update.birth_date.is_some_and(|birth| birth > update.death_date) {
        return Err(AppError::BadRequest("birth date must not be after death date".into()));
    }
    if update.death_date > Utc::now().date_naive() + Duration::days(1) {
        return Err(AppError::BadRequest("death date cannot be in the future".into()));
    }
    for precision in [&update.birth_precision, &update.death_precision].into_iter().flatten() {
        if !PRECISIONS.contains(&precision.as_str()) {
            return Err(AppError::BadRequest("invalid date precision".into()));
        }
    }
    normalize_optional_text(&mut update.occupation, "occupation", 500)?;
    normalize_optional_text(&mut update.birth_place, "birth place", 300)?;
    normalize_optional_text(&mut update.death_place, "death place", 300)?;

    let existing_bio: String = sqlx::query_scalar("SELECT bio FROM people WHERE id = $1")
        .bind(id)
        .fetch_optional(&s.db)
        .await?
        .ok_or(AppError::NotFound)?;
    // Re-detect the language only when the story text changed.
    let lang_changed = existing_bio != update.bio;
    let lang = if lang_changed && !update.bio.is_empty() {
        translate::detect(&s, &update.bio).await
    } else {
        None
    };

    sqlx::query(
        "UPDATE people SET
            name = $2, birth_date = $3, death_date = $4, bio = $5, occupation = $6,
            birth_place = $7, death_place = $8,
            birth_precision = COALESCE($9, birth_precision),
            death_precision = COALESCE($10, death_precision),
            lang = CASE WHEN $11 THEN $12 ELSE lang END
         WHERE id = $1",
    )
    .bind(id)
    .bind(update.name)
    .bind(update.birth_date)
    .bind(update.death_date)
    .bind(update.bio)
    .bind(update.occupation)
    .bind(update.birth_place)
    .bind(update.death_place)
    .bind(update.birth_precision)
    .bind(update.death_precision)
    .bind(lang_changed)
    .bind(lang)
    .execute(&s.db)
    .await?;
    Ok(Json(json!({ "updated": true })))
}

pub async fn admin_delete_person(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    let mut tx = s.db.begin().await?;
    let urls: Vec<String> = sqlx::query_scalar(
        "SELECT url FROM media WHERE person_id = $1 AND url LIKE '/uploads/%'
         UNION SELECT photo_url FROM people WHERE id = $1 AND photo_url LIKE '/uploads/%'",
    )
    .bind(id)
    .fetch_all(&mut *tx)
    .await?;
    let result = sqlx::query("DELETE FROM people WHERE id = $1")
        .bind(id)
        .execute(&mut *tx)
        .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    tx.commit().await?;
    for url in urls {
        if let Some(name) = url.strip_prefix("/uploads/") {
            if !name.is_empty() && !name.contains('/') && !name.contains("..") {
                let _ = fs::remove_file(s.upload_dir.join(name)).await;
            }
        }
    }
    Ok(Json(json!({ "deleted": true })))
}

pub async fn pending_people(
    State(s): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<PendingPerson>>, AppError> {
    require_admin(&headers, &s, false).await?;
    let pending = sqlx::query_as::<_, PendingPerson>(
        "SELECT id, wikidata_id, name, birth_date, death_date, bio, occupation,
                birth_place, death_place, wikidata_url, wikipedia_url, created_at, updated_at
         FROM pending_people
         WHERE status = 'pending'
         ORDER BY death_date DESC, name",
    )
    .fetch_all(&s.db)
    .await?;
    Ok(Json(pending))
}

pub async fn update_pending_person(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(update): Json<PendingPersonUpdate>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    let update = validate_pending_update(update)?;
    let result = sqlx::query(
        "UPDATE pending_people SET
            name = $2, birth_date = $3, death_date = $4, bio = $5, occupation = $6,
            birth_place = $7, death_place = $8, updated_at = now()
         WHERE id = $1 AND status = 'pending'",
    )
    .bind(id)
    .bind(update.name)
    .bind(update.birth_date)
    .bind(update.death_date)
    .bind(update.bio)
    .bind(update.occupation)
    .bind(update.birth_place)
    .bind(update.death_place)
    .execute(&s.db)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "updated": true })))
}

pub async fn approve_pending_person(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    let mut tx = s.db.begin().await?;
    let pending = sqlx::query_as::<_, PendingPerson>(
        "SELECT id, wikidata_id, name, birth_date, death_date, bio, occupation,
                birth_place, death_place, wikidata_url, wikipedia_url, created_at, updated_at
         FROM pending_people
         WHERE id = $1 AND status = 'pending'
         FOR UPDATE",
    )
    .bind(id)
    .fetch_optional(&mut *tx)
    .await?
    .ok_or(AppError::NotFound)?;
    if pending.bio.starts_with("Review required:") {
        return Err(AppError::BadRequest(
            "edit the candidate story after verifying it before approval".into(),
        ));
    }
    let bio = format!(
        "{}\n\nSources:\nWikidata: {}\nEnglish Wikipedia: {}",
        pending.bio, pending.wikidata_url, pending.wikipedia_url
    );
    sqlx::query(
        "INSERT INTO people (
            id, name, birth_date, death_date, bio, lang, occupation, birth_place, death_place,
            birth_precision, death_precision
         )
         SELECT $1, $2, $3, $4, $5, 'en', $6, $7, $8,
                CASE WHEN $3 IS NULL THEN 'year' ELSE 'day' END, 'day'
         WHERE NOT EXISTS (
            SELECT 1 FROM people WHERE lower(name) = lower($2) AND death_date = $4
         )",
    )
    .bind(Uuid::new_v4())
    .bind(&pending.name)
    .bind(pending.birth_date)
    .bind(pending.death_date)
    .bind(bio)
    .bind(&pending.occupation)
    .bind(&pending.birth_place)
    .bind(&pending.death_place)
    .execute(&mut *tx)
    .await?;
    sqlx::query(
        "UPDATE pending_people SET status = 'approved', reviewed_at = now(), updated_at = now()
         WHERE id = $1",
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Json(json!({ "approved": true })))
}

pub async fn reject_pending_person(
    State(s): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    let result = sqlx::query(
        "UPDATE pending_people SET status = 'rejected', reviewed_at = now(), updated_at = now()
         WHERE id = $1 AND status = 'pending'",
    )
    .bind(id)
    .execute(&s.db)
    .await?;
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(Json(json!({ "rejected": true })))
}

#[cfg(test)]
mod country_filter_tests {
    use super::*;

    fn person(name: &str, birth_place: &str) -> Person {
        Person {
            id: Uuid::new_v4(),
            name: name.to_string(),
            birth_date: None,
            death_date: NaiveDate::from_ymd_opt(2000, 1, 1).unwrap(),
            bio: String::new(),
            lang: None,
            photo_url: None,
            occupation: None,
            birth_place: Some(birth_place.to_string()),
            death_place: None,
            birth_precision: "year".to_string(),
            death_precision: "day".to_string(),
            created_at: Utc::now(),
        }
    }

    #[test]
    fn supplements_sparse_local_results_with_nearby_then_global_people() {
        let people = vec![
            person("Local 1", "Dhaka, Bangladesh"),
            person("Local 2", "Sylhet, Bangladesh"),
            person("Nearby 1", "Kolkata, India"),
            person("Nearby 2", "Delhi, India"),
            person("Nearby 3", "Chennai, India"),
        ]
        .into_iter()
        .chain((1..=10).map(|n| person(&format!("Global {n}"), "Paris, France")))
        .collect();

        let results = supplement_by_country(people, Some("Bangladesh"), |person| person);

        assert_eq!(results.len(), 10);
        assert_eq!(
            results.iter().filter(|person| person.name.starts_with("Local")).count(),
            2
        );
        assert_eq!(
            results.iter().filter(|person| person.name.starts_with("Nearby")).count(),
            3
        );
        assert_eq!(
            results.iter().filter(|person| person.name.starts_with("Global")).count(),
            5
        );
    }

    #[test]
    fn keeps_all_local_results_when_threshold_is_met() {
        let people = (1..=10)
            .map(|n| person(&format!("Local {n}"), "Dhaka, Bangladesh"))
            .chain(std::iter::once(person("Nearby", "Kolkata, India")))
            .collect();

        let results = supplement_by_country(people, Some("Bangladesh"), |person| person);

        assert_eq!(results.len(), 10);
        assert!(results.iter().all(|person| person.name.starts_with("Local")));
    }

    #[test]
    fn bearer_token_comparison_checks_the_entire_token() {
        assert!(constant_time_eq(b"admin-token", b"admin-token"));
        assert!(!constant_time_eq(b"admin-token", b"admin-tokeN"));
        assert!(!constant_time_eq(b"admin-token", b"admin-token-extra"));
    }
}
