//! Politique d'isolation MWS.

use crate::config::IsolationPolicy;

/// Vérifie si une URL de destination est autorisée selon la politique.
pub fn is_allowed_destination(url: &str, policy: IsolationPolicy) -> bool {
    match policy {
        IsolationPolicy::Unrestricted => true,
        IsolationPolicy::InternalOnly => {
            // Autoriser uniquement les IPs privées et localhost
            is_internal_url(url)
        }
        IsolationPolicy::Airgap => false,
    }
}

/// Vérifie si une URL pointe vers une ressource interne.
fn is_internal_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();
    url_lower.contains("localhost")
        || url_lower.contains("127.0.0.1")
        || url_lower.contains("::1")
        || url_lower.contains("192.168.")
        || url_lower.contains("10.")
        || url_lower.contains("172.16.")
        || url_lower.contains("172.17.")
        || url_lower.contains("172.18.")
        || url_lower.contains("172.19.")
        || url_lower.contains("172.20.")
        || url_lower.contains("172.21.")
        || url_lower.contains("172.22.")
        || url_lower.contains("172.23.")
        || url_lower.contains("172.24.")
        || url_lower.contains("172.25.")
        || url_lower.contains("172.26.")
        || url_lower.contains("172.27.")
        || url_lower.contains("172.28.")
        || url_lower.contains("172.29.")
        || url_lower.contains("172.30.")
        || url_lower.contains("172.31.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airgap_blocks_all() {
        assert!(!is_allowed_destination("http://192.168.1.1:7800", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("https://example.com", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("http://localhost:11434", IsolationPolicy::Airgap));
    }

    #[test]
    fn test_internal_only_allows_private() {
        assert!(is_allowed_destination("http://192.168.1.42:11434", IsolationPolicy::InternalOnly));
        assert!(is_allowed_destination("http://localhost:7800", IsolationPolicy::InternalOnly));
        assert!(is_allowed_destination("http://10.0.0.1:11434", IsolationPolicy::InternalOnly));
    }

    #[test]
    fn test_internal_only_blocks_public() {
        assert!(!is_allowed_destination("https://api.example.com", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("https://openai.com/v1/chat", IsolationPolicy::InternalOnly));
    }

    #[test]
    fn test_unrestricted_allows_all() {
        assert!(is_allowed_destination("https://example.com", IsolationPolicy::Unrestricted));
        assert!(is_allowed_destination("http://192.168.1.1", IsolationPolicy::Unrestricted));
    }
}
