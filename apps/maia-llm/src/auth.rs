//! @id     maia_llm_auth
//! @do     auth_middleware
//! @role   securite
//! @layer  app
//! @human  maia-llm — module « auth » : requêtes signées par des appelants épinglés (sans secret partagé).

//! Authentification des appelants de maia-llm (sortie du lot 2, point #9).
//!
//! Plus de clé statique (`auth_key` / `X-MAIA-Key`) : chaque appelant (maia, …)
//! signe ses requêtes avec **sa** clé Ed25519 (`miyauth::http::service`), et
//! maia-llm épingle les clés publiques autorisées par famille de routes :
//!
//! | Famille | Routes | Usage signé | Configuration |
//! |---|---|---|---|
//! | inférence | `/v1/*` | [`USAGE_INFERENCE`] | `appelants_inference` |
//! | modèles | `/maia/llm/models*` | [`USAGE_MODELES`] | `appelants_modeles` |
//!
//! Fail-closed : aucune clé épinglée pour une famille ⇒ 503 ; signature
//! absente, clé inconnue, horodatage hors fenêtre (± 300 s), nonce rejoué ou
//! corps altéré ⇒ 401. Seul `/maia/llm/health` reste public.

use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use miyauth::http::service::{maintenant_unix, GardeSignee};
use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};
use serde_json::json;

/// Usage signé des routes d'inférence (`/v1/*`).
pub const USAGE_INFERENCE: &str = "maia-llm/inference";
/// Usage signé des routes de gestion des modèles (`/maia/llm/models*`).
pub const USAGE_MODELES: &str = "maia-llm/modeles";

/// Taille maximale d'un corps lu pour vérification (octets).
pub const MAX_CORPS_SIGNE: usize = 4 * 1024 * 1024;

/// Gardes par famille de routes.
#[derive(Clone, Debug)]
pub struct Gardes {
    /// Appelants autorisés sur `/v1/*`.
    pub inference: Arc<GardeSignee>,
    /// Appelants autorisés sur `/maia/llm/models*`.
    pub modeles: Arc<GardeSignee>,
}

impl Gardes {
    /// Gardes à partir des clés publiques déjà analysées.
    pub fn new(inference: Vec<[u8; 32]>, modeles: Vec<[u8; 32]>) -> Self {
        Self { inference: Arc::new(GardeSignee::new(inference)), modeles: Arc::new(GardeSignee::new(modeles)) }
    }

    fn pour(&self, chemin: &str) -> (&GardeSignee, &'static str) {
        if chemin.starts_with("/maia/llm/models") {
            (&self.modeles, USAGE_MODELES)
        } else {
            (&self.inference, USAGE_INFERENCE)
        }
    }
}

fn refus(statut: StatusCode, motif: &str) -> Response {
    (statut, Json(json!({"error": {"message": motif, "type": "auth_error"}}))).into_response()
}

/// Middleware axum : requête signée par un appelant épinglé pour la famille de la route.
pub async fn auth_middleware(State(gardes): State<Gardes>, req: Request, next: Next) -> Response {
    let (garde, usage) = gardes.pour(req.uri().path());
    if !garde.est_configuree() {
        return refus(StatusCode::SERVICE_UNAVAILABLE, "aucun appelant épinglé pour cette route");
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
        Err(r) => return refus(StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED), "Unauthorized"),
    };
    let (parts, corps) = req.into_parts();
    let Ok(corps) = axum::body::to_bytes(corps, MAX_CORPS_SIGNE).await else {
        return refus(StatusCode::PAYLOAD_TOO_LARGE, "corps trop volumineux");
    };
    let chemin = parts.uri.path_and_query().map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    if let Err(r) = garde.verifier(&entetes, usage, parts.method.as_str(), chemin, &corps, maintenant) {
        return refus(StatusCode::from_u16(r.statut()).unwrap_or(StatusCode::UNAUTHORIZED), "Unauthorized");
    }
    next.run(Request::from_parts(parts, Body::from(corps))).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{middleware, routing::get, routing::post, Router};
    use miyauth::http::service::entetes_signees;
    use miyusceau::CleSignature;
    use tower::ServiceExt;

    fn app(gardes: Gardes) -> Router {
        Router::new()
            .route("/v1/chat/completions", post(|corps: String| async move { corps }))
            .route("/maia/llm/models", get(|| async { "modeles" }))
            .route_layer(middleware::from_fn_with_state(gardes, auth_middleware))
            .route("/maia/llm/health", get(|| async { "ok" }))
    }

    fn requete(methode: &str, chemin: &str, corps: &str, cle: Option<(&CleSignature, &str)>) -> Request {
        let mut b = Request::builder().method(methode).uri(chemin);
        if let Some((cle, usage)) = cle {
            for (n, v) in entetes_signees(cle, usage, methode, chemin, corps.as_bytes()) {
                b = b.header(n, v);
            }
        }
        b.body(Body::from(corps.to_string())).unwrap()
    }

    async fn statut(app: &Router, req: Request) -> StatusCode {
        app.clone().oneshot(req).await.unwrap().status()
    }

    #[tokio::test]
    async fn signee_par_cle_epinglee_acceptee() {
        let cle = CleSignature::generer();
        let a = app(Gardes::new(vec![cle.cle_publique()], vec![cle.cle_publique()]));
        let r = requete("POST", "/v1/chat/completions", "{}", Some((&cle, USAGE_INFERENCE)));
        assert_eq!(statut(&a, r).await, StatusCode::OK);
        let r = requete("GET", "/maia/llm/models", "", Some((&cle, USAGE_MODELES)));
        assert_eq!(statut(&a, r).await, StatusCode::OK);
    }

    #[tokio::test]
    async fn refus_non_signee_inconnue_mauvaise_famille() {
        let cle = CleSignature::generer();
        let a = app(Gardes::new(vec![cle.cle_publique()], vec![]));
        assert_eq!(statut(&a, requete("POST", "/v1/chat/completions", "{}", None)).await, StatusCode::UNAUTHORIZED);
        let autre = CleSignature::generer();
        let r = requete("POST", "/v1/chat/completions", "{}", Some((&autre, USAGE_INFERENCE)));
        assert_eq!(statut(&a, r).await, StatusCode::UNAUTHORIZED);
        // Usage d'une autre famille : signature invalide.
        let r = requete("POST", "/v1/chat/completions", "{}", Some((&cle, USAGE_MODELES)));
        assert_eq!(statut(&a, r).await, StatusCode::UNAUTHORIZED);
        // Famille sans clé épinglée : 503, jamais de passage.
        let r = requete("GET", "/maia/llm/models", "", Some((&cle, USAGE_MODELES)));
        assert_eq!(statut(&a, r).await, StatusCode::SERVICE_UNAVAILABLE);
    }

    #[tokio::test]
    async fn aucune_cle_configuree_refus_sauf_sante() {
        let a = app(Gardes::new(vec![], vec![]));
        let cle = CleSignature::generer();
        let r = requete("POST", "/v1/chat/completions", "{}", Some((&cle, USAGE_INFERENCE)));
        assert_eq!(statut(&a, r).await, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(statut(&a, requete("POST", "/v1/chat/completions", "{}", None)).await, StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(statut(&a, requete("GET", "/maia/llm/health", "", None)).await, StatusCode::OK);
    }

    #[tokio::test]
    async fn rejeu_perime_et_corps_altere_refuses() {
        let cle = CleSignature::generer();
        let a = app(Gardes::new(vec![cle.cle_publique()], vec![]));
        let entetes = entetes_signees(&cle, USAGE_INFERENCE, "POST", "/v1/chat/completions", b"{}");
        let construire = |corps: &str| {
            let mut b = Request::builder().method("POST").uri("/v1/chat/completions");
            for (n, v) in &entetes {
                b = b.header(*n, v.clone());
            }
            b.body(Body::from(corps.to_string())).unwrap()
        };
        // Corps altéré : refus, et le nonce n'est pas consommé.
        assert_eq!(statut(&a, construire("{\"x\":1}")).await, StatusCode::UNAUTHORIZED);
        assert_eq!(statut(&a, construire("{}")).await, StatusCode::OK);
        // Rejeu.
        assert_eq!(statut(&a, construire("{}")).await, StatusCode::UNAUTHORIZED);
        // Horodatage périmé.
        let vieux = miyauth::http::signer_requete(&cle, USAGE_INFERENCE, "POST", "/v1/chat/completions", b"{}", maintenant_unix() - 600);
        let mut b = Request::builder().method("POST").uri("/v1/chat/completions");
        for (n, v) in vieux.en_entetes() {
            b = b.header(n, v);
        }
        assert_eq!(statut(&a, b.body(Body::from("{}")).unwrap()).await, StatusCode::UNAUTHORIZED);
    }
}
