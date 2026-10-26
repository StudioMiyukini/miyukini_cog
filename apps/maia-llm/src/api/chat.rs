//! @id     maia_llm_api_chat
//! @do     definir_chat_request
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — module « api chat ».

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

use crate::engine::{Generation, LlmError, StreamMsg};
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
    /// Grammaire GBNF (racine `root`) contraignant la sortie (compatible
    /// llama.cpp server) ; bornée à [`crate::engine::GRAMMAIRE_MAX`] octets.
    #[serde(default)]
    pub grammar: Option<String>,
    /// Graine de l'échantillonneur (défaut 42).
    #[serde(default)]
    pub seed: Option<u32>,
}

impl ChatRequest {
    /// Paramètres de génération du moteur.
    pub fn generation(&self) -> Generation {
        Generation {
            temperature: self.temperature as f32,
            max_tokens: self.max_tokens.clamp(1, MAX_TOKENS_PLAFOND),
            graine: self.seed.unwrap_or(42),
            grammaire: self.grammar.clone(),
        }
    }
}

/// Plafond de jetons générés par requête (C-13).
const MAX_TOKENS_PLAFOND: u32 = 4096;

/// Marquage des contenus générés (AI Act art. 50 §2) : toute réponse de
/// maia-llm est un contenu produit par une IA, signalé de façon lisible par machine.
pub fn marquage_ia(model: &str) -> serde_json::Value {
    json!({ "genere_par_ia": true, "moteur": "maia-llm", "modele": model })
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
    /// Marquage IA lisible par machine (AI Act art. 50 §2).
    x_miyukini: serde_json::Value,
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
// Noms de champs imposés par le format `usage` OpenAI.
#[allow(clippy::struct_field_names)]
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
        .model_info().map_or_else(|| "unknown".into(), |m| m.name);

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
    let generation = req.generation();
    let messages = req.messages;

    let result = tokio::task::spawn_blocking(move || {
        state.engine.infer_blocking(&messages, &generation)
    })
    .await;

    match result {
        Ok(Ok((content, prompt_tokens, completion_tokens))) => {
            let resp = ChatResponse {
                x_miyukini: marquage_ia(&model_name),
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
        Ok(Err(e @ LlmError::ParametresInvalides(_))) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": {"message": e.to_string(), "type": "invalid_request_error"}})),
        )
            .into_response(),
        Ok(Err(e)) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": {"message": e.to_string(), "type": "inference_error"}})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": {"message": format!("Task join : {e}"), "type": "inference_error"}})),
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
    let generation = req.generation();
    let messages = req.messages;

    let rx = match state.engine.chat_stream(messages, generation).await {
        Ok(rx) => rx,
        Err(e) => {
            let statut = if matches!(e, LlmError::ParametresInvalides(_)) {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::SERVICE_UNAVAILABLE
            };
            return (
                statut,
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
                        "x_miyukini": marquage_ia(&model_name),
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
                        "x_miyukini": marquage_ia(&model_name),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requete_avec_grammaire_et_bornes() {
        let r: ChatRequest = serde_json::from_value(json!({
            "messages": [],
            "temperature": 0.0,
            "max_tokens": 100000,
            "grammar": "root ::= \"a\"",
            "seed": 7
        }))
        .unwrap();
        let g = r.generation();
        assert_eq!(g.max_tokens, MAX_TOKENS_PLAFOND);
        assert_eq!(g.graine, 7);
        assert_eq!(g.grammaire.as_deref(), Some("root ::= \"a\""));
        assert!(g.temperature <= 0.0);
    }

    #[test]
    fn reponse_marquee_comme_generee_par_ia() {
        let m = marquage_ia("qwen");
        assert_eq!(m["genere_par_ia"], true);
        assert_eq!(m["modele"], "qwen");
    }
}
