//! Client LM Studio — API OpenAI-compatible pour le chat Miou.
//!
//! Version dev : endpoint cloudflare tunnel, pas de fallback.

use serde::{Deserialize, Serialize};

/// URL de base de LM Studio (tunnel Cloudflare dev).
const LM_STUDIO_BASE: &str = "https://hitting-mighty-decade-feelings.trycloudflare.com";

/// Modèle par défaut à charger.
const DEFAULT_MODEL: &str = "glm-4-9b-chat-hf-GGUF";

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
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("reqwest client");
        Self {
            client,
            base_url: LM_STUDIO_BASE.to_string(),
        }
    }

    /// Liste les modèles chargés.
    pub async fn list_models(&self) -> Result<Vec<String>, LlmError> {
        let url = format!("{}/v1/models", self.base_url);
        let resp = self.client.get(&url).send().await
            .map_err(|e| LlmError::Network(e.to_string()))?;
        let body: ModelsResponse = resp.json().await
            .map_err(|e| LlmError::Api(e.to_string()))?;
        Ok(body.data.into_iter().map(|m| m.id).collect())
    }

    /// Vérifie qu'un modèle est chargé, sinon tente de charger le modèle par défaut.
    /// Retourne le nom du modèle prêt.
    pub async fn ensure_model(&self) -> Result<String, LlmError> {
        let models = self.list_models().await?;
        if let Some(m) = models.first() {
            return Ok(m.clone());
        }

        // Tenter de charger le modèle par défaut
        let url = format!("{}/v1/models/load", self.base_url);
        let body = serde_json::json!({ "model": DEFAULT_MODEL });
        let resp = self.client.post(&url).json(&body).send().await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if resp.status().is_success() {
            Ok(DEFAULT_MODEL.to_string())
        } else {
            // Peut-être que le load n'est pas supporté — re-check models
            let models = self.list_models().await?;
            models.into_iter().next().ok_or(LlmError::NoModel)
        }
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

        let resp = self.client.post(&url).json(&req).send().await
            .map_err(|e| LlmError::Network(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(LlmError::Api(format!("HTTP {status}: {body}")));
        }

        let body: ChatResponse = resp.json().await
            .map_err(|e| LlmError::Api(e.to_string()))?;

        body.choices.into_iter().next()
            .map(|c| c.message.content)
            .ok_or_else(|| LlmError::Api("Pas de réponse".into()))
    }
}
