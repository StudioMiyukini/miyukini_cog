//! @id     miou_llm_bridge_signature
//! @do     verifier_requetes_signees
//! @role   securite
//! @layer  app
//! @human  miou-llm-bridge — module « signature » : requêtes signées entrantes (appelants épinglés) et sortantes (clé de MAIA).

//! Requêtes signées machine à machine (sortie du lot 2, points #6–#9).
//!
//! **Entrant** (API protégée de MAIA) : plus de jeton Bearer partagé ni de
//! loopback ouvert (K-09). Chaque appelant signe avec sa clé Ed25519
//! (`miyauth::http::service`) ; MAIA épingle :
//!
//! | Appelant | Routes | Usage signé | Configuration |
//! |---|---|---|---|
//! | Central, services COG | toute l'API protégée | [`USAGE_API`] | `[security] cles_appelants` |
//! | hub TEAM | `POST /v1/agents/{id}/chat` seulement | [`USAGE_TEAM`] | `[security] cles_hub_team` |
//!
//! Aucune clé épinglée ⇒ 503 ; signature absente, clé inconnue, horodatage
//! hors fenêtre (± 300 s), nonce rejoué ou corps altéré ⇒ 401.
//!
//! **Sortant** : MAIA signe ses appels à maia-llm et au hub TEAM avec **sa**
//! clé de service ([`signer`]).

use axum::{body::Body, extract::Request, http::StatusCode};
use miyauth::http::service::{entetes_signees, maintenant_unix, GardeSignee};
use miyauth::http::{RefusHttp, ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};
use miyusceau::CleSignature;

/// Usage signé des appels à l'API protégée de MAIA.
pub const USAGE_API: &str = "maia/api";
/// Usage signé des appels du hub TEAM (`POST /v1/agents/{id}/chat`).
pub const USAGE_TEAM: &str = "maia/team";

/// Corps maximal lu pour vérification (octets) — même borne que le proxy.
pub const MAX_CORPS_SIGNE: usize = 10 * 1024 * 1024;

/// Gardes des requêtes entrantes.
#[derive(Debug, Default)]
pub struct GardesMaia {
    /// Appelants de l'API complète.
    pub appelants: GardeSignee,
    /// Hubs TEAM (chat d'agent seulement).
    pub hubs_team: GardeSignee,
}

impl GardesMaia {
    /// Gardes à partir des clés publiques déjà analysées.
    pub fn new(appelants: Vec<[u8; 32]>, hubs_team: Vec<[u8; 32]>) -> Self {
        Self { appelants: GardeSignee::new(appelants), hubs_team: GardeSignee::new(hubs_team) }
    }

    /// Vrai si au moins une clé est épinglée.
    pub fn est_configuree(&self) -> bool {
        self.appelants.est_configuree() || self.hubs_team.est_configuree()
    }
}

/// `POST /v1/agents/{id}/chat` (un seul segment d'identifiant).
pub fn est_route_team(methode: &str, chemin: &str) -> bool {
    methode == "POST"
        && chemin
            .strip_prefix("/v1/agents/")
            .and_then(|r| r.strip_suffix("/chat"))
            .is_some_and(|id| !id.is_empty() && !id.contains('/'))
}

/// Refus d'une requête entrante.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Refus {
    /// Statut HTTP.
    pub statut: StatusCode,
    /// Motif court (journal de sécurité, corps de réponse).
    pub motif: &'static str,
}

fn refus_http(r: &RefusHttp) -> Refus {
    Refus {
        statut: StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED),
        motif: match r {
            RefusHttp::EnteteAbsente(_) => "signature_absente",
            RefusHttp::SignataireNonAutorise => "signataire_non_autorise",
            RefusHttp::HorodatageHorsFenetre => "horodatage_hors_fenetre",
            RefusHttp::Rejeu => "rejeu",
            RefusHttp::NoncesSatures => "nonces_satures",
            _ => "signature_invalide",
        },
    }
}

/// Vérifie une requête entrante et la reconstruit (corps lu, borné). Le hub
/// TEAM n'est accepté que sur la route de chat d'agent, avec son usage.
pub async fn verifier(gardes: &GardesMaia, req: Request) -> Result<Request, Refus> {
    if !gardes.est_configuree() {
        return Err(Refus { statut: StatusCode::SERVICE_UNAVAILABLE, motif: "aucun_appelant_epingle" });
    }
    let maintenant = maintenant_unix();
    let route_team = est_route_team(req.method().as_str(), req.uri().path());
    let h = req.headers();
    let valeur = |n: &str| h.get(n).and_then(|v| v.to_str().ok());
    let brut = (valeur(ENTETE_SIGNATAIRE), valeur(ENTETE_HORODATAGE), valeur(ENTETE_NONCE), valeur(ENTETE_SIGNATURE));
    let controler = |g: &GardeSignee| g.controler_entetes(brut.0, brut.1, brut.2, brut.3, maintenant);
    let (garde, usage, entetes) = match controler(&gardes.appelants) {
        Ok(e) => (&gardes.appelants, USAGE_API, e),
        Err(RefusHttp::SignataireNonAutorise) if route_team => match controler(&gardes.hubs_team) {
            Ok(e) => (&gardes.hubs_team, USAGE_TEAM, e),
            Err(r) => return Err(refus_http(&r)),
        },
        Err(r) => return Err(refus_http(&r)),
    };
    let (parts, corps) = req.into_parts();
    let corps = axum::body::to_bytes(corps, MAX_CORPS_SIGNE)
        .await
        .map_err(|_| Refus { statut: StatusCode::PAYLOAD_TOO_LARGE, motif: "corps_trop_volumineux" })?;
    let chemin = parts.uri.path_and_query().map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    garde
        .verifier(&entetes, usage, parts.method.as_str(), chemin, &corps, maintenant)
        .map_err(|r| refus_http(&r))?;
    Ok(Request::from_parts(parts, Body::from(corps)))
}

/// Chemin + requête d'une URL (ce qui est signé). Vide si l'URL est invalide
/// (la signature sera alors refusée par le service).
pub fn chemin_url(url: &str) -> String {
    reqwest::Url::parse(url).map_or_else(
        |_| String::new(),
        |u| match u.query() {
            Some(q) => format!("{}?{q}", u.path()),
            None => u.path().to_string(),
        },
    )
}

/// Ajoute les en-têtes signés à une requête sortante (sans cle : inchangée,
/// donc refusée par un service fail-closed).
pub fn signer(
    rb: reqwest::RequestBuilder,
    cle: Option<&CleSignature>,
    usage: &str,
    methode: &str,
    url: &str,
    corps: &[u8],
) -> reqwest::RequestBuilder {
    let Some(cle) = cle else {
        return rb;
    };
    entetes_signees(cle, usage, methode, &chemin_url(url), corps)
        .into_iter()
        .fold(rb, |rb, (nom, valeur)| rb.header(nom, valeur))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_team_exacte() {
        assert!(est_route_team("POST", "/v1/agents/code/chat"));
        for (m, c) in [
            ("GET", "/v1/agents/code/chat"),
            ("POST", "/v1/agents//chat"),
            ("POST", "/v1/agents/a/b/chat"),
            ("POST", "/v1/agents/code"),
            ("POST", "/v1/skills/x/execute"),
        ] {
            assert!(!est_route_team(m, c), "{m} {c}");
        }
    }

    #[test]
    fn chemin_url_avec_requete() {
        assert_eq!(chemin_url("http://127.0.0.1:11435/v1/models?x=1"), "/v1/models?x=1");
        assert_eq!(chemin_url("http://127.0.0.1:11435/maia/llm/models"), "/maia/llm/models");
    }
}
