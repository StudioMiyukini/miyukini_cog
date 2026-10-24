//! Tool MWS MiyuWebwayTracker — mws.address.tracker_default.
//! Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.

use crate::context::GovernedContext;
use crate::errors::MiyuwebwayTrackerError;

/// Longueur maximale d'un nom d'hôte (RFC 1035).
pub const MAX_HOST_BYTES: usize = 253;

/// @id miyuwebway_tracker_mws_address_tracker_default
/// @role accessor
/// @layer tool
/// @do mws_address_tracker_default_under_governance
//@human / @human Résout l'adresse Tracker (port 21000) ; exécution seule ; déterministe.
/// Adresse par défaut du Tracker (port 21000). En production, préférer config ou env.
/// L'hôte fourni est validé (nom DNS ou IPv4 : `[A-Za-z0-9.-]`, ≤ 253 octets) :
/// un hôte portant port, chemin ou espace est refusé.
pub fn tracker_default(
    ctx: &GovernedContext,
    host: Option<&str>,
) -> Result<String, MiyuwebwayTrackerError> {
    if !ctx.has_mandate() {
        return Err(MiyuwebwayTrackerError::NoMandate);
    }
    let h = host.unwrap_or("127.0.0.1").trim();
    if h.is_empty() {
        return Ok("127.0.0.1:21000".to_string());
    }
    if h.len() > MAX_HOST_BYTES {
        return Err(MiyuwebwayTrackerError::InvalidHost("too long"));
    }
    if !h
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'-'))
        || h.starts_with(['.', '-'])
        || h.ends_with(['.', '-'])
    {
        return Err(MiyuwebwayTrackerError::InvalidHost("forbidden character"));
    }
    Ok(format!("{h}:21000"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adresse_par_defaut() {
        let ctx = GovernedContext::new("m".into(), 1);
        assert_eq!(tracker_default(&ctx, None).unwrap(), "127.0.0.1:21000");
        assert_eq!(
            tracker_default(&ctx, Some("  ")).unwrap(),
            "127.0.0.1:21000"
        );
        assert_eq!(
            tracker_default(&ctx, Some("tracker.miyukini.com")).unwrap(),
            "tracker.miyukini.com:21000"
        );
    }

    #[test]
    fn hote_invalide_refuse() {
        let ctx = GovernedContext::new("m".into(), 1);
        for bad in ["evil.com:80", "a/b", "a b", "-a", "a.", "user@host"] {
            assert!(
                matches!(
                    tracker_default(&ctx, Some(bad)),
                    Err(MiyuwebwayTrackerError::InvalidHost(_))
                ),
                "accepté à tort : {bad}"
            );
        }
        let long = "a".repeat(MAX_HOST_BYTES + 1);
        assert_eq!(
            tracker_default(&ctx, Some(&long)),
            Err(MiyuwebwayTrackerError::InvalidHost("too long"))
        );
    }

    #[test]
    fn refuse_sans_mandat() {
        let empty = GovernedContext::new(String::new(), 1);
        assert_eq!(
            tracker_default(&empty, None),
            Err(MiyuwebwayTrackerError::NoMandate)
        );
    }
}
