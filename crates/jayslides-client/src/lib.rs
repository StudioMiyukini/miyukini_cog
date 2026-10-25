//! # jayslides-client
//!
//! Adaptateur entre Alicia et **Jay Slides** (présentations collaboratives).

#![doc(html_root_url = "https://docs.miyukini.com/jayslides-client")]

// C-10 : aucun unwrap/expect/panic hors tests dans le code exposé au réseau.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

// @id service.jayslides.alicia_client
// @role alicia_to_jayslides_adapter
// @layer 7
// @do bridge_alicia_to_jayslides
// @human Adaptateur Alicia → JaySlides : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError, RwLock};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'une présentation pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PresentationSummary {
    /// Identifiant stable.
    pub id: String,
    /// Titre de la présentation.
    pub title: String,
    /// Nombre de slides.
    pub slide_count: usize,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Date de dernière modification (RFC 3339).
    pub updated_at: String,
}

/// Slide ajoutée à une présentation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlideSummary {
    /// Identifiant de la slide.
    pub id: String,
    /// Titre.
    pub title: String,
    /// Contenu (peut être vide).
    pub content: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JaySlidesError {
    /// Erreur backend.
    #[error("backend Jay Slides : {0}")]
    Backend(String),
    /// Paramètre invalide.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Présentation introuvable.
    #[error("présentation introuvable : {0}")]
    PresentationNotFound(String),
}

/// Trait async des opérations Jay Slides exposées à Alicia.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait JaySlidesBackend: Send + Sync {
    /// Crée une nouvelle présentation.
    async fn create_presentation(
        &self,
        title: &str,
    ) -> Result<PresentationSummary, JaySlidesError>;

    /// Liste les présentations.
    async fn list_my_presentations(
        &self,
        limit: usize,
    ) -> Result<Vec<PresentationSummary>, JaySlidesError>;

    /// Ajoute une slide à une présentation existante.
    async fn add_slide(
        &self,
        presentation_id: &str,
        title: &str,
        content: Option<&str>,
    ) -> Result<(PresentationSummary, SlideSummary), JaySlidesError>;
}

/// Client Alicia ↔ Jay Slides.
pub struct JaySlidesClient {
    backend: Arc<dyn JaySlidesBackend>,
}

impl JaySlidesClient {
    /// Construit un client.
    #[must_use]
    pub fn new(backend: Arc<dyn JaySlidesBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JaySlidesError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JaySlidesError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JaySlidesError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_int(value: &Value, key: &str, default: usize) -> Result<usize, JaySlidesError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .and_then(|v| usize::try_from(v).ok())
                .ok_or_else(|| {
                    JaySlidesError::InvalidParam(format!("{key} doit être > 0, reçu {n}"))
                }),
            Some(other) => Err(JaySlidesError::InvalidParam(format!(
                "{key} doit être un entier, reçu {other}"
            ))),
        }
    }

    async fn handle_create(&self, params: &Value) -> ServiceCallResult {
        let title = match Self::parse_string(params, "title", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("title manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if title.trim().is_empty() {
            return ServiceCallResult::fail("title ne peut pas être vide");
        }
        match self.backend.create_presentation(&title).await {
            Ok(pres) => ServiceCallResult {
                success: true,
                data: json!({ "presentation": pres }),
                message: Some(format!("présentation « {title} » créée")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_my_presentations(limit).await {
            Ok(presentations) => {
                let count = presentations.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "presentations": presentations, "count": count }),
                    message: Some(format!("{count} présentation(s) trouvée(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_add_slide(&self, params: &Value) -> ServiceCallResult {
        let presentation_id = match Self::parse_string(params, "presentation_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("presentation_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let title = match Self::parse_string(params, "title", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("title manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if title.trim().is_empty() {
            return ServiceCallResult::fail("title ne peut pas être vide");
        }
        let content = match Self::parse_string(params, "content", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .add_slide(&presentation_id, &title, content.as_deref())
            .await
        {
            Ok((pres, slide)) => ServiceCallResult {
                success: true,
                data: json!({ "presentation": pres, "slide": slide }),
                message: Some(format!("slide « {title} » ajoutée")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JaySlidesClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jayslides" {
                    return ServiceCallResult::fail(format!(
                        "JaySlidesClient appelé avec un service '{service}' inattendu"
                    ));
                }
                intent
            }
            Err(_) => {
                return ServiceCallResult::fail(format!(
                    "qualified_id invalide : '{qualified_id}'"
                ));
            }
        };
        match intent.as_str() {
            "create_presentation" => self.handle_create(params).await,
            "list_my_presentations" => self.handle_list(params).await,
            "add_slide" => self.handle_add_slide(params).await,
            other => ServiceCallResult::fail(format!(
                "intent Jay Slides non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests.
#[derive(Default)]
pub struct FakeJaySlidesBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    presentations: Vec<PresentationSummary>,
    slides: std::collections::HashMap<String, Vec<SlideSummary>>,
    fail_with: Option<String>,
    seq: usize,
    slide_seq: usize,
}

impl FakeJaySlidesBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Force toutes les méthodes à renvoyer une erreur.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }

    /// Snapshot des présentations.
    pub fn snapshot(&self) -> Vec<PresentationSummary> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).presentations.clone()
    }

    /// Slides d'une présentation.
    pub fn slides_of(&self, id: &str) -> Vec<SlideSummary> {
        self.inner
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .slides
            .get(id)
            .cloned()
            .unwrap_or_default()
    }
}

#[async_trait]
impl JaySlidesBackend for FakeJaySlidesBackend {
    async fn create_presentation(
        &self,
        title: &str,
    ) -> Result<PresentationSummary, JaySlidesError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySlidesError::Backend(msg.clone()));
        }
        state.seq += 1;
        let now = Utc::now().to_rfc3339();
        let id = format!("pres_{}", state.seq);
        let pres = PresentationSummary {
            id: id.clone(),
            title: title.to_string(),
            slide_count: 0,
            created_at: now.clone(),
            updated_at: now,
        };
        state.presentations.push(pres.clone());
        state.slides.insert(id, Vec::new());
        Ok(pres)
    }

    async fn list_my_presentations(
        &self,
        limit: usize,
    ) -> Result<Vec<PresentationSummary>, JaySlidesError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySlidesError::Backend(msg.clone()));
        }
        let mut all: Vec<PresentationSummary> = state.presentations.clone();
        all.reverse();
        all.truncate(limit);
        Ok(all)
    }

    async fn add_slide(
        &self,
        presentation_id: &str,
        title: &str,
        content: Option<&str>,
    ) -> Result<(PresentationSummary, SlideSummary), JaySlidesError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySlidesError::Backend(msg.clone()));
        }
        if !state.presentations.iter().any(|p| p.id == presentation_id) {
            return Err(JaySlidesError::PresentationNotFound(
                presentation_id.to_string(),
            ));
        }
        state.slide_seq += 1;
        let slide = SlideSummary {
            id: format!("slide_{}", state.slide_seq),
            title: title.to_string(),
            content: content.unwrap_or("").to_string(),
        };
        state
            .slides
            .entry(presentation_id.to_string())
            .or_default()
            .push(slide.clone());
        let now = Utc::now().to_rfc3339();
        let pres = state
            .presentations
            .iter_mut()
            .find(|p| p.id == presentation_id)
            .ok_or_else(|| JaySlidesError::PresentationNotFound(presentation_id.to_string()))?;
        pres.slide_count += 1;
        pres.updated_at = now;
        Ok((pres.clone(), slide))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_presentation_returns_summary() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        let client = JaySlidesClient::new(backend.clone());
        let r = client
            .call(
                "jayslides.create_presentation",
                &json!({"title": "Pitch Q1"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["presentation"]["title"], "Pitch Q1");
        assert_eq!(r.data["presentation"]["slide_count"], 0);
        assert_eq!(backend.snapshot().len(), 1);
    }

    #[tokio::test]
    async fn create_rejects_empty_title() {
        let client = JaySlidesClient::new(Arc::new(FakeJaySlidesBackend::new()));
        let r = client
            .call("jayslides.create_presentation", &json!({"title": "   "}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_returns_all() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        let client = JaySlidesClient::new(backend);
        client
            .call("jayslides.create_presentation", &json!({"title": "A"}))
            .await;
        client
            .call("jayslides.create_presentation", &json!({"title": "B"}))
            .await;
        let r = client
            .call("jayslides.list_my_presentations", &json!({}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn add_slide_increments_count() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        let client = JaySlidesClient::new(backend.clone());
        client
            .call("jayslides.create_presentation", &json!({"title": "X"}))
            .await;
        let r = client
            .call(
                "jayslides.add_slide",
                &json!({"presentation_id": "pres_1", "title": "Intro", "content": "Bonjour"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["presentation"]["slide_count"], 1);
        assert_eq!(backend.slides_of("pres_1").len(), 1);
    }

    #[tokio::test]
    async fn add_slide_rejects_empty_title() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        let client = JaySlidesClient::new(backend);
        let r = client
            .call(
                "jayslides.add_slide",
                &json!({"presentation_id": "pres_1", "title": "  "}),
            )
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn add_slide_unknown_presentation_fails() {
        let client = JaySlidesClient::new(Arc::new(FakeJaySlidesBackend::new()));
        let r = client
            .call(
                "jayslides.add_slide",
                &json!({"presentation_id": "ghost", "title": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JaySlidesClient::new(Arc::new(FakeJaySlidesBackend::new()));
        let r = client.call("jayslides.delete_all", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JaySlidesClient::new(Arc::new(FakeJaySlidesBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        backend.force_failure("network down");
        let client = JaySlidesClient::new(backend);
        let r = client
            .call("jayslides.create_presentation", &json!({"title": "x"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("network down"));
    }

    #[tokio::test]
    async fn slide_without_content_uses_empty() {
        let backend = Arc::new(FakeJaySlidesBackend::new());
        let client = JaySlidesClient::new(backend.clone());
        client
            .call("jayslides.create_presentation", &json!({"title": "X"}))
            .await;
        let r = client
            .call(
                "jayslides.add_slide",
                &json!({"presentation_id": "pres_1", "title": "Conclusion"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(backend.slides_of("pres_1")[0].content, "");
    }
}
