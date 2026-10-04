mod error;
mod models;
mod routes;
mod translate;

use std::{env, path::PathBuf};

use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
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
    };

    let app = Router::new()
        .route("/api/health", get(routes::health))
        .route("/api/people", get(routes::list).post(routes::create))
        .route("/api/people/today", get(routes::today))
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
