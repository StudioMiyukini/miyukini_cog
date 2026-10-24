//! @id     team_auth
//! @do     auth_middleware
//! @role   securite
//! @layer  app
//! @human  team — module « auth » : requêtes signées par des nœuds et clients épinglés (sans secret partagé).

//! Authentification du hub TEAM (C-22, sortie du lot 2, points #7–8).
//!
//! Plus de secret Bearer pré-partagé (`hmac_secret` / `TEAM_HMAC_SECRET`) :
//! chaque appelant signe ses requêtes avec **sa** clé Ed25519
//! (`miyauth::http::service`) et le hub épingle les clés autorisées par famille :
//!
//! | Famille | Routes | Usage signé | Configuration |
//! |---|---|---|---|
//! | nœud MAIA | `POST /team/register`, `POST /team/heartbeat`, `DELETE /team/nodes/{id}` | [`USAGE_NOEUD`] | `noeuds_autorises` |
//! | client | `GET /team/nodes`, `/api/*` | [`USAGE_API`] | `clients_autorises` |
//!
//! Le signataire vérifié est transmis aux handlers ([`Signataire`]) : un nœud
//! enregistré reste lié à la clé qui l'a enregistré (heartbeat et
//! désenregistrement par une autre clé refusés).
//!
//! Fail-closed : aucune clé épinglée pour une famille ⇒ 503, y compris en
//! loopback ; signature absente, clé inconnue, horodatage hors fenêtre
//! (± 300 s), nonce rejoué ou corps altéré ⇒ 401. Seul `GET /team/health`
//! reste public.

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use miyauth::http::service::{maintenant_unix, GardeSignee};
use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};
use serde_json::json;

/// Usage signé des routes de nœuds MAIA (enregistrement, heartbeat, retrait).
pub const USAGE_NOEUD: &str = "team/noeud";
/// Usage signé des routes clientes (`/team/nodes`, `/api/*`).
pub const USAGE_API: &str = "team/api";

/// Corps maximal lu par le middleware pour vérifier la signature (octets) ;
/// les limites par route (`DefaultBodyLimit`) s'appliquent ensuite.
pub const MAX_CORPS_SIGNE: usize = crate::app::MAX_TASK_BODY_BYTES;

/// Clé publique de l'appelant vérifié (extension de requête).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Signataire(pub [u8; 32]);

/// Appelants épinglés, par famille de routes.
#[derive(Clone, Debug, Default)]
pub struct AuthConfig {
    noeuds: Arc<GardeSignee>,
    clients: Arc<GardeSignee>,
}

impl AuthConfig {
    /// Configuration à partir des clés publiques déjà analysées.
    pub fn new(noeuds: Vec<[u8; 32]>, clients: Vec<[u8; 32]>) -> Self {
        Self { noeuds: Arc::new(GardeSignee::new(noeuds)), clients: Arc::new(GardeSignee::new(clients)) }
    }

    fn famille(&self, methode: &Method, chemin: &str) -> (&GardeSignee, &'static str) {
        let noeud = chemin == "/team/register"
            || chemin == "/team/heartbeat"
            || (methode == Method::DELETE && chemin.starts_with("/team/nodes/"));
        if noeud {
            (&self.noeuds, USAGE_NOEUD)
        } else {
            (&self.clients, USAGE_API)
        }
    }
}

fn refus(statut: u16) -> Response {
    let statut = StatusCode::from_u16(statut).unwrap_or(StatusCode::UNAUTHORIZED);
    let motif = if statut == StatusCode::SERVICE_UNAVAILABLE { "unavailable" } else { "unauthorized" };
    (statut, Json(json!({"error": motif}))).into_response()
}

/// Middleware axum : requête signée par un appelant épinglé pour la famille de la route.
pub async fn auth_middleware(State(auth): State<AuthConfig>, req: Request, next: Next) -> Response {
    let (garde, usage) = auth.famille(req.method(), req.uri().path());
    if !garde.est_configuree() {
        return refus(503);
    }
    let maintenant = maintenant_unix();
    let h = req.headers();
    let valeur = |n: &str| h.get(n).and_then(|v| v.to_str().ok());
    let entetes = match garde.controler_entetes(
        valeur(ENTETE_SIGNATAIRE),
        valeur(ENTETE_HORODATAGE),
        valeur(ENTETE_NONCE),
        valeur(ENTETE_SIGNATURE),
        maintenant,
    ) {
        Ok(e) => e,
        Err(r) => return refus(r.statut()),
    };
    let (mut parts, corps) = req.into_parts();
    let Ok(corps) = axum::body::to_bytes(corps, MAX_CORPS_SIGNE).await else {
        return (StatusCode::PAYLOAD_TOO_LARGE, Json(json!({"error": "payload too large"}))).into_response();
    };
    let chemin = parts.uri.path_and_query().map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    if let Err(r) = garde.verifier(&entetes, usage, parts.method.as_str(), chemin, &corps, maintenant) {
        return refus(r.statut());
    }
    parts.extensions.insert(Signataire(entetes.signataire));
    next.run(Request::from_parts(parts, Body::from(corps))).await
}
