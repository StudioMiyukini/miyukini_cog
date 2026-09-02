//! Handlers API de MiyukiniAdmin

/// @id miyukiniadmin_api_handler_trait
/// @role infrastructure
/// @layer operator
/// @do define_api_handler_contract
//@human / @human Trait de handler API. Traite les requêtes API backend.
pub trait APIHandler {
    /// @id miyukiniadmin_api_handler_handle_request
    /// @role infrastructure
    /// @layer operator
    /// @do handle_api_request
    //@human / @human Traite une requête API.
    /// @depends miyukiniadmin_api_handler_trait
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

/// @id miyukiniadmin_api_response
/// @role data
/// @layer operator
/// @do represent_api_response
//@human / @human Réponse API backend.
#[derive(Debug, Clone)]
pub struct APIResponse {
    /// @id miyukiniadmin_api_response_status
    /// @role data
    /// @layer operator
    /// @do store_status_code
    //@human / @human Code de statut HTTP.
    /// @depends miyukiniadmin_api_response
    pub status: u16,
    /// @id miyukiniadmin_api_response_body
    /// @role data
    /// @layer operator
    /// @do store_response_body
    //@human / @human Corps de la réponse.
    /// @depends miyukiniadmin_api_response
    pub body: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_api_handler_test_response_creation
    /// @role test
    /// @layer operator
    /// @do verify_api_response_creation
    //@human / @human Test de création d'une réponse API.
    /// @depends miyukiniadmin_api_response
    #[test]
    fn test_api_response_creation() {
        let response = APIResponse {
            status: 200,
            body: "OK".to_string(),
        };
        assert_eq!(response.status, 200);
    }
}
