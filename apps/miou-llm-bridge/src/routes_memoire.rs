//! @id     maia_routes_memoire
//! @do     exposer_memoire_a_son_utilisateur
//! @role   security
//! @layer  app
//! @human  MAIA — routes HTTP de la mémoire (EX-56) : voir, corriger, exporter, oublier, valider /
//! @human  rejeter les candidats (TAMR pour les branches sensibles), poser une règle ; réservées à
//! @human  l'utilisateur authentifié, sur SA base `maia-<utilisateur>` ; désactivées par défaut.

//! # Routes de la mémoire de l'utilisateur (EX-56)
//!
//! | Méthode | Chemin | Effet |
//! |---|---|---|
//! | POST | `/v1/memoire/{u}/consulter` | notes courantes + candidats |
//! | POST | `/v1/memoire/{u}/historique` | versions d'une note (`{"id"}`) |
//! | POST | `/v1/memoire/{u}/exporter` | export `maia-memoire/v1` |
//! | POST | `/v1/memoire/{u}/corriger` | nouvelle version (`{"id","texte","importance"?}`) |
//! | DELETE | `/v1/memoire/{u}/notes` | oubli (`{"chemin","motif"}`), pierre tombale |
//! | POST | `/v1/memoire/{u}/regles` | pose une Règle (`{"branche","texte","importance"}`) |
//! | POST | `/v1/memoire/{u}/candidats/{id}/valider` | valide ; branche sensible ⇒ 202 + demande TAMR |
//! | DELETE | `/v1/memoire/{u}/candidats/{id}` | rejette |
//! | POST | `/v1/memoire/{u}/approbations/{iv}` | approuve (`{"empreinte"}`) ⇒ décision StrongFather |
//! | DELETE | `/v1/memoire/{u}/approbations/{iv}` | refuse l'approbation |
//!
//! ## Authentification (minimum fail-closed)
//!
//! Le pont n'a pas encore d'authentification **utilisateur** (seulement des
//! requêtes signées de services : Central, hub TEAM). Ces routes exigent une
//! requête signée Ed25519 (`miyauth::http::service`) d'usage
//! [`USAGE_MEMOIRE`] par une clé **de l'utilisateur** de l'instance, épinglée
//! dans `MAIA_MEMOIRE_CLES_UTILISATEUR` (liste séparée par des virgules) :
//!
//! - clés absentes ou mal formées, ou mémoire non ouverte ⇒ routes
//!   **désactivées** (503) ;
//! - signature absente, clé non épinglée (dont les clés de Central et du hub
//!   TEAM), horodatage hors fenêtre, nonce rejoué, corps altéré ⇒ 401 ;
//! - `{u}` différent de l'utilisateur de l'instance ⇒ 403 (une seule base,
//!   `maia-<utilisateur>`) ;
//! - méthodes POST / DELETE seulement (autre méthode ⇒ 405) ; en-tête `Cookie`
//!   ⇒ 400 (aucune authentification ambiante) ; aucun en-tête CORS ;
//! - origine réseau contrôlée comme le reste de l'API.
//!
//! Revue de sécurité du lot 7 :
//!
//! - **L2 (rejeu après redémarrage)** : l'usage signé lie l'**identifiant
//!   d'instance** (128 bits aléatoires tirés au démarrage) et l'utilisateur :
//!   `maia/memoire/<instance>/<utilisateur>` ; une requête capturée avant un
//!   redémarrage (registre de nonces vidé) ne se rejoue pas. L'identifiant est
//!   rendu dans l'en-tête [`ENTETE_INSTANCE`] de toute réponse de ces routes ;
//! - **F7 (approbation TAMR)** : `POST …/approbations/{iv}` exige une requête
//!   signée d'usage `maia/tamr-approbation/<instance>/<utilisateur>` par une clé
//!   épinglée dans `MAIA_TAMR_CLES_APPROBATION` (voir [`crate::tamr`]),
//!   **distincte** des clés mémoire (clé commune ⇒ approbations refusées ;
//!   liste vide ⇒ approbations refusées). Le refus d'une approbation (`DELETE`)
//!   reste ouvert à la clé mémoire ;
//! - **L3** : limiteur de débit (`security::RateLimiter`, par IP) appliqué
//!   avant tout contrôle ; la signature porte sur l'empreinte du corps, elle ne
//!   peut donc être vérifiée qu'après lecture : les en-têtes (clé épinglée,
//!   fenêtre, nonce) sont contrôlés **avant** la lecture et le corps reste borné
//!   à [`MAX_CORPS`].
//!
//! Le chemin d'inférence et les outils de MAIA ne détiennent qu'une
//! `VueMaia` : ces routes sont le seul accès à `AccesUtilisateur`.

use std::net::SocketAddr;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{ConnectInfo, Path, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, post};
use axum::{Extension, Json, Router};
use maia_memoire::{
    Branche, Entrepot, ErreurMemoire, IdMemoire, Issue, MemoireMaia, Uuid, Validation,
};
use miyauth::http::service::{analyser_cles_publiques, maintenant_unix, GardeSignee};
use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};
use serde::Deserialize;

use crate::config::SecurityConfig;
use crate::gouvernance::GouvernanceMaia;
use crate::security::{RateLimitConfig, RateLimiter};
use crate::tamr::{
    ActionTamr, RefusTamr, RegistreApprobations, SignatureApprobation, ENV_CLES_APPROBATION,
    USAGE_APPROBATION,
};

/// Usage signé (préfixe) des requêtes de l'utilisateur sur sa mémoire ; usage
/// complet : `maia/memoire/<instance>/<utilisateur>` (L2).
pub const USAGE_MEMOIRE: &str = "maia/memoire";
/// Clés publiques de l'utilisateur de l'instance (base64 URL ou hex, séparées par des virgules).
pub const ENV_CLES_UTILISATEUR: &str = "MAIA_MEMOIRE_CLES_UTILISATEUR";
/// Corps maximal d'une requête mémoire (octets).
pub const MAX_CORPS: usize = 256 * 1024;
/// En-tête de réponse portant l'identifiant d'instance à lier dans l'usage signé (L2).
pub const ENTETE_INSTANCE: &str = "x-maia-instance";
/// Requêtes au plus par minute et par IP sur ces routes (L3).
pub const DEBIT_MAX_PAR_MINUTE: u32 = 60;

/// Utilisateur authentifié (posé par la garde).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UtilisateurAuthentifie(pub String);

/// État des routes mémoire.
pub struct EtatMemoireHttp<E: Entrepot> {
    acces: Option<(String, Arc<MemoireMaia<E>>)>,
    garde: GardeSignee,
    garde_approbation: GardeSignee,
    instance: String,
    limiteur: RateLimiter,
    approbations: RegistreApprobations,
    gouvernance: Arc<GouvernanceMaia>,
    securite: SecurityConfig,
}

pub(crate) fn analyser_env(nom: &str) -> Vec<[u8; 32]> {
    let brut = std::env::var(nom).unwrap_or_default();
    let textes: Vec<&str> = brut
        .split(',')
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .collect();
    match analyser_cles_publiques(&textes) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("{nom} : {e} — clés ignorées (fail-closed)");
            Vec::new()
        }
    }
}

impl<E: Entrepot> EtatMemoireHttp<E> {
    /// Routes actives pour `utilisateur`, sa mémoire et ses clés ; sans clé :
    /// désactivées. `cles_approbation` (F7) : clés d'approbation TAMR,
    /// distinctes des clés mémoire ; une clé commune ⇒ aucune approbation.
    pub fn new(
        acces: Option<(String, Arc<MemoireMaia<E>>)>,
        cles_utilisateur: Vec<[u8; 32]>,
        cles_approbation: Vec<[u8; 32]>,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let cles_approbation = if cles_approbation
            .iter()
            .any(|c| cles_utilisateur.contains(c))
        {
            tracing::error!("{ENV_CLES_APPROBATION} : clé aussi épinglée pour « {USAGE_MEMOIRE} » — approbations TAMR refusées");
            Vec::new()
        } else {
            cles_approbation
        };
        Self {
            acces,
            garde: GardeSignee::new(cles_utilisateur),
            garde_approbation: GardeSignee::new(cles_approbation),
            instance: crate::garde_etranger::identifiant_aleatoire(),
            limiteur: RateLimiter::new(RateLimitConfig {
                max_requests: DEBIT_MAX_PAR_MINUTE,
                window: std::time::Duration::from_secs(60),
            }),
            approbations: RegistreApprobations::new(),
            gouvernance,
            securite,
        }
    }

    /// Clés lues dans [`ENV_CLES_UTILISATEUR`] et [`ENV_CLES_APPROBATION`] ;
    /// absentes ou mal formées ⇒ routes (ou approbations) désactivées.
    pub fn depuis_env(
        acces: Option<(String, Arc<MemoireMaia<E>>)>,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let etat = Self::new(
            acces,
            analyser_env(ENV_CLES_UTILISATEUR),
            analyser_env(ENV_CLES_APPROBATION),
            gouvernance,
            securite,
        );
        if etat.est_active() {
            tracing::info!("routes mémoire actives (requêtes signées « {USAGE_MEMOIRE}/<instance>/<utilisateur> » de l'utilisateur)");
        } else {
            tracing::info!(
                "routes mémoire désactivées (mémoire non ouverte ou {ENV_CLES_UTILISATEUR} vide)"
            );
        }
        if !etat.garde_approbation.est_configuree() {
            tracing::info!(
                "approbations TAMR désactivées ({ENV_CLES_APPROBATION} vide ou invalide)"
            );
        }
        etat
    }

    /// Vrai si les routes sont servies.
    pub fn est_active(&self) -> bool {
        self.acces.is_some() && self.garde.est_configuree()
    }

    /// Identifiant d'instance fixé (tests).
    #[cfg(test)]
    fn avec_instance(mut self, instance: &str) -> Self {
        self.instance = instance.to_string();
        self
    }

    /// Identifiant d'instance (aléatoire, tiré au démarrage) lié dans l'usage signé (L2).
    pub fn instance(&self) -> &str {
        &self.instance
    }

    /// Usage signé attendu des requêtes mémoire de `utilisateur` (L2).
    pub fn usage_memoire(&self, utilisateur: &str) -> String {
        format!("{USAGE_MEMOIRE}/{}/{utilisateur}", self.instance)
    }

    /// Usage signé attendu des approbations TAMR de `utilisateur` (F7, L2).
    pub fn usage_approbation(&self, utilisateur: &str) -> String {
        format!("{USAGE_APPROBATION}/{}/{utilisateur}", self.instance)
    }
}

/// Routeur des routes mémoire (à fusionner avec le routeur principal).
pub fn routeur<E: Entrepot + 'static>(etat: Arc<EtatMemoireHttp<E>>) -> Router {
    Router::new()
        .route("/v1/memoire/{utilisateur}/consulter", post(consulter::<E>))
        .route(
            "/v1/memoire/{utilisateur}/historique",
            post(historique::<E>),
        )
        .route("/v1/memoire/{utilisateur}/exporter", post(exporter::<E>))
        .route("/v1/memoire/{utilisateur}/corriger", post(corriger::<E>))
        .route("/v1/memoire/{utilisateur}/notes", delete(oublier::<E>))
        .route("/v1/memoire/{utilisateur}/regles", post(poser_regle::<E>))
        .route(
            "/v1/memoire/{utilisateur}/candidats/{id}/valider",
            post(valider::<E>),
        )
        .route(
            "/v1/memoire/{utilisateur}/candidats/{id}",
            delete(rejeter::<E>),
        )
        .route(
            "/v1/memoire/{utilisateur}/approbations/{intervention}",
            post(approuver::<E>).delete(refuser_approbation::<E>),
        )
        .layer(axum::middleware::from_fn_with_state(
            Arc::clone(&etat),
            garde::<E>,
        ))
        .with_state(etat)
}

fn refus(statut: StatusCode, motif: &str) -> Response {
    (
        statut,
        Json(serde_json::json!({ "error": "accès mémoire refusé", "reason": motif })),
    )
        .into_response()
}

/// `POST /v1/memoire/{u}/approbations/{iv}` : approbation TAMR (F7).
fn est_approbation(methode: &axum::http::Method, chemin: &str) -> bool {
    let segments: Vec<&str> = chemin.split('/').collect();
    methode == axum::http::Method::POST
        && matches!(segments.as_slice(), ["", "v1", "memoire", u, "approbations", iv] if !u.is_empty() && !iv.is_empty())
}

/// Garde : débit, routes actives, pas de cookie, origine, requête signée par
/// une clé de l'utilisateur (ou d'approbation), usage lié à l'instance.
/// Toute réponse porte [`ENTETE_INSTANCE`].
async fn garde<E: Entrepot + 'static>(
    State(etat): State<Arc<EtatMemoireHttp<E>>>,
    req: Request,
    next: Next,
) -> Response {
    let mut reponse = garder(&etat, req, next).await;
    if let Ok(v) = axum::http::HeaderValue::from_str(&etat.instance) {
        reponse.headers_mut().insert(ENTETE_INSTANCE, v);
    }
    reponse
}

async fn garder<E: Entrepot + 'static>(
    etat: &EtatMemoireHttp<E>,
    req: Request,
    next: Next,
) -> Response {
    let Some(peer) = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip())
    else {
        return refus(StatusCode::FORBIDDEN, "no_connect_info");
    };
    let ip =
        crate::security::resolve_client_ip(peer, req.headers(), &etat.securite.trusted_proxies)
            .to_string();
    // L3 : limiteur de débit avant tout autre travail.
    if etat.limiteur.check(&format!("memoire:{ip}")).is_err() {
        tracing::warn!(target: "maia::memoire_http", ip = %ip, "débit dépassé sur les routes mémoire");
        return refus(StatusCode::TOO_MANY_REQUESTS, "debit_depasse");
    }
    let Some((utilisateur, _)) = etat.acces.as_ref().filter(|_| etat.garde.est_configuree()) else {
        return refus(StatusCode::SERVICE_UNAVAILABLE, "memoire_http_desactivee");
    };
    if req.headers().contains_key(axum::http::header::COOKIE) {
        return refus(StatusCode::BAD_REQUEST, "cookie_refuse");
    }
    if let crate::security::OriginCheck::Blocked(_) =
        crate::security::check_origin(Some(&ip), req.headers(), &etat.securite)
    {
        return refus(StatusCode::FORBIDDEN, "origin_blocked");
    }
    let approbation = est_approbation(req.method(), req.uri().path());
    let (garde, usage) = if approbation {
        if !etat.garde_approbation.est_configuree() {
            return refus(StatusCode::SERVICE_UNAVAILABLE, "approbations_desactivees");
        }
        (&etat.garde_approbation, etat.usage_approbation(utilisateur))
    } else {
        (&etat.garde, etat.usage_memoire(utilisateur))
    };
    let maintenant = maintenant_unix();
    let h = req.headers();
    let v = |n: &str| h.get(n).and_then(|x| x.to_str().ok());
    let entetes = match garde.controler_entetes(
        v(ENTETE_SIGNATAIRE),
        v(ENTETE_HORODATAGE),
        v(ENTETE_NONCE),
        v(ENTETE_SIGNATURE),
        maintenant,
    ) {
        Ok(e) => e,
        Err(r) => {
            tracing::warn!(target: "maia::memoire_http", refus = ?r, approbation, "requête mémoire refusée");
            return refus(
                StatusCode::UNAUTHORIZED,
                if approbation {
                    "signature_approbation_requise"
                } else {
                    "signature_utilisateur_requise"
                },
            );
        }
    };
    let (parts, corps) = req.into_parts();
    let Ok(corps) = axum::body::to_bytes(corps, MAX_CORPS).await else {
        return refus(StatusCode::PAYLOAD_TOO_LARGE, "corps_trop_volumineux");
    };
    let chemin = parts
        .uri
        .path_and_query()
        .map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    if let Err(r) = garde.verifier(
        &entetes,
        &usage,
        parts.method.as_str(),
        chemin,
        &corps,
        maintenant,
    ) {
        tracing::warn!(target: "maia::memoire_http", refus = ?r, approbation, "requête mémoire refusée");
        return refus(
            StatusCode::UNAUTHORIZED,
            if approbation {
                "signature_approbation_requise"
            } else {
                "signature_utilisateur_requise"
            },
        );
    }
    let mut parts = parts;
    parts
        .extensions
        .insert(UtilisateurAuthentifie(utilisateur.clone()));
    if approbation {
        parts
            .extensions
            .insert(SignatureApprobation::verifiee(&entetes.signataire));
    }
    next.run(Request::from_parts(parts, Body::from(corps)))
        .await
}

/// Refus léger (statut, motif), rendu en JSON.
struct Refus(StatusCode, &'static str);

impl IntoResponse for Refus {
    fn into_response(self) -> Response {
        refus(self.0, self.1)
    }
}

/// Mémoire de `utilisateur` si c'est l'utilisateur authentifié ; sinon 403.
fn memoire<'a, E: Entrepot>(
    etat: &'a EtatMemoireHttp<E>,
    auth: &UtilisateurAuthentifie,
    utilisateur: &str,
) -> Result<&'a Arc<MemoireMaia<E>>, Refus> {
    match &etat.acces {
        Some((proprietaire, m)) if proprietaire == utilisateur && auth.0 == utilisateur => Ok(m),
        _ => {
            tracing::warn!(target: "maia::memoire_http", demande = utilisateur, "accès à la mémoire d'un autre utilisateur refusé");
            Err(Refus(StatusCode::FORBIDDEN, "autre_utilisateur"))
        }
    }
}

fn erreur(e: &ErreurMemoire) -> Response {
    let statut = match e {
        ErreurMemoire::TamrRequis(_)
        | ErreurMemoire::RegleReserveeUtilisateur
        | ErreurMemoire::ProvenanceRefusee(_) => StatusCode::FORBIDDEN,
        ErreurMemoire::CandidatInconnu(_) | ErreurMemoire::NoteInconnue(_) => StatusCode::NOT_FOUND,
        ErreurMemoire::NoteOubliee(_) => StatusCode::GONE,
        ErreurMemoire::Conflit(_) => StatusCode::CONFLICT,
        ErreurMemoire::CandidatsPlafond(_) => StatusCode::TOO_MANY_REQUESTS,
        ErreurMemoire::Entrepot(_)
        | ErreurMemoire::NonChiffree
        | ErreurMemoire::EtiquetteInvalide(_) => StatusCode::SERVICE_UNAVAILABLE,
        _ => StatusCode::BAD_REQUEST,
    };
    (statut, Json(serde_json::json!({ "error": e.to_string() }))).into_response()
}

fn refus_tamr(r: &RefusTamr) -> Response {
    let statut = match r {
        RefusTamr::Inconnue => StatusCode::NOT_FOUND,
        RefusTamr::RegistrePlein
        | RefusTamr::Indisponible
        | RefusTamr::AutoriteIndisponible
        | RefusTamr::Serialisation => StatusCode::SERVICE_UNAVAILABLE,
        RefusTamr::SignatureRequise => StatusCode::UNAUTHORIZED,
        _ => StatusCode::FORBIDDEN,
    };
    (
        statut,
        Json(serde_json::json!({ "error": r.to_string(), "tamr": "refus" })),
    )
        .into_response()
}

fn issue(i: Issue) -> Response {
    match i {
        Issue::Ecrite(id) => Json(serde_json::json!({ "ecrite": id.as_str() })).into_response(),
        Issue::EnAttente(k) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "candidat": k.to_string() })),
        )
            .into_response(),
    }
}

macro_rules! essayer {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(r) => return r.into_response(),
        }
    };
}

type Etat<E> = State<Arc<EtatMemoireHttp<E>>>;
type Auth = Extension<UtilisateurAuthentifie>;

async fn consulter<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(u): Path<String>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let acces = m.utilisateur();
    match (acces.consulter().await, acces.candidats().await) {
        (Ok(notes), Ok(candidats)) => {
            Json(serde_json::json!({ "notes": notes, "candidats": candidats })).into_response()
        }
        (Err(e), _) | (_, Err(e)) => erreur(&e),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsId {
    id: String,
}

async fn historique<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(u): Path<String>,
    Json(c): Json<CorpsId>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let id = essayer!(IdMemoire::parse(&c.id).map_err(|e| erreur(&e)));
    match m.utilisateur().historique(&id).await {
        Ok(v) => Json(serde_json::json!({ "versions": v })).into_response(),
        Err(e) => erreur(&e),
    }
}

async fn exporter<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(u): Path<String>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    match m.utilisateur().exporter().await {
        Ok(x) => {
            tracing::info!(target: "maia_memoire::audit", "export demandé par l'utilisateur");
            Json(x).into_response()
        }
        Err(e) => erreur(&e),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsCorriger {
    id: String,
    texte: String,
    #[serde(default)]
    importance: Option<u8>,
}

async fn corriger<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(u): Path<String>,
    Json(c): Json<CorpsCorriger>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let id = essayer!(IdMemoire::parse(&c.id).map_err(|e| erreur(&e)));
    match m.utilisateur().corriger(&id, &c.texte, c.importance).await {
        Ok(i) => issue(i),
        Err(e) => erreur(&e),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsOubli {
    chemin: String,
    motif: String,
}

async fn oublier<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(u): Path<String>,
    Json(c): Json<CorpsOubli>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    match m.utilisateur().oublier(&c.chemin, &c.motif).await {
        Ok(t) => Json(serde_json::json!({ "pierres_tombales": t })).into_response(),
        Err(e) => erreur(&e),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsRegle {
    branche: String,
    texte: String,
    importance: u8,
}

async fn poser_regle<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path(utilisateur): Path<String>,
    Json(corps): Json<CorpsRegle>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &utilisateur));
    let branche = essayer!(Branche::parse(&corps.branche).map_err(|e| erreur(&e)));
    match m
        .utilisateur()
        .poser_regle(branche, &corps.texte, corps.importance)
        .await
    {
        Ok(issue_ecrite) => issue(issue_ecrite),
        Err(e) => erreur(&e),
    }
}

fn uuid(id: &str) -> Result<Uuid, Refus> {
    Uuid::parse_str(id).map_err(|_| Refus(StatusCode::BAD_REQUEST, "candidat_invalide"))
}

/// Valide un candidat ; branche sensible ⇒ demande d'approbation TAMR (202), rien n'est écrit.
async fn valider<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path((u, id)): Path<(String, String)>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let uid = essayer!(uuid(&id));
    let acces = m.utilisateur();
    let candidats = essayer!(acces.candidats().await.map_err(|e| erreur(&e)));
    let Some(k) = candidats.into_iter().find(|k| k.id == uid) else {
        return erreur(&ErreurMemoire::CandidatInconnu(uid));
    };
    if !k.sensible {
        return match acces.valider(uid, Validation::Utilisateur).await {
            Ok(id) => Json(serde_json::json!({ "ecrite": id.as_str() })).into_response(),
            Err(e) => erreur(&e),
        };
    }
    // F8 : version courante de la cible figée, affichée et liée à l'empreinte.
    let cible_actuelle = match &k.cible {
        Some(id) => {
            let notes = essayer!(acces.consulter().await.map_err(|e| erreur(&e)));
            match notes.into_iter().find(|n| &n.id == id) {
                Some(n) => Some(Box::new(n)),
                None => return erreur(&ErreurMemoire::NoteInconnue(id.to_string())),
            }
        }
        None => None,
    };
    match etat.approbations.demander(ActionTamr::ValiderCandidat {
        proprietaire: u,
        candidat: k,
        cible_actuelle,
    }) {
        Ok(d) => (
            StatusCode::ACCEPTED,
            Json(serde_json::json!({ "approbation": d })),
        )
            .into_response(),
        Err(r) => refus_tamr(&r),
    }
}

async fn rejeter<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path((u, id)): Path<(String, String)>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let uid = essayer!(uuid(&id));
    match m.utilisateur().rejeter(uid).await {
        Ok(()) => Json(serde_json::json!({ "rejete": uid.to_string() })).into_response(),
        Err(e) => erreur(&e),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsApprobation {
    empreinte: String,
}

/// Approbation TAMR : décision StrongFather ; seule une approbation accordée
/// pour le candidat exact produit `Validation::UtilisateurEtTamr`.
async fn approuver<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    signature: Option<Extension<SignatureApprobation>>,
    Path((u, intervention)): Path<(String, String)>,
    Json(c): Json<CorpsApprobation>,
) -> Response {
    let m = essayer!(memoire(&etat, &auth, &u));
    let Some(Extension(signature)) = signature else {
        return refus_tamr(&RefusTamr::SignatureRequise);
    };
    let ok = essayer!(etat
        .approbations
        .approuver(
            &intervention,
            &auth.0,
            &c.empreinte,
            &signature,
            &etat.gouvernance
        )
        .map_err(|r| refus_tamr(&r)));
    let ActionTamr::ValiderCandidat { candidat, .. } = ok.action() else {
        return refus_tamr(&RefusTamr::HorsPortee);
    };
    let uid = candidat.id;
    let validation = essayer!(ok.validation_memoire(uid).map_err(|r| refus_tamr(&r)));
    match m.utilisateur().valider(uid, validation).await {
        Ok(id) => {
            Json(serde_json::json!({ "ecrite": id.as_str(), "tamr": "approuve" })).into_response()
        }
        Err(e) => erreur(&e),
    }
}

async fn refuser_approbation<E: Entrepot + 'static>(
    State(etat): Etat<E>,
    Extension(auth): Auth,
    Path((u, intervention)): Path<(String, String)>,
) -> Response {
    let _ = essayer!(memoire(&etat, &auth, &u));
    match etat.approbations.refuser(&intervention, &auth.0) {
        Ok(()) => Json(serde_json::json!({ "tamr": "refuse" })).into_response(),
        Err(r) => refus_tamr(&r),
    }
}

#[cfg(test)]
#[allow(clippy::assert_is_empty, clippy::many_single_char_names)]
mod tests {
    use super::*;
    use axum::http::header;
    use maia_memoire::{BorneOctets, Ecrit, EntrepotMemoire, TypeUnite};
    use miyauth::http::service::entetes_signees;
    use miyusceau::CleSignature;
    use tower::ServiceExt;

    fn alice() -> CleSignature {
        CleSignature::depuis_graine(&[7_u8; 32])
    }

    /// Clé d'approbation TAMR d'Alice (distincte de sa clé mémoire, F7).
    fn alice_approbation() -> CleSignature {
        CleSignature::depuis_graine(&[8_u8; 32])
    }

    const INSTANCE: &str = "instance-essai";

    fn usage_memoire() -> String {
        format!("{USAGE_MEMOIRE}/{INSTANCE}/alice")
    }

    fn usage_approbation() -> String {
        format!("{USAGE_APPROBATION}/{INSTANCE}/alice")
    }

    async fn monter(gouvernance: GouvernanceMaia) -> (Router, Arc<MemoireMaia<EntrepotMemoire>>) {
        let m = Arc::new(
            MemoireMaia::ouvrir(EntrepotMemoire::new(), Arc::new(BorneOctets))
                .await
                .expect("m"),
        );
        let r = monter_sur(Arc::clone(&m), gouvernance);
        (r, m)
    }

    fn monter_sur(m: Arc<MemoireMaia<EntrepotMemoire>>, gouvernance: GouvernanceMaia) -> Router {
        routeur(Arc::new(
            EtatMemoireHttp::new(
                Some(("alice".into(), m)),
                vec![alice().cle_publique()],
                vec![alice_approbation().cle_publique()],
                Arc::new(gouvernance),
                SecurityConfig::default(),
            )
            .avec_instance(INSTANCE),
        ))
    }

    fn requete(
        methode: &str,
        chemin: &str,
        corps: &str,
        cle: Option<(&CleSignature, &str)>,
    ) -> Request {
        let mut b = axum::http::Request::builder()
            .method(methode)
            .uri(chemin)
            .header(header::CONTENT_TYPE, "application/json");
        if let Some((cle, usage)) = cle {
            for (n, v) in entetes_signees(cle, usage, methode, chemin, corps.as_bytes()) {
                b = b.header(n, v);
            }
        }
        let mut r = b.body(Body::from(corps.to_string())).expect("requête");
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40001))));
        r
    }

    fn signee(methode: &str, chemin: &str, corps: &str) -> Request {
        requete(
            methode,
            chemin,
            corps,
            Some((&alice(), usage_memoire().as_str())),
        )
    }

    /// Requête d'approbation signée par la clé d'approbation (F7).
    fn approuvee(chemin: &str, corps: &str) -> Request {
        requete(
            "POST",
            chemin,
            corps,
            Some((&alice_approbation(), usage_approbation().as_str())),
        )
    }

    async fn appel(app: &Router, r: Request) -> (StatusCode, serde_json::Value) {
        let rep = app.clone().oneshot(r).await.expect("réponse");
        let s = rep.status();
        let o = axum::body::to_bytes(rep.into_body(), 1 << 20)
            .await
            .expect("corps");
        (
            s,
            serde_json::from_slice(&o).unwrap_or(serde_json::Value::Null),
        )
    }

    fn sans_tamr() -> GouvernanceMaia {
        crate::tamr::tests::gouvernance(crate::tamr::tests::POLITIQUE_SANS_TAMR)
    }

    #[tokio::test]
    async fn fusion_avec_le_passthrough_sans_conflit() {
        let (memoire, _) = monter(sans_tamr()).await;
        let app = Router::new()
            .route("/v1/{*path}", axum::routing::any(|| async { "amont" }))
            .merge(memoire);
        let (s, _) = appel(
            &app,
            requete("POST", "/v1/memoire/alice/consulter", "", None),
        )
        .await;
        assert_eq!(
            s,
            StatusCode::UNAUTHORIZED,
            "route mémoire prioritaire sur le passthrough"
        );
        let (s, _) = appel(&app, signee("POST", "/v1/memoire/alice/consulter", "")).await;
        assert_eq!(s, StatusCode::OK);
    }

    #[tokio::test]
    async fn desactivees_par_defaut() {
        let m = Arc::new(
            MemoireMaia::ouvrir(EntrepotMemoire::new(), Arc::new(BorneOctets))
                .await
                .expect("m"),
        );
        for etat in [
            EtatMemoireHttp::new(
                Some(("alice".into(), Arc::clone(&m))),
                Vec::new(),
                Vec::new(),
                Arc::new(sans_tamr()),
                SecurityConfig::default(),
            ),
            EtatMemoireHttp::new(
                None,
                vec![alice().cle_publique()],
                Vec::new(),
                Arc::new(sans_tamr()),
                SecurityConfig::default(),
            ),
        ] {
            assert!(!etat.est_active());
            let app = routeur(Arc::new(etat));
            let (s, _) = appel(&app, signee("POST", "/v1/memoire/alice/consulter", "")).await;
            assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE);
        }
    }

    #[tokio::test]
    async fn sans_authentification_ou_autre_cle_refuse() {
        let (app, _) = monter(sans_tamr()).await;
        let (s, _) = appel(
            &app,
            requete("POST", "/v1/memoire/alice/consulter", "", None),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        // Clé de service (Central) avec l'usage de l'API : refusée ici.
        let central = CleSignature::depuis_graine(&[3_u8; 32]);
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/alice/consulter",
                "",
                Some((&central, crate::signature::USAGE_API)),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        // Clé de l'utilisateur, mauvais usage : refusée.
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/alice/consulter",
                "",
                Some((&alice(), crate::signature::USAGE_API)),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        // Rejeu.
        let r = signee("POST", "/v1/memoire/alice/consulter", "");
        let entetes = r.headers().clone();
        assert_eq!(appel(&app, r).await.0, StatusCode::OK);
        let mut rejeu = requete("POST", "/v1/memoire/alice/consulter", "", None);
        rejeu.headers_mut().extend(entetes);
        assert_eq!(appel(&app, rejeu).await.0, StatusCode::UNAUTHORIZED);
        // Cookie ambiant : refusé même signé.
        let mut r = signee("POST", "/v1/memoire/alice/consulter", "");
        r.headers_mut()
            .insert(header::COOKIE, "session=x".parse().expect("cookie"));
        assert_eq!(appel(&app, r).await.0, StatusCode::BAD_REQUEST);
        // Méthode GET : non routée.
        assert_eq!(
            appel(&app, signee("GET", "/v1/memoire/alice/consulter", ""))
                .await
                .0,
            StatusCode::METHOD_NOT_ALLOWED
        );
    }

    #[tokio::test]
    async fn autre_utilisateur_refuse() {
        let (app, m) = monter(sans_tamr()).await;
        m.utilisateur()
            .ecrire(Ecrit::new(
                Branche::parse("travail").expect("b"),
                TypeUnite::Fait,
                "Secret",
                5,
            ))
            .await
            .expect("e");
        // Alice authentifiée demande la base de Bob : 403.
        for (methode, chemin, corps) in [
            ("POST", "/v1/memoire/bob/consulter", ""),
            ("POST", "/v1/memoire/bob/exporter", ""),
            (
                "DELETE",
                "/v1/memoire/bob/notes",
                r#"{"chemin":"travail","motif":"x"}"#,
            ),
        ] {
            let (s, v) = appel(&app, signee(methode, chemin, corps)).await;
            assert_eq!(s, StatusCode::FORBIDDEN, "{chemin}");
            assert_eq!(v["reason"], "autre_utilisateur");
        }
        // La clé de Bob n'est pas épinglée : 401, même sur sa propre base.
        let bob = CleSignature::depuis_graine(&[9_u8; 32]);
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/bob/consulter",
                "",
                Some((&bob, usage_memoire().as_str())),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        assert_eq!(
            m.utilisateur().consulter().await.expect("c").len(),
            1,
            "rien d'oublié"
        );
    }

    #[tokio::test]
    async fn voir_corriger_exporter_regle_oublier() {
        let (app, m) = monter(sans_tamr()).await;
        m.utilisateur()
            .ecrire(Ecrit::new(
                Branche::parse("travail").expect("b"),
                TypeUnite::Fait,
                "Lot 7",
                5,
            ))
            .await
            .expect("e");
        let (s, v) = appel(&app, signee("POST", "/v1/memoire/alice/consulter", "")).await;
        assert_eq!(s, StatusCode::OK);
        assert_eq!(v["notes"][0]["id"], "travail.f1");
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/corriger",
                r#"{"id":"travail.f1","texte":"Lot 8"}"#,
            ),
        )
        .await;
        assert_eq!(
            (s, v["ecrite"].as_str()),
            (StatusCode::OK, Some("travail.f1"))
        );
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/historique",
                r#"{"id":"travail.f1"}"#,
            ),
        )
        .await;
        assert_eq!(
            (s, v["versions"].as_array().map(Vec::len)),
            (StatusCode::OK, Some(2))
        );
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/regles",
                r#"{"branche":"moi","texte":"Tutoie-moi","importance":9}"#,
            ),
        )
        .await;
        assert_eq!(s, StatusCode::OK, "{v}");
        let (s, v) = appel(&app, signee("POST", "/v1/memoire/alice/exporter", "")).await;
        assert_eq!(
            (s, v["format"].as_str()),
            (StatusCode::OK, Some("maia-memoire/v1"))
        );
        let (s, _) = appel(
            &app,
            signee(
                "DELETE",
                "/v1/memoire/alice/notes",
                r#"{"chemin":"travail","motif":"plus utile"}"#,
            ),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        assert_eq!(
            m.utilisateur().consulter().await.expect("c").len(),
            1,
            "seule la règle reste"
        );
        // Champ inconnu : refus.
        let (s, _) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/corriger",
                r#"{"id":"moi.r1","texte":"x","auteur":"maia"}"#,
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn candidat_ordinaire_valide_ou_rejete() {
        let (app, m) = monter(sans_tamr()).await;
        let p = |t: &str| maia_memoire::Proposition {
            cible: None,
            branche: Branche::parse("travail").expect("b"),
            type_unite: TypeUnite::Fait,
            texte: t.into(),
            importance: 4,
            confiance: 5,
            provenance: maia_memoire::Provenance::Etranger,
            liens: Vec::new(),
            sources: Vec::new(),
        };
        let k1 = m.proposer(p("Un")).await.expect("k1");
        let k2 = m.proposer(p("Deux")).await.expect("k2");
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k1}/valider"),
                "",
            ),
        )
        .await;
        assert_eq!(
            (s, v["ecrite"].as_str()),
            (StatusCode::OK, Some("travail.f1"))
        );
        let (s, _) = appel(
            &app,
            signee("DELETE", &format!("/v1/memoire/alice/candidats/{k2}"), ""),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        assert!(m.utilisateur().candidats().await.expect("k").is_empty());
    }

    async fn candidat_sensible(m: &MemoireMaia<EntrepotMemoire>) -> Uuid {
        let issue = m
            .utilisateur()
            .ecrire(Ecrit::new(
                Branche::parse("sante.sommeil").expect("b"),
                TypeUnite::Fait,
                "Dort 6 h",
                6,
            ))
            .await
            .expect("écrire");
        let Issue::EnAttente(k) = issue else {
            panic!("TAMR attendu")
        };
        k
    }

    #[tokio::test]
    async fn sensible_tamr_refuse_rien_ecrit() {
        let (app, m) = monter(sans_tamr()).await;
        let k = candidat_sensible(&m).await;
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        let d = &v["approbation"];
        assert_eq!(d["type_intervention"], "APPROVAL");
        let iv = d["intervention_id"].as_str().expect("iv");
        let corps = serde_json::json!({ "empreinte": d["empreinte"] }).to_string();
        // StrongFather sans règle TAMR : refus, rien n'est écrit.
        let (s, v) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::FORBIDDEN, "{v}");
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
        assert_eq!(
            m.utilisateur().candidats().await.expect("k").len(),
            1,
            "candidat conservé"
        );
        // Demande consommée : pas de seconde réponse.
        let (s, _) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::NOT_FOUND);
        // StrongFather indisponible : refus.
        let app2 = monter_sur(Arc::clone(&m), GouvernanceMaia::indisponible("test"));
        let (_, v) = appel(
            &app2,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        let d = &v["approbation"];
        let iv = d["intervention_id"].as_str().expect("iv");
        let corps = serde_json::json!({ "empreinte": d["empreinte"] }).to_string();
        let (s, _) = appel(
            &app2,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::FORBIDDEN);
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
    }

    #[tokio::test]
    async fn sensible_tamr_accorde_pour_le_candidat_exact() {
        let (app, m) = monter(crate::tamr::tests::gouvernance(
            crate::tamr::tests::POLITIQUE_TAMR,
        ))
        .await;
        let k = candidat_sensible(&m).await;
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        let d = &v["approbation"];
        let iv = d["intervention_id"].as_str().expect("iv").to_string();
        // Empreinte d'une autre action : refus, demande consommée, rien n'est écrit.
        let faux = serde_json::json!({ "empreinte": "00".repeat(32) }).to_string();
        let (s, _) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &faux),
        )
        .await;
        assert_eq!(s, StatusCode::FORBIDDEN);
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
        // Candidat rejeté entre la demande et l'approbation : approbation accordée mais rien n'est écrit.
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        let d = v["approbation"].clone();
        let iv = d["intervention_id"].as_str().expect("iv").to_string();
        m.utilisateur().rejeter(k).await.expect("rejet");
        let k2 = candidat_sensible(&m).await;
        let corps = serde_json::json!({ "empreinte": d["empreinte"] }).to_string();
        let (s, _) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::NOT_FOUND);
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
        // Approbation du candidat exact : écrit.
        let k = k2;
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        let d = v["approbation"].clone();
        let iv = d["intervention_id"].as_str().expect("iv").to_string();
        let corps = serde_json::json!({ "empreinte": d["empreinte"] }).to_string();
        let (s, v) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::OK, "{v}");
        assert_eq!(v["ecrite"], "sante.sommeil.f1");
        assert_eq!(m.utilisateur().consulter().await.expect("c").len(), 1);
    }

    // ── Revue de sécurité du lot 7 ──

    #[tokio::test]
    async fn l2_usage_lie_a_l_instance_et_a_l_utilisateur() {
        let (app, _) = monter(sans_tamr()).await;
        // Ancienne forme (usage seul) : rejouable après redémarrage ⇒ refusée.
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/alice/consulter",
                "",
                Some((&alice(), USAGE_MEMOIRE)),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        // Requête signée pour une autre instance (avant redémarrage) : refusée.
        let ancienne = format!("{USAGE_MEMOIRE}/instance-precedente/alice");
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/alice/consulter",
                "",
                Some((&alice(), ancienne.as_str())),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        // L'identifiant d'instance est annoncé dans toute réponse, même un refus.
        let rep = app
            .clone()
            .oneshot(requete("POST", "/v1/memoire/alice/consulter", "", None))
            .await
            .expect("r");
        assert_eq!(
            rep.headers()
                .get(ENTETE_INSTANCE)
                .and_then(|v| v.to_str().ok()),
            Some(INSTANCE)
        );
        assert_eq!(
            appel(&app, signee("POST", "/v1/memoire/alice/consulter", ""))
                .await
                .0,
            StatusCode::OK
        );
        // Instances distinctes à chaque démarrage.
        let m = Arc::new(
            MemoireMaia::ouvrir(EntrepotMemoire::new(), Arc::new(BorneOctets))
                .await
                .expect("m"),
        );
        let e1 = EtatMemoireHttp::new(
            Some(("alice".into(), Arc::clone(&m))),
            vec![alice().cle_publique()],
            Vec::new(),
            Arc::new(sans_tamr()),
            SecurityConfig::default(),
        );
        let e2 = EtatMemoireHttp::new(
            Some(("alice".into(), m)),
            vec![alice().cle_publique()],
            Vec::new(),
            Arc::new(sans_tamr()),
            SecurityConfig::default(),
        );
        assert_ne!(e1.instance(), e2.instance());
        assert_eq!(
            e1.usage_memoire("alice"),
            format!("maia/memoire/{}/alice", e1.instance())
        );
    }

    #[tokio::test]
    async fn l3_debit_limite_sur_les_routes_memoire() {
        let (app, _) = monter(sans_tamr()).await;
        for _ in 0..DEBIT_MAX_PAR_MINUTE {
            assert_eq!(
                appel(
                    &app,
                    requete("POST", "/v1/memoire/alice/consulter", "", None)
                )
                .await
                .0,
                StatusCode::UNAUTHORIZED
            );
        }
        let (s, v) = appel(&app, signee("POST", "/v1/memoire/alice/consulter", "")).await;
        assert_eq!(s, StatusCode::TOO_MANY_REQUESTS, "{v}");
    }

    async fn demande_sensible(app: &Router, m: &MemoireMaia<EntrepotMemoire>) -> (String, String) {
        let k = candidat_sensible(m).await;
        let (_, v) = appel(
            app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k}/valider"),
                "",
            ),
        )
        .await;
        let d = &v["approbation"];
        (
            d["intervention_id"].as_str().expect("iv").to_string(),
            serde_json::json!({ "empreinte": d["empreinte"] }).to_string(),
        )
    }

    #[tokio::test]
    async fn f7_approbation_exige_la_cle_d_approbation_distincte() {
        let (app, m) = monter(crate::tamr::tests::gouvernance(
            crate::tamr::tests::POLITIQUE_TAMR,
        ))
        .await;
        let (iv, corps) = demande_sensible(&app, &m).await;
        let chemin = format!("/v1/memoire/alice/approbations/{iv}");
        // Clé mémoire, avec l'usage mémoire ou l'usage d'approbation : 401, demande intacte.
        for usage in [usage_memoire(), usage_approbation()] {
            let (s, v) = appel(
                &app,
                requete("POST", &chemin, &corps, Some((&alice(), usage.as_str()))),
            )
            .await;
            assert_eq!(s, StatusCode::UNAUTHORIZED, "{v}");
        }
        // Clé d'approbation avec l'usage mémoire : 401.
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                &chemin,
                &corps,
                Some((&alice_approbation(), usage_memoire().as_str())),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
        assert!(m.utilisateur().consulter().await.expect("c").is_empty());
        // Clé d'approbation et usage d'approbation : accordée.
        let (s, v) = appel(&app, approuvee(&chemin, &corps)).await;
        assert_eq!(s, StatusCode::OK, "{v}");
        // La clé d'approbation n'ouvre pas les autres routes.
        let (s, _) = appel(
            &app,
            requete(
                "POST",
                "/v1/memoire/alice/consulter",
                "",
                Some((&alice_approbation(), usage_memoire().as_str())),
            ),
        )
        .await;
        assert_eq!(s, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn f7_sans_cle_d_approbation_ou_cle_commune_approbations_refusees() {
        let m = Arc::new(
            MemoireMaia::ouvrir(EntrepotMemoire::new(), Arc::new(BorneOctets))
                .await
                .expect("m"),
        );
        for approbation in [Vec::new(), vec![alice().cle_publique()]] {
            let app = routeur(Arc::new(
                EtatMemoireHttp::new(
                    Some(("alice".into(), Arc::clone(&m))),
                    vec![alice().cle_publique()],
                    approbation,
                    Arc::new(crate::tamr::tests::gouvernance(
                        crate::tamr::tests::POLITIQUE_TAMR,
                    )),
                    SecurityConfig::default(),
                )
                .avec_instance(INSTANCE),
            ));
            let (iv, corps) = demande_sensible(&app, &m).await;
            let chemin = format!("/v1/memoire/alice/approbations/{iv}");
            for cle in [alice(), alice_approbation()] {
                let (s, v) = appel(
                    &app,
                    requete(
                        "POST",
                        &chemin,
                        &corps,
                        Some((&cle, usage_approbation().as_str())),
                    ),
                )
                .await;
                assert_eq!(s, StatusCode::SERVICE_UNAVAILABLE, "{v}");
            }
            assert!(m.utilisateur().consulter().await.expect("c").is_empty());
            // Le refus d'une approbation reste possible avec la clé mémoire.
            assert_eq!(
                appel(&app, signee("DELETE", &chemin, "")).await.0,
                StatusCode::OK
            );
        }
    }

    #[tokio::test]
    async fn f8_cible_modifiee_avant_approbation_refusee() {
        let (app, m) = monter(crate::tamr::tests::gouvernance(
            crate::tamr::tests::POLITIQUE_TAMR,
        ))
        .await;
        // Note sensible v1 (validée sous TAMR), puis correction k2 demandée.
        let (iv, corps) = demande_sensible(&app, &m).await;
        let (s, _) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv}"), &corps),
        )
        .await;
        assert_eq!(s, StatusCode::OK);
        let (s, v) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/corriger",
                r#"{"id":"sante.sommeil.f1","texte":"Dort 7 h"}"#,
            ),
        )
        .await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        let k2 = v["candidat"].as_str().expect("k2").to_string();
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k2}/valider"),
                "",
            ),
        )
        .await;
        let d = v["approbation"].clone();
        let description = d["description"].as_str().expect("description");
        assert!(
            description.contains("texte actuel : « Dort 6 h »"),
            "{description}"
        );
        assert!(description.contains("version 1"), "{description}");
        let iv2 = d["intervention_id"].as_str().expect("iv").to_string();
        let corps2 = serde_json::json!({ "empreinte": d["empreinte"] }).to_string();
        // La cible change (k3 validé) avant l'approbation de k2.
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                "/v1/memoire/alice/corriger",
                r#"{"id":"sante.sommeil.f1","texte":"Dort 8 h"}"#,
            ),
        )
        .await;
        let k3 = v["candidat"].as_str().expect("k3").to_string();
        let (_, v) = appel(
            &app,
            signee(
                "POST",
                &format!("/v1/memoire/alice/candidats/{k3}/valider"),
                "",
            ),
        )
        .await;
        let d3 = v["approbation"].clone();
        let iv3 = d3["intervention_id"].as_str().expect("iv").to_string();
        let corps3 = serde_json::json!({ "empreinte": d3["empreinte"] }).to_string();
        assert_eq!(
            appel(
                &app,
                approuvee(&format!("/v1/memoire/alice/approbations/{iv3}"), &corps3)
            )
            .await
            .0,
            StatusCode::OK
        );
        let (s, v) = appel(
            &app,
            approuvee(&format!("/v1/memoire/alice/approbations/{iv2}"), &corps2),
        )
        .await;
        assert_eq!(s, StatusCode::FORBIDDEN, "{v}");
        let n = m.utilisateur().consulter().await.expect("c");
        assert_eq!(
            n[0].texte, "Dort 8 h",
            "la correction approuvée sur une cible périmée n'est pas écrite"
        );
    }
}
