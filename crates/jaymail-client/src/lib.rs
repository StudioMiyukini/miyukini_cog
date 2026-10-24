//! # jaymail-client
//!
//! Adaptateur entre Alicia et **Jay Mail** (client email Miyukini).
//!
//! Capabilités exposées (cf. `crates/jaybureau-suite/jaymail/service.manifest.json`) :
//! - `send_mail` (write, `always_for_writes`)
//! - `list_recent_inbox` (read)
//! - `search_mail` (read)
//!
//! ## Garde-fous `send_mail` (R-018, C-50, C-51)
//!
//! `send_mail` est piloté par l'IA (tool call) : une injection de prompt peut
//! tenter d'exfiltrer des données vers un destinataire caché. En conséquence :
//! - `bcc` est **refusé** dès qu'il est présent dans les paramètres
//!   ([`JayMailError::BccRefused`]) ;
//! - `cc` est limité à [`MAX_CC_RECIPIENTS`] adresses ;
//! - tout paramètre inconnu est refusé (pas d'en-tête additionnel) ;
//! - adresses strictes (une seule par champ `to`, pas de CR/LF, ni `,`/`;`/`<>`),
//!   sujet et corps bornés (C-13) ;
//! - [`JayMailClient::confirmation_preview`] produit l'aperçu de confirmation
//!   listant **tous** les destinataires et le sujet, à partir du **même**
//!   parseur que l'exécution (aperçu et envoi ne peuvent pas diverger).
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

use std::sync::{Arc, PoisonError, RwLock};

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
    /// `bcc` refusé : un appel piloté par l'IA ne peut pas ajouter de
    /// destinataire invisible à la confirmation (R-018).
    #[error(
        "bcc refusé : les destinataires en copie cachée ne sont pas autorisés depuis l'assistant"
    )]
    BccRefused,
    /// Trop de destinataires en copie.
    #[error("trop de destinataires en copie : {count} (maximum {max})")]
    TooManyRecipients {
        /// Nombre reçu.
        count: usize,
        /// Maximum autorisé.
        max: usize,
    },
    /// Champ au-delà de sa borne (C-13).
    #[error("{field} trop long (maximum {max} octets)")]
    TooLong {
        /// Champ concerné.
        field: &'static str,
        /// Borne en octets.
        max: usize,
    },
    /// Paramètre non prévu par la capability : refusé (fail-closed).
    #[error("paramètre non autorisé : {0}")]
    UnknownParam(String),
    /// Caractère de contrôle (CR/LF…) dans un champ d'en-tête.
    #[error("caractère de contrôle interdit dans {0}")]
    ControlChar(&'static str),
}

/// Nombre maximal de destinataires en copie (`cc`) pour un envoi piloté par l'IA.
pub const MAX_CC_RECIPIENTS: usize = 5;
/// Longueur maximale d'une adresse email (octets, RFC 5321).
pub const MAX_ADDRESS_BYTES: usize = 254;
/// Longueur maximale du sujet (octets).
pub const MAX_SUBJECT_BYTES: usize = 512;
/// Longueur maximale du corps (octets).
pub const MAX_BODY_BYTES: usize = 64 * 1024;
/// Nombre maximal de mails retournés par une lecture (`limit`).
pub const MAX_LIST_LIMIT: usize = 100;
/// Longueur maximale d'une requête de recherche (octets).
pub const MAX_QUERY_BYTES: usize = 256;
/// Nombre de caractères du corps repris dans l'aperçu de confirmation.
pub const PREVIEW_BODY_CHARS: usize = 280;

/// Paramètres acceptés par `send_mail` (tout autre paramètre est refusé).
const SEND_MAIL_PARAMS: &[&str] = &["to", "subject", "body", "cc"];

/// Requête d'envoi validée — seule forme acceptée par l'exécution comme par
/// l'aperçu de confirmation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SendMailRequest {
    /// Destinataire principal (une seule adresse).
    pub to: String,
    /// Destinataires en copie (≤ [`MAX_CC_RECIPIENTS`]).
    pub cc: Vec<String>,
    /// Sujet.
    pub subject: String,
    /// Corps (texte brut).
    pub body: String,
}

/// Aperçu de confirmation d'un `send_mail` : **tous** les destinataires et le sujet.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MailPreview {
    /// Destinataire principal.
    pub to: String,
    /// Destinataires en copie.
    pub cc: Vec<String>,
    /// Sujet complet.
    pub subject: String,
    /// Début du corps (≤ [`PREVIEW_BODY_CHARS`] caractères).
    pub body_excerpt: String,
    /// Taille totale du corps (octets).
    pub body_bytes: usize,
    /// Nombre total de destinataires (to + cc).
    pub recipient_count: usize,
}

impl MailPreview {
    /// Construit l'aperçu d'une requête validée.
    #[must_use]
    pub fn from_request(req: &SendMailRequest) -> Self {
        let mut body_excerpt: String = req.body.chars().take(PREVIEW_BODY_CHARS).collect();
        if body_excerpt.len() < req.body.len() {
            body_excerpt.push('…');
        }
        Self {
            to: req.to.clone(),
            cc: req.cc.clone(),
            subject: req.subject.clone(),
            body_excerpt,
            body_bytes: req.body.len(),
            recipient_count: 1 + req.cc.len(),
        }
    }

    /// Texte de confirmation à présenter à l'utilisateur.
    #[must_use]
    pub fn render(&self) -> String {
        let cc = if self.cc.is_empty() {
            "(aucun)".to_string()
        } else {
            self.cc.join(", ")
        };
        format!(
            "Envoyer un courriel à {n} destinataire(s)\nÀ : {to}\nCc : {cc}\nSujet : « {subject} »\nCorps ({bytes} octets) : {excerpt}",
            n = self.recipient_count,
            to = self.to,
            subject = self.subject,
            bytes = self.body_bytes,
            excerpt = self.body_excerpt,
        )
    }
}

/// Trait abstrait pour les opérations Jay Mail exposables à Alicia.
// `async_trait` génère un `#[must_use]` redondant sur les méthodes.
#[allow(clippy::double_must_use)]
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
                .map(|v| usize::try_from(v).unwrap_or(usize::MAX))
                .ok_or_else(|| {
                    JayMailError::InvalidParam(format!("{key} doit être un entier > 0, reçu {n}"))
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

    /// Valide une adresse email unique, stricte : ASCII imprimable, un seul `@`,
    /// partie locale et domaine non vides, domaine avec un point, aucun
    /// séparateur (`,` `;` espace `<` `>`) ni caractère de contrôle. La
    /// validation complète reste faite côté backend (lettre).
    fn validate_address(addr: &str) -> Result<(), JayMailError> {
        let invalid = || JayMailError::InvalidEmail(addr.chars().take(64).collect());
        if addr.is_empty() || addr.len() > MAX_ADDRESS_BYTES {
            return Err(invalid());
        }
        if addr.chars().any(char::is_control) {
            return Err(JayMailError::ControlChar("adresse"));
        }
        let (local, domain) = addr.split_once('@').ok_or_else(invalid)?;
        let local_ok = !local.is_empty()
            && local.len() <= 64
            && local
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+/=?^_`{|}~.-".contains(&b));
        let domain_ok = domain.contains('.')
            && !domain.starts_with(['.', '-'])
            && !domain.ends_with(['.', '-'])
            && !domain.contains("..")
            && domain
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'.' || b == b'-');
        if local_ok && domain_ok {
            Ok(())
        } else {
            Err(invalid())
        }
    }

    /// Champ texte requis, borné ; `header` = interdit les caractères de contrôle.
    fn required_text(
        params: &Value,
        key: &'static str,
        max: usize,
        header: bool,
    ) -> Result<String, JayMailError> {
        let s = Self::parse_string(params, key, true)?
            .ok_or_else(|| JayMailError::InvalidParam(format!("{key} manquant")))?;
        if s.len() > max {
            return Err(JayMailError::TooLong { field: key, max });
        }
        if header && s.chars().any(char::is_control) {
            return Err(JayMailError::ControlChar(key));
        }
        Ok(s.to_string())
    }

    /// Analyse et valide les paramètres de `send_mail` — **seul** chemin vers
    /// une [`SendMailRequest`], partagé par l'exécution et l'aperçu.
    ///
    /// Refus : `bcc` présent (même vide ou nul), paramètre inconnu, plus de
    /// [`MAX_CC_RECIPIENTS`] `cc`, adresse invalide, champ trop long, CR/LF.
    pub fn parse_send_request(params: &Value) -> Result<SendMailRequest, JayMailError> {
        let obj = params
            .as_object()
            .ok_or_else(|| JayMailError::InvalidParam("paramètres attendus en objet".into()))?;
        if obj.contains_key("bcc") {
            return Err(JayMailError::BccRefused);
        }
        if let Some(k) = obj.keys().find(|k| !SEND_MAIL_PARAMS.contains(&k.as_str())) {
            return Err(JayMailError::UnknownParam(k.chars().take(64).collect()));
        }
        let to = Self::required_text(params, "to", MAX_ADDRESS_BYTES, true)?
            .trim()
            .to_string();
        Self::validate_address(&to)?;
        let subject = Self::required_text(params, "subject", MAX_SUBJECT_BYTES, true)?;
        let body = Self::required_text(params, "body", MAX_BODY_BYTES, false)?;

        let mut cc = Vec::new();
        if let Some(raw) = Self::parse_string(params, "cc", false)? {
            if raw.len() > MAX_CC_RECIPIENTS * (MAX_ADDRESS_BYTES + 2) {
                return Err(JayMailError::TooLong {
                    field: "cc",
                    max: MAX_CC_RECIPIENTS * (MAX_ADDRESS_BYTES + 2),
                });
            }
            for addr in raw.split(',').map(str::trim).filter(|x| !x.is_empty()) {
                Self::validate_address(addr)?;
                if addr != to && !cc.iter().any(|c| c == addr) {
                    cc.push(addr.to_string());
                }
            }
            if cc.len() > MAX_CC_RECIPIENTS {
                return Err(JayMailError::TooManyRecipients {
                    count: cc.len(),
                    max: MAX_CC_RECIPIENTS,
                });
            }
        }
        Ok(SendMailRequest {
            to,
            cc,
            subject,
            body,
        })
    }

    /// Aperçu de confirmation d'un `send_mail` : liste **tous** les
    /// destinataires (to + cc) et le sujet. Refuse exactement ce que
    /// l'exécution refuserait (même parseur).
    pub fn confirmation_preview(params: &Value) -> Result<MailPreview, JayMailError> {
        Self::parse_send_request(params).map(|req| MailPreview::from_request(&req))
    }

    async fn handle_send(&self, params: &Value) -> ServiceCallResult {
        let req = match Self::parse_send_request(params) {
            Ok(r) => r,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let preview = MailPreview::from_request(&req);
        let cc = if req.cc.is_empty() {
            None
        } else {
            Some(req.cc.clone())
        };
        // `bcc` n'est jamais transmis depuis un appel de capability (R-018).
        match self
            .backend
            .send_mail(&req.to, &req.subject, &req.body, cc, None)
            .await
        {
            Ok(sent) => {
                let cc_label = if req.cc.is_empty() {
                    String::new()
                } else {
                    format!(" (cc : {})", req.cc.join(", "))
                };
                ServiceCallResult {
                    success: true,
                    data: json!({ "sent": sent, "recipients": preview }),
                    message: Some(format!(
                        "mail envoyé à {}{cc_label} (sujet : « {} »)",
                        req.to, req.subject
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_inbox(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n.min(MAX_LIST_LIMIT),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let unread_only = match Self::parse_bool(params, "unread_only", false) {
            Ok(b) => b,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_recent_inbox(limit, unread_only).await {
            Ok(mails) => {
                let count = mails.len();
                let label = if unread_only {
                    "non lu(s)"
                } else {
                    "récent(s)"
                };
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
        if query.len() > MAX_QUERY_BYTES {
            return ServiceCallResult::fail(
                JayMailError::TooLong {
                    field: "query",
                    max: MAX_QUERY_BYTES,
                }
                .to_string(),
            );
        }
        let limit = match Self::parse_int(params, "limit", 20) {
            Ok(n) => n.min(MAX_LIST_LIMIT),
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
            other => ServiceCallResult::fail(format!("intent JayMail non supporté : '{other}'")),
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
    /// (to, cc, bcc) effectivement transmis au backend, par envoi.
    recipients: Vec<(String, Vec<String>, Vec<String>)>,
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
        self.inner
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .inbox
            .push(mail);
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .fail_with = Some(message.into());
    }

    /// Snapshot des destinataires (to, cc, bcc) reçus par le backend.
    pub fn snapshot_recipients(&self) -> Vec<(String, Vec<String>, Vec<String>)> {
        self.inner
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .recipients
            .clone()
    }

    /// Snapshot des mails envoyés (utile en test).
    pub fn snapshot_sent(&self) -> Vec<SentMail> {
        self.inner
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .sent
            .clone()
    }
}

#[async_trait]
impl JayMailBackend for FakeJayMailBackend {
    async fn send_mail(
        &self,
        to: &str,
        subject: &str,
        body: &str,
        cc: Option<Vec<String>>,
        bcc: Option<Vec<String>>,
    ) -> Result<SentMail, JayMailError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
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
        state.recipients.push((
            to.to_string(),
            cc.unwrap_or_default(),
            bcc.unwrap_or_default(),
        ));
        state.sent.push(sent.clone());
        Ok(sent)
    }

    async fn list_recent_inbox(
        &self,
        limit: usize,
        unread_only: bool,
    ) -> Result<Vec<MailSummary>, JayMailError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
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
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
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
        assert!(r.data["sent"]["message_id"]
            .as_str()
            .unwrap()
            .starts_with('<'));
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
        backend.push_inbox(mail(
            "c",
            "boss@x.com",
            "Réunion",
            "facture à valider",
            false,
        ));
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
    // ----- R-018 : bcc, cc bornés, aperçu complet -----

    fn send_params(extra: Value) -> Value {
        let mut p = json!({"to": "jean@x.com", "subject": "Facture", "body": "ci-joint"});
        if let (Some(obj), Some(add)) = (p.as_object_mut(), extra.as_object()) {
            for (k, v) in add {
                obj.insert(k.clone(), v.clone());
            }
        }
        p
    }

    #[tokio::test]
    async fn send_mail_refuse_bcc() {
        let backend = Arc::new(FakeJayMailBackend::new());
        let client = JayMailClient::new(backend.clone());
        for bcc in [json!("pirate@evil.com"), json!(""), Value::Null] {
            let r = client
                .call("jaymail.send_mail", &send_params(json!({ "bcc": bcc })))
                .await;
            assert!(!r.success, "bcc {bcc} accepté à tort");
            assert!(r.message.unwrap().contains("bcc refusé"));
        }
        assert_eq!(backend.snapshot_sent().len(), 0);
        assert!(matches!(
            JayMailClient::confirmation_preview(&send_params(json!({"bcc": "a@b.co"}))),
            Err(JayMailError::BccRefused)
        ));
    }

    #[tokio::test]
    async fn send_mail_ne_transmet_jamais_bcc_au_backend() {
        let backend = Arc::new(FakeJayMailBackend::new());
        let client = JayMailClient::new(backend.clone());
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"cc": "alice@x.com, bob@x.com"})),
            )
            .await;
        assert!(r.success);
        let rec = backend.snapshot_recipients();
        assert_eq!(rec.len(), 1);
        assert_eq!(rec[0].0, "jean@x.com");
        assert_eq!(
            rec[0].1,
            vec!["alice@x.com".to_string(), "bob@x.com".to_string()]
        );
        assert_eq!(rec[0].2.len(), 0);
        // Le résultat expose aussi tous les destinataires.
        assert_eq!(r.data["recipients"]["cc"][1], "bob@x.com");
        assert!(r.message.unwrap().contains("bob@x.com"));
    }

    #[tokio::test]
    async fn send_mail_refuse_trop_de_cc() {
        let backend = Arc::new(FakeJayMailBackend::new());
        let client = JayMailClient::new(backend.clone());
        let cc: Vec<String> = (0..=MAX_CC_RECIPIENTS)
            .map(|i| format!("u{i}@x.com"))
            .collect();
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"cc": cc.join(",")})),
            )
            .await;
        assert!(!r.success);
        assert_eq!(backend.snapshot_sent().len(), 0);
        // Exactement le maximum : accepté.
        let ok: Vec<String> = (0..MAX_CC_RECIPIENTS)
            .map(|i| format!("u{i}@x.com"))
            .collect();
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"cc": ok.join(",")})),
            )
            .await;
        assert!(r.success);
    }

    #[tokio::test]
    async fn send_mail_refuse_parametre_inconnu() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        for extra in [
            json!({"reply_to": "x@evil.com"}),
            json!({"headers": "Bcc: x@evil.com"}),
        ] {
            let r = client.call("jaymail.send_mail", &send_params(extra)).await;
            assert!(!r.success);
            assert!(r.message.unwrap().contains("non autorisé"));
        }
    }

    #[tokio::test]
    async fn send_mail_refuse_destinataires_caches_dans_to() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        for to in [
            "jean@x.com,pirate@evil.com",
            "jean@x.com; pirate@evil.com",
            "Jean <jean@x.com>",
            "jean@x.com\r\nBcc: pirate@evil.com",
            "a@@x.com",
            "jean@x..com",
        ] {
            let r = client
                .call(
                    "jaymail.send_mail",
                    &json!({"to": to, "subject": "s", "body": "b"}),
                )
                .await;
            assert!(!r.success, "to accepté à tort : {to}");
        }
    }

    #[tokio::test]
    async fn send_mail_refuse_injection_entete_sujet_et_tailles() {
        let client = JayMailClient::new(Arc::new(FakeJayMailBackend::new()));
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"subject": "Salut\r\nBcc: pirate@evil.com"})),
            )
            .await;
        assert!(!r.success);
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"subject": "s".repeat(MAX_SUBJECT_BYTES + 1)})),
            )
            .await;
        assert!(!r.success);
        let r = client
            .call(
                "jaymail.send_mail",
                &send_params(json!({"body": "b".repeat(MAX_BODY_BYTES + 1)})),
            )
            .await;
        assert!(!r.success);
    }

    #[test]
    fn apercu_liste_tous_les_destinataires_et_le_sujet() {
        let p = send_params(json!({"cc": "alice@x.com, bob@y.org, alice@x.com"}));
        let preview = JayMailClient::confirmation_preview(&p).unwrap();
        assert_eq!(preview.to, "jean@x.com");
        assert_eq!(
            preview.cc,
            vec!["alice@x.com".to_string(), "bob@y.org".to_string()]
        );
        assert_eq!(preview.recipient_count, 3);
        let text = preview.render();
        for needle in ["jean@x.com", "alice@x.com", "bob@y.org", "Facture"] {
            assert!(text.contains(needle), "aperçu sans {needle} : {text}");
        }
    }

    #[test]
    fn apercu_et_execution_partagent_le_meme_parseur() {
        let p = send_params(json!({"cc": "pas_un_email"}));
        assert!(JayMailClient::confirmation_preview(&p).is_err());
        assert!(JayMailClient::parse_send_request(&p).is_err());
    }

    #[test]
    fn apercu_tronque_le_corps() {
        let p = send_params(json!({"body": "é".repeat(PREVIEW_BODY_CHARS + 10)}));
        let preview = JayMailClient::confirmation_preview(&p).unwrap();
        assert_eq!(preview.body_excerpt.chars().count(), PREVIEW_BODY_CHARS + 1);
        assert!(preview.body_excerpt.ends_with('…'));
        assert_eq!(preview.body_bytes, (PREVIEW_BODY_CHARS + 10) * 2);
    }

    #[tokio::test]
    async fn lecture_limit_bornee() {
        let backend = Arc::new(FakeJayMailBackend::new());
        for i in 0..(MAX_LIST_LIMIT + 5) {
            backend.push_inbox(mail(&format!("m{i}"), "x@y.z", "s", "p", true));
        }
        let client = JayMailClient::new(backend);
        let r = client
            .call("jaymail.list_recent_inbox", &json!({"limit": 100_000}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], MAX_LIST_LIMIT);
        let r = client
            .call(
                "jaymail.search_mail",
                &json!({"query": "q".repeat(MAX_QUERY_BYTES + 1)}),
            )
            .await;
        assert!(!r.success);
    }
}
