use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

use crate::{error::AppError, AppState};

const MAX_TEXTS: usize = 200;
const MAX_CHARS: usize = 60_000;

#[derive(Deserialize)]
pub struct TranslateRequest {
    pub texts: Vec<String>,
    pub target: String,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct TranslateResponse {
    pub translations: Vec<String>,
}

fn hash(text: &str) -> String {
    hex::encode(Sha256::digest(text.as_bytes()))
}

fn with_key(state: &AppState, mut body: Value) -> Value {
    if let Some(k) = &state.translate_key {
        body["api_key"] = json!(k);
    }
    body
}

/// Detect the language of `text`; `None` if the service is unavailable.
pub async fn detect(state: &AppState, text: &str) -> Option<String> {
    let sample: String = text.chars().take(1000).collect();
    let body = with_key(state, json!({ "q": sample }));
    let resp = state
        .http
        .post(format!("{}/detect", state.translate_url))
        .json(&body)
        .send()
        .await
        .ok()?
        .error_for_status()
        .ok()?;
    let v: Vec<Value> = resp.json().await.ok()?;
    v.first()?.get("language")?.as_str().map(String::from)
}

pub async fn languages(state: &AppState) -> Result<Value, AppError> {
    let resp = state
        .http
        .get(format!("{}/languages", state.translate_url))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_err(|e| AppError::Upstream(format!("translation service unavailable: {e}")))?;
    resp.json().await.map_err(|e| AppError::Upstream(e.to_string()))
}

pub async fn translate(
    state: &AppState,
    req: TranslateRequest,
) -> Result<Vec<String>, AppError> {
    if req.texts.len() > MAX_TEXTS || req.texts.iter().map(|t| t.chars().count()).sum::<usize>() > MAX_CHARS {
        return Err(AppError::BadRequest("too much text".into()));
    }
    let target = req.target.trim().to_lowercase();
    if target.is_empty() || target.len() > 8 {
        return Err(AppError::BadRequest("invalid target language".into()));
    }
    let source = req.source.filter(|s| !s.is_empty()).unwrap_or_else(|| "auto".into());
    if source == target {
        return Ok(req.texts);
    }

    let hashes: Vec<String> = req.texts.iter().map(|t| hash(t)).collect();
    let cached: Vec<(String, String)> = sqlx::query_as(
        "SELECT src_hash, output FROM translations WHERE target = $1 AND src_hash = ANY($2)",
    )
    .bind(&target)
    .bind(&hashes)
    .fetch_all(&state.db)
    .await?;
    let mut found: HashMap<String, String> = cached.into_iter().collect();

    // unique, non-empty strings not yet cached
    let mut missing: Vec<(&String, &String)> = Vec::new();
    for (t, h) in req.texts.iter().zip(&hashes) {
        if !t.trim().is_empty()
            && !found.contains_key(h)
            && !missing.iter().any(|(_, mh)| *mh == h)
        {
            missing.push((t, h));
        }
    }

    if !missing.is_empty() {
        let q: Vec<&String> = missing.iter().map(|(t, _)| *t).collect();
        let body = with_key(
            state,
            json!({ "q": q, "source": source, "target": target, "format": "text" }),
        );
        let resp = state
            .http
            .post(format!("{}/translate", state.translate_url))
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::Upstream(format!("translation service unavailable: {e}")))?;
        if !resp.status().is_success() {
            let msg = resp.text().await.unwrap_or_default();
            return Err(AppError::Upstream(format!("translation failed: {msg}")));
        }
        let v: Value = resp.json().await.map_err(|e| AppError::Upstream(e.to_string()))?;
        let out: Vec<String> = v
            .get("translatedText")
            .and_then(|t| t.as_array())
            .map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect())
            .ok_or_else(|| AppError::Upstream("unexpected translation response".into()))?;
        if out.len() != missing.len() {
            return Err(AppError::Upstream("translation count mismatch".into()));
        }
        for ((_, h), translated) in missing.iter().zip(out) {
            sqlx::query(
                "INSERT INTO translations (src_hash, target, output) VALUES ($1, $2, $3)
                 ON CONFLICT DO NOTHING",
            )
            .bind(h.as_str())
            .bind(&target)
            .bind(&translated)
            .execute(&state.db)
            .await?;
            found.insert((*h).clone(), translated);
        }
    }

    Ok(req
        .texts
        .into_iter()
        .zip(hashes)
        .map(|(t, h)| found.get(&h).cloned().unwrap_or(t))
        .collect())
}
