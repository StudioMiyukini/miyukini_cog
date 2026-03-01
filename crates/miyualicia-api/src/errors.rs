//! Erreurs de l'API REST Alicia.
//!
//! Chaque variante se traduit en un code HTTP et un body JSON d'erreur standard.

// @id: service.alicia.api.errors
// @role: error_types
// @layer: 7
// @human: Erreurs API REST Alicia avec mapping HTTP status codes.
// @do: define_api_error_types

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

/// Erreurs de l'API REST Alicia.
#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    /// Token JWT absent ou malformed.
    #[error("authentification requise")]
    Unauthorized,

    /// Token JWT expire.
    #[error("token expire")]
    TokenExpired,

    /// Token valide mais scope insuffisant.
    #[error("permission insuffisante : scope '{required}' requis")]
    Forbidden {
        /// Scope manquant.
        required: String,
    },

    /// Ressource non trouvee.
    #[error("ressource introuvable : {0}")]
    NotFound(String),

    /// Donnees de requete invalides.
    #[error("requete invalide : {0}")]
    UnprocessableEntity(String),

    /// Trop de requetes (rate limiting).
    #[error("trop de requetes : reessayer dans {retry_after_secs} secondes")]
    TooManyRequests {
        /// Secondes restantes avant reinitialisation.
        retry_after_secs: u64,
    },

    /// Erreur interne du service Alicia.
    #[error("erreur interne : {0}")]
    InternalError(String),

    /// Erreur JWT (jsonwebtoken).
    #[error("erreur JWT : {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    /// Erreur du crate miyualicia (orchestrateur).
    #[error("erreur service Alicia : {0}")]
    AliciaError(String),
}

impl ApiError {
    /// Retourne le code HTTP correspondant a l'erreur.
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized | Self::TokenExpired | Self::JwtError(_) => {
                StatusCode::UNAUTHORIZED
            }
            Self::Forbidden { .. } => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::InternalError(_) | Self::AliciaError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }

    /// Retourne le type d'erreur sous forme de chaine (pour le JSON de reponse).
    fn error_type(&self) -> &'static str {
        match self {
            Self::Unauthorized => "Unauthorized",
            Self::TokenExpired => "TokenExpired",
            Self::Forbidden { .. } => "Forbidden",
            Self::NotFound(_) => "NotFound",
            Self::UnprocessableEntity(_) => "UnprocessableEntity",
            Self::TooManyRequests { .. } => "TooManyRequests",
            Self::InternalError(_) => "InternalError",
            Self::JwtError(_) => "JwtError",
            Self::AliciaError(_) => "AliciaError",
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let error_type = self.error_type();
        let body = serde_json::json!({
            "error": {
                "code": status.as_u16(),
                "message": self.to_string(),
                "type": error_type,
            }
        });
        (status, Json(body)).into_response()
    }
}

impl From<miyualicia::AliciaError> for ApiError {
    fn from(err: miyualicia::AliciaError) -> Self {
        Self::AliciaError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unauthorized_status_code() {
        let err = ApiError::Unauthorized;
        assert_eq!(err.status_code(), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn test_forbidden_status_code() {
        let err = ApiError::Forbidden {
            required: "write".to_string(),
        };
        assert_eq!(err.status_code(), StatusCode::FORBIDDEN);
    }

    #[test]
    fn test_not_found_status_code() {
        let err = ApiError::NotFound("device xyz".to_string());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
    }

    #[test]
    fn test_too_many_requests_status_code() {
        let err = ApiError::TooManyRequests {
            retry_after_secs: 30,
        };
        assert_eq!(err.status_code(), StatusCode::TOO_MANY_REQUESTS);
    }

    #[test]
    fn test_error_type_string() {
        assert_eq!(ApiError::Unauthorized.error_type(), "Unauthorized");
        assert_eq!(ApiError::TokenExpired.error_type(), "TokenExpired");
        assert_eq!(
            ApiError::Forbidden {
                required: "x".to_string()
            }
            .error_type(),
            "Forbidden"
        );
    }
}
