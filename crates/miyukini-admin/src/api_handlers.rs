//! Handlers API de MiyukiniAdmin

/// @id: miyukiniadmin_api_handler_trait
/// @role: infrastructure
/// @layer: operator
/// @human: Trait de handler API. Traite les requêtes API backend.
/// @do: define_api_handler_contract
pub trait APIHandler {
    /// @id: miyukiniadmin_api_handler_handle_request
    /// @role: infrastructure
    /// @layer: operator
    /// @human: Traite une requête API.
    /// @do: handle_api_request
    /// @depends: miyukiniadmin_api_handler_trait
    /// Traite une requête API.
    ///
    /// # Arguments
    ///
    /// * `path` - Chemin de la requête
    /// * `method` - Méthode HTTP
    ///
    /// # Returns
    ///
    /// Réponse API.
    fn handle_request(&self, path: &str, method: &str) -> APIResponse;
}

/// @id: miyukiniadmin_api_response
/// @role: data
/// @layer: operator
/// @human: Réponse API backend.
/// @do: represent_api_response
#[derive(Debug, Clone)]
pub struct APIResponse {
    /// @id: miyukiniadmin_api_response_status
    /// @role: data
    /// @layer: operator
    /// @human: Code de statut HTTP.
    /// @do: store_status_code
    /// @depends: miyukiniadmin_api_response
    pub status: u16,
    /// @id: miyukiniadmin_api_response_body
    /// @role: data
    /// @layer: operator
    /// @human: Corps de la réponse.
    /// @do: store_response_body
    /// @depends: miyukiniadmin_api_response
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: miyukiniadmin_api_handler_test_response_creation
    /// @role: test
    /// @layer: operator
    /// @human: Test de création d'une réponse API.
    /// @do: verify_api_response_creation
    /// @depends: miyukiniadmin_api_response
    #[test]
    fn test_api_response_creation() {
        let response = APIResponse {
            status: 200,
            body: "OK".to_string(),
        };
        assert_eq!(response.status, 200);
    }
}
