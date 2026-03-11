//! Couche d'inférence — abstraction qui route vers maia-llm ou l'upstream.
//!
//! Architecture :
//! - `MaiaLlmClient` : client HTTP vers le service maia-llm (GGUF, local)
//! - `UpstreamBackend` : proxy HTTP vers LM Studio / Ollama (backup)
//! - `InferenceRouter` : choisit automatiquement le meilleur backend

use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

// ── Types de requête / réponse (format OpenAI-compatible) ──────────

/// Message dans une conversation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

/// Requête de chat completion.
#[derive(Debug, Clone, Deserialize)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<serde_json::Value>,
    #[serde(default = "default_temperature")]
    pub temperature: f64,
    #[serde(default = "default_max_tokens")]
    pub max_tokens: u32,
    #[serde(default)]
    pub tools: Option<serde_json::Value>,
    #[serde(default)]
    pub tool_choice: Option<String>,
    #[serde(default)]
    pub stop: Option<Vec<String>>,
}

fn default_temperature() -> f64 {
    0.7
}
fn default_max_tokens() -> u32 {
    2048
}

/// Réponse de chat completion (format simplifié OpenAI).
#[derive(Debug, Clone, Serialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
    /// Quel backend a traité la requête.
    pub backend: BackendKind,
}

#[derive(Debug, Clone, Serialize)]
pub struct Choice {
    pub index: u32,
    pub message: serde_json::Value,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// Type de backend qui a traité la requête.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
    /// maia-llm (inférence GGUF locale).
    Native,
    /// LM Studio / Ollama (backup).
    Upstream,
}

/// Erreur d'inférence.
#[derive(Debug)]
pub enum InferenceError {
    NoModelLoaded,
    LlamaError(String),
    UpstreamError(String),
    NoBackendAvailable,
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle chargé dans maia-llm"),
            Self::LlamaError(e) => write!(f, "Erreur maia-llm : {e}"),
            Self::UpstreamError(e) => write!(f, "Erreur upstream : {e}"),
            Self::NoBackendAvailable => write!(f, "Aucun backend d'inférence disponible"),
        }
    }
}

// ── MaiaLlmClient ──────────────────────────────────────────────────

/// Client HTTP vers le service maia-llm (remplace NativeBackend llama-cpp-2).
/// Maintient un cache local du modèle chargé pour les appels synchrones.
#[derive(Clone)]
pub struct MaiaLlmClient {
    pub base_url: String,
    auth_key: Option<String>,
    client: reqwest::Client,
    /// Cache : nom du modèle actuellement chargé dans maia-llm (None = aucun).
    loaded_model: Arc<RwLock<Option<String>>>,
}

impl MaiaLlmClient {
    pub fn new(base_url: String, auth_key: Option<String>, client: reqwest::Client) -> Self {
        Self {
            base_url,
            auth_key,
            client,
            loaded_model: Arc::new(RwLock::new(None)),
        }
    }

    // ── Méthodes synchrones (cache local) ──────────────────────────

    /// Vérifie si un modèle est chargé (via cache local).
    pub fn is_loaded(&self) -> bool {
        self.loaded_model.read().unwrap().is_some()
    }

    /// Retourne le nom du modèle chargé (via cache local).
    pub fn loaded_model_name(&self) -> Option<String> {
        self.loaded_model.read().unwrap().clone()
    }

    // ── Méthodes async (appels HTTP maia-llm) ─────────────────────

    /// Vérifie si le service maia-llm est joignable et met à jour le cache.
    pub async fn is_available(&self) -> bool {
        let url = format!("{}/maia/llm/health", self.base_url);
        match self
            .client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => {
                if let Ok(body) = r.json::<serde_json::Value>().await {
                    let model_name = body
                        .get("model_name")
                        .and_then(|m| m.as_str())
                        .map(|s| s.to_string());
                    *self.loaded_model.write().unwrap() = model_name;
                }
                true
            }
            _ => false,
        }
    }

    /// Charge un modèle dans maia-llm via POST /maia/llm/models/load.
    pub async fn load_model(
        &self,
        path: std::path::PathBuf,
    ) -> Result<String, InferenceError> {
        let filename = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();

        let url = format!("{}/maia/llm/models/load", self.base_url);
        let body = serde_json::json!({ "model": filename });

        let mut req = self.client.post(&url).json(&body);
        if let Some(key) = &self.auth_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }

        let resp = req
            .timeout(std::time::Duration::from_secs(120))
            .send()
            .await
            .map_err(|e| InferenceError::LlamaError(format!("Connexion maia-llm : {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(InferenceError::LlamaError(format!(
                "maia-llm load HTTP {status}: {text}"
            )));
        }

        let json: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| InferenceError::LlamaError(format!("Parse réponse : {e}")))?;

        let model_name = json
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or(&filename)
            .to_string();

        *self.loaded_model.write().unwrap() = Some(model_name.clone());

        Ok(model_name)
    }

    /// Décharge le modèle actuel dans maia-llm via DELETE /maia/llm/models/unload.
    pub async fn unload_model(&self) -> bool {
        let url = format!("{}/maia/llm/models/unload", self.base_url);
        let mut req = self.client.delete(&url);
        if let Some(key) = &self.auth_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }

        let success = req
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false);

        if success {
            *self.loaded_model.write().unwrap() = None;
        }

        success
    }

    /// Liste les modèles GGUF disponibles sur maia-llm (GET /maia/llm/models).
    /// Retourne les `model_id` — noms sans extension pour les requêtes API.
    pub async fn available_models(&self) -> Vec<String> {
        let url = format!("{}/maia/llm/models", self.base_url);
        let mut req = self.client.get(&url).timeout(std::time::Duration::from_secs(10));
        if let Some(key) = &self.auth_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }
        let resp = match req.send().await {
            Ok(r) if r.status().is_success() => r,
            _ => return Vec::new(),
        };
        #[derive(serde::Deserialize)]
        struct ModelEntry {
            model_id: String,
        }
        resp.json::<Vec<ModelEntry>>()
            .await
            .map(|v| v.into_iter().map(|m| m.model_id).collect())
            .unwrap_or_default()
    }

    /// Liste les modèles chargés via GET /v1/models (OpenAI-compat) de maia-llm.
    pub async fn loaded_models(&self) -> Vec<String> {
        let url = format!("{}/v1/models", self.base_url);
        let mut req = self.client.get(&url).timeout(std::time::Duration::from_secs(5));
        if let Some(key) = &self.auth_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }
        let resp = match req.send().await {
            Ok(r) if r.status().is_success() => r,
            _ => return Vec::new(),
        };
        #[derive(serde::Deserialize)]
        struct ModelEntry {
            id: String,
        }
        #[derive(serde::Deserialize)]
        struct ModelsResponse {
            #[serde(default)]
            data: Vec<ModelEntry>,
        }
        resp.json::<ModelsResponse>()
            .await
            .map(|r| r.data.into_iter().map(|m| m.id).collect())
            .unwrap_or_default()
    }

    /// Envoie une requête de chat completion à maia-llm.
    pub async fn chat_completion(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, InferenceError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let mut req = self.client.post(&url).json(body);
        if let Some(key) = &self.auth_key {
            req = req.header("Authorization", format!("Bearer {key}"));
        }

        let resp = req
            .send()
            .await
            .map_err(|e| InferenceError::LlamaError(format!("Connexion maia-llm : {e}")))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(InferenceError::LlamaError(format!(
                "maia-llm HTTP {status}: {text}"
            )));
        }

        resp.json()
            .await
            .map_err(|e| InferenceError::LlamaError(format!("Parse JSON : {e}")))
    }
}

// ── UpstreamBackend ────────────────────────────────────────────────

/// Backend HTTP upstream (LM Studio, Ollama, etc.) — backup si maia-llm indisponible.
#[derive(Clone)]
pub struct UpstreamBackend {
    client: reqwest::Client,
    upstream_url: String,
}

impl UpstreamBackend {
    pub fn new(client: reqwest::Client, upstream_url: String) -> Self {
        Self {
            client,
            upstream_url,
        }
    }

    pub async fn is_available(&self) -> bool {
        let url = format!("{}/v1/models", self.upstream_url);
        self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    pub async fn chat_completion(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, InferenceError> {
        let url = format!("{}/v1/chat/completions", self.upstream_url);

        let resp = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(|e| InferenceError::UpstreamError(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(InferenceError::UpstreamError(format!(
                "HTTP {status}: {text}"
            )));
        }

        resp.json()
            .await
            .map_err(|e| InferenceError::UpstreamError(format!("Parse JSON : {e}")))
    }

    pub fn upstream_url(&self) -> &str {
        &self.upstream_url
    }
}

// ── InferenceRouter ────────────────────────────────────────────────

/// Routeur d'inférence — essaie maia-llm d'abord, puis l'upstream.
#[derive(Clone)]
pub struct InferenceRouter {
    /// Backend principal : maia-llm (GGUF local).
    pub native: MaiaLlmClient,
    /// Backend backup : LM Studio / Ollama.
    pub upstream: UpstreamBackend,
    prefer_native: bool,
}

impl InferenceRouter {
    pub fn new(native: MaiaLlmClient, upstream: UpstreamBackend, prefer_native: bool) -> Self {
        Self {
            native,
            upstream,
            prefer_native,
        }
    }

    /// Exécute une chat completion en routant vers le meilleur backend.
    pub async fn chat_completion(
        &self,
        body: serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        if self.prefer_native && self.native.is_loaded() {
            match self.try_native(&body).await {
                Ok(resp) => return Ok(resp),
                Err(e) => tracing::warn!("maia-llm échoué, fallback upstream : {e}"),
            }
        }

        if self.upstream.is_available().await {
            return self.try_upstream(&body).await;
        }

        if !self.prefer_native && self.native.is_loaded() {
            return self.try_native(&body).await;
        }

        Err(InferenceError::NoBackendAvailable)
    }

    /// Détermine le backend actuellement actif.
    pub async fn active_backend(&self) -> Option<BackendKind> {
        if self.prefer_native && self.native.is_loaded() {
            Some(BackendKind::Native)
        } else if self.upstream.is_available().await {
            Some(BackendKind::Upstream)
        } else if self.native.is_loaded() {
            Some(BackendKind::Native)
        } else {
            None
        }
    }

    async fn try_native(
        &self,
        body: &serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        let raw = self.native.chat_completion(body).await?;
        parse_response(raw, BackendKind::Native)
    }

    async fn try_upstream(
        &self,
        body: &serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        let raw = self.upstream.chat_completion(body).await?;
        parse_response(raw, BackendKind::Upstream)
    }
}

// ── Helpers ────────────────────────────────────────────────────────

fn parse_response(
    raw: serde_json::Value,
    backend: BackendKind,
) -> Result<ChatCompletionResponse, InferenceError> {
    let model = raw
        .get("model")
        .and_then(|m| m.as_str())
        .unwrap_or("unknown")
        .to_string();

    let choices: Vec<Choice> = raw
        .get("choices")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .enumerate()
                .map(|(i, c)| Choice {
                    index: i as u32,
                    message: c.get("message").cloned().unwrap_or(serde_json::json!({})),
                    finish_reason: c
                        .get("finish_reason")
                        .and_then(|f| f.as_str())
                        .unwrap_or("stop")
                        .to_string(),
                })
                .collect()
        })
        .unwrap_or_default();

    let usage = raw.get("usage").cloned().unwrap_or(serde_json::json!({}));
    let prompt_tokens = usage
        .get("prompt_tokens")
        .and_then(|t| t.as_u64())
        .unwrap_or(0) as u32;
    let completion_tokens = usage
        .get("completion_tokens")
        .and_then(|t| t.as_u64())
        .unwrap_or(0) as u32;

    Ok(ChatCompletionResponse {
        id: raw
            .get("id")
            .and_then(|i| i.as_str())
            .unwrap_or("maia")
            .to_string(),
        model,
        choices,
        usage: Usage {
            prompt_tokens,
            completion_tokens,
            total_tokens: prompt_tokens + completion_tokens,
        },
        backend,
    })
}
