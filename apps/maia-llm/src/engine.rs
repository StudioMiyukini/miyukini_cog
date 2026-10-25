//! @id     maia_llm_engine
//! @do     definir_stream_msg
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — module « engine ».

//! Moteur d'inférence GGUF — wrapper Rust autour de llama.cpp via llama-cpp-2.
//!
//! Architecture :
//! - `LlmEngine` : état partagé Arc<Mutex<LlmState>>, thread-safe
//! - `load_model` / `unload` : chargement/déchargement GGUF
//! - `chat_completion` : inférence bloquante (spawn_blocking)
//! - `chat_stream` : streaming SSE token-par-token via mpsc channel
//!
//! Seul C conservé : llama.cpp (moteur tenseur GPU — irremplaçable en pur Rust pour GGUF).

use std::fmt::Write as _;
use std::path::PathBuf;
// C-10 : un mutex empoisonné est levé sans panique (l'état — modèle chargé,
// cache de scan — reste utilisable ; chaque inférence recrée son contexte).
use std::sync::{Arc, Mutex, PoisonError};

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::LlamaModel;
use llama_cpp_2::sampling::LlamaSampler;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::MaiaLlmConfig;
use maia_modeles::{Autorisation, PolitiqueModeles};

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
    /// Provenance du modèle non vérifiée (EX-26) : chargement refusé.
    ProvenanceRefusee(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle GGUF chargé"),
            Self::LlamaError(e) => write!(f, "Erreur llama.cpp : {e}"),
            Self::AlreadyRunning => write!(f, "Inférence déjà en cours"),
            Self::ProvenanceRefusee(e) => write!(f, "Modèle refusé (provenance) : {e}"),
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
    /// Politique de provenance des modèles (EX-26), fixée au démarrage.
    politique: Arc<PolitiqueModeles>,
}

impl LlmEngine {
    /// Initialise le moteur llama.cpp.
    pub fn new(config: MaiaLlmConfig, politique: PolitiqueModeles) -> Result<Self, LlmError> {
        let backend = LlamaBackend::init().map_err(|e| {
            LlmError::LlamaError(format!("Impossible d'initialiser llama.cpp : {e}"))
        })?;
        Ok(Self {
            state: Arc::new(Mutex::new(LlmState {
                backend,
                loaded: None,
                config,
            })),
            politique: Arc::new(politique),
        })
    }

    // ── Gestion modèle ────────────────────────────────────────────

    /// Charge un modèle GGUF depuis le chemin donné.
    /// Décharge l'éventuel modèle précédent.
    ///
    /// EX-26 : le modèle doit d'abord passer le contrôle de provenance
    /// (manifeste signé par une ancre épinglée + empreinte SHA-256). Le
    /// contrôle a lieu hors du verrou du moteur (hachage potentiellement long).
    pub fn load_model(&self, model_path: PathBuf) -> Result<ModelInfo, LlmError> {
        let ouvert = match self.politique.autoriser(&model_path) {
            Ok((Autorisation::Verifie(_) | Autorisation::DerogationDev(_), ouvert)) => ouvert,
            Err(e) => {
                tracing::warn!("Chargement refusé ({}) : {e}", model_path.display());
                return Err(LlmError::ProvenanceRefusee(e.to_string()));
            }
        };

        let mut state = self
            .state
            .lock()
            .map_err(|_| LlmError::LlamaError("verrou du moteur empoisonné".into()))?;

        let model_name = model_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let size_bytes = ouvert.taille();

        tracing::info!("Chargement modèle GGUF : {model_name}");

        let gpu_layers = resolve_gpu_layers(state.config.gpu_layers);
        let mut model_params = LlamaModelParams::default();
        model_params = model_params.with_n_gpu_layers(gpu_layers);

        // Chargement des octets vérifiés (même descripteur), encadré par deux
        // contrôles d'identité : un fichier modifié entre-temps est refusé.
        ouvert
            .inchange()
            .map_err(|e| LlmError::ProvenanceRefusee(e.to_string()))?;
        let model =
            LlamaModel::load_from_file(&state.backend, ouvert.chemin_de_chargement(), &model_params)
                .map_err(|e| LlmError::LlamaError(format!("Chargement échoué : {e}")))?;
        ouvert
            .inchange()
            .map_err(|e| LlmError::ProvenanceRefusee(e.to_string()))?;

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
        let mut state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        if let Some(m) = state.loaded.take() {
            tracing::info!("Modèle déchargé : {}", m.info.name);
            true
        } else {
            false
        }
    }

    /// Retourne les infos du modèle chargé, ou None.
    pub fn model_info(&self) -> Option<ModelInfo> {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.loaded.as_ref().map(|l| l.info.clone())
    }

    /// Vérifie si un modèle est chargé.
    pub fn is_loaded(&self) -> bool {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
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
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
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
        let debut = i32::try_from(tokens.len())
            .map_err(|_| LlmError::LlamaError("Prompt trop long pour le contexte".into()))?;

        for n_cur in (debut..=i32::MAX).take(max_tokens as usize) {
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
    // `async` conservé : API publique appelée avec `.await` par les handlers HTTP.
    #[allow(clippy::unused_async, clippy::unused_async_trait_impl)]
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
                let pos = i32::try_from(n_cur + i)
                    .map_err(|_| LlmError::LlamaError("Position de token hors bornes".into()))?;
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
    let state = engine.state.lock().unwrap_or_else(PoisonError::into_inner);
    let Some(loaded) = state.loaded.as_ref() else {
        let _ = tx.blocking_send(StreamMsg::Error("Aucun modèle chargé".into()));
        return;
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
    let Ok(debut) = i32::try_from(tokens.len()) else {
        let _ = tx.blocking_send(StreamMsg::Error("Prompt trop long pour le contexte".into()));
        return;
    };

    for n_cur in (debut..=i32::MAX).take(max_tokens as usize) {
        let mut candidates = ctx.token_data_array();
        sampler.apply(&mut candidates);
        let Some(token) = candidates.selected_token() else {
            break;
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

        if !piece.is_empty()
            && tx.blocking_send(StreamMsg::Token(piece)).is_err() {
                // Le receiver a été drop — arrêter la génération
                return;
            }

        completion_tokens += 1;

        let mut batch = LlamaBatch::new(1, 1);
        if batch.add(token, n_cur, &[0], true).is_err() {
            break;
        }
        if ctx.decode(&mut batch).is_err() {
            break;
        }
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

        // L'écriture dans une String ne peut pas échouer.
        let _ = writeln!(prompt, "<|im_start|>{role}\n{content}<|im_end|>");
    }

    // Amorcer la réponse assistant
    prompt.push_str("<|im_start|>assistant\n");
    prompt
}
