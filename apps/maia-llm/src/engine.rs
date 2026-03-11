//! Moteur d'inférence GGUF — wrapper Rust autour de llama.cpp via llama-cpp-2.
//!
//! Architecture :
//! - `LlmEngine` : état partagé Arc<Mutex<LlmState>>, thread-safe
//! - `load_model` / `unload` : chargement/déchargement GGUF
//! - `chat_completion` : inférence bloquante (spawn_blocking)
//! - `chat_stream` : streaming SSE token-par-token via mpsc channel
//!
//! Seul C conservé : llama.cpp (moteur tenseur GPU — irremplaçable en pur Rust pour GGUF).

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::MaiaLlmConfig;

// ── Types publics ─────────────────────────────────────────────────

/// Message envoyé dans le channel de streaming.
pub enum StreamMsg {
    /// Fragment de texte généré.
    Token(String),
    /// Fin de génération — prompt_tokens + completion_tokens.
    Done(u32, u32),
    /// Erreur pendant la génération.
    Error(String),
}

/// Erreur du moteur LLM.
#[derive(Debug)]
pub enum LlmError {
    NoModelLoaded,
    LlamaError(String),
    AlreadyRunning,
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle GGUF chargé"),
            Self::LlamaError(e) => write!(f, "Erreur llama.cpp : {e}"),
            Self::AlreadyRunning => write!(f, "Inférence déjà en cours"),
        }
    }
}

/// Informations sur le modèle actuellement chargé.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub gpu_layers: u32,
    pub context_size: u32,
}

// ── État interne ──────────────────────────────────────────────────

struct LoadedModel {
    model: LlamaModel,
    info: ModelInfo,
}

struct LlmState {
    backend: LlamaBackend,
    loaded: Option<LoadedModel>,
    config: MaiaLlmConfig,
}

// ── LlmEngine ─────────────────────────────────────────────────────

/// Moteur d'inférence GGUF partagé et thread-safe.
#[derive(Clone)]
pub struct LlmEngine {
    state: Arc<Mutex<LlmState>>,
}

impl LlmEngine {
    /// Initialise le moteur llama.cpp.
    pub fn new(config: MaiaLlmConfig) -> Self {
        let backend = LlamaBackend::init().expect("Impossible d'initialiser llama.cpp");
        Self {
            state: Arc::new(Mutex::new(LlmState {
                backend,
                loaded: None,
                config,
            })),
        }
    }

    // ── Gestion modèle ────────────────────────────────────────────

    /// Charge un modèle GGUF depuis le chemin donné.
    /// Décharge l'éventuel modèle précédent.
    pub fn load_model(&self, model_path: PathBuf) -> Result<ModelInfo, LlmError> {
        let mut state = self.state.lock().unwrap();

        let model_name = model_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let size_bytes = std::fs::metadata(&model_path)
            .map(|m| m.len())
            .unwrap_or(0);

        tracing::info!("Chargement modèle GGUF : {model_name}");

        let gpu_layers = resolve_gpu_layers(state.config.gpu_layers);
        let mut model_params = LlamaModelParams::default();
        model_params = model_params.with_n_gpu_layers(gpu_layers);

        let model = LlamaModel::load_from_file(&state.backend, &model_path, &model_params)
            .map_err(|e| LlmError::LlamaError(format!("Chargement échoué : {e}")))?;

        let ctx_size = if state.config.context_size > 0 {
            state.config.context_size
        } else {
            4096
        };

        let info = ModelInfo {
            name: model_name.clone(),
            path: model_path.to_string_lossy().to_string(),
            size_bytes,
            gpu_layers,
            context_size: ctx_size,
        };

        state.loaded = Some(LoadedModel { model, info: info.clone() });
        tracing::info!("Modèle chargé : {model_name} ({gpu_layers} couches GPU, ctx {ctx_size})");

        Ok(info)
    }

    /// Décharge le modèle actuellement chargé.
    pub fn unload(&self) -> bool {
        let mut state = self.state.lock().unwrap();
        if let Some(m) = state.loaded.take() {
            tracing::info!("Modèle déchargé : {}", m.info.name);
            true
        } else {
            false
        }
    }

    /// Retourne les infos du modèle chargé, ou None.
    pub fn model_info(&self) -> Option<ModelInfo> {
        let state = self.state.lock().unwrap();
        state.loaded.as_ref().map(|l| l.info.clone())
    }

    /// Vérifie si un modèle est chargé.
    pub fn is_loaded(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.loaded.is_some()
    }

    // ── Inférence bloquante ───────────────────────────────────────

    /// Inférence synchrone — à appeler depuis spawn_blocking.
    /// Retourne (texte, prompt_tokens, completion_tokens).
    pub fn infer_blocking(
        &self,
        messages: &[serde_json::Value],
        temperature: f32,
        max_tokens: u32,
    ) -> Result<(String, u32, u32), LlmError> {
        let state = self.state.lock().unwrap();
        let loaded = state.loaded.as_ref().ok_or(LlmError::NoModelLoaded)?;

        let prompt = build_chat_prompt(messages);
        let ctx_size = loaded.info.context_size;

        let ctx_params =
            LlamaContextParams::default().with_n_ctx(std::num::NonZeroU32::new(ctx_size));

        let mut ctx = loaded
            .model
            .new_context(&state.backend, ctx_params)
            .map_err(|e| LlmError::LlamaError(format!("Contexte : {e}")))?;

        let tokens = loaded
            .model
            .str_to_token(&prompt, llama_cpp_2::model::AddBos::Always)
            .map_err(|e| LlmError::LlamaError(format!("Tokenisation : {e}")))?;

        if tokens.len() >= ctx_size as usize {
            return Err(LlmError::LlamaError(format!(
                "Prompt trop long ({} tokens > contexte {})",
                tokens.len(),
                ctx_size
            )));
        }

        let prompt_tokens = tokens.len() as u32;
        decode_prompt(&mut ctx, &tokens)?;

        let mut sampler =
            LlamaSampler::chain_simple([LlamaSampler::temp(temperature), LlamaSampler::dist(42)]);

        let eos = loaded.model.token_eos();
        let mut output = Vec::new();
        let mut n_cur = tokens.len() as i32;

        for _ in 0..max_tokens {
            let mut candidates = ctx.token_data_array();
            sampler.apply(&mut candidates);
            let token = candidates
                .selected_token()
                .ok_or_else(|| LlmError::LlamaError("Aucun token sélectionné".into()))?;
            sampler.accept(token);

            if token == eos {
                break;
            }

            output.push(token);

            let mut batch = LlamaBatch::new(1, 1);
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| LlmError::LlamaError(format!("Batch add : {e}")))?;
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::LlamaError(format!("Decode gen : {e}")))?;
            n_cur += 1;
        }

        let text: String = output
            .iter()
            .filter_map(|t| {
                loaded
                    .model
                    .token_to_piece_bytes(*t, 32, true, None)
                    .ok()
                    .and_then(|b| String::from_utf8(b).ok())
            })
            .collect();

        Ok((text, prompt_tokens, output.len() as u32))
    }

    // ── Streaming SSE ─────────────────────────────────────────────

    /// Lance une inférence en streaming. Retourne un Receiver qui émet les tokens
    /// au fur et à mesure de leur génération.
    ///
    /// La tâche bloquante est gérée en interne via `tokio::task::spawn_blocking`.
    pub async fn chat_stream(
        &self,
        messages: Vec<serde_json::Value>,
        temperature: f32,
        max_tokens: u32,
    ) -> Result<mpsc::Receiver<StreamMsg>, LlmError> {
        if !self.is_loaded() {
            return Err(LlmError::NoModelLoaded);
        }

        let (tx, rx) = mpsc::channel::<StreamMsg>(256);
        let engine = self.clone();

        tokio::task::spawn_blocking(move || {
            stream_inference(&engine, &messages, temperature, max_tokens, tx);
        });

        Ok(rx)
    }
}

// ── Helpers internes ─────────────────────────────────────────────

/// Résout le nombre de couches GPU depuis la config (-1 = auto = tout sur GPU).
fn resolve_gpu_layers(cfg: i32) -> u32 {
    if cfg < 0 { 99 } else { cfg as u32 }
}

/// Décode le prompt par chunks dans le contexte llama.
fn decode_prompt(
    ctx: &mut llama_cpp_2::context::LlamaContext<'_>,
    tokens: &[llama_cpp_2::token::LlamaToken],
) -> Result<(), LlmError> {
    let n_batch = ctx.n_batch() as usize;
    let mut n_cur = 0usize;

    while n_cur < tokens.len() {
        let end = std::cmp::min(n_cur + n_batch, tokens.len());
        let chunk = &tokens[n_cur..end];

        if n_cur == 0 {
            let mut batch = LlamaBatch::get_one(chunk)
                .map_err(|e| LlmError::LlamaError(format!("Batch get_one : {e}")))?;
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::LlamaError(format!("Decode prompt : {e}")))?;
        } else {
            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, token) in chunk.iter().enumerate() {
                let pos = (n_cur + i) as i32;
                let is_last = n_cur + i == tokens.len() - 1;
                batch
                    .add(*token, pos, &[0], is_last)
                    .map_err(|e| LlmError::LlamaError(format!("Batch add : {e}")))?;
            }
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::LlamaError(format!("Decode prompt chunk : {e}")))?;
        }

        n_cur = end;
    }

    Ok(())
}

/// Fonction bloquante exécutée dans spawn_blocking — génère les tokens et les envoie
/// via le channel StreamMsg.
fn stream_inference(
    engine: &LlmEngine,
    messages: &[serde_json::Value],
    temperature: f32,
    max_tokens: u32,
    tx: mpsc::Sender<StreamMsg>,
) {
    let state = engine.state.lock().unwrap();
    let loaded = match state.loaded.as_ref() {
        Some(l) => l,
        None => {
            let _ = tx.blocking_send(StreamMsg::Error("Aucun modèle chargé".into()));
            return;
        }
    };

    let prompt = build_chat_prompt(messages);
    let ctx_size = loaded.info.context_size;

    let ctx_params =
        LlamaContextParams::default().with_n_ctx(std::num::NonZeroU32::new(ctx_size));

    let mut ctx = match loaded.model.new_context(&state.backend, ctx_params) {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.blocking_send(StreamMsg::Error(format!("Contexte : {e}")));
            return;
        }
    };

    let tokens = match loaded
        .model
        .str_to_token(&prompt, llama_cpp_2::model::AddBos::Always)
    {
        Ok(t) => t,
        Err(e) => {
            let _ = tx.blocking_send(StreamMsg::Error(format!("Tokenisation : {e}")));
            return;
        }
    };

    if tokens.len() >= ctx_size as usize {
        let _ = tx.blocking_send(StreamMsg::Error(format!(
            "Prompt trop long ({} tokens)",
            tokens.len()
        )));
        return;
    }

    let prompt_tokens = tokens.len() as u32;

    if let Err(e) = decode_prompt(&mut ctx, &tokens) {
        let _ = tx.blocking_send(StreamMsg::Error(e.to_string()));
        return;
    }

    let mut sampler =
        LlamaSampler::chain_simple([LlamaSampler::temp(temperature), LlamaSampler::dist(42)]);

    let eos = loaded.model.token_eos();
    let mut completion_tokens = 0u32;
    let mut n_cur = tokens.len() as i32;

    for _ in 0..max_tokens {
        let mut candidates = ctx.token_data_array();
        sampler.apply(&mut candidates);
        let token = match candidates.selected_token() {
            Some(t) => t,
            None => break,
        };
        sampler.accept(token);

        if token == eos {
            break;
        }

        // Convertir le token en texte et envoyer immédiatement
        let piece = loaded
            .model
            .token_to_piece_bytes(token, 32, true, None)
            .ok()
            .and_then(|b| String::from_utf8(b).ok())
            .unwrap_or_default();

        if !piece.is_empty() {
            if tx.blocking_send(StreamMsg::Token(piece)).is_err() {
                // Le receiver a été drop — arrêter la génération
                return;
            }
        }

        completion_tokens += 1;

        let mut batch = LlamaBatch::new(1, 1);
        if batch.add(token, n_cur, &[0], true).is_err() {
            break;
        }
        if ctx.decode(&mut batch).is_err() {
            break;
        }
        n_cur += 1;
    }

    let _ = tx.blocking_send(StreamMsg::Done(prompt_tokens, completion_tokens));
}

// ── Format ChatML ─────────────────────────────────────────────────

/// Construit un prompt ChatML à partir des messages OpenAI-compat.
pub fn build_chat_prompt(messages: &[serde_json::Value]) -> String {
    let mut prompt = String::new();

    for msg in messages {
        let role = msg.get("role").and_then(|r| r.as_str()).unwrap_or("user");
        let content = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");

        prompt.push_str(&format!("<|im_start|>{role}\n{content}<|im_end|>\n"));
    }

    // Amorcer la réponse assistant
    prompt.push_str("<|im_start|>assistant\n");
    prompt
}
