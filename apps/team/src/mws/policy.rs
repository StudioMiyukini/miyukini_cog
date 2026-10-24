//! @id     team_mws_policy
//! @do     is_allowed_destination
//! @role   orchestration
//! @layer  app
//! @human  team — module « mws policy ».

//! Politique d'isolation MWS et validation des URL de nœuds (anti-SSRF).
//!
//! Toute URL de nœud MAIA est validée **à l'enregistrement** et **avant chaque
//! requête sortante** (heartbeat) par [`validate_node_url`] :
//! - schéma `http` ou `https` uniquement, sans identifiants (`user:pass@`) ;
//! - destination autorisée par la politique d'isolation ([`is_allowed_destination`]) ;
//! - hôte loopback (`127.0.0.0/8`, `::1`, `localhost`) **ou** présent dans la
//!   liste blanche explicite `node_allowed_hosts` de la configuration.
//!
//! Fail-closed : toute URL non parsable ou ambiguë est refusée.

use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

use reqwest::Url;

use crate::config::IsolationPolicy;

/// Longueur maximale acceptée pour une URL de nœud (octets).
pub const MAX_NODE_URL_LEN: usize = 2048;

/// Cause de refus d'une URL de nœud (une variante par cause, C-11).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NodeUrlError {
    /// URL vide ou trop longue.
    TooLong,
    /// URL non parsable.
    Invalid,
    /// Schéma autre que http/https.
    SchemeNotAllowed,
    /// Identifiants embarqués dans l'URL.
    CredentialsNotAllowed,
    /// URL sans hôte.
    MissingHost,
    /// Refusée par la politique d'isolation.
    BlockedByPolicy,
    /// Hôte ni loopback ni dans la liste blanche.
    HostNotAllowed,
}

impl fmt::Display for NodeUrlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::TooLong => "URL de nœud vide ou trop longue",
            Self::Invalid => "URL de nœud invalide",
            Self::SchemeNotAllowed => "schéma non autorisé (http/https uniquement)",
            Self::CredentialsNotAllowed => "identifiants interdits dans l'URL de nœud",
            Self::MissingHost => "URL de nœud sans hôte",
            Self::BlockedByPolicy => "destination refusée par la politique d'isolation",
            Self::HostNotAllowed => "hôte ni loopback ni dans node_allowed_hosts",
        };
        f.write_str(msg)
    }
}

impl std::error::Error for NodeUrlError {}

/// Vérifie si une URL de destination est autorisée selon la politique.
///
/// - `Airgap` : seule la machine locale, par **adresse IP** loopback
///   (`127.0.0.0/8`, `::1`) ; aucun nom d'hôte, aucune autre adresse.
/// - `InternalOnly` : uniquement les **adresses IP** loopback / privées
///   (RFC 1918, ULA `fc00::/7`) et `localhost`. Un nom d'hôte quelconque est
///   refusé (sa résolution DNS n'est pas maîtrisée).
/// - `Unrestricted` : toute URL parsable.
pub fn is_allowed_destination(url: &str, policy: IsolationPolicy) -> bool {
    match policy {
        IsolationPolicy::Airgap => Url::parse(url)
            .is_ok_and(|u| host_ip(&u).is_some_and(|ip| ip.to_canonical().is_loopback())),
        IsolationPolicy::Unrestricted => Url::parse(url).is_ok(),
        IsolationPolicy::InternalOnly => Url::parse(url).is_ok_and(|u| is_internal_url(&u)),
    }
}

/// Valide une URL de nœud MAIA avant enregistrement ou requête sortante.
pub fn validate_node_url(
    url: &str,
    policy: IsolationPolicy,
    allowed_hosts: &[String],
) -> Result<(), NodeUrlError> {
    if url.is_empty() || url.len() > MAX_NODE_URL_LEN {
        return Err(NodeUrlError::TooLong);
    }
    let parsed = Url::parse(url).map_err(|_| NodeUrlError::Invalid)?;
    if !matches!(parsed.scheme(), "http" | "https") {
        return Err(NodeUrlError::SchemeNotAllowed);
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(NodeUrlError::CredentialsNotAllowed);
    }
    let host = host_string(&parsed).ok_or(NodeUrlError::MissingHost)?;
    if !is_allowed_destination(url, policy) {
        return Err(NodeUrlError::BlockedByPolicy);
    }
    if is_loopback_host(&parsed) {
        return Ok(());
    }
    if allowed_hosts.iter().any(|h| normalize_host(h) == host) {
        Ok(())
    } else {
        Err(NodeUrlError::HostNotAllowed)
    }
}

/// Hôte normalisé (minuscule, sans crochets IPv6).
fn host_string(url: &Url) -> Option<String> {
    url.host_str()
        .filter(|h| !h.is_empty())
        .map(normalize_host)
}

fn normalize_host(h: &str) -> String {
    h.trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .trim_end_matches('.')
        .to_ascii_lowercase()
}

/// Adresse IP de l'hôte, si c'en est une.
fn host_ip(url: &Url) -> Option<IpAddr> {
    // `Url` normalise déjà les IPv4 (ex. `127.1` → `127.0.0.1`) ; IPv6 entre crochets.
    host_string(url)?.parse::<IpAddr>().ok()
}

/// Vrai si l'hôte est loopback (IP loopback ou `localhost`).
fn is_loopback_host(url: &Url) -> bool {
    if let Some(ip) = host_ip(url) {
        return ip.to_canonical().is_loopback();
    }
    host_string(url).is_some_and(|h| h == "localhost")
}

/// Vrai si l'URL pointe vers une ressource interne (loopback, RFC 1918, ULA).
fn is_internal_url(url: &Url) -> bool {
    if is_loopback_host(url) {
        return true;
    }
    match host_ip(url).map(|ip| ip.to_canonical()) {
        Some(IpAddr::V4(v4)) => is_private_v4(v4),
        Some(IpAddr::V6(v6)) => is_unique_local_v6(v6),
        None => false,
    }
}

/// RFC 1918 : 10/8, 172.16/12, 192.168/16.
fn is_private_v4(ip: Ipv4Addr) -> bool {
    ip.is_private()
}

/// IPv6 ULA : fc00::/7.
fn is_unique_local_v6(ip: Ipv6Addr) -> bool {
    (ip.segments()[0] & 0xfe00) == 0xfc00
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_airgap_ip_loopback_seulement() {
        // Décision 2026-10-24 : airgap autorise la machine locale, par IP uniquement.
        assert!(is_allowed_destination("http://127.0.0.1:11434", IsolationPolicy::Airgap));
        assert!(is_allowed_destination("http://[::1]:11434", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("http://localhost:11434", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("http://192.168.1.1:7800", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("https://example.com", IsolationPolicy::Airgap));
        assert!(!is_allowed_destination("http://127.0.0.1.evil.com", IsolationPolicy::Airgap));
    }

    #[test]
    fn test_internal_only_allows_private() {
        assert!(is_allowed_destination("http://192.168.1.42:11434", IsolationPolicy::InternalOnly));
        assert!(is_allowed_destination("http://localhost:7800", IsolationPolicy::InternalOnly));
        assert!(is_allowed_destination("http://10.0.0.1:11434", IsolationPolicy::InternalOnly));
        assert!(is_allowed_destination("http://[::1]:11434", IsolationPolicy::InternalOnly));
    }

    #[test]
    fn test_internal_only_blocks_public() {
        assert!(!is_allowed_destination("https://api.example.com", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("https://openai.com/v1/chat", IsolationPolicy::InternalOnly));
    }

    #[test]
    fn test_internal_only_blocks_substring_tricks() {
        // Anciennes failles : recherche de sous-chaîne « localhost » / « 127.0.0.1 »
        assert!(!is_allowed_destination("http://localhost.evil.com/", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("http://evil.com/?h=127.0.0.1", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("http://127.0.0.1.evil.com/", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("http://10.evil.com/", IsolationPolicy::InternalOnly));
        assert!(!is_allowed_destination("pas une url", IsolationPolicy::InternalOnly));
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

    #[test]
    fn node_url_loopback_ok() {
        let p = IsolationPolicy::InternalOnly;
        assert_eq!(validate_node_url("http://localhost:11434", p, &[]), Ok(()));
        assert_eq!(validate_node_url("http://127.0.0.1:11434", p, &[]), Ok(()));
        assert_eq!(validate_node_url("https://[::1]:11434", p, &[]), Ok(()));
    }

    #[test]
    fn node_url_private_requires_allow_list() {
        let p = IsolationPolicy::InternalOnly;
        assert_eq!(
            validate_node_url("http://192.168.1.42:11434", p, &[]),
            Err(NodeUrlError::HostNotAllowed)
        );
        let allow = vec!["192.168.1.42".to_string()];
        assert_eq!(validate_node_url("http://192.168.1.42:11434", p, &allow), Ok(()));
    }

    #[test]
    fn node_url_refusals() {
        let p = IsolationPolicy::InternalOnly;
        let allow = vec!["169.254.169.254".to_string(), "example.com".to_string()];
        assert_eq!(validate_node_url("", p, &[]), Err(NodeUrlError::TooLong));
        assert_eq!(validate_node_url("::::", p, &[]), Err(NodeUrlError::Invalid));
        assert_eq!(
            validate_node_url("file:///etc/passwd", p, &[]),
            Err(NodeUrlError::SchemeNotAllowed)
        );
        assert_eq!(
            validate_node_url("gopher://127.0.0.1:70/", p, &[]),
            Err(NodeUrlError::SchemeNotAllowed)
        );
        assert_eq!(
            validate_node_url("http://user:pw@127.0.0.1/", p, &[]),
            Err(NodeUrlError::CredentialsNotAllowed)
        );
        // Métadonnées cloud : hors RFC 1918 → refusé par la politique même si listé
        assert_eq!(
            validate_node_url("http://169.254.169.254/", p, &allow),
            Err(NodeUrlError::BlockedByPolicy)
        );
        // Nom d'hôte public refusé en internal_only même si listé
        assert_eq!(
            validate_node_url("http://example.com/", p, &allow),
            Err(NodeUrlError::BlockedByPolicy)
        );
        assert_eq!(
            validate_node_url("http://localhost.evil.com/", p, &[]),
            Err(NodeUrlError::BlockedByPolicy)
        );
        let long = format!("http://127.0.0.1/{}", "a".repeat(MAX_NODE_URL_LEN));
        assert_eq!(validate_node_url(&long, p, &[]), Err(NodeUrlError::TooLong));
    }

    #[test]
    fn node_url_unrestricted_still_needs_allow_list() {
        let p = IsolationPolicy::Unrestricted;
        assert_eq!(
            validate_node_url("http://example.com/", p, &[]),
            Err(NodeUrlError::HostNotAllowed)
        );
        let allow = vec!["Example.com".to_string()];
        assert_eq!(validate_node_url("http://example.com/", p, &allow), Ok(()));
    }

    #[test]
    fn node_url_airgap_loopback_seulement() {
        assert_eq!(
            validate_node_url("http://127.0.0.1:11434", IsolationPolicy::Airgap, &[]),
            Ok(())
        );
        // Même listé, un hôte non loopback reste refusé en airgap.
        let allow = vec!["192.168.1.10".to_string()];
        assert_eq!(
            validate_node_url("http://192.168.1.10:11434", IsolationPolicy::Airgap, &allow),
            Err(NodeUrlError::BlockedByPolicy)
        );
    }
}
