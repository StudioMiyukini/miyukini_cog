//! Couche d'inférence — abstraction qui route vers le backend natif ou upstream.
//!
//! Architecture :
//! - `NativeBackend` : inférence GGUF locale via llama-cpp-2 (moteur principal)
//! - `UpstreamBackend` : proxy HTTP vers LM Studio / Ollama (backup)
//! - `InferenceRouter` : choisit automatiquement le meilleur backend disponible

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::data_array::LlamaTokenDataArray;
use llama_cpp_2::token::LlamaToken;

use crate::config::NativeInferenceConfig;
use crate::model_manager::ModelManager;

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
    Native,
    Upstream,
}

/// Erreur d'inférence.
#[derive(Debug)]
pub enum InferenceError {
    /// Aucun modèle chargé nativement.
    NoModelLoaded,
    /// Erreur du moteur llama.cpp.
    LlamaError(String),
    /// Erreur réseau vers l'upstream.
    UpstreamError(String),
    /// Aucun backend disponible.
    NoBackendAvailable,
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle GGUF chargé"),
            Self::LlamaError(e) => write!(f, "Erreur llama.cpp : {e}"),
            Self::UpstreamError(e) => write!(f, "Erreur upstream : {e}"),
            Self::NoBackendAvailable => write!(f, "Aucun backend d'inférence disponible"),
        }
    }
}

// ── État du modèle natif chargé ────────────────────────────────────

/// Modèle GGUF chargé en mémoire.
struct LoadedModel {
    model: LlamaModel,
    model_path: PathBuf,
    model_name: String,
}

/// État interne partagé du NativeBackend.
struct NativeState {
    backend: LlamaBackend,
    loaded: Option<LoadedModel>,
    config: NativeInferenceConfig,
}

// ── NativeBackend ──────────────────────────────────────────────────

/// Backend d'inférence natif via llama-cpp-2.
#[derive(Clone)]
pub struct NativeBackend {
    state: Arc<Mutex<NativeState>>,
}

impl NativeBackend {
    /// Initialise le backend llama.cpp.
    pub fn new(config: NativeInferenceConfig) -> Self {
        let backend = LlamaBackend::init().expect("Impossible d'initialiser llama.cpp");

        Self {
            state: Arc::new(Mutex::new(NativeState {
                backend,
                loaded: None,
                config,
            })),
        }
    }

    /// Charge un modèle GGUF depuis un chemin de fichier.
    pub fn load_model(&self, model_path: PathBuf) -> Result<String, InferenceError> {
        let mut state = self.state.lock().unwrap();

        let model_name = model_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        tracing::info!("Chargement du modèle GGUF : {model_name}");

        // Configurer les paramètres du modèle
        let mut model_params = LlamaModelParams::default();

        // GPU layers
        let gpu_layers = if state.config.gpu_layers < 0 {
            99 // auto = tout sur GPU si possible
        } else {
            state.config.gpu_layers as u32
        };
        model_params = model_params.with_n_gpu_layers(gpu_layers);

        let model = LlamaModel::load_from_file(&state.backend, &model_path, &model_params)
            .map_err(|e| InferenceError::LlamaError(format!("Chargement échoué : {e}")))?;

        tracing::info!("Modèle chargé : {model_name} ({gpu_layers} couches GPU)");

        state.loaded = Some(LoadedModel {
            model,
            model_path,
            model_name: model_name.clone(),
        });

        Ok(model_name)
    }

    /// Décharge le modèle actuellement chargé.
    pub fn unload_model(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        if state.loaded.is_some() {
            let name = state.loaded.as_ref().unwrap().model_name.clone();
            state.loaded = None;
            tracing::info!("Modèle déchargé : {name}");
            true
        } else {
            false
        }
    }

    /// Retourne le nom du modèle actuellement chargé.
    pub fn loaded_model_name(&self) -> Option<String> {
        let state = self.state.lock().unwrap();
        state.loaded.as_ref().map(|l| l.model_name.clone())
    }

    /// Vérifie si un modèle est chargé.
    pub fn is_loaded(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.loaded.is_some()
    }

    /// Exécute une complétion de chat sur le modèle chargé.
    /// Cette fonction est bloquante — elle doit être appelée dans spawn_blocking.
    pub fn chat_completion_blocking(
        &self,
        messages: &[serde_json::Value],
        temperature: f32,
        max_tokens: u32,
    ) -> Result<(String, u32, u32), InferenceError> {
        let state = self.state.lock().unwrap();

        let loaded = state.loaded.as_ref().ok_or(InferenceError::NoModelLoaded)?;

        // Construire le prompt à partir des messages
        let prompt = build_chat_prompt(messages);

        // Créer le contexte
        let ctx_size = if state.config.context_size > 0 {
            state.config.context_size
        } else {
            4096
        };

        let ctx_params =
            LlamaContextParams::default().with_n_ctx(std::num::NonZeroU32::new(ctx_size));

        let mut ctx = loaded
            .model
            .new_context(&state.backend, ctx_params)
            .map_err(|e| InferenceError::LlamaError(format!("Contexte : {e}")))?;

        // Tokeniser le prompt
        let tokens = loaded
            .model
            .str_to_token(&prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| InferenceError::LlamaError(format!("Tokenisation : {e}")))?;

        let prompt_tokens = tokens.len() as u32;

        // Vérifier que le prompt ne dépasse pas le contexte
        if tokens.len() >= ctx_size as usize {
            return Err(InferenceError::LlamaError(format!(
                "Prompt trop long ({} tokens) pour le contexte ({} tokens)",
                tokens.len(),
                ctx_size
            )));
        }

        // Évaluer le prompt via add_sequence (gère automatiquement logits sur le dernier token)
        let n_batch = ctx.n_batch() as usize;
        let mut n_cur = 0i32;

        // Traiter le prompt par chunks de n_batch tokens
        while (n_cur as usize) < tokens.len() {
            let chunk_start = n_cur as usize;
            let chunk_end = std::cmp::min(chunk_start + n_batch, tokens.len());
            let chunk = &tokens[chunk_start..chunk_end];

            let mut batch = LlamaBatch::get_one(chunk)
                .map_err(|e| InferenceError::LlamaError(format!("Batch get_one : {e}")))?;

            // Ajuster les positions pour les chunks suivants
            if chunk_start > 0 {
                // get_one commence à pos 0, il faut utiliser add_sequence pour les positions correctes
                let mut batch2 = LlamaBatch::new(chunk.len(), 1);
                for (i, token) in chunk.iter().enumerate() {
                    let pos = (chunk_start + i) as i32;
                    let is_last = chunk_start + i == tokens.len() - 1;
                    batch2
                        .add(*token, pos, &[0], is_last)
                        .map_err(|e| InferenceError::LlamaError(format!("Batch add : {e}")))?;
                }
                ctx.decode(&mut batch2)
                    .map_err(|e| InferenceError::LlamaError(format!("Decode prompt : {e}")))?;
            } else {
                ctx.decode(&mut batch)
                    .map_err(|e| InferenceError::LlamaError(format!("Decode prompt : {e}")))?;
            }

            n_cur = chunk_end as i32;
        }

        // Configurer le sampler
        let mut sampler =
            LlamaSampler::chain_simple([LlamaSampler::temp(temperature), LlamaSampler::dist(42)]);

        // Générer les tokens
        let mut output_tokens: Vec<LlamaToken> = Vec::new();
        let eos_token = loaded.model.token_eos();

        for _ in 0..max_tokens {
            // Récupérer les logits du dernier token via candidates() et sampler via apply()
            let mut candidates = ctx.token_data_array();
            sampler.apply(&mut candidates);

            let token = candidates
                .selected_token()
                .ok_or_else(|| InferenceError::LlamaError("Aucun token sélectionné".into()))?;

            sampler.accept(token);

            // Vérifier si c'est un token de fin
            if token == eos_token {
                break;
            }

            output_tokens.push(token);

            // Préparer le batch pour le token suivant
            let mut batch = LlamaBatch::new(1, 1);
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| InferenceError::LlamaError(format!("Batch add gen : {e}")))?;

            ctx.decode(&mut batch)
                .map_err(|e| InferenceError::LlamaError(format!("Decode gen : {e}")))?;

            n_cur += 1;
        }

        // Détokeniser la sortie
        let output_text: String = output_tokens
            .iter()
            .filter_map(|t| {
                loaded
                    .model
                    .token_to_piece_bytes(*t, 32, true, None)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())
            })
            .collect();

        let completion_tokens = output_tokens.len() as u32;

        Ok((output_text, prompt_tokens, completion_tokens))
    }
}

// ── UpstreamBackend ────────────────────────────────────────────────

/// Backend HTTP upstream (LM Studio, Ollama, etc.).
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

    /// Vérifie si l'upstream est joignable.
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

    /// Envoie une requête de chat completion à l'upstream.
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

/// Routeur d'inférence — essaie le backend natif d'abord, puis l'upstream.
#[derive(Clone)]
pub struct InferenceRouter {
    pub native: NativeBackend,
    pub upstream: UpstreamBackend,
    prefer_native: bool,
}

impl InferenceRouter {
    pub fn new(native: NativeBackend, upstream: UpstreamBackend, prefer_native: bool) -> Self {
        Self {
            native,
            upstream,
            prefer_native,
        }
    }

    /// Exécute une chat completion en routant vers le meilleur backend.
    /// Essaie natif d'abord (si préféré et chargé), puis upstream.
    pub async fn chat_completion(
        &self,
        body: serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        // 1. Essayer le backend natif si préféré et un modèle est chargé
        if self.prefer_native && self.native.is_loaded() {
            match self.try_native(&body).await {
                Ok(resp) => return Ok(resp),
                Err(e) => {
                    tracing::warn!("Inférence native échouée, fallback upstream : {e}");
                }
            }
        }

        // 2. Essayer l'upstream (backup)
        if self.upstream.is_available().await {
            return self.try_upstream(&body).await;
        }

        // 3. Si native non préféré mais disponible, essayer quand même
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

    /// Exécute sur le backend natif (dans un thread bloquant).
    async fn try_native(
        &self,
        body: &serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        let messages = body
            .get("messages")
            .and_then(|m| m.as_array())
            .cloned()
            .unwrap_or_default();

        let temperature = body
            .get("temperature")
            .and_then(|t| t.as_f64())
            .unwrap_or(0.7) as f32;

        let max_tokens = body
            .get("max_tokens")
            .and_then(|t| t.as_u64())
            .unwrap_or(2048) as u32;

        let model_name = self
            .native
            .loaded_model_name()
            .unwrap_or_else(|| "native".into());

        // Exécuter l'inférence dans un thread bloquant (llama-cpp est sync)
        let native = self.native.clone();
        let (content, prompt_tokens, completion_tokens) = tokio::task::spawn_blocking(move || {
            native.chat_completion_blocking(&messages, temperature, max_tokens)
        })
        .await
        .map_err(|e| InferenceError::LlamaError(format!("Task join : {e}")))??;

        Ok(ChatCompletionResponse {
            id: format!("native-{}", uuid_simple()),
            model: model_name,
            choices: vec![Choice {
                index: 0,
                message: serde_json::json!({
                    "role": "assistant",
                    "content": content,
                }),
                finish_reason: "stop".into(),
            }],
            usage: Usage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
            },
            backend: BackendKind::Native,
        })
    }

    /// Exécute sur le backend upstream.
    async fn try_upstream(
        &self,
        body: &serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        let raw = self.upstream.chat_completion(body).await?;

        // Extraire les champs de la réponse upstream
        let model = raw
            .get("model")
            .and_then(|m| m.as_str())
            .unwrap_or("upstream")
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
                .unwrap_or("upstream")
                .to_string(),
            model,
            choices,
            usage: Usage {
                prompt_tokens,
                completion_tokens,
                total_tokens: prompt_tokens + completion_tokens,
            },
            backend: BackendKind::Upstream,
        })
    }
}

// ── Helpers ────────────────────────────────────────────────────────

/// Construit un prompt de chat à partir des messages (format ChatML simplifié).
fn build_chat_prompt(messages: &[serde_json::Value]) -> String {
    let mut prompt = String::new();

    for msg in messages {
        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("user");
        let content = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");

        match role {
            "system" => {
                prompt.push_str("<|im_start|>system\n");
                prompt.push_str(content);
                prompt.push_str("<|im_end|>\n");
            }
            "user" => {
                prompt.push_str("<|im_start|>user\n");
                prompt.push_str(content);
                prompt.push_str("<|im_end|>\n");
            }
            "assistant" => {
                prompt.push_str("<|im_start|>assistant\n");
                prompt.push_str(content);
                prompt.push_str("<|im_end|>\n");
            }
            "tool" => {
                prompt.push_str("<|im_start|>tool\n");
                prompt.push_str(content);
                prompt.push_str("<|im_end|>\n");
            }
            _ => {
                prompt.push_str(&format!("<|im_start|>{role}\n"));
                prompt.push_str(content);
                prompt.push_str("<|im_end|>\n");
            }
        }
    }

    // Amorcer la réponse de l'assistant
    prompt.push_str("<|im_start|>assistant\n");
    prompt
}

/// UUID simple pour les ID de requête.
fn uuid_simple() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    format!("{ts:x}")
}
