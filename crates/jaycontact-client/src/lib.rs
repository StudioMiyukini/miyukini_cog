//! # jaycontact-client
//!
//! Adaptateur entre Alicia (via `miyualicia_dispatcher::ServiceClient`) et un
//! carnet de contacts. Le backend de référence sera un wrapper du toolkit
//! `miyucontacts` (qui gère friends/foes sous mandat StrongFather +
//! WriteIntent KindMother). Ce crate expose un trait abstrait + un
//! `FakeJayContactBackend` pour les tests.
//!
//! Capabilités exposées (cf. `crates/jaycontact/service.manifest.json`) :
//! - `add_contact` (write, confirmation par défaut `always_for_writes`)
//! - `list_contacts` (read)
//! - `find_contact` (read)
//! - `remove_contact` (write, confirmation par défaut `always_for_writes`)

#![doc(html_root_url = "https://docs.miyukini.com/jaycontact-client")]

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

// @id service.jaycontact.alicia_client
// @role alicia_to_jaycontact_adapter
// @layer 7
// @do bridge_alicia_to_jaycontact
// @human Adaptateur Alicia → JayContact : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError, RwLock};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Type de relation avec un contact.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContactKind {
    /// Ami / contact de confiance.
    Friend,
    /// Bloqué / liste noire.
    Foe,
}

impl ContactKind {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "friend" | "ami" => Some(Self::Friend),
            "foe" | "bloque" => Some(Self::Foe),
            _ => None,
        }
    }

    /// Étiquette française.
    #[must_use]
    pub fn label_fr(self) -> &'static str {
        match self {
            Self::Friend => "ami",
            Self::Foe => "bloqué",
        }
    }
}

/// Représentation d'un contact pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContactSummary {
    /// Identifiant stable du contact (ID interne ou tag réseau).
    pub id: String,
    /// Nom affichable.
    pub display_name: String,
    /// Type de relation.
    pub kind: ContactKind,
}

/// Erreurs renvoyées par le backend contacts.
#[derive(Debug, thiserror::Error)]
pub enum JayContactError {
    /// Erreur côté backend (DB / mandat refusé / lock).
    #[error("backend contacts : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Contact introuvable pour `remove_contact` / `find_contact`.
    #[error("contact introuvable : {0}")]
    NotFound(String),
}

/// Trait abstrait pour les opérations sur le carnet de contacts.
#[async_trait]
pub trait JayContactBackend: Send + Sync {
    /// Ajoute un contact et retourne son `ContactSummary`.
    async fn add_contact(
        &self,
        id: &str,
        display_name: &str,
        kind: ContactKind,
    ) -> Result<ContactSummary, JayContactError>;

    /// Liste les contacts (filtrés optionnellement par type, limités).
    async fn list_contacts(
        &self,
        kind: Option<ContactKind>,
        limit: usize,
    ) -> Result<Vec<ContactSummary>, JayContactError>;

    /// Recherche un contact par son `id` ou par début de `display_name`.
    async fn find_contact(
        &self,
        query: &str,
    ) -> Result<Option<ContactSummary>, JayContactError>;

    /// Supprime un contact par son `id`.
    async fn remove_contact(&self, id: &str) -> Result<(), JayContactError>;
}

/// Le client Alicia ↔ JayContact.
pub struct JayContactClient {
    backend: Arc<dyn JayContactBackend>,
}

impl JayContactClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayContactBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(params: &'a Value, key: &str) -> Result<&'a str, JayContactError> {
        match params.get(key) {
            Some(Value::String(s)) if !s.trim().is_empty() => Ok(s.as_str()),
            Some(Value::String(_)) | None | Some(Value::Null) => Err(
                JayContactError::InvalidParam(format!("{key} manquant ou vide")),
            ),
            Some(other) => Err(JayContactError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_string_opt<'a>(
        params: &'a Value,
        key: &str,
    ) -> Result<Option<&'a str>, JayContactError> {
        match params.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) if s.trim().is_empty() => Ok(None),
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayContactError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_kind(params: &Value) -> Result<ContactKind, JayContactError> {
        match params.get("kind") {
            None | Some(Value::Null) => Ok(ContactKind::Friend),
            Some(Value::String(s)) => ContactKind::from_str(s).ok_or_else(|| {
                JayContactError::InvalidParam(format!(
                    "kind doit être 'friend' ou 'foe', reçu '{s}'"
                ))
            }),
            Some(other) => Err(JayContactError::InvalidParam(format!(
                "kind doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_limit(params: &Value) -> Result<usize, JayContactError> {
        match params.get("limit") {
            None | Some(Value::Null) => Ok(50),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .map(|v| v as usize)
                .ok_or_else(|| {
                    JayContactError::InvalidParam(format!(
                        "limit doit être un entier > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayContactError::InvalidParam(format!(
                "limit doit être un entier, reçu {other}"
            ))),
        }
    }

    async fn handle_add(&self, params: &Value) -> ServiceCallResult {
        let id = match Self::parse_string(params, "id") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let display_name = match Self::parse_string_opt(params, "display_name") {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => id.clone(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let kind = match Self::parse_kind(params) {
            Ok(k) => k,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .add_contact(&id, &display_name, kind)
            .await
        {
            Ok(contact) => {
                let label = kind.label_fr();
                ServiceCallResult {
                    success: true,
                    data: json!({ "contact": contact }),
                    message: Some(format!("« {display_name} » ajouté ({label})")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list(&self, params: &Value) -> ServiceCallResult {
        let kind = match Self::parse_string_opt(params, "kind") {
            Ok(Some(s)) => match ContactKind::from_str(s) {
                Some(k) => Some(k),
                None => {
                    return ServiceCallResult::fail(format!(
                        "kind doit être 'friend' ou 'foe', reçu '{s}'"
                    ));
                }
            },
            Ok(None) => None,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let limit = match Self::parse_limit(params) {
            Ok(l) => l,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_contacts(kind, limit).await {
            Ok(contacts) => {
                let count = contacts.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "contacts": contacts,
                        "count": count,
                    }),
                    message: Some(format!("{count} contact(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_find(&self, params: &Value) -> ServiceCallResult {
        let query = match Self::parse_string(params, "query") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.find_contact(&query).await {
            Ok(Some(contact)) => {
                let name = contact.display_name.clone();
                ServiceCallResult {
                    success: true,
                    data: json!({ "contact": contact }),
                    message: Some(format!("trouvé : {name}")),
                }
            }
            Ok(None) => ServiceCallResult {
                success: true,
                data: json!({ "contact": Value::Null }),
                message: Some(format!("aucun contact pour « {query} »")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_remove(&self, params: &Value) -> ServiceCallResult {
        let id = match Self::parse_string(params, "id") {
            Ok(s) => s.to_string(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.remove_contact(&id).await {
            Ok(()) => ServiceCallResult {
                success: true,
                data: json!({ "id": id }),
                message: Some(format!("contact « {id} » supprimé")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayContactClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaycontact" {
                    return ServiceCallResult::fail(format!(
                        "JayContactClient appelé avec un service '{service}' inattendu"
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
            "add_contact" => self.handle_add(params).await,
            "list_contacts" => self.handle_list(params).await,
            "find_contact" => self.handle_find(params).await,
            "remove_contact" => self.handle_remove(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayContact non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend en mémoire pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayContactBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    contacts: Vec<ContactSummary>,
    fail_with: Option<String>,
}

impl FakeJayContactBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Pose la liste initiale (utile en démo / dev).
    pub fn seed(&self, contacts: Vec<ContactSummary>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).contacts = contacts;
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }

    /// Snapshot des contacts actuellement en mémoire (utile en test).
    pub fn snapshot(&self) -> Vec<ContactSummary> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).contacts.clone()
    }
}

#[async_trait]
impl JayContactBackend for FakeJayContactBackend {
    async fn add_contact(
        &self,
        id: &str,
        display_name: &str,
        kind: ContactKind,
    ) -> Result<ContactSummary, JayContactError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayContactError::Backend(msg.clone()));
        }
        let contact = ContactSummary {
            id: id.to_string(),
            display_name: display_name.to_string(),
            kind,
        };
        // Idempotent : si l'id existe déjà, on remplace.
        state.contacts.retain(|c| c.id != id);
        state.contacts.push(contact.clone());
        Ok(contact)
    }

    async fn list_contacts(
        &self,
        kind: Option<ContactKind>,
        limit: usize,
    ) -> Result<Vec<ContactSummary>, JayContactError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayContactError::Backend(msg.clone()));
        }
        Ok(state
            .contacts
            .iter()
            .filter(|c| kind.map_or(true, |k| c.kind == k))
            .take(limit)
            .cloned()
            .collect())
    }

    async fn find_contact(
        &self,
        query: &str,
    ) -> Result<Option<ContactSummary>, JayContactError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayContactError::Backend(msg.clone()));
        }
        let q = query.to_lowercase();
        Ok(state
            .contacts
            .iter()
            .find(|c| {
                c.id.to_lowercase() == q || c.display_name.to_lowercase().starts_with(&q)
            })
            .cloned())
    }

    async fn remove_contact(&self, id: &str) -> Result<(), JayContactError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayContactError::Backend(msg.clone()));
        }
        let len_before = state.contacts.len();
        state.contacts.retain(|c| c.id != id);
        if state.contacts.len() == len_before {
            return Err(JayContactError::NotFound(id.to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_contact_succeeds() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend.clone());
        let r = client
            .call(
                "jaycontact.add_contact",
                &json!({"id": "user_marc", "display_name": "Marc Dupont", "kind": "friend"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(backend.snapshot().len(), 1);
    }

    #[tokio::test]
    async fn add_without_display_name_uses_id() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend.clone());
        let r = client
            .call("jaycontact.add_contact", &json!({"id": "user_solo"}))
            .await;
        assert!(r.success);
        assert_eq!(backend.snapshot()[0].display_name, "user_solo");
    }

    #[tokio::test]
    async fn add_missing_id_fails() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend);
        let r = client
            .call("jaycontact.add_contact", &json!({}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_contacts_default() {
        let backend = Arc::new(FakeJayContactBackend::new());
        backend.seed(vec![
            ContactSummary {
                id: "a".into(),
                display_name: "Alice".into(),
                kind: ContactKind::Friend,
            },
            ContactSummary {
                id: "b".into(),
                display_name: "Bob".into(),
                kind: ContactKind::Foe,
            },
        ]);
        let client = JayContactClient::new(backend);
        let r = client.call("jaycontact.list_contacts", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 2);
    }

    #[tokio::test]
    async fn list_contacts_filtered_by_kind() {
        let backend = Arc::new(FakeJayContactBackend::new());
        backend.seed(vec![
            ContactSummary {
                id: "a".into(),
                display_name: "Alice".into(),
                kind: ContactKind::Friend,
            },
            ContactSummary {
                id: "b".into(),
                display_name: "Bob".into(),
                kind: ContactKind::Foe,
            },
        ]);
        let client = JayContactClient::new(backend);
        let r = client
            .call("jaycontact.list_contacts", &json!({"kind": "friend"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn find_contact_by_name_prefix() {
        let backend = Arc::new(FakeJayContactBackend::new());
        backend.seed(vec![ContactSummary {
            id: "u_marc".into(),
            display_name: "Marc Dupont".into(),
            kind: ContactKind::Friend,
        }]);
        let client = JayContactClient::new(backend);
        let r = client
            .call("jaycontact.find_contact", &json!({"query": "marc"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["contact"]["id"], "u_marc");
    }

    #[tokio::test]
    async fn remove_contact_succeeds() {
        let backend = Arc::new(FakeJayContactBackend::new());
        backend.seed(vec![ContactSummary {
            id: "u_x".into(),
            display_name: "X".into(),
            kind: ContactKind::Friend,
        }]);
        let client = JayContactClient::new(backend.clone());
        let r = client
            .call("jaycontact.remove_contact", &json!({"id": "u_x"}))
            .await;
        assert!(r.success);
        assert!(backend.snapshot().is_empty());
    }

    #[tokio::test]
    async fn remove_nonexistent_fails() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend);
        let r = client
            .call("jaycontact.remove_contact", &json!({"id": "ghost"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend);
        let r = client.call("jaycontact.delete_all", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let backend = Arc::new(FakeJayContactBackend::new());
        let client = JayContactClient::new(backend);
        let r = client.call("jaykonta.add_contact", &json!({})).await;
        assert!(!r.success);
    }
}
