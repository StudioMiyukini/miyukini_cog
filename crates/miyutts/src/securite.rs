//! @id     miyutts_securite
//! @do     securiser_endpoint_tts
//! @role   securite
//! @layer  core
//! @human  miyutts — module « securite » : requetes signees par appelants epingles, origine locale stricte, adresse d'ecoute.

//! Controles de securite des points d'acces MiyuTTS (R-010, C-12, C-21, C-22, lot 2 #10).
//!
//! - Plus de jeton bearer partage : chaque appelant signe ses requetes avec sa
//!   propre cle Ed25519 (`miyauth::http::service`, usage [`USAGE`]) et le service
//!   epingle les cles publiques autorisees. Ordre fail-closed : origine, en-tetes
//!   (cle epinglee, horodatage +/- 300 s, nonce neuf) **avant** la lecture du
//!   corps, puis signature sur le corps lu (borne).
//! - Aucune cle epinglee : toute requete protegee est refusee (503), y compris
//!   en loopback.
//! - Origine navigateur analysee (schema + hote exact + port optionnel) :
//!   seuls `localhost`, `127.0.0.1` et `[::1]` sont locaux.
//! - Adresse d'ecoute : hors loopback, au moins un appelant epingle est
//!   obligatoire, sinon refus de demarrer.
//!
//! Module duplique a l'identique dans `miyustt` et `miyutts` (toolkits independants) :
//! toute correction doit etre reportee dans les deux.

use std::net::SocketAddr;

use axum::body::Body;
use axum::extract::Request;
use axum::http::{header, HeaderMap, StatusCode};
use miyauth::http::service::{maintenant_unix, GardeSignee};
use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};

/// Usage signe des requetes vers MiyuTTS (separation de domaine).
pub const USAGE: &str = "miyutts/synthese";

/// Erreurs de configuration de securite detectees au demarrage.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum SecurityConfigError {
    /// Adresse d'ecoute non analysable (nom d'hote, format invalide).
    #[error("adresse d'ecoute invalide (IP:port attendu) : {0}")]
    InvalidBind(String),
    /// Ecoute hors loopback sans appelant epingle : demarrage refuse.
    #[error("ecoute non-loopback {0} sans appelant epingle : demarrage refuse")]
    CallerRequired(String),
}

/// Valide le couple (adresse d'ecoute, appelants) avant d'ouvrir un port.
///
/// - Adresse non analysable comme `IP:port` → refus (pas de resolution DNS).
/// - Adresse non-loopback sans appelant epingle → refus.
pub(crate) fn check_bind(bind_addr: &str, garde: &GardeSignee) -> Result<SocketAddr, SecurityConfigError> {
    let addr: SocketAddr = bind_addr
        .parse()
        .map_err(|_| SecurityConfigError::InvalidBind(bind_addr.to_string()))?;
    if !addr.ip().to_canonical().is_loopback() && !garde.est_configuree() {
        return Err(SecurityConfigError::CallerRequired(bind_addr.to_string()));
    }
    Ok(addr)
}

/// Controle d'une requete : origine, puis requete signee par un appelant epingle.
/// Rend la requete reconstruite (corps lu, borne a `max_body`) si elle est admise.
pub(crate) async fn check_request(
    req: Request,
    garde: &GardeSignee,
    enforce_local_origin: bool,
    max_body: usize,
) -> Result<Request, StatusCode> {
    if enforce_local_origin {
        check_origin(req.headers())?;
    }
    if !garde.est_configuree() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let now = maintenant_unix();
    let headers = req.headers();
    let value = |n: &str| headers.get(n).and_then(|v| v.to_str().ok());
    let signed = garde
        .controler_entetes(
            value(ENTETE_SIGNATAIRE),
            value(ENTETE_HORODATAGE),
            value(ENTETE_NONCE),
            value(ENTETE_SIGNATURE),
            now,
        )
        .map_err(|r| StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED))?;
    let (parts, body) = req.into_parts();
    let body = axum::body::to_bytes(body, max_body)
        .await
        .map_err(|_| StatusCode::PAYLOAD_TOO_LARGE)?;
    let path = parts
        .uri
        .path_and_query()
        .map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    garde
        .verifier(&signed, USAGE, parts.method.as_str(), path, &body, now)
        .map_err(|r| StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED))?;
    Ok(Request::from_parts(parts, Body::from(body)))
}

fn check_origin(headers: &HeaderMap) -> Result<(), StatusCode> {
    // Sans `Origin` : client non navigateur, accepte (la signature reste exigee).
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
            "http://127.0.0.1:3004",
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

    fn garde_avec_appelant() -> GardeSignee {
        GardeSignee::new([miyusceau::CleSignature::generer().cle_publique()])
    }

    #[test]
    fn bind_non_loopback_sans_appelant_refuse() {
        assert_eq!(
            check_bind("0.0.0.0:3004", &GardeSignee::default()),
            Err(SecurityConfigError::CallerRequired("0.0.0.0:3004".into()))
        );
        assert!(check_bind("192.168.1.10:3004", &GardeSignee::default()).is_err());
        assert!(check_bind("[::]:3004", &GardeSignee::default()).is_err());
    }

    #[test]
    fn bind_hote_non_analysable_refuse() {
        assert!(matches!(
            check_bind("localhost:3004", &garde_avec_appelant()),
            Err(SecurityConfigError::InvalidBind(_))
        ));
    }

    #[test]
    fn bind_loopback_ou_avec_appelant_accepte() {
        assert!(check_bind("127.0.0.1:3004", &GardeSignee::default()).is_ok());
        assert!(check_bind("[::1]:3004", &GardeSignee::default()).is_ok());
        assert!(check_bind("0.0.0.0:3004", &garde_avec_appelant()).is_ok());
    }
}
