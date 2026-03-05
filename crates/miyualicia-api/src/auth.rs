//! Authentification JWT HS256 pour l'API REST Alicia.
//!
//! Generation, verification et extraction de tokens JWT.

// @id: service.alicia.api.auth
// @role: jwt_authentication
// @layer: 7
// @human: JWT HS256 Alicia : claims, scopes, generation, verification, extractor axum.
// @do: provide_jwt_auth_for_alicia_api

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::ApiError;

/// Scopes d'acces autorises dans un token JWT Alicia.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JwtScope {
    /// Lecture de l'etat complet.
    Read,
    /// Envoi de commandes.
    Write,
    /// Gestion des automatisations.
    Automations,
    /// Lecture de l'historique.
    History,
    /// Acces administrateur (toutes les operations).
    Admin,
}

impl std::fmt::Display for JwtScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Automations => "automations",
            Self::History => "history",
            Self::Admin => "admin",
        };
        write!(f, "{s}")
    }
}

/// Claims du token JWT Alicia.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject : identifiant du client.
    pub sub: String,
    /// Expiration : timestamp UNIX en secondes.
    pub exp: u64,
    /// Issued at : timestamp UNIX en secondes.
    pub iat: u64,
    /// Scopes d'acces autorises.
    pub scopes: Vec<JwtScope>,
}

impl JwtClaims {
    /// Cree des claims pour un client avec les scopes donnes.
    pub fn new(sub: impl Into<String>, scopes: Vec<JwtScope>, ttl_secs: u64) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            sub: sub.into(),
            exp: now + ttl_secs,
            iat: now,
            scopes,
        }
    }

    /// Retourne `true` si le claim est expire.
    pub fn is_expired(&self) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.exp < now
    }

    /// Retourne `true` si le scope demande est present.
    pub fn has_scope(&self, scope: &JwtScope) -> bool {
        self.scopes.contains(scope) || self.is_admin()
    }

    /// Retourne `true` si le client a le scope Admin.
    pub fn is_admin(&self) -> bool {
        self.scopes.contains(&JwtScope::Admin)
    }

    /// Verifie qu'un scope est present, retourne `Forbidden` sinon.
    pub fn require_scope(&self, scope: &JwtScope) -> Result<(), ApiError> {
        if self.has_scope(scope) {
            Ok(())
        } else {
            Err(ApiError::Forbidden {
                required: scope.to_string(),
            })
        }
    }
}

/// Genere un token JWT signe HS256.
///
/// La cle doit faire au moins 32 octets (256 bits).
pub fn generate_token(claims: &JwtClaims, secret: &[u8]) -> Result<String, ApiError> {
    let header = Header::new(Algorithm::HS256);
    let key = EncodingKey::from_secret(secret);
    jsonwebtoken::encode(&header, claims, &key).map_err(ApiError::JwtError)
}

/// Verifie et decode un token JWT HS256.
///
/// Verifie la signature HMAC-SHA256, l'expiration et l'algorithme.
pub fn verify_token(token: &str, secret: &[u8]) -> Result<JwtClaims, ApiError> {
    let key = DecodingKey::from_secret(secret);
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.leeway = 0; // Pas de tolerance pour le test d'expiration
    validation.required_spec_claims.clear();
    validation.required_spec_claims.insert("exp".to_string());

    let token_data = jsonwebtoken::decode::<JwtClaims>(token, &key, &validation).map_err(|e| {
        match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => ApiError::TokenExpired,
            jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::InvalidToken => ApiError::Unauthorized,
            _ => ApiError::JwtError(e),
        }
    })?;

    Ok(token_data.claims)
}

/// Etat partage contenant la cle JWT.
#[derive(Debug, Clone)]
pub struct JwtSecret(pub Arc<Vec<u8>>);

/// Extractor axum qui verifie le JWT dans `Authorization: Bearer <token>`.
pub struct JwtAuth(pub JwtClaims);

impl<S> FromRequestParts<S> for JwtAuth
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        let parts_ref = &*parts;
        let auth_header = parts_ref
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .map(String::from);

        let jwt_secret = parts_ref.extensions.get::<JwtSecret>().cloned();

        async move {
            let header = auth_header.ok_or(ApiError::Unauthorized)?;

            let token = header
                .strip_prefix("Bearer ")
                .ok_or(ApiError::Unauthorized)?;

            let secret = jwt_secret
                .ok_or_else(|| ApiError::InternalError("JWT secret non configure".to_string()))?;

            let claims = verify_token(token, &secret.0)?;
            Ok(JwtAuth(claims))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SECRET: &[u8] = b"test-secret-key-at-least-32-bytes-long!!";

    // TC-API-01 : generation et verification token roundtrip
    #[test]
    fn test_generate_and_verify_token_roundtrip() {
        let claims = JwtClaims::new("test-client", vec![JwtScope::Read, JwtScope::Write], 3600);
        let token = generate_token(&claims, TEST_SECRET).expect("generate token");
        let verified = verify_token(&token, TEST_SECRET).expect("verify token");
        assert_eq!(verified.sub, "test-client");
        assert_eq!(verified.scopes.len(), 2);
        assert!(verified.has_scope(&JwtScope::Read));
        assert!(verified.has_scope(&JwtScope::Write));
    }

    // TC-API-02 : token expire retourne TokenExpired
    #[test]
    fn test_verify_expired_token_fails() {
        let mut claims = JwtClaims::new("test-client", vec![JwtScope::Read], 0);
        // Forcer l'expiration dans le passe
        claims.exp = claims.iat.saturating_sub(10);
        let token = generate_token(&claims, TEST_SECRET).expect("generate expired token");
        let result = verify_token(&token, TEST_SECRET);
        assert!(matches!(result, Err(ApiError::TokenExpired)));
    }

    // TC-API-03 : signature incorrecte retourne Unauthorized
    #[test]
    fn test_verify_wrong_signature_fails() {
        let claims = JwtClaims::new("test-client", vec![JwtScope::Read], 3600);
        let token = generate_token(&claims, TEST_SECRET).expect("generate token");
        let wrong_secret = b"wrong-secret-key-at-least-32-bytes-long!!";
        let result = verify_token(&token, wrong_secret);
        assert!(matches!(
            result,
            Err(ApiError::Unauthorized | ApiError::JwtError(_))
        ));
    }

    // TC-API-04 : has_scope correct
    #[test]
    fn test_claims_has_scope() {
        let claims = JwtClaims::new("test-client", vec![JwtScope::Read, JwtScope::History], 3600);
        assert!(claims.has_scope(&JwtScope::Read));
        assert!(claims.has_scope(&JwtScope::History));
        assert!(!claims.has_scope(&JwtScope::Write));
        assert!(!claims.has_scope(&JwtScope::Admin));
    }

    #[test]
    fn test_admin_scope_grants_all() {
        let claims = JwtClaims::new("admin-client", vec![JwtScope::Admin], 3600);
        assert!(claims.has_scope(&JwtScope::Read));
        assert!(claims.has_scope(&JwtScope::Write));
        assert!(claims.has_scope(&JwtScope::Automations));
        assert!(claims.has_scope(&JwtScope::History));
        assert!(claims.is_admin());
    }

    #[test]
    fn test_require_scope_ok() {
        let claims = JwtClaims::new("client", vec![JwtScope::Read], 3600);
        assert!(claims.require_scope(&JwtScope::Read).is_ok());
    }

    #[test]
    fn test_require_scope_forbidden() {
        let claims = JwtClaims::new("client", vec![JwtScope::Read], 3600);
        let result = claims.require_scope(&JwtScope::Write);
        assert!(matches!(result, Err(ApiError::Forbidden { .. })));
    }

    #[test]
    fn test_jwt_scope_display() {
        assert_eq!(JwtScope::Read.to_string(), "read");
        assert_eq!(JwtScope::Write.to_string(), "write");
        assert_eq!(JwtScope::Admin.to_string(), "admin");
    }

    #[test]
    fn test_claims_not_expired() {
        let claims = JwtClaims::new("client", vec![JwtScope::Read], 3600);
        assert!(!claims.is_expired());
    }
}
