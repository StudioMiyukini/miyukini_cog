//! Client d'exécution d'un service COG.
//!
//! Trait `ServiceClient` : abstraction du « comment Alicia parle au service ».
//! Les vraies impls (HTTP / IPC / channel local) viennent en P3+ par service.
//! P2 fournit un `MockServiceClient` programmable pour les tests.

// @id service.alicia.dispatcher.service_client
// @role service_executor
// @layer 7
// @do declare_service_client
// @human Trait ServiceClient + mock programmable pour les tests dispatcher.

use std::collections::HashMap;
// C-10 : verrous du client factice levés sans panique en cas d'empoisonnement.
use std::sync::{Mutex, PoisonError};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Résultat d'un appel service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ServiceCallResult {
    /// `true` si le service a accompli l'action.
    pub success: bool,
    /// Données retournées (forme libre).
    pub data: Value,
    /// Message lisible (succès ou erreur).
    pub message: Option<String>,
}

impl ServiceCallResult {
    /// Helper : succès avec données.
    #[must_use]
    pub fn ok(data: Value) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    /// Helper : échec avec message.
    #[must_use]
    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: Value::Null,
            message: Some(message.into()),
        }
    }
}

/// Trait async pour exécuter une capability via un service donné.
///
/// L'identifiant qualifié (`service_id.intent`) est passé en argument. L'impl
/// route en interne vers le bon endpoint.
#[async_trait]
// `#[async_trait]` pose lui-même `#[must_use]` sur la future retournée (déjà must_use).
#[allow(clippy::double_must_use)]
pub trait ServiceClient: Send + Sync {
    /// Invoque la capability.
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult;

    /// Aperçu lisible des effets **réels** de l'appel, montré à l'utilisateur
    /// dans la demande de confirmation (R-018, C-51). Doit être calculé avec
    /// le même analyseur que l'exécution.
    ///
    /// - `None` : pas d'aperçu spécifique (description de la capability seule)
    /// - `Some(Err(_))` : paramètres refusés — aucune confirmation proposée
    fn confirmation_preview(
        &self,
        _qualified_id: &str,
        _params: &Value,
    ) -> Option<Result<String, String>> {
        None
    }
}

/// Mock programmable — utile pour les tests d'intégration du dispatcher.
///
/// Programmable :
/// - réponse par `qualified_id` via [`MockServiceClient::expect`]
/// - réponse par défaut si non programmé : `success = true, data = null`
pub struct MockServiceClient {
    responses: Mutex<HashMap<String, ServiceCallResult>>,
    /// Liste des appels reçus (qualified_id, params).
    calls: Mutex<Vec<(String, Value)>>,
}

impl Default for MockServiceClient {
    fn default() -> Self {
        Self {
            responses: Mutex::new(HashMap::new()),
            calls: Mutex::new(Vec::new()),
        }
    }
}

impl MockServiceClient {
    /// Crée un mock vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Programme la réponse renvoyée pour un `qualified_id`.
    pub fn expect(&self, qualified_id: impl Into<String>, result: ServiceCallResult) {
        self.responses
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(qualified_id.into(), result);
    }

    /// Récupère la liste des appels reçus.
    pub fn calls(&self) -> Vec<(String, Value)> {
        self.calls.lock().unwrap_or_else(PoisonError::into_inner).clone()
    }
}

#[async_trait]
impl ServiceClient for MockServiceClient {
    async fn call(&self, qualified_id: &str, params: &Value) -> ServiceCallResult {
        self.calls
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push((qualified_id.to_string(), params.clone()));
        self.responses
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .get(qualified_id)
            .cloned()
            .unwrap_or_else(|| ServiceCallResult::ok(Value::Null))
    }
}

/// Routeur multi-services : garde un `Arc<dyn ServiceClient>` par `service_id`
/// et délègue chaque appel au bon client.
///
/// Le dispatcher prend un `ServiceClient` unique en paramètre — au runtime,
/// Alicia interagit avec **plusieurs** services COG (JayKoa, JayKonta,
/// Jay Bureau...). Ce routeur agrège les clients sous un trait object
/// stable que le dispatcher peut consommer.
#[derive(Default)]
pub struct ServiceClientRouter {
    clients: std::collections::HashMap<
        String,
        std::sync::Arc<dyn ServiceClient + Send + Sync>,
    >,
}

impl ServiceClientRouter {
    /// Crée un routeur vide.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Enregistre un client pour un `service_id` donné. Remplace silencieusement
    /// tout client préexistant pour le même service.
    pub fn register(
        &mut self,
        service_id: impl Into<String>,
        client: std::sync::Arc<dyn ServiceClient + Send + Sync>,
    ) {
        self.clients.insert(service_id.into(), client);
    }

    /// Liste les service_ids couverts.
    #[must_use]
    pub fn services(&self) -> Vec<String> {
        let mut out: Vec<String> = self.clients.keys().cloned().collect();
        out.sort();
        out
    }

    /// `true` si un client est enregistré pour ce `service_id`.
    #[must_use]
    pub fn has_service(&self, service_id: &str) -> bool {
        self.clients.contains_key(service_id)
    }
}

#[async_trait]
impl ServiceClient for ServiceClientRouter {
    async fn call(&self, qualified_id: &str, params: &serde_json::Value) -> ServiceCallResult {
        let Some((service_id, _)) = qualified_id.split_once('.') else {
            return ServiceCallResult::fail(format!(
                "qualified_id sans point : '{qualified_id}'"
            ));
        };
        match self.clients.get(service_id) {
            Some(client) => client.call(qualified_id, params).await,
            None => ServiceCallResult::fail(format!(
                "aucun ServiceClient enregistré pour le service '{service_id}'"
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn mock_returns_default_when_unprogrammed() {
        let m = MockServiceClient::new();
        let r = m.call("jaykoa.x", &json!({})).await;
        assert!(r.success);
        assert_eq!(r.data, Value::Null);
    }

    #[tokio::test]
    async fn mock_returns_programmed_response() {
        let m = MockServiceClient::new();
        m.expect("jaykoa.schedule_event", ServiceCallResult::ok(json!({"id": "evt_1"})));
        let r = m.call("jaykoa.schedule_event", &json!({"title": "x"})).await;
        assert!(r.success);
        assert_eq!(r.data["id"], "evt_1");
    }

    #[tokio::test]
    async fn mock_records_calls() {
        let m = MockServiceClient::new();
        m.call("a.b", &json!({"k": 1})).await;
        m.call("c.d", &json!({})).await;
        let calls = m.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, "a.b");
        assert_eq!(calls[0].1, json!({"k": 1}));
    }

    // -----------------------------------------------------------------
    // ServiceClientRouter
    // -----------------------------------------------------------------

    #[tokio::test]
    async fn router_routes_qualified_id_to_correct_client() {
        let jaykoa = MockServiceClient::new();
        jaykoa.expect("jaykoa.next_event", ServiceCallResult::ok(json!({"k": "koa"})));
        let jaykonta = MockServiceClient::new();
        jaykonta.expect(
            "jaykonta.add_expense",
            ServiceCallResult::ok(json!({"k": "konta"})),
        );

        let mut router = ServiceClientRouter::new();
        router.register("jaykoa", std::sync::Arc::new(jaykoa));
        router.register("jaykonta", std::sync::Arc::new(jaykonta));

        let r1 = router.call("jaykoa.next_event", &json!({})).await;
        assert_eq!(r1.data["k"], "koa");
        let r2 = router.call("jaykonta.add_expense", &json!({})).await;
        assert_eq!(r2.data["k"], "konta");
    }

    #[tokio::test]
    async fn router_returns_failure_for_unknown_service() {
        let router = ServiceClientRouter::new();
        let r = router.call("ghost.x", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("ghost"));
    }

    #[tokio::test]
    async fn router_rejects_unqualified_id() {
        let router = ServiceClientRouter::new();
        let r = router.call("notqualified", &json!({})).await;
        assert!(!r.success);
        assert!(r.message.unwrap().contains("sans point"));
    }

    #[test]
    fn router_lists_services_sorted() {
        let mut router = ServiceClientRouter::new();
        router.register("jaykonta", std::sync::Arc::new(MockServiceClient::new()));
        router.register("jaykoa", std::sync::Arc::new(MockServiceClient::new()));
        assert_eq!(router.services(), vec!["jaykoa", "jaykonta"]);
        assert!(router.has_service("jaykoa"));
        assert!(!router.has_service("ghost"));
    }
}
