//! POST /v1/chat/completions — inférence normal + streaming SSE.
//!
//! Compatible avec l'API OpenAI (format messages, stream flag, temperature, max_tokens).

use axum::{
    extract::State,
    http::StatusCode,
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    Json,
};
use futures::Stream;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::convert::Infallible;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::engine::StreamMsg;
use crate::AppState;

// ── Types requête ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct ChatRequest {
    #[allow(dead_code)]
    pub model: Option<String>,
    pub messages: Vec<serde_json::Value>,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub stream: bool,
}

fn default_temperature() -> f64 {
    0.7
}
fn default_max_tokens() -> u32 {
    2048
}

// ── Types réponse non-streaming ───────────────────────────────────

#[derive(Serialize)]
struct ChatResponse {
    id: String,
    object: &'static str,
    created: u64,
    model: String,
    choices: Vec<ChatChoice>,
    usage: UsageInfo,
}

#[derive(Serialize)]
struct ChatChoice {
    index: u32,
    message: serde_json::Value,
    finish_reason: &'static str,
}

#[derive(Serialize)]
struct UsageInfo {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

// ── Handler principal ─────────────────────────────────────────────

pub async fn chat_handler(
    State(state): State<AppState>,
    Json(req): Json<ChatRequest>,
) -> Response {
    if !state.engine.is_loaded() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"error": {"message": "Aucun modèle chargé", "type": "model_not_loaded"}})),
        )
            .into_response();
    }

    let model_name = state
        .engine
        .model_info()
        .map(|m| m.name)
        .unwrap_or_else(|| "unknown".into());

    if req.stream {
        handle_stream(state, req, model_name).await
    } else {
        handle_blocking(state, req, model_name).await
    }
}

// ── Inférence bloquante ───────────────────────────────────────────

async fn handle_blocking(
    state: AppState,
    req: ChatRequest,
    model_name: String,
) -> Response {
    let temperature = req.temperature as f32;
    let max_tokens = req.max_tokens;
    let messages = req.messages;

    let result = tokio::task::spawn_blocking(move || {
        state.engine.infer_blocking(&messages, temperature, max_tokens)
    })
    .await
    .map_err(|e| format!("Task join : {e}"))
    .and_then(|r| r.map_err(|e| e.to_string()));

    match result {
        Ok((content, prompt_tokens, completion_tokens)) => {
            let resp = ChatResponse {
                id: gen_id("chatcmpl"),
                object: "chat.completion",
                created: unix_ts(),
                model: model_name,
                choices: vec![ChatChoice {
                    index: 0,
                    message: json!({"role": "assistant", "content": content}),
                    finish_reason: "stop",
                }],
                usage: UsageInfo {
                    prompt_tokens,
                    completion_tokens,
                    total_tokens: prompt_tokens + completion_tokens,
                },
            };
            Json(resp).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": {"message": e, "type": "inference_error"}})),
        )
            .into_response(),
    }
}

// ── Streaming SSE ─────────────────────────────────────────────────

async fn handle_stream(
    state: AppState,
    req: ChatRequest,
    model_name: String,
) -> Response {
    let temperature = req.temperature as f32;
    let max_tokens = req.max_tokens;
    let messages = req.messages;

    let rx = match state
        .engine
        .chat_stream(messages, temperature, max_tokens)
        .await
    {
        Ok(rx) => rx,
        Err(e) => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error": {"message": e.to_string(), "type": "inference_error"}})),
            )
                .into_response();
        }
    };

    let req_id = gen_id("chatcmpl");
    let created = unix_ts();
    let stream = build_sse_stream(rx, req_id, model_name, created);

    Sse::new(stream)
        .keep_alive(KeepAlive::default())
        .into_response()
}

/// Construit le Stream<Item = Result<Event, Infallible>> depuis le Receiver<StreamMsg>.
fn build_sse_stream(
    mut rx: tokio::sync::mpsc::Receiver<StreamMsg>,
    req_id: String,
    model_name: String,
    created: u64,
) -> impl Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        loop {
            match rx.recv().await {
                Some(StreamMsg::Token(piece)) => {
                    let chunk = json!({
                        "id": req_id,
                        "object": "chat.completion.chunk",
                        "created": created,
                        "model": model_name,
                        "choices": [{
                            "index": 0,
                            "delta": {"content": piece},
                            "finish_reason": null
                        }]
                    });
                    yield Ok(Event::default().data(chunk.to_string()));
                }
                Some(StreamMsg::Done(_, _)) | None => {
                    // Chunk de fin
                    let finish_chunk = json!({
                        "id": req_id,
                        "object": "chat.completion.chunk",
                        "created": created,
                        "model": model_name,
                        "choices": [{
                            "index": 0,
                            "delta": {},
                            "finish_reason": "stop"
                        }]
                    });
                    yield Ok(Event::default().data(finish_chunk.to_string()));
                    yield Ok(Event::default().data("[DONE]"));
                    break;
                }
                Some(StreamMsg::Error(e)) => {
                    let err_chunk = json!({"error": {"message": e, "type": "inference_error"}});
                    yield Ok(Event::default().data(err_chunk.to_string()));
                    break;
                }
            }
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────

fn unix_ts() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn gen_id(prefix: &str) -> String {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{prefix}-{ts:x}")
}
