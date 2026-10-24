//! @id     miyustt_securite
//! @do     securiser_endpoint_stt
//! @role   securite
//! @layer  core
//! @human  miyustt — module « securite » : jeton bearer, origine locale stricte, adresse d'ecoute.

//! Controles de securite des points d'acces MiyuSTT (R-010, C-12, C-21, C-22).
//!
//! - Jeton bearer compare en temps constant (`subtle`).
//! - Origine navigateur analysee (schema + hote exact + port optionnel) :
//!   seuls `localhost`, `127.0.0.1` et `[::1]` sont locaux.
//! - Adresse d'ecoute : hors loopback, un jeton est obligatoire, sinon refus de demarrer.
//!
//! Tout echec de controle est un refus (fail-closed).
//!
//! Module duplique a l'identique dans `miyustt` et `miyutts` (toolkits independants) :
//! toute correction doit etre reportee dans les deux.

use std::net::SocketAddr;

use axum::http::{header, HeaderMap, StatusCode};
use subtle::ConstantTimeEq;

/// Erreurs de configuration de securite detectees au demarrage.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SecurityConfigError {
    /// Adresse d'ecoute non analysable (nom d'hote, format invalide).
    #[error("adresse d'ecoute invalide (IP:port attendu) : {0}")]
    InvalidBind(String),
    /// Jeton configure mais vide.
    #[error("jeton bearer configure mais vide")]
    EmptyToken,
    /// Ecoute hors loopback sans jeton : demarrage refuse.
    #[error("ecoute non-loopback {0} sans jeton bearer : demarrage refuse")]
    TokenRequired(String),
}

/// Valide le couple (adresse d'ecoute, jeton) avant d'ouvrir un port.
///
/// - Adresse non analysable comme `IP:port` → refus (pas de resolution DNS).
/// - Jeton present mais vide → refus.
/// - Adresse non-loopback sans jeton → refus.
pub(crate) fn check_bind(
    bind_addr: &str,
    bearer_token: Option<&str>,
) -> Result<SocketAddr, SecurityConfigError> {
    let addr: SocketAddr = bind_addr
        .parse()
        .map_err(|_| SecurityConfigError::InvalidBind(bind_addr.to_string()))?;
    let token_present = match bearer_token {
        Some(token) if token.trim().is_empty() => return Err(SecurityConfigError::EmptyToken),
        Some(_) => true,
        None => false,
    };
    if !addr.ip().to_canonical().is_loopback() && !token_present {
        return Err(SecurityConfigError::TokenRequired(bind_addr.to_string()));
    }
    Ok(addr)
}

/// Controle d'une requete : origine puis jeton.
pub(crate) fn check_request(
    headers: &HeaderMap,
    bearer_token: Option<&str>,
    enforce_local_origin: bool,
) -> Result<(), StatusCode> {
    if enforce_local_origin {
        check_origin(headers)?;
    }
    check_bearer(headers, bearer_token)
}

fn check_bearer(headers: &HeaderMap, bearer_token: Option<&str>) -> Result<(), StatusCode> {
    let Some(expected) = bearer_token else {
        return Ok(());
    };
    // Jeton vide configure : aucune requete ne peut s'authentifier (fail-closed).
    if expected.trim().is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let provided = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));
    match provided {
        Some(token) if bool::from(token.as_bytes().ct_eq(expected.as_bytes())) => Ok(()),
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn check_origin(headers: &HeaderMap) -> Result<(), StatusCode> {
    // Sans `Origin` : client non navigateur, accepte (le jeton reste exige s'il est configure).
    let Some(origin) = headers.get(header::ORIGIN) else {
        return Ok(());
    };
    let origin = origin.to_str().map_err(|_| StatusCode::FORBIDDEN)?;
    if is_local_origin(origin) {
        Ok(())
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

/// Vrai si `origin` designe exactement un hote local.
///
/// Forme acceptee : `http(s)://HOTE[:PORT]` avec HOTE ∈ {`localhost`, `127.0.0.1`, `[::1]`}
/// et PORT decimal 0..=65535. Tout autre contenu (sous-domaine, chemin, identifiants,
/// `null`) est refuse.
#[must_use]
pub fn is_local_origin(origin: &str) -> bool {
    let Some(rest) = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
    else {
        return false;
    };

    // Separation hote / suite (`""` ou `:PORT`) puis comparaison exacte de l'hote.
    let port = if let Some(after) = rest.strip_prefix('[') {
        let Some((host, tail)) = after.split_once(']') else {
            return false;
        };
        if host != "::1" {
            return false;
        }
        tail
    } else {
        let (host, tail) = match rest.find(':') {
            Some(idx) => rest.split_at(idx),
            None => (rest, ""),
        };
        if !(host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1") {
            return false;
        }
        tail
    };

    if port.is_empty() {
        return true;
    }
    let Some(digits) = port.strip_prefix(':') else {
        return false;
    };
    !digits.is_empty()
        && digits.len() <= 5
        && digits.bytes().all(|b| b.is_ascii_digit())
        && digits.parse::<u16>().is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn origines_locales_exactes_acceptees() {
        for o in [
            "http://localhost",
            "http://localhost:8080",
            "https://localhost:5173",
            "http://127.0.0.1",
            "http://127.0.0.1:3003",
            "http://[::1]",
            "https://[::1]:8443",
            "http://LOCALHOST:80",
        ] {
            assert!(is_local_origin(o), "{o} devrait etre local");
        }
    }

    #[test]
    fn origines_trompeuses_refusees() {
        for o in [
            "http://localhost.evil.com",
            "http://localhost.evil.com:8080",
            "http://127.0.0.1.evil.com",
            "http://[::1].evil.com",
            "http://[::2]:80",
            "http://localhost:80.evil.com",
            "http://localhost:99999",
            "http://localhost:",
            "http://localhost/",
            "http://localhost:80/x",
            "http://user@localhost",
            "ftp://localhost",
            "null",
            "",
            "https://evil.example",
        ] {
            assert!(!is_local_origin(o), "{o} devrait etre refuse");
        }
    }

    #[test]
    fn bind_non_loopback_sans_jeton_refuse() {
        assert_eq!(
            check_bind("0.0.0.0:3003", None),
            Err(SecurityConfigError::TokenRequired("0.0.0.0:3003".into()))
        );
        assert!(check_bind("192.168.1.10:3003", None).is_err());
        assert!(check_bind("[::]:3003", None).is_err());
    }

    #[test]
    fn bind_jeton_vide_refuse() {
        assert_eq!(
            check_bind("127.0.0.1:3003", Some("  ")),
            Err(SecurityConfigError::EmptyToken)
        );
        assert_eq!(
            check_bind("0.0.0.0:3003", Some("")),
            Err(SecurityConfigError::EmptyToken)
        );
    }

    #[test]
    fn bind_hote_non_analysable_refuse() {
        assert!(matches!(
            check_bind("localhost:3003", Some("secret")),
            Err(SecurityConfigError::InvalidBind(_))
        ));
    }

    #[test]
    fn bind_loopback_ou_avec_jeton_accepte() {
        assert!(check_bind("127.0.0.1:3003", None).is_ok());
        assert!(check_bind("[::1]:3003", None).is_ok());
        assert!(check_bind("0.0.0.0:3003", Some("secret")).is_ok());
    }

    #[test]
    fn jeton_vide_configure_refuse_toute_requete() {
        let mut headers = HeaderMap::new();
        headers.insert(header::AUTHORIZATION, "Bearer ".parse().unwrap());
        assert_eq!(
            check_request(&headers, Some(""), false),
            Err(StatusCode::UNAUTHORIZED)
        );
    }
}
