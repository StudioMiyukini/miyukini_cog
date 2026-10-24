//! @id     miyucloud_interfaces_middleware_basic_policy
//! @do     encadrer_authentification_basic
//! @role   securite
//! @layer  app
//! @human  miyucloud — Basic + mot de passe d'application : TLS obligatoire et limitation des échecs par IP.

//! Exception acceptée « Basic + mot de passe d'application » (inventaire #19).
//!
//! Les clients DAV / Nextcloud ne savent pas produire de preuve de possession :
//! l'authentification Basic par mot de passe d'application reste acceptée, mais
//! encadrée (fail-closed) :
//!
//! 1. **portée** : appliquée par [`super::auth::app_password_allows`] (M-05) ;
//! 2. **TLS obligatoire** ([`transport_securise`]) : le mot de passe circule en
//!    clair dans l'en-tête ; il est refusé sur HTTP simple, sauf depuis la boucle
//!    locale (client sur la même machine, tunnel MWS local) ;
//! 3. **limitation des échecs par IP** ([`CompteurEchecs`]) en plus du
//!    verrouillage par compte : au-delà de [`ECHECS_MAX`] échecs en
//!    [`FENETRE_ECHECS_SECS`] secondes, l'IP reçoit 429 sans vérification Argon2.

use moka::sync::Cache;
use std::net::IpAddr;
use std::sync::OnceLock;
use std::time::Duration;

use super::rate_limit::TrustedProxies;

/// Échecs Basic tolérés par IP et par fenêtre.
pub const ECHECS_MAX: u32 = 20;
/// Fenêtre de comptage des échecs (secondes).
pub const FENETRE_ECHECS_SECS: u64 = 300;

/// Vrai si le transport protège un secret envoyé dans un en-tête.
///
/// - TLS natif (`MIYUCLOUD_TLS_ENABLED`) : oui ;
/// - pair derrière un **proxy de confiance** : seulement si la dernière valeur
///   de `X-Forwarded-Proto` (posée par le proxy le plus proche) vaut `https` ;
/// - pair en boucle locale qui n'est pas un proxy de confiance : oui (client local) ;
/// - sinon (HTTP simple depuis le réseau, pair inconnu) : non.
pub fn transport_securise(
    tls_natif: bool,
    peer: Option<IpAddr>,
    forwarded_proto: Option<&str>,
    trusted: &TrustedProxies,
) -> bool {
    if tls_natif {
        return true;
    }
    let Some(peer) = peer else {
        return false;
    };
    if trusted.contains(&peer) {
        return forwarded_proto
            .and_then(|v| v.rsplit(',').next())
            .is_some_and(|p| p.trim().eq_ignore_ascii_case("https"));
    }
    let peer = match peer {
        IpAddr::V6(v6) => v6.to_ipv4_mapped().map_or(IpAddr::V6(v6), IpAddr::V4),
        v4 => v4,
    };
    peer.is_loopback()
}

/// Compteur d'échecs d'authentification par IP (fenêtre glissante, borné).
pub struct CompteurEchecs {
    cache: Cache<String, u32>,
    max: u32,
}

impl CompteurEchecs {
    /// `max` échecs par `fenetre_secs`, au plus `entrees` IP suivies.
    pub fn new(max: u32, fenetre_secs: u64, entrees: u64) -> Self {
        Self {
            cache: Cache::builder()
                .time_to_live(Duration::from_secs(fenetre_secs))
                .max_capacity(entrees)
                .build(),
            max,
        }
    }

    /// Vrai si l'IP a atteint le plafond d'échecs.
    pub fn bloque(&self, ip: &str) -> bool {
        self.cache.get(ip).is_some_and(|n| n >= self.max)
    }

    /// Enregistre un échec pour l'IP (incrément atomique : des échecs
    /// concurrents ne se perdent pas).
    pub fn noter_echec(&self, ip: &str) {
        self.cache
            .entry(ip.to_string())
            .and_upsert_with(|actuel| actuel.map_or(1, |e| e.into_value().saturating_add(1)));
    }
}

/// Compteur global des échecs Basic (API, DAV, Nextcloud).
pub fn echecs_basic() -> &'static CompteurEchecs {
    static C: OnceLock<CompteurEchecs> = OnceLock::new();
    C.get_or_init(|| CompteurEchecs::new(ECHECS_MAX, FENETRE_ECHECS_SECS, 100_000))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(s: &str) -> Option<IpAddr> {
        Some(s.parse().unwrap())
    }

    #[test]
    fn basic_over_plain_http_from_network_is_refused() {
        let aucun = TrustedProxies::none();
        assert!(!transport_securise(false, ip("192.168.1.20"), None, &aucun));
        assert!(!transport_securise(
            false,
            ip("203.0.113.9"),
            Some("https"),
            &aucun
        ));
        assert!(!transport_securise(false, None, None, &aucun));
    }

    #[test]
    fn native_tls_and_loopback_are_accepted() {
        let aucun = TrustedProxies::none();
        assert!(transport_securise(true, ip("203.0.113.9"), None, &aucun));
        assert!(transport_securise(false, ip("127.0.0.1"), None, &aucun));
        assert!(transport_securise(false, ip("::1"), None, &aucun));
        assert!(transport_securise(
            false,
            ip("::ffff:127.0.0.1"),
            None,
            &aucun
        ));
    }

    #[test]
    fn trusted_proxy_must_declare_https() {
        let proxy = TrustedProxies::parse("10.0.0.2,127.0.0.1");
        assert!(transport_securise(
            false,
            ip("10.0.0.2"),
            Some("https"),
            &proxy
        ));
        assert!(transport_securise(
            false,
            ip("10.0.0.2"),
            Some("http, https"),
            &proxy
        ));
        assert!(!transport_securise(
            false,
            ip("10.0.0.2"),
            Some("https, http"),
            &proxy
        ));
        assert!(!transport_securise(false, ip("10.0.0.2"), None, &proxy));
        // Proxy local de confiance : la boucle locale ne suffit plus.
        assert!(!transport_securise(
            false,
            ip("127.0.0.1"),
            Some("http"),
            &proxy
        ));
        assert!(!transport_securise(false, ip("127.0.0.1"), None, &proxy));
        // X-Forwarded-Proto d'un pair non fiable : ignoré.
        assert!(!transport_securise(
            false,
            ip("10.0.0.3"),
            Some("https"),
            &proxy
        ));
    }

    #[test]
    fn failures_are_limited_per_ip() {
        let c = CompteurEchecs::new(3, 60, 100);
        for _ in 0..3 {
            assert!(!c.bloque("1.2.3.4"));
            c.noter_echec("1.2.3.4");
        }
        assert!(c.bloque("1.2.3.4"));
        assert!(!c.bloque("1.2.3.5"));
    }

    #[test]
    fn echecs_concurrents_tous_comptes() {
        let c = std::sync::Arc::new(CompteurEchecs::new(1_000, 300, 100));
        let fils: Vec<_> = (0..8)
            .map(|_| {
                let c = c.clone();
                std::thread::spawn(move || {
                    for _ in 0..50 {
                        c.noter_echec("9.9.9.9");
                    }
                })
            })
            .collect();
        for f in fils {
            f.join().unwrap();
        }
        assert_eq!(c.cache.get("9.9.9.9"), Some(400));
    }
}
