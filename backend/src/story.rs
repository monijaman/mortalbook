use std::time::Duration;

use axum::{extract::State, http::HeaderMap, Json};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::{error::AppError, routes::require_admin, AppState};

const MAX_PROMPT_CHARS: usize = 2_000;
const MAX_CONTEXT_CHARS: usize = 6_000;

#[derive(Deserialize)]
pub struct StoryRequest {
    name: String,
    birth_date: Option<NaiveDate>,
    death_date: Option<NaiveDate>,
    occupation: Option<String>,
    birth_place: Option<String>,
    death_place: Option<String>,
    /// Optional extra instructions from the admin.
    prompt: Option<String>,
    /// Existing story, so the model can build on it rather than contradict it.
    current_story: Option<String>,
}

const INSTRUCTIONS: &str = "You write memorial biographies for Mortalbook, a site that remembers \
people who have passed away. Search the web for reliable sources about the person, then write a \
respectful, accurate, neutral life story in 3 to 6 paragraphs (about 250 to 500 words unless the \
admin asks otherwise). Use only facts supported by the sources you found; if sources disagree or \
a detail is uncertain, leave it out or say it is reported. Never invent quotes, dates or \
relatives. If you cannot confidently identify the person, say so briefly instead of guessing. \
Format: plain Markdown only. Separate paragraphs with a blank line. You may use **bold**, \
*italic*, '## ' headings, '- ' bullet lists and '> ' quotes sparingly. Do NOT include links, \
citations, footnotes, a title line or a sources list in the text. Write in the language the admin \
asks for, otherwise in English.";

fn clean(value: &Option<String>, max: usize) -> Option<String> {
    value
        .as_deref()
        .map(str::trim)
        .filter(|v| !v.is_empty())
        .map(|v| v.chars().take(max).collect())
}

pub async fn generate_story(
    State(s): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<StoryRequest>,
) -> Result<Json<Value>, AppError> {
    require_admin(&headers, &s, true).await?;
    let Some(api_key) = s.openai_api_key.as_deref() else {
        return Err(AppError::ServiceUnavailable(
            "story generation is not configured (set OPENAI_API_KEY)".into(),
        ));
    };
    let name = req.name.trim();
    if name.is_empty() || name.chars().count() > 200 {
        return Err(AppError::BadRequest("name is required (max 200 characters)".into()));
    }

    let mut facts = vec![format!("Name: {name}")];
    if let Some(d) = req.birth_date {
        facts.push(format!("Born: {d}"));
    }
    if let Some(d) = req.death_date {
        facts.push(format!("Died: {d}"));
    }
    for (label, value) in [
        ("Occupation", &req.occupation),
        ("Birth place", &req.birth_place),
        ("Death place", &req.death_place),
    ] {
        if let Some(v) = clean(value, 300) {
            facts.push(format!("{label}: {v}"));
        }
    }
    let mut input = format!("Person (known details):\n{}\n", facts.join("\n"));
    if let Some(story) = clean(&req.current_story, MAX_CONTEXT_CHARS) {
        input.push_str(&format!("\nExisting draft story (improve or extend it, keep it consistent):\n{story}\n"));
    }
    if let Some(prompt) = clean(&req.prompt, MAX_PROMPT_CHARS) {
        input.push_str(&format!("\nAdmin instructions:\n{prompt}\n"));
    }

    let response = s
        .http
        .post("https://api.openai.com/v1/responses")
        .bearer_auth(api_key)
        .timeout(Duration::from_secs(240))
        .json(&json!({
            "model": s.openai_model,
            "instructions": INSTRUCTIONS,
            "input": input,
            "tools": [{ "type": "web_search" }],
            "tool_choice": "auto",
        }))
        .send()
        .await
        .map_err(|e| AppError::Upstream(format!("could not reach OpenAI: {e}")))?;

    let status = response.status();
    let body: Value = response
        .json()
        .await
        .map_err(|e| AppError::Upstream(format!("unreadable OpenAI response: {e}")))?;
    if !status.is_success() {
        let message = body["error"]["message"].as_str().unwrap_or("request failed");
        tracing::warn!("openai error {status}: {message}");
        return Err(AppError::Upstream(format!("OpenAI error: {message}")));
    }

    let mut story = String::new();
    let mut sources: Vec<Value> = Vec::new();
    for item in body["output"].as_array().into_iter().flatten() {
        if item["type"] != "message" {
            continue;
        }
        for part in item["content"].as_array().into_iter().flatten() {
            if part["type"] != "output_text" {
                continue;
            }
            story.push_str(part["text"].as_str().unwrap_or(""));
            for note in part["annotations"].as_array().into_iter().flatten() {
                let Some(url) = note["url"].as_str() else { continue };
                let url = url.split("?utm_source").next().unwrap_or(url);
                if !(url.starts_with("https://") || url.starts_with("http://"))
                    || sources.iter().any(|s| s["url"] == url)
                {
                    continue;
                }
                let title = note["title"].as_str().filter(|t| !t.is_empty()).unwrap_or(url);
                sources.push(json!({ "title": title, "url": url }));
            }
        }
    }
    let story = story.trim().to_string();
    if story.is_empty() {
        return Err(AppError::Upstream("OpenAI returned no story, please try again".into()));
    }
    Ok(Json(json!({ "story": story, "sources": sources })))
}
