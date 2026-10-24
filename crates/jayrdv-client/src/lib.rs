//! # jayrdv-client
//!
//! Adaptateur entre Alicia et **JayRDV** (rendez-vous et réservation en ligne).
//!
//! Capabilités exposées (cf. `crates/jayrdv/service.manifest.json`) :
//! - `book_appointment` (write, `always_for_writes`)
//! - `cancel_appointment` (write, `always_for_writes`)
//! - `list_my_appointments` (read)
//! - `list_available_slots` (read)
//!
//! Le **backend** (`JayRdvBackend`) est un trait async — l'implémentation
//! réelle (HTTP / KindMother / in-process) sera branchée au runtime.
//! P-RDV livre un `FakeJayRdvBackend` programmable pour les tests.

#![doc(html_root_url = "https://docs.miyukini.com/jayrdv-client")]

// @id service.jayrdv.alicia_client
// @role alicia_to_jayrdv_adapter
// @layer 7
// @do bridge_alicia_to_jayrdv
// @human Adaptateur Alicia → JayRDV : route les intents vers un backend pluggable.

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::{DateTime, Duration, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Statut simplifié d'un rendez-vous, exposé à Alicia.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AppointmentStatus {
    /// Confirmé / actif.
    Confirmed,
    /// Annulé.
    Cancelled,
    /// Passé / honoré.
    Completed,
}

/// Représentation simplifiée d'un rendez-vous pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppointmentSummary {
    /// Identifiant stable.
    pub id: String,
    /// Identifiant du professionnel.
    pub professional_id: String,
    /// Nom affichable du professionnel (libellé court).
    pub professional_name: Option<String>,
    /// Identifiant du service.
    pub service_id: String,
    /// Libellé du service.
    pub service_label: Option<String>,
    /// Début du rendez-vous (RFC 3339).
    pub start: String,
    /// Fin du rendez-vous (RFC 3339).
    pub end: String,
    /// Statut.
    pub status: AppointmentStatus,
    /// Note client si fournie.
    pub client_note: Option<String>,
}

/// Représentation simplifiée d'un créneau libre pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SlotSummary {
    /// Identifiant du professionnel.
    pub professional_id: String,
    /// Identifiant du service compatible.
    pub service_id: String,
    /// Début du créneau (RFC 3339).
    pub start: String,
    /// Fin du créneau (RFC 3339).
    pub end: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayRdvError {
    /// Erreur d'I/O ou DB côté backend.
    #[error("backend JayRDV : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Rendez-vous introuvable.
    #[error("rendez-vous introuvable : {0}")]
    AppointmentNotFound(String),
    /// Créneau déjà réservé ou indisponible.
    #[error("créneau indisponible : {0}")]
    SlotUnavailable(String),
}

/// Trait abstrait pour les opérations JayRDV exposables à Alicia.
#[async_trait]
pub trait JayRdvBackend: Send + Sync {
    /// Réserve un rendez-vous et retourne le `AppointmentSummary` posé.
    async fn book_appointment(
        &self,
        professional_id: &str,
        service_id: &str,
        slot_start: DateTime<Utc>,
        client_note: Option<String>,
    ) -> Result<AppointmentSummary, JayRdvError>;

    /// Annule un rendez-vous. Retourne le résumé final (statut Cancelled).
    async fn cancel_appointment(
        &self,
        appointment_id: &str,
        reason: Option<String>,
    ) -> Result<AppointmentSummary, JayRdvError>;

    /// Liste les rendez-vous de l'utilisateur dans la fenêtre `[from, to]`
    /// (dates incluses). `from` et `to` sont des dates locales.
    async fn list_my_appointments(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<AppointmentSummary>, JayRdvError>;

    /// Liste les créneaux disponibles dans la fenêtre `[from, to]`.
    async fn list_available_slots(
        &self,
        professional_id: &str,
        service_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<SlotSummary>, JayRdvError>;
}

/// Le client Alicia ↔ JayRDV.
pub struct JayRdvClient {
    backend: Arc<dyn JayRdvBackend>,
}

impl JayRdvClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayRdvBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JayRdvError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JayRdvError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayRdvError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_datetime(value: &Value, key: &str) -> Result<Option<DateTime<Utc>>, JayRdvError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => DateTime::parse_from_rfc3339(s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(|e| {
                    JayRdvError::InvalidParam(format!(
                        "{key} doit être ISO 8601 RFC3339 ({e})"
                    ))
                }),
            Some(other) => Err(JayRdvError::InvalidParam(format!(
                "{key} doit être une chaîne datetime, reçu {other}"
            ))),
        }
    }

    fn parse_date(value: &Value, key: &str) -> Result<Option<NaiveDate>, JayRdvError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => NaiveDate::parse_from_str(s, "%Y-%m-%d")
                .map(Some)
                .map_err(|e| {
                    JayRdvError::InvalidParam(format!("{key} doit être YYYY-MM-DD ({e})"))
                }),
            Some(other) => Err(JayRdvError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    async fn handle_book(&self, params: &Value) -> ServiceCallResult {
        let professional_id = match Self::parse_string(params, "professional_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("professional_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let service_id = match Self::parse_string(params, "service_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("service_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let slot_start = match Self::parse_datetime(params, "slot_start") {
            Ok(Some(d)) => d,
            Ok(None) => return ServiceCallResult::fail("slot_start manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let client_note = match Self::parse_string(params, "client_note", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .book_appointment(&professional_id, &service_id, slot_start, client_note)
            .await
        {
            Ok(appt) => {
                let start = appt.start.clone();
                let prof = appt
                    .professional_name
                    .clone()
                    .unwrap_or_else(|| professional_id.clone());
                ServiceCallResult {
                    success: true,
                    data: json!({ "appointment": appt }),
                    message: Some(format!("rendez-vous réservé chez {prof} le {start}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_cancel(&self, params: &Value) -> ServiceCallResult {
        let appointment_id = match Self::parse_string(params, "appointment_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("appointment_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let reason = match Self::parse_string(params, "reason", false) {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.cancel_appointment(&appointment_id, reason).await {
            Ok(appt) => ServiceCallResult {
                success: true,
                data: json!({ "appointment": appt }),
                message: Some(format!("rendez-vous {appointment_id} annulé")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_my(&self, params: &Value) -> ServiceCallResult {
        let from = match Self::parse_date(params, "from") {
            Ok(Some(d)) => d,
            Ok(None) => chrono::Local::now().date_naive(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let to = match Self::parse_date(params, "to") {
            Ok(Some(d)) => d,
            Ok(None) => from + Duration::days(30),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if to < from {
            return ServiceCallResult::fail(format!(
                "to ({to}) antérieur à from ({from})"
            ));
        }
        match self.backend.list_my_appointments(from, to).await {
            Ok(list) => {
                let count = list.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "appointments": list,
                        "from":         from.to_string(),
                        "to":           to.to_string(),
                        "count":        count,
                    }),
                    message: Some(format!(
                        "{count} rendez-vous entre {from} et {to}"
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_slots(&self, params: &Value) -> ServiceCallResult {
        let professional_id = match Self::parse_string(params, "professional_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("professional_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let service_id = match Self::parse_string(params, "service_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("service_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let from = match Self::parse_datetime(params, "from") {
            Ok(Some(d)) => d,
            Ok(None) => Utc::now(),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let to = match Self::parse_datetime(params, "to") {
            Ok(Some(d)) => d,
            Ok(None) => from + Duration::days(7),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if to <= from {
            return ServiceCallResult::fail("to doit être strictement postérieur à from");
        }
        match self
            .backend
            .list_available_slots(&professional_id, &service_id, from, to)
            .await
        {
            Ok(slots) => {
                let count = slots.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "slots": slots,
                        "professional_id": professional_id,
                        "service_id": service_id,
                        "count": count,
                    }),
                    message: Some(format!("{count} créneau(x) disponible(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayRdvClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jayrdv" {
                    return ServiceCallResult::fail(format!(
                        "JayRdvClient appelé avec un service '{service}' inattendu"
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
            "book_appointment" => self.handle_book(params).await,
            "cancel_appointment" => self.handle_cancel(params).await,
            "list_my_appointments" => self.handle_list_my(params).await,
            "list_available_slots" => self.handle_list_slots(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayRDV non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayRdvBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    appointments: Vec<AppointmentSummary>,
    available_slots: Vec<SlotSummary>,
    fail_with: Option<String>,
    seq: usize,
}

impl FakeJayRdvBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap().fail_with = Some(message.into());
    }

    /// Programme un créneau disponible (pour `list_available_slots`).
    pub fn add_available_slot(&self, slot: SlotSummary) {
        self.inner.write().unwrap().available_slots.push(slot);
    }

    /// Snapshot des rendez-vous actuellement en mémoire.
    pub fn snapshot_appointments(&self) -> Vec<AppointmentSummary> {
        self.inner.read().unwrap().appointments.clone()
    }
}

#[async_trait]
impl JayRdvBackend for FakeJayRdvBackend {
    async fn book_appointment(
        &self,
        professional_id: &str,
        service_id: &str,
        slot_start: DateTime<Utc>,
        client_note: Option<String>,
    ) -> Result<AppointmentSummary, JayRdvError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayRdvError::Backend(msg.clone()));
        }
        state.seq += 1;
        let id = format!("appt_{}", state.seq);
        let end = slot_start + Duration::minutes(30);
        let appt = AppointmentSummary {
            id,
            professional_id: professional_id.to_string(),
            professional_name: None,
            service_id: service_id.to_string(),
            service_label: None,
            start: slot_start.to_rfc3339(),
            end: end.to_rfc3339(),
            status: AppointmentStatus::Confirmed,
            client_note,
        };
        state.appointments.push(appt.clone());
        Ok(appt)
    }

    async fn cancel_appointment(
        &self,
        appointment_id: &str,
        _reason: Option<String>,
    ) -> Result<AppointmentSummary, JayRdvError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayRdvError::Backend(msg.clone()));
        }
        let appt = state
            .appointments
            .iter_mut()
            .find(|a| a.id == appointment_id)
            .ok_or_else(|| JayRdvError::AppointmentNotFound(appointment_id.to_string()))?;
        appt.status = AppointmentStatus::Cancelled;
        Ok(appt.clone())
    }

    async fn list_my_appointments(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> Result<Vec<AppointmentSummary>, JayRdvError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayRdvError::Backend(msg.clone()));
        }
        Ok(state
            .appointments
            .iter()
            .filter(|a| {
                if a.status == AppointmentStatus::Cancelled {
                    return false;
                }
                if let Ok(dt) = DateTime::parse_from_rfc3339(&a.start) {
                    let date = dt.date_naive();
                    date >= from && date <= to
                } else {
                    false
                }
            })
            .cloned()
            .collect())
    }

    async fn list_available_slots(
        &self,
        professional_id: &str,
        service_id: &str,
        from: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> Result<Vec<SlotSummary>, JayRdvError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayRdvError::Backend(msg.clone()));
        }
        Ok(state
            .available_slots
            .iter()
            .filter(|s| {
                s.professional_id == professional_id && s.service_id == service_id
            })
            .filter(|s| {
                DateTime::parse_from_rfc3339(&s.start)
                    .map(|d| {
                        let d = d.with_timezone(&Utc);
                        d >= from && d <= to
                    })
                    .unwrap_or(false)
            })
            .cloned()
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(prof: &str, service: &str, start_rfc: &str) -> SlotSummary {
        SlotSummary {
            professional_id: prof.into(),
            service_id: service.into(),
            start: start_rfc.into(),
            end: start_rfc.into(),
        }
    }

    #[tokio::test]
    async fn book_appointment_returns_confirmed() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        let client = JayRdvClient::new(backend.clone());
        let r = client
            .call(
                "jayrdv.book_appointment",
                &json!({
                    "professional_id": "dr_smith",
                    "service_id": "consult_30",
                    "slot_start": "2026-05-15T10:00:00Z",
                    "client_note": "première visite"
                }),
            )
            .await;
        assert!(r.success);
        let appt = &r.data["appointment"];
        assert_eq!(appt["status"], "confirmed");
        assert_eq!(appt["client_note"], "première visite");
        assert_eq!(backend.snapshot_appointments().len(), 1);
    }

    #[tokio::test]
    async fn book_appointment_rejects_invalid_datetime() {
        let client = JayRdvClient::new(Arc::new(FakeJayRdvBackend::new()));
        let r = client
            .call(
                "jayrdv.book_appointment",
                &json!({
                    "professional_id": "x",
                    "service_id": "y",
                    "slot_start": "demain matin"
                }),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("slot_start"));
    }

    #[tokio::test]
    async fn book_appointment_rejects_missing_required() {
        let client = JayRdvClient::new(Arc::new(FakeJayRdvBackend::new()));
        let r = client
            .call(
                "jayrdv.book_appointment",
                &json!({"professional_id": "x"}),
            )
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn cancel_appointment_marks_cancelled() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        let client = JayRdvClient::new(backend.clone());
        client
            .call(
                "jayrdv.book_appointment",
                &json!({
                    "professional_id": "p",
                    "service_id": "s",
                    "slot_start": "2026-05-15T10:00:00Z"
                }),
            )
            .await;
        let r = client
            .call(
                "jayrdv.cancel_appointment",
                &json!({"appointment_id": "appt_1", "reason": "empêchement"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["appointment"]["status"], "cancelled");
    }

    #[tokio::test]
    async fn cancel_unknown_appointment_fails() {
        let client = JayRdvClient::new(Arc::new(FakeJayRdvBackend::new()));
        let r = client
            .call(
                "jayrdv.cancel_appointment",
                &json!({"appointment_id": "ghost"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn list_my_appointments_filters_by_date_window() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        let client = JayRdvClient::new(backend);
        client
            .call(
                "jayrdv.book_appointment",
                &json!({
                    "professional_id": "p",
                    "service_id": "s",
                    "slot_start": "2026-05-15T10:00:00Z"
                }),
            )
            .await;
        client
            .call(
                "jayrdv.book_appointment",
                &json!({
                    "professional_id": "p",
                    "service_id": "s",
                    "slot_start": "2026-06-20T10:00:00Z"
                }),
            )
            .await;
        let r = client
            .call(
                "jayrdv.list_my_appointments",
                &json!({"from": "2026-05-01", "to": "2026-05-31"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 1);
    }

    #[tokio::test]
    async fn list_my_appointments_excludes_cancelled() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        let client = JayRdvClient::new(backend);
        client
            .call(
                "jayrdv.book_appointment",
                &json!({"professional_id": "p", "service_id": "s", "slot_start": "2026-05-15T10:00:00Z"}),
            )
            .await;
        client
            .call(
                "jayrdv.cancel_appointment",
                &json!({"appointment_id": "appt_1"}),
            )
            .await;
        let r = client
            .call(
                "jayrdv.list_my_appointments",
                &json!({"from": "2026-05-01", "to": "2026-05-31"}),
            )
            .await;
        assert_eq!(r.data["count"], 0);
    }

    #[tokio::test]
    async fn list_available_slots_filters_by_pro_service_window() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        backend.add_available_slot(slot("p1", "s1", "2026-05-15T10:00:00Z"));
        backend.add_available_slot(slot("p1", "s1", "2026-05-15T14:00:00Z"));
        backend.add_available_slot(slot("p2", "s1", "2026-05-15T10:00:00Z")); // mauvais pro
        backend.add_available_slot(slot("p1", "s2", "2026-05-15T10:00:00Z")); // mauvais service
        let client = JayRdvClient::new(backend);
        let r = client
            .call(
                "jayrdv.list_available_slots",
                &json!({
                    "professional_id": "p1",
                    "service_id": "s1",
                    "from": "2026-05-15T00:00:00Z",
                    "to": "2026-05-15T23:59:59Z"
                }),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JayRdvClient::new(Arc::new(FakeJayRdvBackend::new()));
        let r = client.call("jayrdv.delete_all", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("non supporté"));
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JayRdvClient::new(Arc::new(FakeJayRdvBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("inattendu"));
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayRdvBackend::new());
        backend.force_failure("KindMother offline");
        let client = JayRdvClient::new(backend);
        let r = client
            .call(
                "jayrdv.list_my_appointments",
                &json!({"from": "2026-05-01"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("KindMother offline"));
    }
}
