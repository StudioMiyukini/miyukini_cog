//! Routes API de MiyukiniAdmin

use crate::api_handlers::{APIHandler, APIResponse};
use std::fmt;

/// @id: miyukiniadmin_api_routes_router
/// @role: infrastructure
/// @layer: operator
/// @human: Routeur API qui dispatch les requêtes vers les handlers appropriés.
/// @do: route_api_requests
pub struct APIRouter {
    /// @id: miyukiniadmin_api_routes_router_handlers
    /// @role: data
    /// @layer: operator
    /// @human: Handlers enregistrés pour chaque route.
    /// @do: store_route_handlers
    /// @depends: miyukiniadmin_api_routes_router
    handlers: std::collections::HashMap<String, Box<dyn APIHandler>>,
}

impl fmt::Debug for APIRouter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("APIRouter")
            .field("handlers_count", &self.handlers.len())
            .finish()
    }
}

impl APIRouter {
    /// @id: miyukiniadmin_api_routes_router_new
    /// @role: infrastructure
    /// @layer: operator
    /// @human: Crée un nouveau routeur API.
    /// @do: create_api_router
    /// @depends: miyukiniadmin_api_routes_router
    pub fn new() -> Self {
        Self {
            handlers: std::collections::HashMap::new(),
        }
    }

    /// @id: miyukiniadmin_api_routes_router_register
    /// @role: mutator
    /// @layer: operator
    /// @human: Enregistre un handler pour une route.
    /// @do: register_route_handler
    /// @depends: miyukiniadmin_api_routes_router
    pub fn register(&mut self, path: &str, handler: Box<dyn APIHandler>) {
        self.handlers.insert(path.to_string(), handler);
    }

    /// @id: miyukiniadmin_api_routes_router_route
    /// @role: infrastructure
    /// @layer: operator
    /// @human: Route une requête vers le handler approprié.
    /// @do: route_request_to_handler
    /// @depends: miyukiniadmin_api_routes_router
    pub fn route(&self, path: &str, method: &str) -> APIResponse {
        if let Some(handler) = self.handlers.get(path) {
            handler.handle_request(path, method)
        } else {
            APIResponse {
                status: 404,
                body: "Not Found".to_string(),
            }
        }
    }
}

impl Default for APIRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: miyukiniadmin_api_routes_test_router_creation
    /// @role: test
    /// @layer: operator
    /// @human: Test de création d'un routeur API.
    /// @do: verify_api_router_creation
    /// @depends: miyukiniadmin_api_routes_router_new
    #[test]
    fn test_router_creation() {
        let router = APIRouter::new();
        // Route inconnue doit retourner 404
        let response = router.route("/unknown", "GET");
        assert_eq!(response.status, 404);
    }
}
