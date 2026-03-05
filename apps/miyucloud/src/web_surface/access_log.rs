//! Journal d'acces RGPD-conforme pour la surface web.
//!
//! @id: miyucloud_web_access_log
//! @do: log_web_access_with_hashed_ip_and_truncated_ua
//! @role: security
//! @layer: app
//!
//! Chaque acces a la surface web est enregistre dans `cloud_access_log` :
//! - IP hashee (SHA-256 avec sel) pour la conformite RGPD
//! - User-Agent tronque a 100 caracteres
//! - Action : download, preview, auth_attempt, auth_fail
//! - Succes ou echec
//!
//! INVARIANT : L'IP brute n'est JAMAIS stockee.

use axum::http::HeaderMap;
use miyucloud::data::types::{AccessAction, AccessLogEntry};
use miyucloud::data::MiyucloudDb;
use miyucloud::errors::MiyucloudError;
use sha2::{Digest, Sha256};
use std::fmt::Write;

/// Tronque un User-Agent a 100 caracteres.
pub fn truncate_user_agent(ua: &str) -> String {
    if ua.len() <= 100 {
        ua.to_string()
    } else {
        ua[..100].to_string()
    }
}

/// Derive un sel pour le hashing IP depuis le COG token via HKDF-SHA256.
///
/// Le sel derive est deterministe pour un meme COG token, permettant
/// des hashes IP reproductibles au sein d'une meme instance.
pub fn derive_ip_salt(cog_token: &str) -> [u8; 32] {
    use hkdf::Hkdf;
    use sha2::Sha256 as HkdfSha256;
    let hk = Hkdf::<HkdfSha256>::new(None, cog_token.as_bytes());
    let mut salt = [0u8; 32];
    hk.expand(b"miyucloud-ip-salt", &mut salt)
        .expect("32 bytes is a valid HKDF output length");
    salt
}

/// Hash une adresse IP avec SHA-256 et un sel derive du COG token.
///
/// Le resultat est un hash hexadecimal de 64 caracteres.
/// L'IP originale ne peut pas etre retrouvee a partir du hash.
pub fn hash_ip(ip: &str, salt: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(ip.as_bytes());
    let result = hasher.finalize();
    result.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Extrait l'IP cliente depuis les en-tetes.
///
/// Si `trust_proxy` est actif, lit `X-Forwarded-For` puis `X-Real-IP`.
/// Sinon, retourne `None` (l'IP socket n'est pas disponible dans ce handler).
pub fn extract_client_ip(headers: &HeaderMap, trust_proxy: bool) -> Option<String> {
    if trust_proxy {
        if let Some(forwarded) = headers.get("X-Forwarded-For") {
            if let Ok(s) = forwarded.to_str() {
                if let Some(ip) = s.split(',').next() {
                    let ip = ip.trim();
                    if !ip.is_empty() {
                        return Some(ip.to_string());
                    }
                }
            }
        }

        if let Some(real_ip) = headers.get("X-Real-IP") {
            if let Ok(s) = real_ip.to_str() {
                let ip = s.trim();
                if !ip.is_empty() {
                    return Some(ip.to_string());
                }
            }
        }
    }

    None
}

/// Extrait le User-Agent de la requete.
pub fn extract_user_agent(headers: &HeaderMap) -> Option<String> {
    headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(ToString::to_string)
}

/// Enregistre un acces dans le journal.
pub fn log_access(
    db: &MiyucloudDb,
    share_link_id: Option<&str>,
    raw_ip: Option<&str>,
    raw_ua: Option<&str>,
    action: AccessAction,
    file_id: Option<&str>,
    success: bool,
    ip_salt: &[u8],
) -> Result<(), MiyucloudError> {
    let hashed_ip = raw_ip.map(|ip| hash_ip(ip, ip_salt));
    let truncated_ua = raw_ua.map(truncate_user_agent);

    let entry = AccessLogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        share_link_id: share_link_id.map(String::from),
        accessor_ip: hashed_ip,
        accessor_ua: truncated_ua,
        action,
        file_id: file_id.map(String::from),
        success,
        created_at: chrono::Utc::now().to_rfc3339(),
    };

    db.access_log_create(&entry)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_COG_TOKEN: &str = "test-cog-token-for-unit-tests";

    fn test_salt() -> [u8; 32] {
        derive_ip_salt(TEST_COG_TOKEN)
    }

    #[test]
    fn test_log_access_creates_entry() {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let salt = test_salt();

        log_access(
            &db,
            Some("link-1"),
            Some("192.168.1.100"),
            Some("Mozilla/5.0 (Windows NT 10.0)"),
            AccessAction::Download,
            Some("file-1"),
            true,
            &salt,
        )
        .unwrap();

        let entries = db.access_log_list(Some("link-1"), 10).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].action, AccessAction::Download);
        assert!(entries[0].success);
    }

    #[test]
    fn test_ip_is_hashed() {
        let salt = test_salt();
        let raw_ip = "192.168.1.100";
        let hashed = hash_ip(raw_ip, &salt);

        // Should be 64 hex chars (SHA-256 = 32 bytes)
        assert_eq!(hashed.len(), 64);
        // Should NOT contain the raw IP
        assert!(!hashed.contains("192.168"));
        // Should be deterministic
        assert_eq!(hash_ip(raw_ip, &salt), hashed);
        // Different IPs should produce different hashes
        assert_ne!(hash_ip("192.168.1.101", &salt), hashed);
    }

    #[test]
    fn test_hash_ip_with_salt_stable() {
        let salt = test_salt();
        let ip = "10.0.0.1";
        let hash1 = hash_ip(ip, &salt);
        let hash2 = hash_ip(ip, &salt);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_ip_different_salt_different_hash() {
        let salt1 = derive_ip_salt("token-1");
        let salt2 = derive_ip_salt("token-2");
        let ip = "192.168.1.1";
        assert_ne!(hash_ip(ip, &salt1), hash_ip(ip, &salt2));
    }

    #[test]
    fn test_user_agent_is_truncated() {
        let short_ua = "Mozilla/5.0";
        assert_eq!(truncate_user_agent(short_ua), short_ua);

        let long_ua = "A".repeat(200);
        let truncated = truncate_user_agent(&long_ua);
        assert_eq!(truncated.len(), 100);
    }

    #[test]
    fn test_log_access_auth_fail() {
        let db = MiyucloudDb::open(":memory:").unwrap();
        let salt = test_salt();

        log_access(
            &db,
            Some("link-2"),
            Some("10.0.0.1"),
            None,
            AccessAction::AuthFail,
            None,
            false,
            &salt,
        )
        .unwrap();

        let entries = db.access_log_list(Some("link-2"), 10).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].action, AccessAction::AuthFail);
        assert!(!entries[0].success);
    }

    #[test]
    fn test_extract_client_ip_trust_proxy() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Forwarded-For", "203.0.113.10, 10.0.0.1".parse().unwrap());
        let ip = extract_client_ip(&headers, true);
        assert_eq!(ip.as_deref(), Some("203.0.113.10"));
    }

    #[test]
    fn test_extract_client_ip_no_trust_proxy() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Real-IP", "203.0.113.11".parse().unwrap());
        let ip = extract_client_ip(&headers, false);
        assert!(ip.is_none());
    }
}
