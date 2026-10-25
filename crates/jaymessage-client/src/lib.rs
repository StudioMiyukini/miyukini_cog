//! # jaymessage-client
//!
//! Adaptateur entre Alicia et **Jay Message** (messagerie instantanée du COG).

#![doc(html_root_url = "https://docs.miyukini.com/jaymessage-client")]

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

// @id service.jaymessage.alicia_client
// @role alicia_to_jaymessage_adapter
// @layer 7
// @do bridge_alicia_to_jaymessage
// @human Adaptateur Alicia → JayMessage : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError, RwLock};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'un thread de discussion pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ThreadSummary {
    /// Identifiant stable.
    pub id: String,
    /// Nom du thread (groupe ou nom du correspondant 1-1).
    pub name: String,
    /// Dernier message (preview).
    pub last_message_preview: String,
    /// Auteur du dernier message.
    pub last_message_from: String,
    /// Date du dernier message (RFC 3339).
    pub last_message_at: String,
    /// Nombre de messages non lus dans ce thread.
    pub unread_count: u32,
}

/// Représentation simplifiée d'un message individuel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageSummary {
    /// Identifiant stable.
    pub id: String,
    /// Thread parent.
    pub thread_id: String,
    /// Auteur (libellé court ou identifiant).
    pub from: String,
    /// Corps texte brut (potentiellement tronqué).
    pub body: String,
    /// Date d'envoi (RFC 3339).
    pub sent_at: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayMessageError {
    /// Erreur réseau ou DB côté backend.
    #[error("backend JayMessage : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Thread cible introuvable.
    #[error("thread introuvable : {0}")]
    ThreadNotFound(String),
}

/// Trait abstrait pour les opérations Jay Message exposables à Alicia.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait JayMessageBackend: Send + Sync {
    /// Envoie un message dans un thread. Retourne le `MessageSummary` posé.
    async fn send_message(
        &self,
        thread_id: &str,
        body: &str,
        reply_to: Option<&str>,
    ) -> Result<MessageSummary, JayMessageError>;

    /// Liste les threads récents (du plus récent au plus ancien).
    async fn list_recent_threads(
        &self,
        limit: usize,
        unread_only: bool,
    ) -> Result<Vec<ThreadSummary>, JayMessageError>;

    /// Recherche dans les messages (matching simple : substring).
    async fn search_threads(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MessageSummary>, JayMessageError>;
}

/// Le client Alicia ↔ JayMessage.
pub struct JayMessageClient {
    backend: Arc<dyn JayMessageBackend>,
}

impl JayMessageClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayMessageBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JayMessageError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JayMessageError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayMessageError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_int(value: &Value, key: &str, default: usize) -> Result<usize, JayMessageError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .and_then(|v| usize::try_from(v).ok())
                .ok_or_else(|| {
                    JayMessageError::InvalidParam(format!(
                        "{key} doit être un entier > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayMessageError::InvalidParam(format!(
                "{key} doit être un entier, reçu {other}"
            ))),
        }
    }

    fn parse_bool(value: &Value, key: &str, default: bool) -> Result<bool, JayMessageError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(other) => Err(JayMessageError::InvalidParam(format!(
                "{key} doit être un booléen, reçu {other}"
            ))),
        }
    }

    async fn handle_send(&self, params: &Value) -> ServiceCallResult {
        let thread_id = match Self::parse_string(params, "thread_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("thread_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let body = match Self::parse_string(params, "body", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("body manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if body.trim().is_empty() {
            return ServiceCallResult::fail("body ne peut pas être vide");
        }
        let reply_to = match Self::parse_string(params, "reply_to", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .send_message(&thread_id, &body, reply_to.as_deref())
            .await
        {
            Ok(msg) => ServiceCallResult {
                success: true,
                data: json!({ "message": msg }),
                message: Some(format!("message envoyé dans le thread {thread_id}")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_threads(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let unread_only = match Self::parse_bool(params, "unread_only", false) {
            Ok(b) => b,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_recent_threads(limit, unread_only).await {
            Ok(threads) => {
                let count = threads.len();
                let label = if unread_only { "non lu(s)" } else { "récent(s)" };
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "threads": threads,
                        "count": count,
                        "unread_only": unread_only,
                    }),
                    message: Some(format!("{count} thread(s) {label}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_search(&self, params: &Value) -> ServiceCallResult {
        let query = match Self::parse_string(params, "query", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("query manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if query.trim().is_empty() {
            return ServiceCallResult::fail("query ne peut pas être vide");
        }
        let limit = match Self::parse_int(params, "limit", 20) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.search_threads(&query, limit).await {
            Ok(messages) => {
                let count = messages.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "messages": messages,
                        "count": count,
                        "query": query,
                    }),
                    message: Some(format!("{count} message(s) trouvé(s) pour « {query} »")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayMessageClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaymessage" {
                    return ServiceCallResult::fail(format!(
                        "JayMessageClient appelé avec un service '{service}' inattendu"
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
            "send_message" => self.handle_send(params).await,
            "list_recent_threads" => self.handle_list_threads(params).await,
            "search_threads" => self.handle_search(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayMessage non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E.
#[derive(Default)]
pub struct FakeJayMessageBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    threads: Vec<ThreadSummary>,
    messages: Vec<MessageSummary>,
    sent: Vec<MessageSummary>,
    fail_with: Option<String>,
    seq: usize,
}

impl FakeJayMessageBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un thread fictif.
    pub fn push_thread(&self, thread: ThreadSummary) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).threads.push(thread);
    }

    /// Ajoute un message indexable par `search_threads`.
    pub fn push_message(&self, msg: MessageSummary) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).messages.push(msg);
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }

    /// Snapshot des messages envoyés via send_message (utile en test).
    pub fn snapshot_sent(&self) -> Vec<MessageSummary> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).sent.clone()
    }
}

#[async_trait]
impl JayMessageBackend for FakeJayMessageBackend {
    async fn send_message(
        &self,
        thread_id: &str,
        body: &str,
        _reply_to: Option<&str>,
    ) -> Result<MessageSummary, JayMessageError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayMessageError::Backend(msg.clone()));
        }
        // Vérifie que le thread existe (si on en a)
        if !state.threads.is_empty() && !state.threads.iter().any(|t| t.id == thread_id) {
            return Err(JayMessageError::ThreadNotFound(thread_id.to_string()));
        }
        state.seq += 1;
        let m = MessageSummary {
            id: format!("msg_{}", state.seq),
            thread_id: thread_id.to_string(),
            from: "me".to_string(),
            body: body.to_string(),
            sent_at: Utc::now().to_rfc3339(),
        };
        state.sent.push(m.clone());
        Ok(m)
    }

    async fn list_recent_threads(
        &self,
        limit: usize,
        unread_only: bool,
    ) -> Result<Vec<ThreadSummary>, JayMessageError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayMessageError::Backend(msg.clone()));
        }
        let mut filtered: Vec<ThreadSummary> = state
            .threads
            .iter()
            .filter(|t| !unread_only || t.unread_count > 0)
            .cloned()
            .collect();
        filtered.reverse();
        filtered.truncate(limit);
        Ok(filtered)
    }

    async fn search_threads(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MessageSummary>, JayMessageError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayMessageError::Backend(msg.clone()));
        }
        let q = query.to_lowercase();
        let mut hits: Vec<MessageSummary> = state
            .messages
            .iter()
            .filter(|m| {
                m.body.to_lowercase().contains(&q) || m.from.to_lowercase().contains(&q)
            })
            .cloned()
            .collect();
        hits.truncate(limit);
        Ok(hits)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn thread(id: &str, name: &str, unread: u32) -> ThreadSummary {
        ThreadSummary {
            id: id.into(),
            name: name.into(),
            last_message_preview: "...".into(),
            last_message_from: "x".into(),
            last_message_at: "2026-05-08T10:00:00Z".into(),
            unread_count: unread,
        }
    }

    fn msg(id: &str, thread_id: &str, from: &str, body: &str) -> MessageSummary {
        MessageSummary {
            id: id.into(),
            thread_id: thread_id.into(),
            from: from.into(),
            body: body.into(),
            sent_at: "2026-05-08T10:00:00Z".into(),
        }
    }

    #[tokio::test]
    async fn send_message_success() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        let client = JayMessageClient::new(backend.clone());
        let r = client
            .call(
                "jaymessage.send_message",
                &json!({"thread_id": "t1", "body": "hello"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["message"]["thread_id"], "t1");
        assert_eq!(r.data["message"]["body"], "hello");
        assert_eq!(backend.snapshot_sent().len(), 1);
    }

    #[tokio::test]
    async fn send_message_to_unknown_thread_fails() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        backend.push_thread(thread("t1", "Jean", 0));
        let client = JayMessageClient::new(backend);
        let r = client
            .call(
                "jaymessage.send_message",
                &json!({"thread_id": "ghost", "body": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn send_message_rejects_empty_body() {
        let client = JayMessageClient::new(Arc::new(FakeJayMessageBackend::new()));
        let r = client
            .call(
                "jaymessage.send_message",
                &json!({"thread_id": "t1", "body": "  "}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("body"));
    }

    #[tokio::test]
    async fn send_message_rejects_missing_required() {
        let client = JayMessageClient::new(Arc::new(FakeJayMessageBackend::new()));
        let r = client
            .call("jaymessage.send_message", &json!({"thread_id": "t1"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("body"));
    }

    #[tokio::test]
    async fn list_recent_threads_default_10() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        for i in 0..15_u32 {
            backend.push_thread(thread(&format!("t{i}"), &format!("nom{i}"), i % 2));
        }
        let client = JayMessageClient::new(backend);
        let r = client
            .call("jaymessage.list_recent_threads", &json!({}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 10);
    }

    #[tokio::test]
    async fn list_recent_threads_unread_only_filters() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        backend.push_thread(thread("t1", "n", 2));
        backend.push_thread(thread("t2", "n", 0));
        backend.push_thread(thread("t3", "n", 5));
        let client = JayMessageClient::new(backend);
        let r = client
            .call(
                "jaymessage.list_recent_threads",
                &json!({"unread_only": true}),
            )
            .await;
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn search_threads_matches_body_or_from_case_insensitive() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        backend.push_message(msg("a", "t1", "alice", "Salut, comment va le projet"));
        backend.push_message(msg("b", "t2", "bob", "RAS"));
        backend.push_message(msg("c", "t3", "Projet-X-bot", "ok"));
        let client = JayMessageClient::new(backend);
        let r = client
            .call("jaymessage.search_threads", &json!({"query": "PROJET"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn search_rejects_empty_query() {
        let client = JayMessageClient::new(Arc::new(FakeJayMessageBackend::new()));
        let r = client
            .call("jaymessage.search_threads", &json!({"query": "  "}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JayMessageClient::new(Arc::new(FakeJayMessageBackend::new()));
        let r = client.call("jaymessage.delete_all", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JayMessageClient::new(Arc::new(FakeJayMessageBackend::new()));
        let r = client.call("jaymail.send_mail", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayMessageBackend::new());
        backend.force_failure("réseau down");
        let client = JayMessageClient::new(backend);
        let r = client
            .call("jaymessage.list_recent_threads", &json!({}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("réseau down"));
    }
}
