//! # jaykonta-client
//!
//! Adaptateur entre Alicia (via `miyualicia_dispatcher::ServiceClient`) et le
//! service **JayKonta** (comptabilité unifiée Purse + Account).
//!
//! Capabilités exposées (cf. `crates/jaykonta/service.manifest.json`) :
//! - `add_movement` (write, confirmation par défaut `always_for_writes`)
//! - `list_recent_movements` (read)
//! - `get_balance` (read)
//!
//! Le **backend** (`JayKontaBackend`) est un trait async — l'implémentation
//! réelle (HTTP / KindMother / in-process) sera branchée au runtime.
//! P-Konta livre un `FakeJayKontaBackend` programmable pour les tests.

#![doc(html_root_url = "https://docs.miyukini.com/jaykonta-client")]

// @id service.jaykonta.alicia_client
// @role alicia_to_jaykonta_adapter
// @layer 7
// @do bridge_alicia_to_jaykonta
// @human Adaptateur Alicia → JayKonta : route les intents vers un backend pluggable.

use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use miyualicia_dispatcher::{ServiceCallResult, ServiceClient};

/// Sens d'un mouvement comptable.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MovementKind {
    /// Sortie d'argent (négatif côté solde).
    Expense,
    /// Entrée d'argent (positif côté solde).
    Income,
}

impl MovementKind {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "expense" => Some(Self::Expense),
            "income" => Some(Self::Income),
            _ => None,
        }
    }

    fn signed_amount(self, amount: f64) -> f64 {
        match self {
            Self::Expense => -amount.abs(),
            Self::Income => amount.abs(),
        }
    }
}

/// Représentation simplifiée d'un mouvement pour Alicia.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MovementSummary {
    /// Identifiant stable.
    pub id: String,
    /// Cagnotte parente.
    pub purse_id: String,
    /// Sens.
    pub kind: MovementKind,
    /// Montant signé (positif si revenu, négatif si dépense). Helper.
    pub signed_amount: f64,
    /// Libellé court.
    pub label: Option<String>,
    /// Catégorie.
    pub category: Option<String>,
    /// Date d'enregistrement (RFC 3339).
    pub created_at: String,
}

/// Snapshot de solde.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BalanceSummary {
    /// Cagnotte concernée.
    pub purse_id: String,
    /// Solde courant en euros.
    pub balance: f64,
    /// Devise (toujours EUR pour P-Konta — extension future).
    pub currency: String,
}

/// Erreurs renvoyées par le backend.
#[derive(Debug, thiserror::Error)]
pub enum JayKontaError {
    /// Erreur d'I/O ou DB côté backend.
    #[error("backend JayKonta : {0}")]
    Backend(String),
    /// Paramètre invalide reçu d'Alicia.
    #[error("paramètre invalide : {0}")]
    InvalidParam(String),
    /// Cagnotte introuvable.
    #[error("cagnotte introuvable : {0}")]
    PurseNotFound(String),
}

/// Trait abstrait pour les opérations JayKonta exposables à Alicia.
#[async_trait]
pub trait JayKontaBackend: Send + Sync {
    /// Crée un mouvement et retourne le `MovementSummary` posé.
    async fn add_movement(
        &self,
        purse_id: &str,
        kind: MovementKind,
        amount_eur: f64,
        label: Option<String>,
        category: Option<String>,
    ) -> Result<MovementSummary, JayKontaError>;

    /// Liste les `limit` derniers mouvements de la cagnotte (plus récent d'abord).
    async fn list_recent_movements(
        &self,
        purse_id: &str,
        limit: usize,
    ) -> Result<Vec<MovementSummary>, JayKontaError>;

    /// Récupère le solde courant de la cagnotte.
    async fn get_balance(&self, purse_id: &str) -> Result<BalanceSummary, JayKontaError>;

    /// Identifiant de la cagnotte par défaut de l'utilisateur (utilisé quand
    /// l'utilisateur ne précise pas `purse_id`). Doit toujours répondre.
    async fn default_purse_id(&self) -> Result<String, JayKontaError>;
}

/// Le client Alicia ↔ JayKonta.
pub struct JayKontaClient {
    backend: Arc<dyn JayKontaBackend>,
}

impl JayKontaClient {
    /// Construit un client autour d'un backend.
    #[must_use]
    pub fn new(backend: Arc<dyn JayKontaBackend>) -> Self {
        Self { backend }
    }

    fn parse_string_opt<'a>(
        params: &'a Value,
        key: &str,
    ) -> Result<Option<&'a str>, JayKontaError> {
        match params.get(key) {
            None | Some(Value::Null) => Ok(None),
            Some(Value::String(s)) => Ok(Some(s.as_str())),
            Some(other) => Err(JayKontaError::InvalidParam(format!(
                "{key} doit être une chaîne, reçu {other}"
            ))),
        }
    }

    fn parse_amount(params: &Value) -> Result<f64, JayKontaError> {
        match params.get("amount") {
            Some(Value::Number(n)) => n
                .as_f64()
                .filter(|v| v.is_finite() && *v > 0.0)
                .ok_or_else(|| {
                    JayKontaError::InvalidParam(format!(
                        "amount doit être > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayKontaError::InvalidParam(format!(
                "amount doit être un nombre, reçu {other}"
            ))),
            None => Err(JayKontaError::InvalidParam("amount manquant".into())),
        }
    }

    fn parse_kind(params: &Value) -> Result<MovementKind, JayKontaError> {
        match params.get("kind") {
            Some(Value::String(s)) => MovementKind::from_str(s).ok_or_else(|| {
                JayKontaError::InvalidParam(format!(
                    "kind doit être 'expense' ou 'income', reçu '{s}'"
                ))
            }),
            Some(other) => Err(JayKontaError::InvalidParam(format!(
                "kind doit être une chaîne, reçu {other}"
            ))),
            None => Err(JayKontaError::InvalidParam("kind manquant".into())),
        }
    }

    fn parse_limit(params: &Value) -> Result<usize, JayKontaError> {
        match params.get("limit") {
            None | Some(Value::Null) => Ok(10),
            Some(Value::Number(n)) => n
                .as_u64()
                .filter(|v| *v > 0)
                .map(|v| v as usize)
                .ok_or_else(|| {
                    JayKontaError::InvalidParam(format!(
                        "limit doit être un entier > 0, reçu {n}"
                    ))
                }),
            Some(other) => Err(JayKontaError::InvalidParam(format!(
                "limit doit être un entier, reçu {other}"
            ))),
        }
    }

    async fn resolve_purse(
        &self,
        params: &Value,
    ) -> Result<String, JayKontaError> {
        match Self::parse_string_opt(params, "purse_id")? {
            Some(s) => Ok(s.to_string()),
            None => self.backend.default_purse_id().await,
        }
    }

    async fn handle_add_movement(&self, params: &Value) -> ServiceCallResult {
        let amount = match Self::parse_amount(params) {
            Ok(a) => a,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let kind = match Self::parse_kind(params) {
            Ok(k) => k,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let label = match Self::parse_string_opt(params, "label") {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let category = match Self::parse_string_opt(params, "category") {
            Ok(s) => s.map(ToString::to_string),
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let purse_id = match self.resolve_purse(params).await {
            Ok(p) => p,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self
            .backend
            .add_movement(&purse_id, kind, amount, label.clone(), category.clone())
            .await
        {
            Ok(mv) => {
                let kind_label = match kind {
                    MovementKind::Expense => "dépense",
                    MovementKind::Income => "revenu",
                };
                let display = label.unwrap_or_else(|| kind_label.into());
                ServiceCallResult {
                    success: true,
                    data: json!({ "movement": mv }),
                    message: Some(format!(
                        "{kind_label} de {amount:.2} € enregistré (« {display} »)"
                    )),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_list_recent(&self, params: &Value) -> ServiceCallResult {
        let limit = match Self::parse_limit(params) {
            Ok(l) => l,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        let purse_id = match self.resolve_purse(params).await {
            Ok(p) => p,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.list_recent_movements(&purse_id, limit).await {
            Ok(movs) => {
                let count = movs.len();
                ServiceCallResult {
                    success: true,
                    data: json!({
                        "movements": movs,
                        "purse_id": purse_id,
                        "count": count,
                    }),
                    message: Some(format!("{count} mouvement(s) récent(s)")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }

    async fn handle_balance(&self, params: &Value) -> ServiceCallResult {
        let purse_id = match self.resolve_purse(params).await {
            Ok(p) => p,
            Err(e) => return ServiceCallResult::fail(e.to_string()),
        };
        match self.backend.get_balance(&purse_id).await {
            Ok(bal) => {
                let balance = bal.balance;
                let currency = bal.currency.clone();
                ServiceCallResult {
                    success: true,
                    data: json!({ "balance": bal }),
                    message: Some(format!("solde : {balance:.2} {currency}")),
                }
            }
            Err(e) => ServiceCallResult::fail(e.to_string()),
        }
    }
}

#[async_trait]
impl ServiceClient for JayKontaClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        let intent = match miyualicia_skills::split_qualified_id(qualified_id) {
            Ok((service, intent)) => {
                if service != "jaykonta" {
                    return ServiceCallResult::fail(format!(
                        "JayKontaClient appelé avec un service '{service}' inattendu"
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
            "add_movement" => self.handle_add_movement(params).await,
            "list_recent_movements" => self.handle_list_recent(params).await,
            "get_balance" => self.handle_balance(params).await,
            other => ServiceCallResult::fail(format!(
                "intent JayKonta non supporté : '{other}'"
            )),
        }
    }
}

// ----------------------------------------------------------------------
// Backend de test
// ----------------------------------------------------------------------

/// Backend programmable pour les tests E2E du dispatcher.
#[derive(Default)]
pub struct FakeJayKontaBackend {
    inner: RwLock<FakeBackendState>,
}

#[derive(Default)]
struct FakeBackendState {
    movements: Vec<MovementSummary>,
    default_purse: String,
    fail_with: Option<String>,
}

impl FakeJayKontaBackend {
    /// Crée un backend vide. La cagnotte par défaut s'appelle `"default"`.
    #[must_use]
    pub fn new() -> Self {
        Self::with_default_purse("default")
    }

    /// Crée un backend avec une cagnotte par défaut nommée.
    #[must_use]
    pub fn with_default_purse(purse: impl Into<String>) -> Self {
        Self {
            inner: RwLock::new(FakeBackendState {
                movements: Vec::new(),
                default_purse: purse.into(),
                fail_with: None,
            }),
        }
    }

    /// Force toutes les méthodes à renvoyer une erreur backend.
    pub fn force_failure(&self, message: impl Into<String>) {
        self.inner.write().unwrap().fail_with = Some(message.into());
    }

    /// Snapshot des mouvements actuellement en mémoire (utile en test).
    pub fn snapshot_movements(&self) -> Vec<MovementSummary> {
        self.inner.read().unwrap().movements.clone()
    }
}

#[async_trait]
impl JayKontaBackend for FakeJayKontaBackend {
    async fn add_movement(
        &self,
        purse_id: &str,
        kind: MovementKind,
        amount_eur: f64,
        label: Option<String>,
        category: Option<String>,
    ) -> Result<MovementSummary, JayKontaError> {
        let mut state = self.inner.write().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayKontaError::Backend(msg.clone()));
        }
        let id = format!("mv_{}", state.movements.len() + 1);
        let now: DateTime<Utc> = Utc::now();
        let mv = MovementSummary {
            id,
            purse_id: purse_id.to_string(),
            kind,
            signed_amount: kind.signed_amount(amount_eur),
            label,
            category,
            created_at: now.to_rfc3339(),
        };
        state.movements.push(mv.clone());
        Ok(mv)
    }

    async fn list_recent_movements(
        &self,
        purse_id: &str,
        limit: usize,
    ) -> Result<Vec<MovementSummary>, JayKontaError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayKontaError::Backend(msg.clone()));
        }
        let mut filtered: Vec<MovementSummary> = state
            .movements
            .iter()
            .filter(|m| m.purse_id == purse_id)
            .cloned()
            .collect();
        filtered.reverse(); // plus récent d'abord (push = ordre chronologique)
        filtered.truncate(limit);
        Ok(filtered)
    }

    async fn get_balance(&self, purse_id: &str) -> Result<BalanceSummary, JayKontaError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayKontaError::Backend(msg.clone()));
        }
        let balance: f64 = state
            .movements
            .iter()
            .filter(|m| m.purse_id == purse_id)
            .map(|m| m.signed_amount)
            .sum();
        Ok(BalanceSummary {
            purse_id: purse_id.to_string(),
            balance,
            currency: "EUR".into(),
        })
    }

    async fn default_purse_id(&self) -> Result<String, JayKontaError> {
        let state = self.inner.read().unwrap();
        if let Some(msg) = &state.fail_with {
            return Err(JayKontaError::Backend(msg.clone()));
        }
        Ok(state.default_purse.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn add_movement_expense_returns_negative_signed_amount() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend.clone());
        let r = client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 12.5, "kind": "expense", "label": "café"}),
            )
            .await;
        assert!(r.success);
        let mv = &r.data["movement"];
        assert_eq!(mv["signed_amount"], -12.5);
        assert_eq!(mv["kind"], "expense");
        assert_eq!(mv["purse_id"], "default");
        assert_eq!(mv["label"], "café");
        assert!(r.message.unwrap().contains("café"));
    }

    #[tokio::test]
    async fn add_movement_income_returns_positive_signed_amount() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend);
        let r = client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 200.0, "kind": "income"}),
            )
            .await;
        assert!(r.success);
        assert_eq!(r.data["movement"]["signed_amount"], 200.0);
    }

    #[tokio::test]
    async fn add_movement_rejects_negative_amount() {
        let client = JayKontaClient::new(Arc::new(FakeJayKontaBackend::new()));
        let r = client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": -5, "kind": "expense"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("amount"));
    }

    #[tokio::test]
    async fn add_movement_rejects_unknown_kind() {
        let client = JayKontaClient::new(Arc::new(FakeJayKontaBackend::new()));
        let r = client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 5, "kind": "transfert"}),
            )
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("kind"));
    }

    #[tokio::test]
    async fn add_movement_rejects_missing_required() {
        let client = JayKontaClient::new(Arc::new(FakeJayKontaBackend::new()));
        let r = client
            .call("jaykonta.add_movement", &json!({"kind": "expense"}))
            .await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("amount"));
    }

    #[tokio::test]
    async fn list_recent_movements_orders_newest_first_and_caps_to_limit() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend);
        for i in 1..=5 {
            client
                .call(
                    "jaykonta.add_movement",
                    &json!({"amount": i, "kind": "expense", "label": format!("dep{i}")}),
                )
                .await;
        }
        let r = client
            .call("jaykonta.list_recent_movements", &json!({"limit": 3}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 3);
        // Plus récent d'abord — dep5
        assert_eq!(r.data["movements"][0]["label"], "dep5");
        assert_eq!(r.data["movements"][1]["label"], "dep4");
        assert_eq!(r.data["movements"][2]["label"], "dep3");
    }

    #[tokio::test]
    async fn list_recent_movements_default_limit_is_10() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend);
        for _ in 0..15 {
            client
                .call(
                    "jaykonta.add_movement",
                    &json!({"amount": 1, "kind": "expense"}),
                )
                .await;
        }
        let r = client
            .call("jaykonta.list_recent_movements", &json!({}))
            .await;
        assert!(r.success);
        assert_eq!(r.data["count"], 10);
    }

    #[tokio::test]
    async fn get_balance_sums_signed_amounts() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend);
        client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 100, "kind": "income"}),
            )
            .await;
        client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 30, "kind": "expense"}),
            )
            .await;
        let r = client.call("jaykonta.get_balance", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data["balance"]["balance"], 70.0);
        assert_eq!(r.data["balance"]["currency"], "EUR");
    }

    #[tokio::test]
    async fn unknown_intent_fails() {
        let client = JayKontaClient::new(Arc::new(FakeJayKontaBackend::new()));
        let r = client.call("jaykonta.delete_all", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("non supporté"));
    }

    #[tokio::test]
    async fn wrong_service_fails() {
        let client = JayKontaClient::new(Arc::new(FakeJayKontaBackend::new()));
        let r = client.call("jaykoa.next_event", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("inattendu"));
    }

    #[tokio::test]
    async fn backend_failure_propagates() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        backend.force_failure("KindMother offline");
        let client = JayKontaClient::new(backend);
        let r = client.call("jaykonta.get_balance", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("KindMother offline"));
    }

    #[tokio::test]
    async fn explicit_purse_id_overrides_default() {
        let backend = Arc::new(FakeJayKontaBackend::new());
        let client = JayKontaClient::new(backend);
        client
            .call(
                "jaykonta.add_movement",
                &json!({"amount": 50, "kind": "expense", "purse_id": "voyage"}),
            )
            .await;
        // Solde de "default" reste 0
        let r1 = client
            .call("jaykonta.get_balance", &json!({"purse_id": "default"}))
            .await;
        assert_eq!(r1.data["balance"]["balance"], 0.0);
        // Solde de "voyage"
        let r2 = client
            .call("jaykonta.get_balance", &json!({"purse_id": "voyage"}))
            .await;
        assert_eq!(r2.data["balance"]["balance"], -50.0);
    }
}
