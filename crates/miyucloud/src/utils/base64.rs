//! Utilitaire base64 centralise (remplace les impls manuelles).
//!
//! @id: miyucloud_utils_base64
//! @do: provide_base64_encode_decode_via_standard_crate
//! @role: util
//! @layer: infra

use base64_crate::engine::general_purpose::STANDARD;
use base64_crate::Engine;

/// Encode des bytes en base64 standard (avec padding).
pub fn encode(data: &[u8]) -> String {
    STANDARD.encode(data)
}

/// Decode une chaine base64 standard.
pub fn decode(input: &str) -> Result<Vec<u8>, crate::errors::MiyucloudError> {
    STANDARD.decode(input).map_err(|e| {
        crate::errors::MiyucloudError::InvalidInput(format!("base64 decode error: {e}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_roundtrip() {
        let original = b"Hello, MiyuCloud! 2FA tokens and secrets.";
        let encoded = encode(original);
        let decoded = decode(&encoded).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_base64_empty() {
        let encoded = encode(b"");
        assert_eq!(encoded, "");
        let decoded = decode("").unwrap();
        assert!(decoded.is_empty());
    }

    #[test]
    fn test_base64_invalid_input() {
        let result = decode("not valid base64!!!");
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("base64 decode error"));
    }

    #[test]
    fn test_base64_known_value() {
        assert_eq!(encode(b"hello"), "aGVsbG8=");
        assert_eq!(decode("aGVsbG8=").unwrap(), b"hello");
    }
}
