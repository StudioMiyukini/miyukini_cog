//! # jaymail-client
//!
//! Adaptateur entre Alicia et **Jay Mail** (client email Miyukini).
//!
//! Capabilités exposées (cf. `crates/jaybureau-suite/jaymail/service.manifest.json`) :
//! - `send_mail` (write, `always_for_writes`)
//! - `list_recent_inbox` (read)
//! - `search_mail` (read)
//!
//! Le **backend** (`JayMailBackend`) est un trait async — l'implémentation
//! réelle (SMTP via `lettre` + IMAP via `imap`) sera branchée au runtime.
//! P-Mail livre un `FakeJayMailBackend` programmable pour les tests.

#![doc(html_root_url = "https://docs.miyukini.com/jaymail-client")]

// @id service.jaymail.alicia_client
// @role alicia_to_jaymail_adapter
// @layer 7
// @do bridge_alicia_to_jaymail
// @human Adaptateur Alicia → JayMail : route les intents vers un backend pluggable.

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'un courriel pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MailSummary {
    /// Identifiant stable (uid IMAP ou message-id).
    pub id: String,
    /// Expéditeur (adresse).
    pub from: String,
    /// Sujet.
    pub subject: String,
    /// Aperçu du corps (premiers caractères).
    pub preview: String,
    /// Date de réception (RFC 3339).
    pub received_at: String,
    /// `true` si non lu.
    pub unread: bool,
}

/// Résultat d'un envoi SMTP.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SentMail {
    /// Identifiant attribué par le serveur SMTP (Message-ID).
    pub message_id: String,
    /// Destinataire principal.
    pub to: String,
    /// Sujet envoyé.
    pub subject: String,
    /// Date d'envoi (RFC 3339).
    pub sent_at: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayMailError {
    /// Erreur SMTP / IMAP côté backend.
    #[error("backend JayMail : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Adresse email invalide.
    #[error("adresse email invalide : {0}")]
    InvalidEmail(String),
}

/// Trait abstrait pour les opérations Jay Mail exposables à Alicia.
#[async_trait]
pub trait JayMailBackend: Send + Sync {
    /// Envoie un mail via SMTP. Retourne le `SentMail` confirmé.
    async fn send_mail(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
    ) -> Result<SentMail, JayMailError>;

    /// Liste les mails récents de la boîte de réception.
    async fn list_recent_inbox(
        &self,
        limit: usize,
        unread_only: bool,
    ) -> Result<Vec<MailSummary>, JayMailError>;

    /// Recherche dans la boîte de réception (matching simple : substring sur
    /// from / subject / preview).
    async fn search_mail(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MailSummary>, JayMailError>;
}

/// Le client Alicia ↔ JayMail.
pub struct JayMailClient {
    backend: Arc<dyn JayMailBackend>,
}

impl JayMailClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayMailBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JayMailError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JayMailError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayMailError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_int(value: &Value, key: &str, default: usize) -> Result<usize, JayMailError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .map(|v| v as usize)
                .ok_or_else(|| {
                    JayMailError::InvalidParam(format!(
                        "{key} doit être un entier > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayMailError::InvalidParam(format!(
                "{key} doit être un entier, reçu {other}"
            ))),
        }
    }

    fn parse_bool(value: &Value, key: &str, default: bool) -> Result<bool, JayMailError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Bool(b)) => Ok(*b),
            Some(other) => Err(JayMailError::InvalidParam(format!(
                "{key} doit être un booléen, reçu {other}"
            ))),
        }
    }

    fn parse_address_list(
        params: &Value,
        key: &str,
    ) -> Result<Option<Vec<String>>, JayMailError> {
        match Self::parse_string(params, key, false)? {
            None => Ok(None),
            Some(s) if s.trim().is_empty() => Ok(None),
            Some(s) => {
                let list: Vec<String> = s
                    .split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect();
                if list.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(list))
                }
            }
        }
    }

    /// Validation très simple d'une adresse email — exige un `@` et un point
    /// après. Suffisant en P-Mail pour rejeter les pires erreurs LLM ; la
    /// vraie validation est faite par lettre côté backend.
    fn looks_like_email(s: &str) -> bool {
        if let Some((local, domain)) = s.trim().split_once('@') {
            !local.is_empty() && domain.contains('.') && !domain.starts_with('.')
        } else {
            false
        }
    }

    async fn handle_send(&self, params: &Value) -> ServiceCallResult {
        let to = match Self::parse_string(params, "to", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("to manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if !Self::looks_like_email(&to) {
            return ServiceCallResult::fail(format!("to '{to}' n'a pas la forme d'un email"));
        }
        let subject = match Self::parse_string(params, "subject", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("subject manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let body = match Self::parse_string(params, "body", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("body manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let cc = match Self::parse_address_list(params, "cc") {
            Ok(v) => v,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let bcc = match Self::parse_address_list(params, "bcc") {
            Ok(v) => v,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        // Validation de chaque cc/bcc
        for list in [&cc, &bcc].into_iter().flatten() {
            for addr in list {
                if !Self::looks_like_email(addr) {
                    return ServiceCallResult::fail(format!(
                        "destinataire en copie '{addr}' n'a pas la forme d'un email"
                    ));
                }
            }
        }
        match self.backend.send_mail(&to, &subject, &body, cc, bcc).await {
            Ok(sent) => ServiceCallResult {
                success: true,
                data: json!({ "sent": sent }),
                message: Some(format!("mail envoyé à {to} (sujet : « {subject} »)")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_inbox(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let unread_only = match Self::parse_bool(params, "unread_only", false) {
            Ok(b) => b,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_recent_inbox(limit, unread_only).await {
            Ok(mails) => {
                let count = mails.len();
                let label = if unread_only { "non lu(s)" } else { "récent(s)" };
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "mails": mails,
                        "count": count,
                        "unread_only": unread_only,
                    }),
                    message: Some(format!("{count} mail(s) {label}")),
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
        match self.backend.search_mail(&query, limit).await {
            Ok(mails) => {
                let count = mails.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "mails": mails,
                        "count": count,
                        "query": query,
                    }),
                    message: Some(format!("{count} résultat(s) pour « {query} »")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayMailClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaymail" {
                    return ServiceCallResult::fail(format!(
                        "JayMailClient appelé avec un service '{service}' inattendu"
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
            "send_mail" => self.handle_send(params).await,
            "list_recent_inbox" => self.handle_list_inbox(params).await,
            "search_mail" => self.handle_search(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayMail non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayMailBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    inbox: Vec<MailSummary>,
    sent: Vec<SentMail>,
    fail_with: Option<String>,
    seq: usize,
}

impl FakeJayMailBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un mail dans la boîte de réception fictive.
    pub fn push_inbox(&self, mail: MailSummary) {
        self.inner.write().unwrap().inbox.push(mail);
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap().fail_with = Some(message.into());
    }

    /// Snapshot des mails envoyés (utile en test).
    pub fn snapshot_sent(&self) -> Vec<SentMail> {
        self.inner.read().unwrap().sent.clone()
    }
}

#[async_trait]
impl JayMailBackend for FakeJayMailBackend {
    async fn send_mail(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        _cc: Option<Vec<String>>,
        _bcc: Option<Vec<String>>,
    ) -> Result<SentMail, JayMailError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayMailError::Backend(msg.clone()));
        }
        state.seq += 1;
        let sent = SentMail {
            message_id: format!("<msg{}@miyukini.test>", state.seq),
            to: to.to_string(),
            subject: subject.to_string(),
            sent_at: Utc::now().to_rfc3339(),
        };
        // body inutilisé en mock — on capture quand même pour ne pas warner
        let _ = body;
        state.sent.push(sent.clone());
        Ok(sent)
    }

    async fn list_recent_inbox(
        &self,
        limit: usize,
        unread_only: bool,
    ) -> Result<Vec<MailSummary>, JayMailError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayMailError::Backend(msg.clone()));
        }
        let mut filtered: Vec<MailSummary> = state
            .inbox
            .iter()
            .filter(|m| !unread_only || m.unread)
            .cloned()
            .collect();
        filtered.reverse();
        filtered.truncate(limit);
        Ok(filtered)
    }

    async fn search_mail(
        &self,
        query: &str,
        limit: usize,
    ) -> Result<Vec<MailSummary>, JayMailError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayMailError::Backend(msg.clone()));
        }
        let q = query.to_lowercase();
        let mut hits: Vec<MailSummary> = state
            .inbox
            .iter()
            .filter(|m| {
                m.from.to_lowercase().contains(&q)
                    || m.subject.to_lowercase().contains(&q)
                    || m.preview.to_lowercase().contains(&q)
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

    fn mail(id: &str, from: &str, subject: &str, preview: &str, unread: bool) -> MailSummary {
        MailSummary {
            id: id.into(),
            from: from.into(),
            subject: subject.into(),
            preview: preview.into(),
            received_at: "2026-05-08T10:00:00Z".into(),
            unread,
        }
    }

    #[tokio::test]
    async fn send_mail_success_returns_sent_summary() {
        let backend = Arc::new(FakeJayMailBackend::new());
        let client = JayMailClient::new(backend.clone());
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({
                    "to": "jean@example.com",
                    "subject": "test",
                    "body": "hello"
                }),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["sent"]["to"], "jean@example.com");
        assert!(r.data["sent"]["message_id"].as_str().unwrap().starts_with('<'));
        assert_eq!(backend.snapshot_sent().len(), 1);
    }

    #[tokio::test]
    async fn send_mail_rejects_invalid_email() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({
                    "to": "pas_un_email",
                    "subject": "x",
                    "body": "y"
                }),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("email"));
    }

    #[tokio::test]
    async fn send_mail_rejects_missing_required() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({"to": "jean@x.com", "subject": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("body"));
    }

    #[tokio::test]
    async fn send_mail_parses_cc_list() {
        let backend = Arc::new(FakeJayMailBackend::new());
        let client = JayMailClient::new(backend);
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({
                    "to": "jean@x.com",
                    "subject": "x",
                    "body": "y",
                    "cc": "alice@x.com, bob@x.com"
                }),
            )
            .await;
        assert!(r.success);
    }

    #[tokio::test]
    async fn send_mail_rejects_invalid_cc() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({
                    "to": "jean@x.com",
                    "subject": "x",
                    "body": "y",
                    "cc": "alice@x.com, pas_un_email"
                }),
            )
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_recent_returns_default_10() {
        let backend = Arc::new(FakeJayMailBackend::new());
        for i in 0..15 {
            backend.push_inbox(mail(
                &format!("m{i}"),
                "x@y.z",
                &format!("s{i}"),
                "p",
                i % 2 == 0,
            ));
        }
        let client = JayMailClient::new(backend);
        let r = client.call("jaymail.list_recent_inbox", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"], 10);
    }

    #[tokio::test]
    async fn list_recent_unread_only_filters() {
        let backend = Arc::new(FakeJayMailBackend::new());
        backend.push_inbox(mail("a", "x@y.z", "s", "p", true));
        backend.push_inbox(mail("b", "x@y.z", "s", "p", false));
        backend.push_inbox(mail("c", "x@y.z", "s", "p", true));
        let client = JayMailClient::new(backend);
        let r = client
            .call("jaymail.list_recent_inbox", &json!({"unread_only": true}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn search_mail_matches_substring_case_insensitive() {
        let backend = Arc::new(FakeJayMailBackend::new());
        backend.push_inbox(mail("a", "alice@x.com", "Facture mai", "à régler", false));
        backend.push_inbox(mail("b", "bob@x.com", "Vacances", "départ samedi", false));
        backend.push_inbox(mail("c", "boss@x.com", "Réunion", "facture à valider", false));
        let client = JayMailClient::new(backend);
        let r = client
            .call("jaymail.search_mail", &json!({"query": "FACTURE"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn search_mail_rejects_empty_query() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client
            .call("jaymail.search_mail", &json!({"query": "  "}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client.call("jaymail.delete_all", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("non supporté"));
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("inattendu"));
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayMailBackend::new());
        backend.force_failure("SMTP timeout");
        let client = JayMailClient::new(backend);
        let r = client
            .call(
                "jaymail.send_mail",
                &json!({"to": "x@y.z", "subject": "x", "body": "y"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("SMTP timeout"));
    }
}
