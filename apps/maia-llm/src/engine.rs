//! @id     maia_llm_engine
//! @do     inferer_gguf_local
//! @role   orchestration
//! @layer  app
//! @human  maia-llm — moteur d'inférence GGUF (llama.cpp) : chargement vérifié, gabarit de chat du modèle, grammaire GBNF, décodage UTF-8 sûr.

//! Moteur d'inférence GGUF — wrapper Rust autour de llama.cpp via llama-cpp-2.
//!
//! Architecture :
//! - `LlmEngine` : état partagé Arc<Mutex<LlmState>>, thread-safe (une
//!   inférence à la fois : le modèle est chargé **une seule fois** et servi à
//!   MAIA comme à Alicia)
//! - `load_model` / `unload` : chargement/déchargement GGUF (provenance vérifiée)
//! - `infer_blocking` : inférence bloquante (spawn_blocking)
//! - `chat_stream` : streaming SSE token-par-token via mpsc channel
//! - [`Generation`] : température, longueur, graine et **grammaire GBNF**
//!   facultative (sortie contrainte, ex. intentions domotiques d'Alicia)
//!
//! Le prompt suit le **gabarit de chat embarqué dans le GGUF** (ChatML en
//! repli). Les morceaux de texte sont décodés en UTF-8 de façon incrémentale :
//! un caractère accentué réparti sur deux jetons n'est jamais perdu.
//!
//! Seul C conservé : llama.cpp (moteur tenseur — irremplaçable en pur Rust pour GGUF).

use std::fmt::Write as _;
use std::path::PathBuf;
// C-10 : un mutex empoisonné est levé sans panique (l'état — modèle chargé,
// cache de scan — reste utilisable ; chaque inférence recrée son contexte).
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::context::LlamaContext;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{LlamaChatMessage, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use llama_cpp_2::token::LlamaToken;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

use crate::config::MaiaLlmConfig;
use maia_modeles::{Autorisation, PolitiqueModeles};

/// Taille maximale d'une grammaire GBNF reçue (C-13).
pub const GRAMMAIRE_MAX: usize = 16 * 1024;

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
    /// Paramètres de génération refusés (grammaire invalide ou trop longue…).
    ParametresInvalides(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle GGUF chargé"),
            Self::LlamaError(e) => write!(f, "Erreur llama.cpp : {e}"),
            Self::AlreadyRunning => write!(f, "Inférence déjà en cours"),
            Self::ProvenanceRefusee(e) => write!(f, "Modèle refusé (provenance) : {e}"),
            Self::ParametresInvalides(e) => write!(f, "Paramètres de génération refusés : {e}"),
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
    /// Date de coupure des connaissances déclarée par le manifeste signé
    /// (`None` sous dérogation de développement).
    #[serde(default)]
    pub date_coupure: Option<String>,
    /// Licence déclarée par le manifeste signé.
    #[serde(default)]
    pub licence: Option<String>,
}

/// Paramètres d'une génération.
#[derive(Debug, Clone)]
pub struct Generation {
    /// Température ; `<= 0` ⇒ décodage glouton (déterministe).
    pub temperature: f32,
    /// Nombre maximal de jetons générés.
    pub max_tokens: u32,
    /// Graine de l'échantillonneur.
    pub graine: u32,
    /// Grammaire GBNF (racine `root`) contraignant la sortie.
    pub grammaire: Option<String>,
}

impl Generation {
    /// Génération libre.
    pub fn libre(temperature: f32, max_tokens: u32) -> Self {
        Self {
            temperature,
            max_tokens,
            graine: 42,
            grammaire: None,
        }
    }

    /// Contrôle des bornes (taille de la grammaire).
    pub fn valider(&self) -> Result<(), LlmError> {
        if let Some(g) = &self.grammaire {
            if g.trim().is_empty() || g.len() > GRAMMAIRE_MAX || g.contains('\0') {
                return Err(LlmError::ParametresInvalides(format!(
                    "grammaire GBNF vide, avec NUL ou de plus de {GRAMMAIRE_MAX} octets"
                )));
            }
        }
        Ok(())
    }
}

/// Mesures d'une génération (banc d'essai).
#[derive(Debug, Clone, Default, Serialize)]
pub struct Mesures {
    /// Jetons du prompt.
    pub jetons_prompt: u32,
    /// Jetons générés.
    pub jetons_generes: u32,
    /// Délai jusqu'au premier jeton (évaluation du prompt comprise).
    pub premier_jeton: Duration,
    /// Durée de la génération après le premier jeton.
    pub generation: Duration,
}

impl Mesures {
    /// Jetons générés par seconde (hors premier jeton).
    pub fn jetons_par_seconde(&self) -> f64 {
        let s = self.generation.as_secs_f64();
        if s <= 0.0 || self.jetons_generes <= 1 {
            return 0.0;
        }
        f64::from(self.jetons_generes - 1) / s
    }
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
        let mut backend = LlamaBackend::init().map_err(|e| {
            LlmError::LlamaError(format!("Impossible d'initialiser llama.cpp : {e}"))
        })?;
        if !config.journal_llama {
            backend.void_logs();
        }
        Ok(Self {
            state: Arc::new(Mutex::new(LlmState {
                backend,
                loaded: None,
                config,
            })),
            politique: Arc::new(politique),
        })
    }

    /// Vrai si ce binaire sait décharger des couches sur un GPU (feature
    /// `cuda`/`rocm`/`metal`) **et** qu'un GPU est visible de llama.cpp.
    pub fn gpu_utilisable(&self) -> bool {
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        state.backend.supports_gpu_offload() && crate::materiel::gpu_present()
    }

    // ── Gestion modèle ────────────────────────────────────────────

    /// Charge un modèle GGUF depuis le chemin donné.
    /// Décharge l'éventuel modèle précédent.
    ///
    /// EX-26 : le modèle doit d'abord passer le contrôle de provenance
    /// (manifeste signé par une ancre épinglée + empreinte SHA-256). Le
    /// contrôle a lieu hors du verrou du moteur (hachage potentiellement long).
    pub fn load_model(&self, model_path: PathBuf) -> Result<ModelInfo, LlmError> {
        let (manifeste, ouvert) = match self.politique.autoriser(&model_path) {
            Ok((Autorisation::Verifie(m), ouvert)) => (Some(m), ouvert),
            Ok((Autorisation::DerogationDev(_), ouvert)) => (None, ouvert),
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

        // Auto (-1) : GPU seulement si la VRAM **libre** suffit (le GPU peut
        // être partagé avec un autre service : jamais de dépassement).
        let offload = state.backend.supports_gpu_offload()
            && match u32::try_from(state.config.gpu_layers) {
                Ok(_) => crate::materiel::gpu_present(),
                Err(_) => crate::materiel::vram_libre_suffisante(size_bytes),
            };
        let gpu_layers = resolve_gpu_layers(state.config.gpu_layers, offload);
        // Libère l'ancien modèle avant d'allouer le nouveau (VRAM).
        state.loaded = None;
        let model_params = LlamaModelParams::default().with_n_gpu_layers(gpu_layers);

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
            date_coupure: manifeste.as_ref().map(|m| m.date_coupure.clone()),
            licence: manifeste.as_ref().map(|m| m.licence.clone()),
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
        generation: &Generation,
    ) -> Result<(String, u32, u32), LlmError> {
        let mut texte = String::new();
        let mesures = self.generer(messages, generation, &mut |morceau| {
            texte.push_str(morceau);
            true
        })?;
        Ok((texte, mesures.jetons_prompt, mesures.jetons_generes))
    }

    /// Génère en appelant `sortie` pour chaque morceau de texte UTF-8 complet
    /// (`false` ⇒ arrêt). Renvoie les mesures (banc d'essai, usage).
    pub fn generer(
        &self,
        messages: &[serde_json::Value],
        generation: &Generation,
        sortie: &mut dyn FnMut(&str) -> bool,
    ) -> Result<Mesures, LlmError> {
        generation.valider()?;
        let state = self.state.lock().unwrap_or_else(PoisonError::into_inner);
        let loaded = state.loaded.as_ref().ok_or(LlmError::NoModelLoaded)?;
        let debut = Instant::now();

        let prompt = prompt_du_modele(&loaded.model, messages);
        let ctx_size = loaded.info.context_size;

        let mut ctx_params =
            LlamaContextParams::default().with_n_ctx(std::num::NonZeroU32::new(ctx_size));
        let threads = threads_effectifs(state.config.threads);
        ctx_params = ctx_params
            .with_n_threads(threads)
            .with_n_threads_batch(threads);

        let mut ctx = loaded
            .model
            .new_context(&state.backend, ctx_params)
            .map_err(|e| LlmError::LlamaError(format!("Contexte : {e}")))?;

        // Le gabarit de chat contient déjà le jeton de début éventuel.
        let tokens = loaded
            .model
            .str_to_token(&prompt, llama_cpp_2::model::AddBos::Never)
            .map_err(|e| LlmError::LlamaError(format!("Tokenisation : {e}")))?;

        let reserve = generation.max_tokens.min(ctx_size / 2) as usize;
        if tokens.is_empty() || tokens.len() + reserve >= ctx_size as usize {
            return Err(LlmError::LlamaError(format!(
                "Prompt trop long ({} tokens + {reserve} réservés > contexte {ctx_size})",
                tokens.len()
            )));
        }

        let jetons_prompt = u32::try_from(tokens.len())
            .map_err(|_| LlmError::LlamaError("Prompt trop long".into()))?;
        decode_prompt(&mut ctx, &tokens)?;

        let mut sampler = echantillonneur(&loaded.model, generation)?;
        let mut decodeur = DecodeurUtf8::default();
        let mut mesures = Mesures {
            jetons_prompt,
            ..Mesures::default()
        };
        let mut premier: Option<Instant> = None;
        let debut_pos = i32::try_from(tokens.len())
            .map_err(|_| LlmError::LlamaError("Prompt trop long pour le contexte".into()))?;

        for n_cur in (debut_pos..=i32::MAX).take(generation.max_tokens as usize) {
            // `sample` échantillonne **et** accepte le jeton (grammaire comprise).
            let token = sampler.sample(&ctx, -1);
            if premier.is_none() {
                premier = Some(Instant::now());
                mesures.premier_jeton = debut.elapsed();
            }
            if loaded.model.is_eog_token(token) {
                break;
            }
            mesures.jetons_generes += 1;

            let octets = loaded
                .model
                .token_to_piece_bytes(token, 64, false, None)
                .map_err(|e| LlmError::LlamaError(format!("Détokenisation : {e}")))?;
            let morceau = decodeur.pousser(&octets);
            if !morceau.is_empty() && !sortie(&morceau) {
                break;
            }

            let mut batch = LlamaBatch::new(1, 1);
            batch
                .add(token, n_cur, &[0], true)
                .map_err(|e| LlmError::LlamaError(format!("Batch add : {e}")))?;
            ctx.decode(&mut batch)
                .map_err(|e| LlmError::LlamaError(format!("Decode gen : {e}")))?;
        }
        let reste = decodeur.terminer();
        if !reste.is_empty() {
            sortie(&reste);
        }
        if let Some(p) = premier {
            mesures.generation = p.elapsed();
        }
        Ok(mesures)
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
        generation: Generation,
    ) -> Result<mpsc::Receiver<StreamMsg>, LlmError> {
        if !self.is_loaded() {
            return Err(LlmError::NoModelLoaded);
        }
        generation.valider()?;

        let (tx, rx) = mpsc::channel::<StreamMsg>(256);
        let engine = self.clone();

        tokio::task::spawn_blocking(move || {
            let mut envoyer = |morceau: &str| tx.blocking_send(StreamMsg::Token(morceau.to_string())).is_ok();
            match engine.generer(&messages, &generation, &mut envoyer) {
                Ok(m) => {
                    let _ = tx.blocking_send(StreamMsg::Done(m.jetons_prompt, m.jetons_generes));
                }
                Err(e) => {
                    let _ = tx.blocking_send(StreamMsg::Error(e.to_string()));
                }
            }
        });

        Ok(rx)
    }
}

// ── Helpers internes ─────────────────────────────────────────────

/// Résout le nombre de couches GPU depuis la config : `-1` = auto (tout sur
/// GPU si ce binaire sait décharger **et** qu'un GPU est présent, sinon CPU).
fn resolve_gpu_layers(cfg: i32, offload_possible: bool) -> u32 {
    if !offload_possible {
        return 0;
    }
    u32::try_from(cfg).unwrap_or(999)
}

/// Fils d'exécution : configuration, sinon la moitié des fils logiques
/// (≈ cœurs physiques ; l'hyperthreading n'aide pas llama.cpp), entre 1 et 8.
fn threads_effectifs(cfg: u32) -> i32 {
    let n = if cfg > 0 {
        cfg
    } else {
        (std::thread::available_parallelism()
            .map_or(4, |n| u32::try_from(n.get()).unwrap_or(4))
            / 2)
        .clamp(1, 8)
    };
    i32::try_from(n).unwrap_or(4)
}

/// Chaîne d'échantillonnage : grammaire éventuelle, puis glouton
/// (température ≤ 0) ou température + tirage.
fn echantillonneur(model: &LlamaModel, g: &Generation) -> Result<LlamaSampler, LlmError> {
    let mut chaine = Vec::with_capacity(3);
    if let Some(grammaire) = &g.grammaire {
        let s = LlamaSampler::grammar(model, grammaire, "root")
            .map_err(|e| LlmError::ParametresInvalides(format!("grammaire GBNF : {e}")))?;
        chaine.push(s);
    }
    if g.temperature <= 0.0 {
        chaine.push(LlamaSampler::greedy());
    } else {
        chaine.push(LlamaSampler::temp(g.temperature));
        chaine.push(LlamaSampler::dist(g.graine));
    }
    Ok(LlamaSampler::chain_simple(chaine))
}

/// Décode le prompt par chunks dans le contexte llama.
fn decode_prompt(ctx: &mut LlamaContext<'_>, tokens: &[LlamaToken]) -> Result<(), LlmError> {
    let n_batch = (ctx.n_batch() as usize).max(1);
    let total = tokens.len();
    for (i_chunk, chunk) in tokens.chunks(n_batch).enumerate() {
        let base = i_chunk * n_batch;
        let mut batch = LlamaBatch::new(chunk.len(), 1);
        for (i, token) in chunk.iter().enumerate() {
            let pos = i32::try_from(base + i)
                .map_err(|_| LlmError::LlamaError("Position de token hors bornes".into()))?;
            let dernier = base + i + 1 == total;
            batch
                .add(*token, pos, &[0], dernier)
                .map_err(|e| LlmError::LlamaError(format!("Batch add : {e}")))?;
        }
        ctx.decode(&mut batch)
            .map_err(|e| LlmError::LlamaError(format!("Decode prompt : {e}")))?;
    }
    Ok(())
}

/// Décodeur UTF-8 incrémental : n'émet que des caractères complets ; un
/// octet invalide devient U+FFFD (jamais de perte silencieuse ni de panique).
#[derive(Debug, Default)]
pub struct DecodeurUtf8 {
    tampon: Vec<u8>,
}

impl DecodeurUtf8 {
    /// Ajoute des octets, renvoie le texte complet disponible.
    pub fn pousser(&mut self, octets: &[u8]) -> String {
        self.tampon.extend_from_slice(octets);
        let mut sortie = String::new();
        loop {
            match std::str::from_utf8(&self.tampon) {
                Ok(s) => {
                    sortie.push_str(s);
                    self.tampon.clear();
                    return sortie;
                }
                Err(e) => {
                    let valide = e.valid_up_to();
                    // `valide` est une frontière de caractère garantie par from_utf8.
                    sortie.push_str(&String::from_utf8_lossy(&self.tampon[..valide]));
                    match e.error_len() {
                        // Séquence incomplète en fin de tampon : on attend la suite.
                        None => {
                            self.tampon.drain(..valide);
                            return sortie;
                        }
                        Some(n) => {
                            sortie.push('\u{FFFD}');
                            self.tampon.drain(..valide + n);
                        }
                    }
                }
            }
        }
    }

    /// Fin de flux : les octets restants (incomplets) deviennent U+FFFD.
    pub fn terminer(&mut self) -> String {
        let reste = String::from_utf8_lossy(&self.tampon).into_owned();
        self.tampon.clear();
        reste
    }
}

// ── Gabarit de chat ───────────────────────────────────────────────

/// Paires (rôle, contenu) des messages OpenAI-compat ; rôles hors
/// `system`/`user`/`assistant`/`tool` ramenés à `user`.
fn paires(messages: &[serde_json::Value]) -> Vec<(String, String)> {
    messages
        .iter()
        .map(|msg| {
            let role = msg
                .get("role")
                .and_then(|r| r.as_str())
                .filter(|r| matches!(*r, "system" | "user" | "assistant" | "tool"))
                .unwrap_or("user");
            let content = msg.get("content").and_then(|c| c.as_str()).unwrap_or("");
            (role.to_string(), content.replace('\0', ""))
        })
        .collect()
}

/// Prompt selon le gabarit de chat du GGUF ; ChatML en repli.
fn prompt_du_modele(model: &LlamaModel, messages: &[serde_json::Value]) -> String {
    let paires = paires(messages);
    let via_gabarit = model.chat_template(None).ok().and_then(|gabarit| {
        let chat: Option<Vec<LlamaChatMessage>> = paires
            .iter()
            .map(|(r, c)| LlamaChatMessage::new(r.clone(), c.clone()).ok())
            .collect();
        model.apply_chat_template(&gabarit, &chat?, true).ok()
    });
    via_gabarit.unwrap_or_else(|| build_chat_prompt(messages))
}

/// Construit un prompt ChatML à partir des messages OpenAI-compat (repli).
pub fn build_chat_prompt(messages: &[serde_json::Value]) -> String {
    let mut prompt = String::new();
    for (role, content) in paires(messages) {
        // L'écriture dans une String ne peut pas échouer.
        let _ = writeln!(prompt, "<|im_start|>{role}\n{content}<|im_end|>");
    }
    // Amorcer la réponse assistant
    prompt.push_str("<|im_start|>assistant\n");
    prompt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodeur_utf8_ne_perd_pas_un_accent_coupe() {
        let mut d = DecodeurUtf8::default();
        let e = "é".as_bytes();
        assert_eq!(d.pousser(&e[..1]), "");
        assert_eq!(d.pousser(&e[1..]), "é");
        assert_eq!(d.pousser(b"t\xc3"), "t");
        assert_eq!(d.pousser(b"\xa9 ok"), "é ok");
        assert_eq!(d.pousser(b"\xff!"), "\u{FFFD}!");
        assert_eq!(d.pousser(b"\xe2\x82"), "");
        assert_eq!(d.terminer(), "\u{FFFD}");
    }

    #[test]
    fn couches_gpu_auto_selon_le_materiel() {
        assert_eq!(resolve_gpu_layers(-1, true), 999);
        assert_eq!(resolve_gpu_layers(-1, false), 0);
        assert_eq!(resolve_gpu_layers(12, true), 12);
        assert_eq!(resolve_gpu_layers(12, false), 0);
        assert_eq!(resolve_gpu_layers(0, true), 0);
    }

    #[test]
    fn grammaire_bornee() {
        let mut g = Generation::libre(0.0, 8);
        assert!(g.valider().is_ok());
        g.grammaire = Some(String::new());
        assert!(g.valider().is_err());
        g.grammaire = Some("x".repeat(GRAMMAIRE_MAX + 1));
        assert!(g.valider().is_err());
        g.grammaire = Some("root ::= \"a\"".into());
        assert!(g.valider().is_ok());
    }

    #[test]
    fn chatml_de_repli_et_roles_normalises() {
        let p = build_chat_prompt(&[
            serde_json::json!({"role": "system", "content": "S"}),
            serde_json::json!({"role": "pirate", "content": "U\0"}),
        ]);
        assert_eq!(
            p,
            "<|im_start|>system\nS<|im_end|>\n<|im_start|>user\nU<|im_end|>\n<|im_start|>assistant\n"
        );
    }

    #[test]
    fn threads_auto_bornes() {
        assert_eq!(threads_effectifs(3), 3);
        let auto = threads_effectifs(0);
        assert!((1..=8).contains(&auto));
    }
}
