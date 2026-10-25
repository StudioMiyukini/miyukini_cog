//! # jaysheets-client
//!
//! Adaptateur entre Alicia et **Jay Sheets** (tableur collaboratif).

#![doc(html_root_url = "https://docs.miyukini.com/jaysheets-client")]

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

// @id service.jaysheets.alicia_client
// @role alicia_to_jaysheets_adapter
// @layer 7
// @do bridge_alicia_to_jaysheets
// @human Adaptateur Alicia → JaySheets : route les intents vers un backend pluggable.

// C-10 : verrous du backend factice levés sans panique en cas d'empoisonnement.
use std::sync::{Arc, PoisonError, RwLock};

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Représentation simplifiée d'un tableur pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SheetSummary {
    /// Identifiant stable.
    pub id: String,
    /// Titre du tableur.
    pub title: String,
    /// Date de création (RFC 3339).
    pub created_at: String,
    /// Date de dernière modification (RFC 3339).
    pub updated_at: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JaySheetsError {
    /// Erreur backend.
    #[error("backend Jay Sheets : {0}")]
    Backend(String),
    /// Paramètre invalide.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Tableur introuvable.
    #[error("tableur introuvable : {0}")]
    SheetNotFound(String),
}

/// Trait async des opérations Jay Sheets exposées à Alicia.
#[async_trait]
pub trait JaySheetsBackend: Send + Sync {
    /// Crée un nouveau tableur.
    async fn create_sheet(&self, title: &str) -> Result<SheetSummary, JaySheetsError>;

    /// Liste les tableurs.
    async fn list_my_sheets(&self, limit: usize) -> Result<Vec<SheetSummary>, JaySheetsError>;

    /// Écrit une valeur dans une cellule. Retourne le sheet mis à jour.
    async fn write_cell(
        &self,
        sheet_id: &str,
        cell: &str,
        value: &str,
    ) -> Result<SheetSummary, JaySheetsError>;
}

/// Client Alicia ↔ Jay Sheets.
pub struct JaySheetsClient {
    backend: Arc<dyn JaySheetsBackend>,
}

impl JaySheetsClient {
    /// Construit un client.
    #[must_use]
    pub fn new(backend: Arc<dyn JaySheetsBackend>) -> Self {
        Self { backend }
    }

    fn parse_string<'a>(
        params: &'a Value,
        key: &str,
        required: bool,
    ) -> Result<Option<&'a str>, JaySheetsError> {
        match params.get(key) {
            None | Some(Value::Null) => {
                if required {
                    Err(JaySheetsError::InvalidParam(format!("{key} manquant")))
                } else {
                    Ok(None)
                }
            }
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JaySheetsError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_int(value: &Value, key: &str, default: usize) -> Result<usize, JaySheetsError> {
        match value.get(key) {
            None | Some(Value::Null) => Ok(default),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .map(|v| v as usize)
                .ok_or_else(|| {
                    JaySheetsError::InvalidParam(format!("{key} doit être > 0, reçu {n}"))
                }),
            Some(other) => Err(JaySheetsError::InvalidParam(format!(
                "{key} doit être un entier, reçu {other}"
            ))),
        }
    }

    fn validate_cell_ref(s: &str) -> Result<(), JaySheetsError> {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(JaySheetsError::InvalidParam("cell ne peut pas être vide".into()));
        }
        let mut letters = 0;
        let mut digits = 0;
        let mut seen_digit = false;
        for c in trimmed.chars() {
            if c.is_ascii_alphabetic() {
                if seen_digit {
                    return Err(JaySheetsError::InvalidParam(format!(
                        "cell '{s}' invalide : lettre après chiffre"
                    )));
                }
                letters += 1;
            } else if c.is_ascii_digit() {
                seen_digit = true;
                digits += 1;
            } else {
                return Err(JaySheetsError::InvalidParam(format!(
                    "cell '{s}' contient un caractère non autorisé"
                )));
            }
        }
        if letters == 0 || digits == 0 {
            return Err(JaySheetsError::InvalidParam(format!(
                "cell '{s}' doit ressembler à A1, B5, ..."
            )));
        }
        Ok(())
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
        match self.backend.create_sheet(&title).await {
            Ok(sheet) => ServiceCallResult {
                success: true,
                data: json!({ "sheet": sheet }),
                message: Some(format!("tableur « {title} » créé")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_int(params, "limit", 10) {
            Ok(n) => n,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_my_sheets(limit).await {
            Ok(sheets) => {
                let count = sheets.len();
                ServiceCallResult {
                    success: true,
                    data: json!({ "sheets": sheets, "count": count }),
                    message: Some(format!("{count} tableur(s) trouvé(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_write_cell(&self, params: &Value) -> ServiceCallResult {
        let sheet_id = match Self::parse_string(params, "sheet_id", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("sheet_id manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let cell = match Self::parse_string(params, "cell", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("cell manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        if let Err(e) = Self::validate_cell_ref(&cell) {
            return ServiceCallResult::fail(e.to_string());
        }
        let value = match Self::parse_string(params, "value", true) {
            Ok(Some(s)) => s.to_string(),
            Ok(None) => return ServiceCallResult::fail("value manquant"),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.write_cell(&sheet_id, &cell, &value).await {
            Ok(sheet) => ServiceCallResult {
                success: true,
                data: json!({ "sheet": sheet, "cell": cell, "value": value }),
                message: Some(format!("écrit '{value}' en {cell}")),
            },
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JaySheetsClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaysheets" {
                    return ServiceCallResult::fail(format!(
                        "JaySheetsClient appelé avec un service '{service}' inattendu"
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
            "create_sheet" => self.handle_create(params).await,
            "list_my_sheets" => self.handle_list(params).await,
            "write_cell" => self.handle_write_cell(params).await,
            other => ServiceCallResult::fail(format!(
                "intent Jay Sheets non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests.
#[derive(Default)]
pub struct FakeJaySheetsBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    sheets: Vec<SheetSummary>,
    cells: std::collections::HashMap<(String, String), String>,
    fail_with: Option<String>,
    seq: usize,
}

impl FakeJaySheetsBackend {
    /// Crée un backend vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Force toutes les méthodes à renvoyer une erreur.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap_or_else(PoisonError::into_inner).fail_with = Some(message.into());
    }

    /// Snapshot des tableurs.
    pub fn snapshot(&self) -> Vec<SheetSummary> {
        self.inner.read().unwrap_or_else(PoisonError::into_inner).sheets.clone()
    }

    /// Lit la valeur d'une cellule (pour vérifier write_cell).
    pub fn cell_value(&self, sheet_id: &str, cell: &str) -> Option<String> {
        self.inner
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .cells
            .get(&(sheet_id.to_string(), cell.to_string()))
            .cloned()
    }
}

#[async_trait]
impl JaySheetsBackend for FakeJaySheetsBackend {
    async fn create_sheet(&self, title: &str) -> Result<SheetSummary, JaySheetsError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySheetsError::Backend(msg.clone()));
        }
        state.seq += 1;
        let now = Utc::now().to_rfc3339();
        let id = format!("sheet_{}", state.seq);
        let sheet = SheetSummary {
            id,
            title: title.to_string(),
            created_at: now.clone(),
            updated_at: now,
        };
        state.sheets.push(sheet.clone());
        Ok(sheet)
    }

    async fn list_my_sheets(&self, limit: usize) -> Result<Vec<SheetSummary>, JaySheetsError> {
        let state = self.inner.read().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySheetsError::Backend(msg.clone()));
        }
        let mut all: Vec<SheetSummary> = state.sheets.clone();
        all.reverse();
        all.truncate(limit);
        Ok(all)
    }

    async fn write_cell(
        &self,
        sheet_id: &str,
        cell: &str,
        value: &str,
    ) -> Result<SheetSummary, JaySheetsError> {
        let mut state = self.inner.write().unwrap_or_else(PoisonError::into_inner);
        if let Some(msg) = &state.fail_with {
            return Err(JaySheetsError::Backend(msg.clone()));
        }
        if !state.sheets.iter().any(|s| s.id == sheet_id) {
            return Err(JaySheetsError::SheetNotFound(sheet_id.to_string()));
        }
        state
            .cells
            .insert((sheet_id.to_string(), cell.to_string()), value.to_string());
        let now = Utc::now().to_rfc3339();
        let sheet = state
            .sheets
            .iter_mut()
            .find(|s| s.id == sheet_id)
            .ok_or_else(|| JaySheetsError::SheetNotFound(sheet_id.to_string()))?;
        sheet.updated_at = now;
        Ok(sheet.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create_sheet_returns_summary() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        let client = JaySheetsClient::new(backend.clone());
        let r = client
            .call("jaysheets.create_sheet", &json!({"title": "Ventes Q1"}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["sheet"]["title"], "Ventes Q1");
        assert_eq!(backend.snapshot().len(), 1);
    }

    #[tokio::test]
    async fn create_rejects_empty_title() {
        let client = JaySheetsClient::new(Arc::new(FakeJaySheetsBackend::new()));
        let r = client
            .call("jaysheets.create_sheet", &json!({"title": "  "}))
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn list_returns_all_sheets() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        let client = JaySheetsClient::new(backend);
        client
            .call("jaysheets.create_sheet", &json!({"title": "A"}))
            .await;
        client
            .call("jaysheets.create_sheet", &json!({"title": "B"}))
            .await;
        let r = client.call("jaysheets.list_my_sheets", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["count"], 2);
    }

    #[tokio::test]
    async fn write_cell_persists_value() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        let client = JaySheetsClient::new(backend.clone());
        client
            .call("jaysheets.create_sheet", &json!({"title": "X"}))
            .await;
        let r = client
            .call(
                "jaysheets.write_cell",
                &json!({"sheet_id": "sheet_1", "cell": "A1", "value": "42"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(backend.cell_value("sheet_1", "A1").as_deref(), Some("42"));
    }

    #[tokio::test]
    async fn write_cell_rejects_invalid_reference() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        let client = JaySheetsClient::new(backend);
        let r = client
            .call(
                "jaysheets.write_cell",
                &json!({"sheet_id": "sheet_1", "cell": "1A", "value": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("invalide"));
    }

    #[tokio::test]
    async fn write_cell_rejects_garbage_reference() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        let client = JaySheetsClient::new(backend);
        let r = client
            .call(
                "jaysheets.write_cell",
                &json!({"sheet_id": "sheet_1", "cell": "@!", "value": "x"}),
            )
            .await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn write_cell_unknown_sheet_fails() {
        let client = JaySheetsClient::new(Arc::new(FakeJaySheetsBackend::new()));
        let r = client
            .call(
                "jaysheets.write_cell",
                &json!({"sheet_id": "ghost", "cell": "A1", "value": "x"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("introuvable"));
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JaySheetsClient::new(Arc::new(FakeJaySheetsBackend::new()));
        let r = client.call("jaysheets.delete_all", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JaySheetsClient::new(Arc::new(FakeJaySheetsBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJaySheetsBackend::new());
        backend.force_failure("disk full");
        let client = JaySheetsClient::new(backend);
        let r = client
            .call("jaysheets.create_sheet", &json!({"title": "x"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("disk full"));
    }

    #[test]
    fn validate_cell_accepts_standard_refs() {
        assert!(JaySheetsClient::validate_cell_ref("A1").is_ok());
        assert!(JaySheetsClient::validate_cell_ref("AB42").is_ok());
        assert!(JaySheetsClient::validate_cell_ref("C12").is_ok());
    }
}
