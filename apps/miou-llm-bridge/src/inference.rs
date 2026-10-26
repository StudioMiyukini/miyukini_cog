//! @id     miou_llm_bridge_inference
//! @do     definir_chat_message
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « inference ».

//! Couche d'inférence — abstraction qui route vers maia-llm ou l'upstream.
//!
//! Architecture :
//! - `MaiaLlmClient` : client HTTP vers le service maia-llm (GGUF, local)
//! - `UpstreamBackend` : moteur compatible OpenAI **hors COG** (LM Studio,
//!   Ollama, koboldcpp…) — **désactivé par défaut** : il n'est actif que si
//!   `upstream_url` est renseigné **et** que StrongFather autorise l'intention
//!   `inference.externe` ; la politique de production (`maia.toml`) la refuse
//!   toujours. Il ne sert donc qu'aux essais (politique d'essai dédiée).
//! - `InferenceRouter` : maia-llm d'abord ; l'upstream seulement s'il est autorisé

// C-10 : un verrou empoisonné (panique d'un autre fil pendant l'écriture)
// est levé sans panique ; les sections critiques ne laissent pas d'état
// partiel et l'état réel est conservé (pas de remise à zéro permissive).
use std::sync::{Arc, PoisonError, RwLock};

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
#[allow(clippy::struct_field_names)] // noms imposés par le format OpenAI `usage`
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
    /// Moteur hors COG non autorisé par la politique (défaut).
    UpstreamRefuse,
}

impl std::fmt::Display for InferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoModelLoaded => write!(f, "Aucun modèle chargé dans maia-llm"),
            Self::LlamaError(e) => write!(f, "Erreur maia-llm : {e}"),
            Self::UpstreamError(e) => write!(f, "Erreur upstream : {e}"),
            Self::NoBackendAvailable => write!(f, "Aucun backend d'inférence disponible"),
            Self::UpstreamRefuse => write!(
                f,
                "Moteur d'IA hors COG refusé par la politique (inference.externe)"
            ),
        }
    }
}

// ── MaiaLlmClient ──────────────────────────────────────────────────

/// Usage signé des appels d'inférence vers maia-llm (`/v1/*`).
pub const USAGE_MAIA_LLM_INFERENCE: &str = "maia-llm/inference";
/// Usage signé des appels de gestion des modèles vers maia-llm (`/maia/llm/models*`).
pub const USAGE_MAIA_LLM_MODELES: &str = "maia-llm/modeles";

/// Client HTTP vers le service maia-llm (remplace NativeBackend llama-cpp-2).
/// Maintient un cache local du modèle chargé pour les appels synchrones.
/// Les requêtes sont signées par la clé de service de MAIA (à épingler dans
/// `appelants_inference` / `appelants_modeles` de maia-llm) : plus de clé partagée.
#[derive(Clone)]
pub struct MaiaLlmClient {
    pub base_url: String,
    cle: Option<Arc<miyusceau::CleSignature>>,
    client: reqwest::Client,
    /// Cache : nom du modèle actuellement chargé dans maia-llm (None = aucun).
    loaded_model: Arc<RwLock<Option<String>>>,
}

impl MaiaLlmClient {
    pub fn new(
        base_url: String,
        cle: Option<Arc<miyusceau::CleSignature>>,
        client: reqwest::Client,
    ) -> Self {
        Self {
            base_url,
            cle,
            client,
            loaded_model: Arc::new(RwLock::new(None)),
        }
    }

    /// Requête sans corps, signée par la clé de MAIA.
    fn requete(&self, methode: reqwest::Method, url: &str, usage: &str) -> reqwest::RequestBuilder {
        let rb = self.client.request(methode.clone(), url);
        crate::signature::signer(rb, self.cle.as_deref(), usage, methode.as_str(), url, b"")
    }

    /// Requête JSON signée (le corps signé est exactement celui envoyé).
    fn requete_json(
        &self,
        methode: reqwest::Method,
        url: &str,
        usage: &str,
        corps: &serde_json::Value,
    ) -> reqwest::RequestBuilder {
        let octets = serde_json::to_vec(corps).unwrap_or_default();
        let rb = self
            .client
            .request(methode.clone(), url)
            .header(reqwest::header::CONTENT_TYPE, "application/json");
        crate::signature::signer(
            rb,
            self.cle.as_deref(),
            usage,
            methode.as_str(),
            url,
            &octets,
        )
        .body(octets)
    }

    // ── Méthodes synchrones (cache local) ──────────────────────────

    /// Vérifie si un modèle est chargé (via cache local).
    pub fn is_loaded(&self) -> bool {
        self.loaded_model
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .is_some()
    }

    /// Retourne le nom du modèle chargé (via cache local).
    pub fn loaded_model_name(&self) -> Option<String> {
        self.loaded_model
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
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
                        .map(std::string::ToString::to_string);
                    *self
                        .loaded_model
                        .write()
                        .unwrap_or_else(PoisonError::into_inner) = model_name;
                }
                true
            }
            _ => false,
        }
    }

    /// Charge un modèle dans maia-llm via POST /maia/llm/models/load.
    pub async fn load_model(&self, path: std::path::PathBuf) -> Result<String, InferenceError> {
        let filename = path
            .file_name()
            .unwrap_or(path.as_os_str())
            .to_string_lossy()
            .to_string();

        let url = format!("{}/maia/llm/models/load", self.base_url);
        let body = serde_json::json!({ "model": filename });

        let req = self.requete_json(reqwest::Method::POST, &url, USAGE_MAIA_LLM_MODELES, &body);

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

        *self
            .loaded_model
            .write()
            .unwrap_or_else(PoisonError::into_inner) = Some(model_name.clone());

        Ok(model_name)
    }

    /// Décharge le modèle actuel dans maia-llm via DELETE /maia/llm/models/unload.
    pub async fn unload_model(&self) -> bool {
        let url = format!("{}/maia/llm/models/unload", self.base_url);
        let req = self.requete(reqwest::Method::DELETE, &url, USAGE_MAIA_LLM_MODELES);

        let success = req
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await
            .is_ok_and(|r| r.status().is_success());

        if success {
            *self
                .loaded_model
                .write()
                .unwrap_or_else(PoisonError::into_inner) = None;
        }

        success
    }

    /// Liste les modèles GGUF disponibles sur maia-llm (GET /maia/llm/models).
    /// Retourne les `model_id` — noms sans extension pour les requêtes API.
    pub async fn available_models(&self) -> Vec<String> {
        let url = format!("{}/maia/llm/models", self.base_url);
        let req = self
            .requete(reqwest::Method::GET, &url, USAGE_MAIA_LLM_MODELES)
            .timeout(std::time::Duration::from_secs(10));
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
        let req = self
            .requete(reqwest::Method::GET, &url, USAGE_MAIA_LLM_INFERENCE)
            .timeout(std::time::Duration::from_secs(5));
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
        let req = self.requete_json(reqwest::Method::POST, &url, USAGE_MAIA_LLM_INFERENCE, body);

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

/// Backend HTTP hors COG (LM Studio, Ollama, koboldcpp…). **Inactif** tant
/// que [`UpstreamBackend::autoriser`] n'a pas été appelé après une décision
/// StrongFather favorable (jamais en production).
#[derive(Clone)]
pub struct UpstreamBackend {
    client: reqwest::Client,
    upstream_url: String,
    autorise: bool,
}

impl UpstreamBackend {
    /// Backend **inactif** (aucun appel réseau tant qu'il n'est pas autorisé).
    pub fn new(client: reqwest::Client, upstream_url: String) -> Self {
        Self {
            client,
            upstream_url,
            autorise: false,
        }
    }

    /// Active le backend : à n'appeler qu'après autorisation de l'intention
    /// `inference.externe` par StrongFather ; URL vide ⇒ reste inactif.
    #[must_use]
    pub fn autoriser(mut self, decision_strongfather: bool) -> Self {
        self.autorise = decision_strongfather && !self.upstream_url.trim().is_empty();
        self
    }

    /// Vrai si le backend est autorisé (politique) et configuré.
    pub fn est_autorise(&self) -> bool {
        self.autorise
    }

    pub async fn is_available(&self) -> bool {
        if !self.autorise {
            return false;
        }
        let url = format!("{}/v1/models", self.upstream_url);
        self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
            .is_ok_and(|r| r.status().is_success())
    }

    pub async fn chat_completion(
        &self,
        body: &serde_json::Value,
    ) -> Result<serde_json::Value, InferenceError> {
        if !self.autorise {
            return Err(InferenceError::UpstreamRefuse);
        }
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
    /// Vrai si maia-llm a un modèle chargé ; le cache est rafraîchi depuis
    /// `/maia/llm/health` quand il est vide (maia-llm démarré après MAIA, ou
    /// modèle chargé par l'auto-chargement de maia-llm).
    pub async fn natif_pret(&self) -> bool {
        self.native.is_loaded() || (self.native.is_available().await && self.native.is_loaded())
    }

    /// Disponibilité : natif, sinon upstream **autorisé**, sinon secours.
    pub async fn disponibilite(&self) -> crate::fallback::LlmAvailability {
        use crate::fallback::LlmAvailability;
        if self.natif_pret().await {
            LlmAvailability::NativeOnline
        } else if self.upstream.is_available().await {
            LlmAvailability::UpstreamOnline
        } else {
            LlmAvailability::Fallback
        }
    }

    pub async fn chat_completion(
        &self,
        body: serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        if self.prefer_native && self.natif_pret().await {
            match self.try_native(&body).await {
                Ok(resp) => return Ok(resp),
                Err(e) => tracing::warn!("maia-llm échoué, fallback upstream : {e}"),
            }
        }

        if self.upstream.is_available().await {
            return self.try_upstream(&body).await;
        }

        if !self.prefer_native && self.natif_pret().await {
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
        Ok(parse_response(raw, BackendKind::Native))
    }

    async fn try_upstream(
        &self,
        body: &serde_json::Value,
    ) -> Result<ChatCompletionResponse, InferenceError> {
        let raw = self.upstream.chat_completion(body).await?;
        Ok(parse_response(raw, BackendKind::Upstream))
    }
}

// ── Helpers ────────────────────────────────────────────────────────

fn parse_response(raw: serde_json::Value, backend: BackendKind) -> ChatCompletionResponse {
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
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0) as u32;
    let completion_tokens = usage
        .get("completion_tokens")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0) as u32;

    ChatCompletionResponse {
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
            total_tokens: prompt_tokens.saturating_add(completion_tokens),
        },
        backend,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{extract::Request, http::StatusCode, routing::any, Router};
    use miyauth::http::service::{maintenant_unix, GardeSignee};
    use miyusceau::CleSignature;

    /// Faux maia-llm : vérifie la signature (usage selon la famille de route,
    /// comme `apps/maia-llm/src/auth.rs`) puis répond.
    async fn faux_maia_llm(cle_epinglee: [u8; 32]) -> String {
        let garde = Arc::new(GardeSignee::new([cle_epinglee]));
        let verifier = move |req: Request| {
            let garde_srv = garde.clone();
            async move {
                let usage = if req.uri().path().starts_with("/maia/llm/models") {
                    USAGE_MAIA_LLM_MODELES
                } else {
                    USAGE_MAIA_LLM_INFERENCE
                };
                let entetes_http = req.headers().clone();
                let v = |nom: &str| entetes_http.get(nom).and_then(|x| x.to_str().ok());
                let instant = maintenant_unix();
                let entetes = garde_srv
                    .controler_entetes(
                        v(miyauth::http::ENTETE_SIGNATAIRE),
                        v(miyauth::http::ENTETE_HORODATAGE),
                        v(miyauth::http::ENTETE_NONCE),
                        v(miyauth::http::ENTETE_SIGNATURE),
                        instant,
                    )
                    .map_err(|_| StatusCode::UNAUTHORIZED)?;
                let (parts, corps) = req.into_parts();
                let corps = axum::body::to_bytes(corps, 1 << 20).await.unwrap();
                let chemin = parts.uri.path_and_query().unwrap().as_str().to_string();
                garde_srv
                    .verifier(
                        &entetes,
                        usage,
                        parts.method.as_str(),
                        &chemin,
                        &corps,
                        instant,
                    )
                    .map_err(|_| StatusCode::UNAUTHORIZED)?;
                let reponse = match parts.uri.path() {
                    "/maia/llm/models" => serde_json::json!([{ "model_id": "m1" }]),
                    "/v1/models" => serde_json::json!({ "data": [{ "id": "m1" }] }),
                    _ => serde_json::json!({ "choices": [{ "message": { "content": "ok" } }] }),
                };
                Ok::<_, StatusCode>(axum::Json(reponse))
            }
        };
        let app = Router::new()
            .route("/maia/llm/models", any(verifier.clone()))
            .route("/v1/models", any(verifier.clone()))
            .route("/v1/chat/completions", any(verifier));
        let ecoute = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let adresse = ecoute.local_addr().unwrap();
        tokio::spawn(async move {
            let _ = axum::serve(ecoute, app).await;
        });
        format!("http://{adresse}")
    }

    #[tokio::test]
    async fn upstream_inactif_par_defaut_et_sans_url() {
        let c = reqwest::Client::new();
        let amont = UpstreamBackend::new(c.clone(), "http://127.0.0.1:9".into());
        assert!(!amont.est_autorise());
        assert!(!amont.is_available().await);
        assert!(matches!(
            amont.chat_completion(&serde_json::json!({})).await,
            Err(InferenceError::UpstreamRefuse)
        ));
        assert!(!UpstreamBackend::new(c.clone(), "http://127.0.0.1:9".into())
            .autoriser(false)
            .est_autorise());
        assert!(!UpstreamBackend::new(c.clone(), "  ".into())
            .autoriser(true)
            .est_autorise());
        // Aucun moteur : secours, jamais un appel hors COG.
        let routeur = InferenceRouter::new(
            MaiaLlmClient::new("http://127.0.0.1:9".into(), None, c.clone()),
            amont,
            true,
        );
        assert_eq!(
            routeur.disponibilite().await,
            crate::fallback::LlmAvailability::Fallback
        );
        assert!(matches!(
            routeur.chat_completion(serde_json::json!({})).await,
            Err(InferenceError::NoBackendAvailable)
        ));
    }

    #[tokio::test]
    async fn client_maia_llm_signe_ses_requetes() {
        let cle = Arc::new(CleSignature::generer());
        let url = faux_maia_llm(cle.cle_publique()).await;
        let client = MaiaLlmClient::new(url.clone(), Some(cle), reqwest::Client::new());
        assert_eq!(client.available_models().await, vec!["m1".to_string()]);
        assert_eq!(client.loaded_models().await, vec!["m1".to_string()]);
        let r = client
            .chat_completion(&serde_json::json!({ "messages": [] }))
            .await
            .unwrap();
        assert_eq!(r["choices"][0]["message"]["content"], "ok");

        // Sans clé, ou avec une clé non épinglée : refus.
        let anonyme = MaiaLlmClient::new(url.clone(), None, reqwest::Client::new());
        assert_eq!(
            anonyme.available_models().await,
            [] as [std::string::String; 0]
        );
        assert!(anonyme
            .chat_completion(&serde_json::json!({}))
            .await
            .is_err());
        let intrus = MaiaLlmClient::new(
            url,
            Some(Arc::new(CleSignature::generer())),
            reqwest::Client::new(),
        );
        assert!(intrus
            .chat_completion(&serde_json::json!({}))
            .await
            .is_err());
    }
}
