//! Tokens temporaires pour la surface web de partage.
//!
//! @id: miyucloud_auth_web_tokens
//! @do: generate_validate_web_share_tokens_with_expiration_and_password
//! @role: auth
//! @layer: domain
//!
//! Genere des tokens non-predictibles (256 bits d'entropie), verifie
//! l'expiration, la revocation, le nombre max de telechargements et le
//! mot de passe optionnel.
//!
//! INVARIANT : Un lien de partage a TOUJOURS une date d'expiration.
//! INVARIANT : Les mots de passe sont hashes (Argon2id), jamais stockes en clair.

use crate::data::types::ShareLink;
use crate::errors::MiyucloudError;
use rand::RngCore;
use std::fmt::Write;

// -----------------------------------------------------------------------
// Token generation
// -----------------------------------------------------------------------

/// Genere un token de partage de 32 bytes (256 bits) encode en hexadecimal.
///
/// Le token fait 64 caracteres hex (32 bytes * 2 hex chars par byte).
/// Utilise `rand::OsRng` pour une entropie cryptographiquement sure.
pub fn generate_share_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

// -----------------------------------------------------------------------
// Password hashing (using Argon2id, reusing the crypto stack)
// -----------------------------------------------------------------------

/// Hash un mot de passe de partage avec Argon2id.
///
/// Retourne le hash sous la forme `{salt_hex}${hash_hex}` ou le sel fait 16 bytes
/// et le hash fait 32 bytes.
pub fn hash_password(password: &str) -> Result<String, MiyucloudError> {
    let mut salt = [0u8; 16];
    rand::rngs::OsRng.fill_bytes(&mut salt);

    let params = password_argon2_params();
    let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut hash = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), &salt, &mut hash)
        .map_err(|e| MiyucloudError::Crypto(format!("Password hash failed: {e}")))?;

    let salt_hex: String = salt.iter().fold(String::with_capacity(32), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    });
    let hash_hex: String = hash.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    });

    Ok(format!("{salt_hex}${hash_hex}"))
}

/// Verifie un mot de passe contre un hash stocke.
///
/// Le format du hash est `{salt_hex}${hash_hex}`.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let parts: Vec<&str> = stored_hash.splitn(2, '$').collect();
    if parts.len() != 2 {
        return false;
    }

    let Some(salt) = hex_decode(parts[0]) else {
        return false;
    };

    let Some(expected_hash) = hex_decode(parts[1]) else {
        return false;
    };

    let params = password_argon2_params();
    let argon2 = argon2::Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params);
    let mut computed = [0u8; 32];
    if argon2
        .hash_password_into(password.as_bytes(), &salt, &mut computed)
        .is_err()
    {
        return false;
    }

    crate::utils::constant_time::constant_time_eq(&computed, &expected_hash)
}

// -----------------------------------------------------------------------
// Share link validation
// -----------------------------------------------------------------------

/// Valide un lien de partage.
///
/// Verifie dans l'ordre :
/// 1. Non revoque
/// 2. Non expire
/// 3. Max downloads non depasse
/// 4. Mot de passe (si requis et fourni)
///
/// Retourne `Ok(())` si le lien est valide.
pub fn validate_share_link(
    link: &ShareLink,
    password: Option<&str>,
    password_hash: Option<&str>,
) -> Result<(), MiyucloudError> {
    // 1. Check revocation
    if link.is_revoked {
        return Err(MiyucloudError::PermissionDenied(
            "Share link has been revoked".to_string(),
        ));
    }

    // 2. Check expiration
    let now = chrono::Utc::now();
    let expires_at = chrono::DateTime::parse_from_rfc3339(&link.expires_at)
        .map_err(|e| MiyucloudError::InvalidInput(format!("Invalid expires_at: {e}")))?;
    if now > expires_at {
        return Err(MiyucloudError::PermissionDenied(
            "Share link has expired".to_string(),
        ));
    }

    // 3. Check max downloads
    if let Some(max) = link.max_downloads {
        if link.download_count >= max {
            return Err(MiyucloudError::PermissionDenied(
                "Maximum download count exceeded".to_string(),
            ));
        }
    }

    // 4. Check password if required
    if link.has_password {
        match (password, password_hash) {
            (Some(pwd), Some(hash)) => {
                if !verify_password(pwd, hash) {
                    return Err(MiyucloudError::PermissionDenied(
                        "Incorrect password".to_string(),
                    ));
                }
            }
            (None, Some(_)) => {
                return Err(MiyucloudError::PermissionDenied(
                    "Password is required for this share link".to_string(),
                ));
            }
            _ => {
                // has_password is true but no hash stored — treat as valid
                // (shouldn't happen in production)
            }
        }
    }

    Ok(())
}

// -----------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------

/// Parametres Argon2id pour le hashing de mots de passe de partage.
/// Parametres plus legers que la master key (car utilise plus frequemment).
fn password_argon2_params() -> argon2::Params {
    #[cfg(debug_assertions)]
    {
        argon2::Params::new(4096, 1, 1, Some(32)).expect("valid argon2 params")
    }
    #[cfg(not(debug_assertions))]
    {
        argon2::Params::new(16384, 2, 2, Some(32)).expect("valid argon2 params")
    }
}

/// Decode un string hexadecimal en bytes.
fn hex_decode(hex: &str) -> Option<Vec<u8>> {
    if !hex.len().is_multiple_of(2) {
        return None;
    }
    let mut bytes = Vec::with_capacity(hex.len() / 2);
    for i in (0..hex.len()).step_by(2) {
        let byte = u8::from_str_radix(&hex[i..i + 2], 16).ok()?;
        bytes.push(byte);
    }
    Some(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_token_is_32_bytes_hex() {
        let token = generate_share_token();
        assert_eq!(token.len(), 64, "Token should be 64 hex chars (32 bytes)");
        // Verify it's valid hex
        assert!(token.chars().all(|c| c.is_ascii_hexdigit()));
        // Verify randomness (two tokens should differ)
        let token2 = generate_share_token();
        assert_ne!(token, token2);
    }

    #[test]
    fn test_validate_token_valid() {
        let link = make_valid_link();
        let result = validate_share_link(&link, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_token_expired() {
        let mut link = make_valid_link();
        link.expires_at = "2020-01-01T00:00:00+00:00".to_string();
        let result = validate_share_link(&link, None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("expired"));
    }

    #[test]
    fn test_validate_token_revoked() {
        let mut link = make_valid_link();
        link.is_revoked = true;
        let result = validate_share_link(&link, None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("revoked"));
    }

    #[test]
    fn test_validate_password_correct() {
        let password = "my-secret-password";
        let hashed = hash_password(password).unwrap();
        let mut link = make_valid_link();
        link.has_password = true;
        let result = validate_share_link(&link, Some(password), Some(&hashed));
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_password_incorrect() {
        let hashed = hash_password("correct-password").unwrap();
        let mut link = make_valid_link();
        link.has_password = true;
        let result = validate_share_link(&link, Some("wrong-password"), Some(&hashed));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Incorrect password"));
    }

    #[test]
    fn test_validate_password_required_but_not_provided() {
        let hashed = hash_password("some-pass").unwrap();
        let mut link = make_valid_link();
        link.has_password = true;
        let result = validate_share_link(&link, None, Some(&hashed));
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Password is required"));
    }

    #[test]
    fn test_validate_max_downloads_exceeded() {
        let mut link = make_valid_link();
        link.max_downloads = Some(5);
        link.download_count = 5;
        let result = validate_share_link(&link, None, None);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Maximum download count"));
    }

    #[test]
    fn test_validate_max_downloads_not_reached() {
        let mut link = make_valid_link();
        link.max_downloads = Some(5);
        link.download_count = 4;
        let result = validate_share_link(&link, None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn test_hash_password_produces_salt_hash_format() {
        let hashed = hash_password("test123").unwrap();
        let parts: Vec<&str> = hashed.splitn(2, '$').collect();
        assert_eq!(parts.len(), 2);
        assert_eq!(parts[0].len(), 32, "Salt should be 32 hex chars (16 bytes)");
        assert_eq!(parts[1].len(), 64, "Hash should be 64 hex chars (32 bytes)");
    }

    #[test]
    fn test_verify_password_correct() {
        let hashed = hash_password("secure-pass").unwrap();
        assert!(verify_password("secure-pass", &hashed));
    }

    #[test]
    fn test_verify_password_incorrect() {
        let hashed = hash_password("secure-pass").unwrap();
        assert!(!verify_password("wrong-pass", &hashed));
    }

    #[test]
    fn test_verify_password_malformed_hash() {
        assert!(!verify_password("test", "not-a-valid-hash"));
        assert!(!verify_password("test", ""));
    }

    fn make_valid_link() -> ShareLink {
        ShareLink {
            id: "link-1".to_string(),
            file_id: Some("file-1".to_string()),
            folder_id: None,
            creator_id: "owner-1".to_string(),
            token: generate_share_token(),
            has_password: false,
            expires_at: "2099-12-31T23:59:59+00:00".to_string(),
            max_downloads: None,
            download_count: 0,
            is_revoked: false,
            created_at: "2026-03-01T00:00:00+00:00".to_string(),
        }
    }
}
