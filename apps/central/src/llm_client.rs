//! Client LLM — API OpenAI-compatible pour le chat Miou.
//!
//! Pointe vers le bridge MiouLLM (par défaut localhost:11435)
//! qui relaie les requêtes vers LM Studio ou tout autre serveur compatible.

use serde::{Deserialize, Serialize};

/// URL par défaut du bridge MiouLLM.
const DEFAULT_BRIDGE_URL: &str = "http://localhost:11435";

// ── Types API ──────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ModelInfo>,
}

#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub id: String,
}

// ── Types hardware / recommandation ──────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct HardwareResponse {
    pub hardware: HardwareData,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HardwareData {
    pub cpu_name: String,
    pub cpu_cores: u32,
    pub ram_total_mb: u64,
    pub ram_available_mb: u64,
    pub gpu: Option<GpuData>,
    pub tier: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GpuData {
    pub name: String,
    pub vram_mb: u64,
    pub vendor: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecommendModelResponse {
    pub recommended: Option<RecommendedModel>,
    pub candidates: Vec<RecommendedModel>,
    pub hardware_tier: String,
    pub models_available: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RecommendedModel {
    pub model_id: String,
    pub display_name: String,
    pub params: String,
    pub quantization: String,
    pub score: u8,
    pub description: String,
    pub already_loaded: bool,
    pub tier_label: String,
}

// ── Client ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct LlmClient {
    client: reqwest::Client,
    base_url: String,
}

#[derive(Debug)]
pub enum LlmError {
    Network(String),
    NoModel,
    Api(String),
}

impl std::fmt::Display for LlmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Network(e) => write!(f, "Réseau : {e}"),
            Self::NoModel => write!(f, "Aucun modèle chargé"),
            Self::Api(e) => write!(f, "API : {e}"),
        }
    }
}

impl LlmClient {
    /// Crée un client LLM pointant vers l'URL spécifiée.
    pub fn new(base_url: &str) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("reqwest client");
        Self {
            client,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    /// Crée un client pointant vers le bridge par défaut (localhost:11435).
    pub fn default_bridge() -> Self {
        Self::new(DEFAULT_BRIDGE_URL)
    }

    /// Liste les modèles chargés.
    pub async fn list_models(&self) -> Result<Vec<String>, LlmError> {
        let url = format!("{}/v1/models", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;
        let body: ModelsResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Api(e.to_string()))?;
        Ok(body.data.into_iter().map(|m| m.id).collect())
    }

    /// Vérifie qu'un modèle est chargé. Retourne le premier modèle disponible.
    pub async fn ensure_model(&self) -> Result<String, LlmError> {
        let models = self.list_models().await?;
        models.into_iter().next().ok_or(LlmError::NoModel)
    }

    /// Récupère les specs hardware du host (via le bridge).
    pub async fn hardware_info(&self) -> Result<HardwareData, LlmError> {
        let url = format!("{}/v1/hardware", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;
        let body: HardwareResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Api(e.to_string()))?;
        Ok(body.hardware)
    }

    /// Demande au bridge de recommander le meilleur modèle pour le hardware.
    pub async fn recommend_model(&self) -> Result<RecommendModelResponse, LlmError> {
        let url = format!("{}/v1/recommend", self.base_url);
        let resp = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;
        let body: RecommendModelResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Api(e.to_string()))?;
        Ok(body)
    }

    /// Envoie un message de chat et retourne la réponse de l'assistant.
    pub async fn chat(&self, model: &str, messages: &[ChatMessage]) -> Result<String, LlmError> {
        let url = format!("{}/v1/chat/completions", self.base_url);
        let req = ChatRequest {
            model: model.to_string(),
            messages: messages.to_vec(),
            temperature: Some(0.8),
            max_tokens: Some(512),
        };

        let resp = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(LlmError::Api(format!("HTTP {status}: {body}")));
        }

        let body: ChatResponse = resp
            .json()
            .await
            .map_err(|e| LlmError::Api(e.to_string()))?;

        body.choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| LlmError::Api("Pas de réponse".into()))
    }
}
