//! @id     miyucloud_application_services_wopi_token_service
//! @do     definir_wopi_token_claims
//! @role   securite
//! @layer  app
//! @human  miyucloud — jetons WOPI courts, liés au fichier et à l'utilisateur, clé propre au processus.

//! Jetons d'accès WOPI (inventaire des secrets partagés #20, exception imposée par WOPI).
//!
//! Le protocole WOPI impose un `?access_token=` opaque que l'éditeur (Collabora,
//! OnlyOffice) renvoie tel quel à l'hôte. Décision (lot 2) :
//!
//! - **HS256 avec une clé aléatoire propre au processus** (32 octets `OsRng`),
//!   jamais configurée, jamais écrite, jamais partagée. `MIYUCLOUD_WOPI_SECRET`
//!   et le repli sur `MIYUCLOUD_JWT_SECRET` sont supprimés.
//! - Pourquoi pas Ed25519 : la signature asymétrique n'apporte quelque chose que
//!   si le vérificateur est distinct de l'émetteur (il ne doit pas pouvoir
//!   frapper de jetons). Ici **l'émetteur et le vérificateur sont le même
//!   processus** : l'éditeur ne vérifie rien, il transporte le jeton. Avec une
//!   clé qui ne quitte pas la mémoire du processus, aucune autre partie ne
//!   détient de quoi vérifier ni frapper ; Ed25519 ne réduirait aucun risque.
//! - **Durée ≤ 15 min** ([`WOPI_TTL_MAX_SECS`], configuration bornée), pas de
//!   tolérance d'horloge ; un jeton expiré est refusé.
//! - **Lié au fichier et à l'utilisateur** : `file_id`, `sub`, `can_write`
//!   signés ; [`WopiTokenService::validate_for_file`] refuse un autre fichier.
//! - Un redémarrage invalide les jetons en cours (l'éditeur demande de recharger).

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::{OsRng, RngCore};
use serde::{Deserialize, Serialize};

use crate::common::errors::{DomainError, ErrorKind};

/// Durée de vie maximale d'un jeton WOPI (secondes).
pub const WOPI_TTL_MAX_SECS: i64 = 900;
/// Durée de vie minimale (secondes).
pub const WOPI_TTL_MIN_SECS: i64 = 60;

/// Claims of a WOPI access token.
#[derive(Debug, Serialize, Deserialize)]
pub struct WopiTokenClaims {
    /// User ID
    pub sub: String,
    /// File ID this token grants access to
    pub file_id: String,
    /// Whether the user can write (edit) the file
    pub can_write: bool,
    /// Token scope — always "wopi" to distinguish from other tokens
    pub scope: String,
    /// Display name for the editor UI
    pub username: String,
    /// Expiration timestamp (seconds since Unix epoch)
    pub exp: i64,
    /// Issued at timestamp
    pub iat: i64,
}

/// Service for generating and validating WOPI access tokens.
pub struct WopiTokenService {
    /// Clé HMAC aléatoire, propre au processus.
    cle: [u8; 32],
    token_ttl_secs: i64,
}

impl WopiTokenService {
    /// Service à clé aléatoire ; `token_ttl_secs` est borné à
    /// [`WOPI_TTL_MIN_SECS`]..=[`WOPI_TTL_MAX_SECS`].
    pub fn new(token_ttl_secs: i64) -> Self {
        let mut cle = [0u8; 32];
        OsRng.fill_bytes(&mut cle);
        let borne = token_ttl_secs.clamp(WOPI_TTL_MIN_SECS, WOPI_TTL_MAX_SECS);
        if borne != token_ttl_secs {
            tracing::warn!(
                "MIYUCLOUD_WOPI_TOKEN_TTL_SECS={} ramené à {} s (bornes {}..={})",
                token_ttl_secs,
                borne,
                WOPI_TTL_MIN_SECS,
                WOPI_TTL_MAX_SECS
            );
        }
        Self {
            cle,
            token_ttl_secs: borne,
        }
    }

    /// Durée de vie effective des jetons (secondes).
    pub fn token_ttl_secs(&self) -> i64 {
        self.token_ttl_secs
    }

    /// Generate a WOPI access token for a specific file and user.
    ///
    /// Returns `(token_string, expiration_unix_ms)`.
    pub fn generate_token(
        &self,
        file_id: &str,
        user_id: &str,
        username: &str,
        can_write: bool,
    ) -> Result<(String, i64), DomainError> {
        self.generate_token_at(
            file_id,
            user_id,
            username,
            can_write,
            Utc::now().timestamp(),
        )
    }

    fn generate_token_at(
        &self,
        file_id: &str,
        user_id: &str,
        username: &str,
        can_write: bool,
        now: i64,
    ) -> Result<(String, i64), DomainError> {
        let claims = WopiTokenClaims {
            sub: user_id.to_string(),
            file_id: file_id.to_string(),
            can_write,
            scope: "wopi".to_string(),
            username: username.to_string(),
            exp: now + self.token_ttl_secs,
            iat: now,
        };

        let token = encode(
            &Header::new(Algorithm::HS256),
            &claims,
            &EncodingKey::from_secret(&self.cle),
        )
        .map_err(|e| {
            DomainError::new(
                ErrorKind::InternalError,
                "WopiTokenService",
                format!("Failed to generate WOPI token: {}", e),
            )
        })?;

        Ok((token, claims.exp * 1000))
    }

    /// Validate a WOPI access token and extract its claims.
    pub fn validate_token(&self, token: &str) -> Result<WopiTokenClaims, DomainError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.leeway = 0;
        validation.validate_exp = true;
        validation.set_required_spec_claims(&["exp"]);

        let token_data =
            decode::<WopiTokenClaims>(token, &DecodingKey::from_secret(&self.cle), &validation)
                .map_err(|e| match e.kind() {
                    jsonwebtoken::errors::ErrorKind::ExpiredSignature => DomainError::new(
                        ErrorKind::AccessDenied,
                        "WopiTokenService",
                        "WOPI token expired",
                    ),
                    _ => DomainError::new(
                        ErrorKind::AccessDenied,
                        "WopiTokenService",
                        "Invalid WOPI token",
                    ),
                })?;

        let claims = token_data.claims;
        if claims.scope != "wopi" || claims.exp - claims.iat > WOPI_TTL_MAX_SECS {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "WopiTokenService",
                "Token is not a WOPI token",
            ));
        }
        Ok(claims)
    }

    /// Valide un jeton **pour ce fichier** (autre fichier = refus).
    pub fn validate_for_file(
        &self,
        token: &str,
        file_id: &str,
    ) -> Result<WopiTokenClaims, DomainError> {
        let claims = self.validate_token(token)?;
        if claims.file_id != file_id {
            return Err(DomainError::new(
                ErrorKind::AccessDenied,
                "WopiTokenService",
                "WOPI token is bound to another file",
            ));
        }
        Ok(claims)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn service() -> WopiTokenService {
        WopiTokenService::new(600)
    }

    #[test]
    fn test_generate_and_validate() {
        let svc = service();
        let (token, ttl_ms) = svc
            .generate_token("file-123", "user-456", "test_user", true)
            .expect("Should generate token");

        let claims = svc
            .validate_for_file(&token, "file-123")
            .expect("Should validate");
        assert_eq!(claims.file_id, "file-123");
        assert_eq!(claims.sub, "user-456");
        assert!(claims.can_write);
        assert_eq!(claims.scope, "wopi");
        assert_eq!(claims.username, "test_user");

        // access_token_ttl must be absolute UNIX time in milliseconds.
        assert_eq!(ttl_ms, claims.exp * 1000);
        assert!(ttl_ms > claims.iat * 1000);
    }

    #[test]
    fn test_reject_invalid_token() {
        assert!(service().validate_token("garbage").is_err());
    }

    #[test]
    fn test_reject_token_from_another_process() {
        // Chaque processus a sa propre clé : un jeton d'ailleurs est refusé.
        let svc1 = service();
        let svc2 = service();
        let (token, _) = svc1
            .generate_token("file-1", "user-1", "test_user", false)
            .expect("Should generate");
        assert!(svc2.validate_token(&token).is_err());
    }

    #[test]
    fn test_reject_tampered_token() {
        let svc = service();
        let (token, _) = svc.generate_token("file-1", "user-1", "u", false).unwrap();
        // Charge utile modifiée (écriture accordée), signature d'origine.
        let parts: Vec<&str> = token.split('.').collect();
        use base64::Engine;
        let b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD;
        let mut claims: serde_json::Value =
            serde_json::from_slice(&b64.decode(parts[1]).unwrap()).unwrap();
        claims["can_write"] = serde_json::Value::Bool(true);
        let forge = format!(
            "{}.{}.{}",
            parts[0],
            b64.encode(serde_json::to_vec(&claims).unwrap()),
            parts[2]
        );
        assert!(svc.validate_token(&forge).is_err());
        // Signature tronquée.
        assert!(svc.validate_token(&token[..token.len() - 2]).is_err());
        // Algorithme « none ».
        let none = format!("{}.{}.", b64.encode(br#"{"alg":"none"}"#), parts[1]);
        assert!(svc.validate_token(&none).is_err());
    }

    #[test]
    fn test_reject_expired_token() {
        let svc = service();
        let past = Utc::now().timestamp() - svc.token_ttl_secs() - 1;
        let (token, _) = svc
            .generate_token_at("file-1", "user-1", "u", true, past)
            .unwrap();
        assert!(svc.validate_token(&token).is_err());
    }

    #[test]
    fn test_reject_other_file() {
        let svc = service();
        let (token, _) = svc.generate_token("file-1", "user-1", "u", true).unwrap();
        assert!(svc.validate_for_file(&token, "file-2").is_err());
        assert!(svc.validate_for_file(&token, "file-1").is_ok());
    }

    #[test]
    fn test_ttl_is_capped_at_15_minutes() {
        assert_eq!(WopiTokenService::new(86_400).token_ttl_secs(), 900);
        assert_eq!(WopiTokenService::new(1).token_ttl_secs(), 60);
        let svc = WopiTokenService::new(86_400);
        let (token, _) = svc.generate_token("f", "u", "n", false).unwrap();
        let c = svc.validate_token(&token).unwrap();
        assert!(c.exp - c.iat <= 900);
    }

    #[test]
    fn test_read_only_token() {
        let svc = service();
        let (token, _) = svc
            .generate_token("file-1", "user-1", "test_user", false)
            .expect("Should generate");
        let claims = svc.validate_token(&token).expect("Should validate");
        assert!(!claims.can_write);
    }
}
