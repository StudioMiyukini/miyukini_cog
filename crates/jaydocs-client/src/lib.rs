//! # jaydocs-client
//!
//! Adaptateur entre Alicia et **Jay Docs** (éditeur de documents collaboratif).

#![doc(html_root_url = "https://docs.miyukini.com/jaydocs-client")]

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

// @id service.jaydocs.alicia_client
// @role alicia_to_jaydocs_adapter
// @layer 7
// @do bridge_alicia_to_jaydocs
// @human Adaptateur Alicia → JayDocs : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError, RwLock};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'un document pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DocumentSummary {
    /// Identifiant stable.
    pub id: String,
    /// Titre du document.
    pub title: String,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Date de dernière modification (RFC 3339).
    pub updated_at: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayDocsError {
    /// Erreur backend.
    #[error("backend Jay Docs : {0}")]
    Backend(String),
    /// Paramètre invalide.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Document introuvable.
    #[error("document introuvable : {0}")]
    DocumentNotFound(String),
}

/// Trait async des opérations Jay Docs exposées à Alicia.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait JayDocsBackend: Send + Sync {
    /// Crée un nouveau document.
    async fn create_document(
        &self,
        title: &str,
        content: Option<&str>,
    ) -> Result<DocumentSummary, JayDocsError>;

    /// Liste les documents (filtre optionnel `search` substring sur le titre).
    async fn list_my_documents(
        &self,
        limit: usize,
        search: Option<&str>,
    ) -> Result<Vec<DocumentSummary>, JayDocsError>;

    /// Ajoute du contenu à la fin d'un document existant.
    async fn append_to_document(
        &self,
        document_id: &str,
        content: &str,
    ) -> Result<DocumentSummary, JayDocsError>;
}

/// Client Alicia ↔ Jay Docs.
pub struct JayDocsClient {
    backend: Arc<dyn JayDocsBackend>,
}

impl JayDocsClient {
    /// Construit un client.
    #[must_use]
    pub fn new(backend: Arc<dyn JayDocsBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JayDocsError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JayDocsError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayDocsError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_int(value: &Value, key: &str, default: usize) -> Result<usize, JayDocsError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .and_then(|v| usize::try_from(v).ok())
                .ok_or_else(|| {
                    JayDocsError::InvalidParam(format!(
                        "{key} doit être > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayDocsError::InvalidParam(format!(
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
        let content = match Self::parse_string(params, "content", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .create_document(&title, content.as_deref())
            .await
        {
            Ok(doc) => ServiceCallResult {
                success: true,
                data: json!({ "document": doc }),
                message: Some(format!("document « {title} » créé")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let search = match Self::parse_string(params, "search", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_my_documents(limit, search.as_deref()).await {
            Ok(docs) => {
                let count = docs.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "documents": docs, "count": count }),
                    message: Some(format!("{count} document(s) trouvé(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_append(&self, params: &Value) -> ServiceCallResult {
        let document_id = match Self::parse_string(params, "document_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("document_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let content = match Self::parse_string(params, "content", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("content manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if content.is_empty() {
            return ServiceCallResult::fail("content ne peut pas être vide");
        }
        match self.backend.append_to_document(&document_id, &content).await {
            Ok(doc) => ServiceCallResult {
                success: true,
                data: json!({ "document": doc }),
                message: Some(format!("contenu ajouté au document {document_id}")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayDocsClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaydocs" {
                    return ServiceCallResult::fail(format!(
                        "JayDocsClient appelé avec un service '{service}' inattendu"
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
            "create_document" => self.handle_create(params).await,
            "list_my_documents" => self.handle_list(params).await,
            "append_to_document" => self.handle_append(params).await,
            other => ServiceCallResult::fail(format!(
                "intent Jay Docs non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests.
#[derive(Default)]
pub struct FakeJayDocsBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    documents: Vec<DocumentSummary>,
    contents: std::collections::HashMap<String, String>,
    fail_with: Option<String>,
    seq: usize,
}

impl FakeJayDocsBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Force toutes les méthodes à renvoyer une erreur.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }

    /// Snapshot des documents.
    pub fn snapshot(&self) -> Vec<DocumentSummary> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).documents.clone()
    }

    /// Lit le contenu interne d'un document (pour vérifier append).
    pub fn content_of(&self, id: &str) -> Option<String> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).contents.get(id).cloned()
    }
}

#[async_trait]
impl JayDocsBackend for FakeJayDocsBackend {
    async fn create_document(
        &self,
        title: &str,
        content: Option<&str>,
    ) -> Result<DocumentSummary, JayDocsError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayDocsError::Backend(msg.clone()));
        }
        state.seq += 1;
        let now = Utc::now().to_rfc3339();
        let id = format!("doc_{}", state.seq);
        let doc = DocumentSummary {
            id: id.clone(),
            title: title.to_string(),
            created_at: now.clone(),
            updated_at: now,
        };
        state.documents.push(doc.clone());
        state
            .contents
            .insert(id, content.unwrap_or("").to_string());
        Ok(doc)
    }

    async fn list_my_documents(
        &self,
        limit: usize,
        search: Option<&str>,
    ) -> Result<Vec<DocumentSummary>, JayDocsError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayDocsError::Backend(msg.clone()));
        }
        let q = search.map(str::to_lowercase);
        let mut filtered: Vec<DocumentSummary> = state
            .documents
            .iter()
            .filter(|d| {
                q.as_ref()
                    .is_none_or(|q| d.title.to_lowercase().contains(q))
            })
            .cloned()
            .collect();
        filtered.reverse();
        filtered.truncate(limit);
        Ok(filtered)
    }

    async fn append_to_document(
        &self,
        document_id: &str,
        content: &str,
    ) -> Result<DocumentSummary, JayDocsError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayDocsError::Backend(msg.clone()));
        }
        if !state.documents.iter().any(|d| d.id == document_id) {
            return Err(JayDocsError::DocumentNotFound(document_id.to_string()));
        }
        let now = Utc::now().to_rfc3339();
        // Append to internal buffer
        let entry = state
            .contents
            .entry(document_id.to_string())
            .or_default();
        entry.push_str(content);
        // Update timestamp
        let doc = state
            .documents
            .iter_mut()
            .find(|d| d.id == document_id)
            .ok_or_else(|| JayDocsError::DocumentNotFound(document_id.to_string()))?;
        doc.updated_at = now;
        Ok(doc.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_document_returns_summary() {
        let backend = Arc::new(FakeJayDocsBackend::new());
        let client = JayDocsClient::new(backend.clone());
        let r = client
            .call(
                "jaydocs.create_document",
                &json!({"title": "Mon doc", "content": "intro"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["document"]["title"], "Mon doc");
        assert_eq!(backend.snapshot().len(), 1);
    }

    #[tokio::test]
    async fn create_rejects_empty_title() {
        let client = JayDocsClient::new(Arc::new(FakeJayDocsBackend::new()));
        let r = client
            .call("jaydocs.create_document", &json!({"title": "  "}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_filters_by_search() {
        let backend = Arc::new(FakeJayDocsBackend::new());
        let client = JayDocsClient::new(backend);
        client
            .call("jaydocs.create_document", &json!({"title": "Projet Alpha"}))
            .await;
        client
            .call("jaydocs.create_document", &json!({"title": "Notes Beta"}))
            .await;
        let r = client
            .call("jaydocs.list_my_documents", &json!({"search": "alpha"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 1);
    }

    #[tokio::test]
    async fn append_to_unknown_document_fails() {
        let client = JayDocsClient::new(Arc::new(FakeJayDocsBackend::new()));
        let r = client
            .call(
                "jaydocs.append_to_document",
                &json!({"document_id": "ghost", "content": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn append_concatenates_content() {
        let backend = Arc::new(FakeJayDocsBackend::new());
        let client = JayDocsClient::new(backend.clone());
        client
            .call(
                "jaydocs.create_document",
                &json!({"title": "Test", "content": "AAA"}),
            )
            .await;
        client
            .call(
                "jaydocs.append_to_document",
                &json!({"document_id": "doc_1", "content": "BBB"}),
            )
            .await;
        assert_eq!(backend.content_of("doc_1").as_deref(), Some("AAABBB"));
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JayDocsClient::new(Arc::new(FakeJayDocsBackend::new()));
        let r = client.call("jaydocs.delete_all", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JayDocsClient::new(Arc::new(FakeJayDocsBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayDocsBackend::new());
        backend.force_failure("DB locked");
        let client = JayDocsClient::new(backend);
        let r = client
            .call("jaydocs.create_document", &json!({"title": "x"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("DB locked"));
    }
}
