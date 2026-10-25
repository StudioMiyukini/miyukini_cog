//! @id     miyuregistre_editeur_http
//! @do     transporter_envois_revocations_https
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — transport HTTP minimal (ureq, rustls) des envois de révocation vers Origin : HTTPS exigé hors boucle locale, sans redirection, délais et réponse bornés.

//! Transport HTTP des envois vers Origin (fonctionnalité `publier`).
//!
//! - `ureq` 2 (MIT OR Apache-2.0) avec rustls et les racines `webpki-roots` ;
//!   bloquant, aucun runtime asynchrone.
//! - **HTTPS exigé**, sauf vers la boucle locale (`127.0.0.1`, `[::1]`,
//!   `localhost` : tests, tunnel local). La sécurité des envois repose sur
//!   l'enveloppe signée (A-014), TLS protège la confidentialité et la
//!   disponibilité.
//! - Aucune redirection suivie ; délais : connexion 10 s, requête 60 s ;
//!   réponse lue sur au plus [`super::origin::TAILLE_MAX_REPONSE`] octets.

use std::io::Read;
use std::time::Duration;

use super::origin::{ReponseHttp, Transport, TAILLE_MAX_REPONSE};
use crate::enveloppe::ROUTE_REVOCATIONS;

/// Transport `ureq` vers une instance d'Origin.
#[derive(Debug)]
pub struct TransportHttp {
    agent: ureq::Agent,
    url: String,
}

fn boucle_locale(hote: &str) -> bool {
    matches!(hote, "127.0.0.1" | "[::1]" | "localhost")
}

impl TransportHttp {
    /// Transport vers `base` (ex. `https://origin.example`) ; la route de synchro est ajoutée.
    pub fn new(base: &str) -> Result<Self, String> {
        let base = base.trim().trim_end_matches('/');
        let (schema, reste) = base.split_once("://").ok_or("URL d'Origin sans schéma")?;
        let autorite = reste.split('/').next().unwrap_or("");
        let hote = autorite.rsplit_once(':').filter(|(h, p)| !h.is_empty() && p.bytes().all(|c| c.is_ascii_digit())).map_or(autorite, |(h, _)| h);
        if autorite.is_empty() || autorite.contains('@') {
            return Err("URL d'Origin : hôte absent ou identifiants dans l'URL".into());
        }
        match schema {
            "https" => {}
            "http" if boucle_locale(hote) => {}
            _ => return Err("URL d'Origin : HTTPS exigé (HTTP seulement vers la boucle locale)".into()),
        }
        if reste.contains(['?', '#']) {
            return Err("URL d'Origin : ni requête ni fragment".into());
        }
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .redirects(0)
            .user_agent(concat!("miyuregistre-publier/", env!("CARGO_PKG_VERSION")))
            .build();
        Ok(Self { agent, url: format!("{base}{ROUTE_REVOCATIONS}") })
    }

    /// URL complète de la route.
    pub fn url(&self) -> &str {
        &self.url
    }
}

fn lire(reponse: ureq::Response) -> ReponseHttp {
    let statut = reponse.status();
    let mut corps = Vec::new();
    // Corps illisible : traité comme vide (le statut suffit à décider).
    let _ = reponse.into_reader().take(TAILLE_MAX_REPONSE).read_to_end(&mut corps);
    ReponseHttp { statut, corps }
}

impl Transport for TransportHttp {
    fn poster(&mut self, corps: &[u8]) -> Result<ReponseHttp, String> {
        match self.agent.post(&self.url).set("Content-Type", "application/json").send_bytes(corps) {
            Ok(r) | Err(ureq::Error::Status(_, r)) => Ok(lire(r)),
            Err(e) => Err(e.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn url_controlee() {
        assert_eq!(TransportHttp::new("https://origin.example/").unwrap().url(), "https://origin.example/api/auth/revocations/sync");
        assert!(TransportHttp::new("http://127.0.0.1:8080").is_ok());
        assert!(TransportHttp::new("http://localhost").is_ok());
        assert!(TransportHttp::new("http://origin.example").is_err());
        assert!(TransportHttp::new("http://127.0.0.1.evil.example").is_err());
        assert!(TransportHttp::new("https://u:p@origin.example").is_err());
        assert!(TransportHttp::new("origin.example").is_err());
        assert!(TransportHttp::new("https://origin.example/?x=1").is_err());
    }
}
