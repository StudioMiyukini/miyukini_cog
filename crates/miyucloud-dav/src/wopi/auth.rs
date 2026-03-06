use crate::common::dav_error::DavError;
use sha2::{Sha256, Digest};

/// WOPI token validator using HMAC-SHA256.
///
/// Tokens encode: `{file_id}:{owner_id}:{timestamp}:{signature}`
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

        // Verify signature
        let payload = format!("{file_id}:{owner_id}:{timestamp}");
        let expected_sig = self.sign(&payload);
        if provided_sig != expected_sig {
            return Err(DavError::Forbidden("Invalid WOPI token signature".into()));
        }

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
        let mut hasher = Sha256::new();
        hasher.update(&self.secret);
        hasher.update(payload.as_bytes());
        hex::encode(hasher.finalize())
    }
}

// Inline hex encode to avoid adding a dependency
mod hex {
    pub fn encode(bytes: impl AsRef<[u8]>) -> String {
        bytes
            .as_ref()
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
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
        let tampered = format!("{}tampered", token);
        assert!(validator.validate_token(&tampered).is_err());
    }
}
