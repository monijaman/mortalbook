mod error;
mod models;
mod routes;
mod story;
mod translate;

use std::{env, path::PathBuf};

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, patch, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub http: reqwest::Client,
    pub translate_url: String,
    pub translate_key: Option<String>,
    pub upload_dir: PathBuf,
    pub admin_username: Option<String>,
    pub admin_password: Option<String>,
    pub admin_review_token: Option<String>,
    pub openai_api_key: Option<String>,
    pub openai_model: String,
    pub recent_deaths_ingest_token: Option<String>,
    pub github_actions_token: Option<String>,
    pub r2: Option<R2Storage>,
}

#[derive(Clone)]
pub struct R2Storage {
    pub client: aws_sdk_s3::Client,
    pub bucket: String,
    pub public_url: String,
}

const MAX_UPLOAD_BYTES: usize = 300 * 1024 * 1024;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=info".into()),
        )
        .init();

    let database_url = env::var("DATABASE_URL")?;
    let upload_dir = PathBuf::from(env::var("UPLOAD_DIR").unwrap_or_else(|_| "./uploads".into()));
    tokio::fs::create_dir_all(&upload_dir).await?;

    let db = PgPoolOptions::new().max_connections(10).connect(&database_url).await?;
    sqlx::migrate!("./migrations").run(&db).await?;

    let r2 = match (env::var("R2_ENDPOINT").ok(), env::var("R2_BUCKET").ok(), env::var("R2_ACCESS_KEY_ID").ok(), env::var("R2_SECRET_ACCESS_KEY").ok(), env::var("R2_PUBLIC_URL").ok()) {
        (Some(endpoint), Some(bucket), Some(access), Some(secret), Some(public_url)) if [endpoint.as_str(), bucket.as_str(), access.as_str(), secret.as_str(), public_url.as_str()].iter().all(|v| !v.is_empty()) => {
            let credentials = aws_sdk_s3::config::Credentials::new(access, secret, None, None, "r2");
            let config = aws_sdk_s3::config::Builder::new().behavior_version(aws_sdk_s3::config::BehaviorVersion::latest()).endpoint_url(endpoint).region(aws_sdk_s3::config::Region::new("auto")).credentials_provider(credentials).force_path_style(true).build();
            Some(R2Storage { client: aws_sdk_s3::Client::from_conf(config), bucket, public_url: public_url.trim_end_matches('/').to_string() })
        }
        _ => None,
    };

    let state = AppState {
        db,
        http: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()?,
        translate_url: env::var("TRANSLATE_URL")
            .unwrap_or_else(|_| "http://localhost:5000".into())
            .trim_end_matches('/')
            .to_string(),
        translate_key: env::var("TRANSLATE_API_KEY").ok().filter(|k| !k.is_empty()),
        upload_dir: upload_dir.clone(),
        admin_username: env::var("ADMIN_USERNAME").ok().filter(|value| !value.is_empty()),
        admin_password: env::var("ADMIN_PASSWORD")
            .ok()
            .filter(|value| !value.is_empty()),
        openai_api_key: env::var("OPENAI_API_KEY").ok().filter(|value| !value.is_empty()),
        openai_model: env::var("OPENAI_MODEL")
            .ok()
            .filter(|value| !value.is_empty())
            .unwrap_or_else(|| "gpt-4.1".into()),
        admin_review_token: env::var("ADMIN_REVIEW_TOKEN")
            .ok()
            .filter(|value| !value.is_empty()),
        recent_deaths_ingest_token: env::var("RECENT_DEATHS_INGEST_TOKEN")
            .ok()
            .filter(|value| !value.is_empty()),
        github_actions_token: env::var("GITHUB_ACTIONS_TOKEN")
            .ok()
            .filter(|value| !value.is_empty()),
        r2,
    };

    let app = Router::new()
        .route("/api/health", get(routes::health))
        .route("/api/people", get(routes::list))
        .route("/api/admin/login", post(routes::admin_login))
        .route("/api/admin/logout", post(routes::admin_logout))
        .route("/api/admin/me", get(routes::admin_me))
        .route("/api/admin/generate-story", post(story::generate_story))
        .route(
            "/api/admin/people",
            get(routes::admin_list_people).post(routes::create),
        )
        .route(
            "/api/admin/people/:id",
            patch(routes::admin_update_person).delete(routes::admin_delete_person),
        )
        .route("/api/admin/people/:id/fetch-photo", post(routes::admin_fetch_person_photo))
        .route("/api/admin/people/:id/photo", post(routes::admin_upload_person_photo))
        .route("/api/admin/refresh-stories", post(routes::refresh_stories))
        .route("/api/admin/pending-people", get(routes::pending_people))
        .route(
            "/api/admin/recent-deaths/history",
            get(routes::recent_death_history),
        )
        .route(
            "/api/admin/recent-deaths/run",
            post(routes::run_recent_deaths),
        )
        .route("/api/admin/pending-people/:id", patch(routes::update_pending_person))
        .route("/api/admin/pending-people/:id/images", post(routes::upload_pending_images))
        .route("/api/admin/pending-people/bulk-approve", post(routes::bulk_approve_pending_people))
        .route(
            "/api/admin/pending-people/:id/approve",
            post(routes::approve_pending_person),
        )
        .route(
            "/api/admin/pending-people/:id/reject",
            post(routes::reject_pending_person),
        )
        .route(
            "/api/admin/recent-deaths",
            post(routes::ingest_recent_deaths).layer(DefaultBodyLimit::max(1024 * 1024)),
        )
        .route("/api/people/today", get(routes::today))
        .route("/api/people/week", get(routes::week))
        .route("/api/people/recent", get(routes::recent))
        .route("/api/people/:id", get(routes::get))
        .route("/api/translate", post(routes::translate_texts))
        .route("/api/languages", get(routes::languages))
        .nest_service("/uploads", ServeDir::new(upload_dir))
        .layer(DefaultBodyLimit::max(MAX_UPLOAD_BYTES))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let bind = env::var("BIND").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&bind).await?;
    tracing::info!("listening on {bind}");
    axum::serve(listener, app).await?;
    Ok(())
}
