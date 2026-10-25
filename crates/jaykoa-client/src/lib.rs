//! # jaykoa-client
//!
//! Adaptateur entre Alicia (via `miyualicia_dispatcher::ServiceClient`) et le
//! service JayKoa.
//!
//! Le crate expose un `JayKoaClient` qui :
//! - implémente `ServiceClient`
//! - route les intents (`list_events`, `next_event`) vers un backend
//!   abstrait `JayKoaBackend`
//! - sérialise les paramètres entrants depuis JSON et le résultat sortant
//!   vers JSON
//!
//! Le **backend** est un trait, pas une dépendance concrète. Cela permet :
//! - en P3 : un `FakeJayKoaBackend` programmable pour tests E2E
//! - en P4+ : un backend HTTP/IPC qui parlera au binaire JayKoa réel
//! - en runtime Central : éventuellement un backend in-process qui appelle
//!   directement la lib `jaykoa::data`
//!
//! Les capabilités de JayKoa sont **read-only** par contrat (« JayKoa ne
//! crée jamais d'événement externe, ne décide jamais du temps ») — le
//! dispatcher applique de toute façon `AliciaPolicy`, mais la lecture est
//! moins exposée à la gate cores.

#![doc(html_root_url = "https://docs.miyukini.com/jaykoa-client")]

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

// @id service.jaykoa.alicia_client
// @role alicia_to_jaykoa_adapter
// @layer 7
// @do bridge_alicia_to_jaykoa
// @human Adaptateur Alicia → JayKoa : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError};

use async_trait::async_trait;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'un événement pour Alicia.
///
/// Volontairement plus petite que `jaykoa::data::TemporalEntry` — on ne
/// remonte au LLM que ce dont il a besoin pour répondre à l'utilisateur.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EventSummary {
    /// Identifiant stable de l'événement (UUID).
    pub id: String,
    /// Titre affichable.
    pub title: String,
    /// Début (RFC 3339).
    pub start: String,
    /// Fin (RFC 3339), si applicable.
    pub end: Option<String>,
    /// Lieu (texte libre), si applicable.
    pub location: Option<String>,
    /// `true` si l'événement couvre toute la journée.
    #[serde(default)]
    pub all_day: bool,
    /// Service source si reflet externe (ex. `"jayrdv"`), `None` si interne.
    pub source_service: Option<String>,
}

/// Erreurs émises par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayKoaError {
    /// Erreur d'I/O ou DB côté backend.
    #[error("backend JayKoa : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
}

/// Trait abstrait pour les opérations JayKoa exposables à Alicia.
///
/// Toutes les méthodes sont read-only. Une vraie impl branchée sur KindMother
/// viendra plus tard ; P3 utilise `FakeJayKoaBackend` pour les tests.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait JayKoaBackend: Send + Sync {
    /// Liste les événements dont la date de début est dans `[from, to]`
    /// (incl.). `to` peut être identique à `from`.
    async fn list_events(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<EventSummary>, JayKoaError>;

    /// Renvoie le prochain événement à venir, ou `None` si l'agenda est vide
    /// dans les jours à venir.
    async fn next_event(&self) -> Result<Option<EventSummary>, JayKoaError>;
}

/// Le client Alicia ↔ JayKoa.
pub struct JayKoaClient {
    backend: Arc<dyn JayKoaBackend>,
}

impl JayKoaClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayKoaBackend>) -> Self {
        Self { backend }
    }

    fn parse_date(value: &Value, key: &str) -> Result<Option<NaiveDate>, JayKoaError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(Some)
                .map_err(|e| {
                    JayKoaError::InvalidParam(format!(
                        "{key} doit être YYYY-MM-DD ({e})"
                    ))
                }),
            Some(other) => Err(JayKoaError::InvalidParam(format!(
                "{key} doit être une chaîne YYYY-MM-DD, reçu {other}"
            ))),
        }
    }

    fn today() -> NaiveDate {
        chrono::Local::now().date_naive()
    }

    async fn handle_list_events(&self, params: &Value) -> ServiceCallResult {
        let from = match Self::parse_date(params, "from") {
            Ok(d) => d.unwrap_or_else(Self::today),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let to = match Self::parse_date(params, "to") {
            Ok(d) => d.unwrap_or(from),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if to < from {
            return ServiceCallResult::fail(format!(
                "to ({to}) est antérieur à from ({from})"
            ));
        }
        match self.backend.list_events(from, to).await {
            Ok(events) => {
                let count = events.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "events": events,
                        "from": from.to_string(),
                        "to": to.to_string(),
                        "count": count,
                    }),
                    message: Some(format!(
                        "{count} événement(s) entre {from} et {to}"
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_next_event(&self, _params: &Value) -> ServiceCallResult {
        match self.backend.next_event().await {
            Ok(None) => ServiceCallResult {
                success: true,
                data: json!({ "event": Value::Null }),
                message: Some("aucun événement à venir".into()),
            },
            Ok(Some(ev)) => {
                let title = ev.title.clone();
                let start = ev.start.clone();
                ServiceCallResult {
                    success: true,
                    data: json!({ "event": ev }),
                    message: Some(format!("prochain : « {title} » à {start}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayKoaClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaykoa" {
                    return ServiceCallResult::fail(format!(
                        "JayKoaClient appelé avec un service '{service}' inattendu"
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
            "list_events" => self.handle_list_events(params).await,
            "next_event" => self.handle_next_event(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayKoa non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E du dispatcher.
///
/// Permet de poser un jeu d'événements en mémoire et de définir le
/// « prochain événement » sans avoir à brancher KindMother.
///
/// Synchronisation via `std::sync::RwLock` — les opérations sont très brèves
/// et ne franchissent pas de point `await`, on évite la dépendance tokio.
#[derive(Default)]
pub struct FakeJayKoaBackend {
    inner: std::sync::RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    events: Vec<EventWithDate>,
    next: Option<EventSummary>,
    fail_with: Option<String>,
}

#[derive(Clone)]
struct EventWithDate {
    date: NaiveDate,
    summary: EventSummary,
}

impl FakeJayKoaBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute un événement positionné sur une date donnée.
    pub fn push_event_on(&self, date: NaiveDate, summary: EventSummary) {
        self.inner
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .events
            .push(EventWithDate { date, summary });
    }

    /// Définit l'événement renvoyé par `next_event`.
    pub fn set_next(&self, ev: Option<EventSummary>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).next = ev;
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }
}

#[async_trait]
impl JayKoaBackend for FakeJayKoaBackend {
    async fn list_events(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<EventSummary>, JayKoaError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayKoaError::Backend(msg.clone()));
        }
        Ok(state
            .events
            .iter()
            .filter(|e| e.date >= from && e.date <= to)
            .map(|e| e.summary.clone())
            .collect())
    }

    async fn next_event(&self) -> Result<Option<EventSummary>, JayKoaError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JayKoaError::Backend(msg.clone()));
        }
        Ok(state.next.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(id: &str, title: &str, start: &str) -> EventSummary {
        EventSummary {
            id: id.into(),
            title: title.into(),
            start: start.into(),
            end: None,
            location: None,
            all_day: false,
            source_service: None,
        }
    }

    #[tokio::test]
    async fn list_events_default_range_uses_today() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let today = chrono::Local::now().date_naive();
        backend.push_event_on(today, ev("e1", "Sync", "2026-05-08T10:00:00Z"));
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykoa.list_events", &json!({})).await;
        assert!(r.success);
        let count = r.data["count"].as_u64().unwrap();
        assert_eq!(count, 1);
    }

    #[tokio::test]
    async fn list_events_explicit_range() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        backend.push_event_on(
            NaiveDate::from_ymd_opt(2026, 5, 8).unwrap(),
            ev("e1", "Sync", "2026-05-08T10:00:00Z"),
        );
        backend.push_event_on(
            NaiveDate::from_ymd_opt(2026, 5, 10).unwrap(),
            ev("e2", "Brief", "2026-05-10T14:00:00Z"),
        );
        let client = JayKoaClient::new(backend);
        let r = client
            .call(
                "jaykoa.list_events",
                &json!({"from": "2026-05-08", "to": "2026-05-09"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"].as_u64().unwrap(), 1);
    }

    #[tokio::test]
    async fn list_events_invalid_date_format_fails() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let client = JayKoaClient::new(backend);
        let r = client
            .call("jaykoa.list_events", &json!({"from": "08/05/2026"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("from"));
    }

    #[tokio::test]
    async fn list_events_inverted_range_fails() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let client = JayKoaClient::new(backend);
        let r = client
            .call(
                "jaykoa.list_events",
                &json!({"from": "2026-05-10", "to": "2026-05-08"}),
            )
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn next_event_returns_summary() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        backend.set_next(Some(ev("e1", "Stand-up", "2026-05-09T09:00:00Z")));
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["event"]["id"], "e1");
        assert!(r.message.unwrap().contains("Stand-up"));
    }

    #[tokio::test]
    async fn next_event_returns_none_when_empty() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(r.success);
        assert!(r.data["event"].is_null());
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykoa.delete_all", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("intent JayKoa non supporté"));
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykonta.add_expense", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("inattendu"));
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayKoaBackend::new());
        backend.force_failure("KindMother indisponible");
        let client = JayKoaClient::new(backend);
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("KindMother indisponible"));
    }
}
