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

/// Vérifie si une URL pointe vers une ressource interne (RFC 1918 + loopback).
fn is_internal_url(url: &str) -> bool {
    let url_lower = url.to_lowercase();

    // Loopback
    if url_lower.contains("localhost") || url_lower.contains("127.0.0.1") || url_lower.contains("::1") {
        return true;
    }

    // Extraire le host de l'URL (entre "://" et le prochain "/" ou ":")
    let host = extract_host(&url_lower);

    // Vérifier les plages privées RFC 1918
    is_rfc1918(host)
}

/// Extrait le host d'une URL (sans port ni path).
fn extract_host(url: &str) -> &str {
    let after_scheme = url
        .find("://")
        .map(|i| &url[i + 3..])
        .unwrap_or(url);
    // Couper au premier ':' (port) ou '/' (path)
    let end = after_scheme
        .find(|c: char| c == ':' || c == '/')
        .unwrap_or(after_scheme.len());
    &after_scheme[..end]
}

/// Vérifie si un host est une IP privée RFC 1918.
fn is_rfc1918(host: &str) -> bool {
    // 10.0.0.0/8
    if host.starts_with("10.") {
        return true;
    }
    // 192.168.0.0/16
    if host.starts_with("192.168.") {
        return true;
    }
    // 172.16.0.0/12 → 172.16.x.x à 172.31.x.x
    if host.starts_with("172.") {
        if let Some(second_octet) = host.split('.').nth(1).and_then(|s| s.parse::<u8>().ok()) {
            return (16..=31).contains(&second_octet);
        }
    }
    false
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
    fn test_internal_only_172_boundaries() {
        // 172.16 → ok (début de la plage)
        assert!(is_allowed_destination("http://172.16.0.1:8080", IsolationPolicy::InternalOnly));
        // 172.31 → ok (fin de la plage)
        assert!(is_allowed_destination("http://172.31.255.1:8080", IsolationPolicy::InternalOnly));
        // 172.15 → hors plage (< 16)
        assert!(!is_allowed_destination("http://172.15.0.1:8080", IsolationPolicy::InternalOnly));
        // 172.32 → hors plage (> 31)
        assert!(!is_allowed_destination("http://172.32.0.1:8080", IsolationPolicy::InternalOnly));
    }

    #[test]
    fn test_unrestricted_allows_all() {
        assert!(is_allowed_destination("https://example.com", IsolationPolicy::Unrestricted));
        assert!(is_allowed_destination("http://192.168.1.1", IsolationPolicy::Unrestricted));
    }
}
