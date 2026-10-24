//! @id     miyucloud_dav_wopi_auth
//! @do     definir_wopi_token_validator
//! @role   securite
//! @layer  core
//! @human  miyucloud-dav — module « wopi auth ».

use crate::common::dav_error::DavError;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// Validateur de jetons WOPI signés en HMAC-SHA256 (RFC 2104).
///
/// Format : `{file_id}:{owner_id}:{timestamp}:{hex(HMAC-SHA256(secret, payload))}`
/// où `payload = "{file_id}:{owner_id}:{timestamp}"`.
///
/// Compatibilité (M-20) : l'ancienne signature `SHA256(secret ‖ payload)`
/// (vulnérable à l'extension de longueur) n'est **plus acceptée**. Les jetons
/// WOPI vivent au plus `max_age_secs` (1 h) : les jetons émis avant la
/// migration expirent d'eux-mêmes, aucune période de double acceptation.
#[derive(Clone)]
pub struct WopiTokenValidator {
    secret: Vec<u8>,
    max_age_secs: u64,
}

impl WopiTokenValidator {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            secret: secret.to_vec(),
            max_age_secs: 3600, // 1 hour
        }
    }

    /// Generate a WOPI access token for a file.
    pub fn generate_token(&self, file_id: &str, owner_id: &str) -> String {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let payload = format!("{file_id}:{owner_id}:{timestamp}");
        let sig = self.sign(&payload);
        format!("{payload}:{sig}")
    }

    /// Validate a WOPI access token. Returns (file_id, owner_id) on success.
    pub fn validate_token(&self, token: &str) -> Result<(String, String), DavError> {
        let parts: Vec<&str> = token.splitn(4, ':').collect();
        if parts.len() != 4 {
            return Err(DavError::Forbidden("Invalid WOPI token format".into()));
        }

        let file_id = parts[0];
        let owner_id = parts[1];
        let timestamp: u64 = parts[2]
            .parse()
            .map_err(|_| DavError::Forbidden("Invalid token timestamp".into()))?;
        let provided_sig = parts[3];

        // Vérification de la signature HMAC en temps constant (`verify_slice`).
        let provided_sig = decode_hex(provided_sig)
            .ok_or_else(|| DavError::Forbidden("Invalid WOPI token signature".into()))?;
        let payload = format!("{file_id}:{owner_id}:{timestamp}");
        self.mac(&payload)
            .verify_slice(&provided_sig)
            .map_err(|_| DavError::Forbidden("Invalid WOPI token signature".into()))?;

        // Verify not expired
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if now.saturating_sub(timestamp) > self.max_age_secs {
            return Err(DavError::Forbidden("WOPI token expired".into()));
        }

        Ok((file_id.to_string(), owner_id.to_string()))
    }

    fn sign(&self, payload: &str) -> String {
        hex::encode(self.mac(payload).finalize().into_bytes())
    }

    fn mac(&self, payload: &str) -> HmacSha256 {
        // HMAC accepte toute longueur de clé : `new_from_slice` ne peut pas échouer.
        let mut mac = <HmacSha256 as Mac>::new_from_slice(&self.secret)
            .expect("HMAC accepte toute longueur de clé");
        mac.update(payload.as_bytes());
        mac
    }
}

/// Décode une chaîne hexadécimale (minuscules ou majuscules) ; `None` si invalide.
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    if !s.len().is_multiple_of(2) || !s.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    (0..s.len())
        .step_by(2)
        .map(|i| s.get(i..i + 2).and_then(|b| u8::from_str_radix(b, 16).ok()))
        .collect()
}

// Encodage hex local (évite une dépendance)
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        use std::fmt::Write;
        bytes.as_ref().iter().fold(String::new(), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let validator = WopiTokenValidator::new(b"test-secret-key");
        let token = validator.generate_token("file123", "user1");
        let (file_id, owner_id) = validator.validate_token(&token).unwrap();
        assert_eq!(file_id, "file123");
        assert_eq!(owner_id, "user1");
    }

    #[test]
    fn test_invalid_token_rejected() {
        let validator = WopiTokenValidator::new(b"test-secret-key");
        assert!(validator.validate_token("garbage").is_err());
        assert!(validator.validate_token("a:b:c:d").is_err()); // bad timestamp
    }

    #[test]
    fn test_tampered_signature_rejected() {
        let validator = WopiTokenValidator::new(b"test-secret-key");
        let token = validator.generate_token("file123", "user1");
        let tampered = format!("{token}tampered");
        assert!(validator.validate_token(&tampered).is_err());
    }

    #[test]
    fn test_wrong_secret_rejected() {
        let a = WopiTokenValidator::new(b"secret-a");
        let b = WopiTokenValidator::new(b"secret-b");
        let token = a.generate_token("file123", "user1");
        assert!(b.validate_token(&token).is_err());
    }

    #[test]
    fn test_tampered_payload_rejected() {
        let validator = WopiTokenValidator::new(b"test-secret-key");
        let token = validator.generate_token("file123", "user1");
        let tampered = token.replacen("file123", "file999", 1);
        assert!(validator.validate_token(&tampered).is_err());
    }

    #[test]
    fn test_legacy_sha256_concat_signature_rejected() {
        use sha2::Digest;
        let secret = b"test-secret-key";
        let validator = WopiTokenValidator::new(secret);
        let ts = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let payload = format!("file123:user1:{ts}");
        let mut h = sha2::Sha256::new();
        h.update(secret);
        h.update(payload.as_bytes());
        let legacy = format!("{payload}:{}", hex::encode(h.finalize()));
        assert!(validator.validate_token(&legacy).is_err());
    }

    #[test]
    fn test_signature_is_rfc4231_hmac() {
        // RFC 4231 cas 2.
        let v = WopiTokenValidator::new(b"Jefe");
        assert_eq!(
            v.sign("what do ya want for nothing?"),
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
    }

    #[test]
    fn test_non_hex_signature_rejected() {
        let validator = WopiTokenValidator::new(b"test-secret-key");
        assert!(validator.validate_token("f:u:1:zz").is_err());
        assert!(validator.validate_token("f:u:1:abc").is_err());
    }
}
