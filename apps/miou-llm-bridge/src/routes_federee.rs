//! @id     maia_routes_federee
//! @do     publier_fiche_derivee_d_une_donnee_etiquetee
//! @role   security
//! @layer  app
//! @human  MAIA — route de publication des Fiches fédérées originales (EX-47, EX-57, A-016) : une
//! @human  Fiche proposée n'est publiée que si elle dérive strictement d'une donnée que
//! @human  l'utilisateur a étiquetée `publiable` dans KindMother ; le serveur calcule les champs
//! @human  structurels, StrongFather tranche, chaque publication et chaque refus sont tracés.

//! # Publication d'une Fiche originale (EX-47, A-015, A-016)
//!
//! | Méthode | Chemin | Effet |
//! |---|---|---|
//! | POST | `/v1/federee/{u}/publier` | propose une Fiche dérivée d'une donnée étiquetée ; publiée si tout passe |
//!
//! ## Principe : MAIA propose, le serveur décide
//!
//! MAIA (ou l'utilisateur, par l'interface) **propose** : la référence exacte
//! de la donnée KindMother (`donnee`), le `sujet`, le `texte`, les `relations`
//! et les `sources`. Aucun champ structurel n'est repris de la proposition :
//!
//! - `id` = `km.` + 32 hex de l'empreinte de la référence de donnée rendue par
//!   KindMother ([`maia_federee::id_fiche_derivee`]) ;
//! - `version` = dernière version publiée de cet `id` + 1
//!   ([`ServiceRequete::derniere_version`]) ;
//! - `date` = horloge du serveur ;
//! - `type`, `langue`, `licence`, `audience` = paramètres **signés par
//!   l'utilisateur** avec la pose de l'étiquette (intention v2) ;
//! - émetteur = clé d'identité du COG, signature par le serveur.
//!
//! Si la proposition **annonce** l'un de ces champs, il est comparé à la
//! valeur calculée : écart ⇒ refus `champ_structurel_falsifie` (date : hors de
//! ±[`FENETRE_DATE_SECS`] s ; version déjà publiée : `version_deja_publiee`).
//! La Fiche signée passe ensuite par [`PublieurOriginal::attester`] (étiquette
//! **relue** auprès de KindMother, règle de dérivation stricte, ≤ 8
//! attestations par donnée et par 24 h), puis par la porte StrongFather
//! (refus par défaut), qui seule exécute [`ServiceRequete::publier`].
//!
//! ## Refus sans appel à KindMother
//!
//! - base `maia-*` (mémoire de MAIA, comparée sans casse) ⇒ `donnee_nationale` ;
//! - `provenance` autre que `locale`, ou champ `reprises` présent ⇒
//!   `republication_tamr_requise` : une donnée étrangère ne se republie que par
//!   le chemin TAMR (A-009 ; [`crate::tamr::ActionTamr::Republier`]), jamais
//!   par cette route.
//!
//! ## Authentification et débit
//!
//! Comme les routes mémoire ([`crate::routes_memoire`]) : requête signée Ed25519
//! par une clé de l'utilisateur épinglée dans `MAIA_MEMOIRE_CLES_UTILISATEUR`,
//! usage **distinct** `maia/federee/<instance>/<utilisateur>` (une requête
//! mémoire ne se rejoue pas ici) ; POST seulement, pas de cookie, pas de CORS,
//! origine contrôlée ; `{u}` = utilisateur de l'instance, sinon 403. Débit :
//! [`DEBIT_IP_PAR_MINUTE`] par IP avant tout contrôle, puis
//! [`PUBLICATIONS_PAR_HEURE`] propositions par utilisateur et par heure
//! (acceptées ou refusées) après authentification.
//!
//! ## Activation (désactivée par défaut, 503)
//!
//! - `MAIA_MEMOIRE_UTILISATEUR` et `MAIA_MEMOIRE_CLES_UTILISATEUR` (comme la mémoire) ;
//! - `MAIA_FEDEREE_CLE_IDENTITE` : graine de la clé d'identité du COG (32 octets,
//!   fichier régulier `0600`, jamais créé ici) ;
//! - `MAIA_FEDEREE_OPERATEUR` : opérateur KindMother qui lit les étiquettes
//!   (justificatif à l'emplacement standard, droit de lecture sur les bases
//!   publiables) ; `KINDMOTHER_ADDR` (défaut `127.0.0.1:50051`).
//!
//! ## Limites (à consigner par amendement)
//!
//! - les Fiches publiées et le plafond d'attestations vivent **en mémoire** :
//!   un redémarrage les oublie (versions recomptées depuis 1) ;
//! - la clé d'identité du COG est lue par le pont (détenteur à confirmer) ;
//! - aucun transport vers les autres COG (lot 7 : reste à faire).

use std::collections::VecDeque;
use std::future::Future;
use std::net::SocketAddr;
use std::path::Path;
use std::sync::{Arc, Mutex, OnceLock};

use axum::body::{Body, Bytes};
use axum::extract::{ConnectInfo, Path as Chemin, Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Extension, Json, Router};
use base64::Engine as _;
use bondingbrother::strongfather::{ActionType, CallContext, IntentData};
use bondingbrother::Intent;
use kindmother::etiquette::{
    AudiencePublication, EtatEtiquette, ParametresPublication, RefDonnee, Sensibilite,
    PREFIXE_NATIONALE,
};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use maia_federee::{
    cog_de_cle, id_fiche_derivee, sceller_fiche, verifier_fiche, Audience, AutoriteEtiquettes,
    ErreurFederee, EtiquetteLue, Fiche, LecteurEtiquettes, ParametresLus, Publication,
    PublieurOriginal, Relation, ServiceRequete, TypeFiche, FENETRE_DATE_SECS,
};
use miyauth::http::service::{maintenant_unix, GardeSignee};
use miyauth::http::{ENTETE_HORODATAGE, ENTETE_NONCE, ENTETE_SIGNATAIRE, ENTETE_SIGNATURE};
use miyauth::CogId;
use miyusceau::CleSignature;
use serde::Deserialize;

use crate::config::SecurityConfig;
use crate::gouvernance::{GouvernanceMaia, ORIGINE_MAIA};
use crate::routes_memoire::{UtilisateurAuthentifie, ENTETE_INSTANCE, ENV_CLES_UTILISATEUR};
use crate::security::{RateLimitConfig, RateLimiter};

/// Usage signé (préfixe) des requêtes de publication ; usage complet :
/// `maia/federee/<instance>/<utilisateur>`.
pub const USAGE_FEDEREE: &str = "maia/federee";
/// Graine de la clé d'identité du COG (chemin d'un fichier de 32 octets, `0600`).
pub const ENV_CLE_IDENTITE: &str = "MAIA_FEDEREE_CLE_IDENTITE";
/// Opérateur KindMother qui lit les étiquettes `publiable`.
pub const ENV_OPERATEUR: &str = "MAIA_FEDEREE_OPERATEUR";
/// Utilisateur de l'instance (partagé avec la mémoire).
pub const ENV_UTILISATEUR: &str = "MAIA_MEMOIRE_UTILISATEUR";
const ENV_KINDMOTHER: &str = "KINDMOTHER_ADDR";
const KINDMOTHER_DEFAUT: &str = "127.0.0.1:50051";
/// Corps maximal d'une proposition (octets) : texte ≤ 8 000 caractères et métadonnées.
pub const MAX_CORPS: usize = 64 * 1024;
/// Requêtes au plus par minute et par IP (avant tout contrôle).
pub const DEBIT_IP_PAR_MINUTE: u32 = 60;
/// Propositions au plus par utilisateur et par heure (après authentification).
pub const PUBLICATIONS_PAR_HEURE: u32 = 30;
/// Entrées gardées dans le journal en mémoire (les plus anciennes sont écartées).
pub const CAPACITE_JOURNAL: usize = 1_024;
/// Valeur de `data.operation` de l'intention StrongFather.
pub const OPERATION_PUBLIER: &str = "federee.publier";

// ═══════════════════════════════════════════════════════════════════════════
// Autorité des étiquettes (jeton unique du processus)
// ═══════════════════════════════════════════════════════════════════════════

static PUBLIEUR: OnceLock<Option<PublieurOriginal>> = OnceLock::new();

fn publieur() -> Option<&'static PublieurOriginal> {
    PUBLIEUR
        .get_or_init(|| {
            let publieur = AutoriteEtiquettes::instituer().map(PublieurOriginal::new);
            if publieur.is_none() {
                tracing::error!(target: "maia::federee", "autorité des étiquettes prise ailleurs : aucune publication originale");
            }
            publieur
        })
        .as_ref()
}

/// Institue l'autorité des étiquettes au démarrage ; `false` si elle a été
/// prise ailleurs (toute publication sera alors refusée).
pub fn instituer_autorite() -> bool {
    publieur().is_some()
}

// ═══════════════════════════════════════════════════════════════════════════
// Lecture des étiquettes auprès de KindMother
// ═══════════════════════════════════════════════════════════════════════════

/// Source de l'état d'étiquette d'une donnée (requête typée `LireEtiquette`).
pub trait SourceEtiquettes: Send + Sync + 'static {
    /// État **courant** de l'étiquette de `donnee` ; erreur : motif lisible.
    fn lire(
        &self,
        donnee: &RefDonnee,
    ) -> impl Future<Output = Result<EtatEtiquette, String>> + Send;
}

/// Lecteur d'étiquettes adossé à un opérateur KindMother authentifié
/// (connexion ouverte au premier usage, rouverte une fois après une erreur de
/// transport).
pub struct EtiquettesKindMother {
    adresse: String,
    operateur: String,
    justificatif: OperatorCredential,
    client: tokio::sync::Mutex<Option<Arc<KindMotherClient>>>,
}

impl std::fmt::Debug for EtiquettesKindMother {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EtiquettesKindMother")
            .field("adresse", &self.adresse)
            .field("operateur", &self.operateur)
            .finish_non_exhaustive()
    }
}

impl EtiquettesKindMother {
    /// Lecteur de l'opérateur `operateur` (justificatif déjà chargé).
    pub fn new(adresse: &str, operateur: &str, justificatif: OperatorCredential) -> Self {
        Self {
            adresse: adresse.to_string(),
            operateur: operateur.to_string(),
            justificatif,
            client: tokio::sync::Mutex::new(None),
        }
    }

    /// Justificatif `operateur` lu dans `credentials`, sinon à l'emplacement standard.
    pub fn charger(
        adresse: &str,
        operateur: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, String> {
        let justificatif = match credentials {
            Some(d) => OperatorCredential::load_from_dir(operateur, d),
            None => OperatorCredential::load(operateur),
        }
        .map_err(|e| e.to_string())?;
        Ok(Self::new(adresse, operateur, justificatif))
    }

    async fn client(&self) -> Result<Arc<KindMotherClient>, String> {
        let mut c = self.client.lock().await;
        if let Some(x) = c.as_ref() {
            return Ok(Arc::clone(x));
        }
        let x = Arc::new(
            KindMotherClient::connect(&self.adresse, &self.justificatif, &self.operateur)
                .await
                .map_err(|e| e.to_string())?,
        );
        *c = Some(Arc::clone(&x));
        Ok(x)
    }

    async fn abandonner(&self, perdu: &Arc<KindMotherClient>) {
        let mut c = self.client.lock().await;
        if c.as_ref().is_some_and(|x| Arc::ptr_eq(x, perdu)) {
            *c = None;
        }
    }
}

impl SourceEtiquettes for EtiquettesKindMother {
    async fn lire(&self, donnee: &RefDonnee) -> Result<EtatEtiquette, String> {
        let mut derniere = String::from("KindMother injoignable");
        for tentative in 0..2 {
            let c = self.client().await?;
            match c.lire_etiquette(donnee.clone()).await {
                Ok(e) => return Ok(e),
                Err(
                    e @ (ClientError::Transport(_)
                    | ClientError::Timeout(_)
                    | ClientError::Connection(_)
                    | ClientError::NotConnected),
                ) if tentative == 0 => {
                    tracing::warn!(target: "maia::federee", erreur = %e, "connexion KindMother perdue : rouverte");
                    self.abandonner(&c).await;
                    derniere = e.to_string();
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(derniere)
    }
}

/// Paramètres signés par l'utilisateur, au format de `maia-federee` ; une
/// valeur hors format (COG nommé invalide) ⇒ absents (jamais publiable).
fn parametres_lus(p: &ParametresPublication) -> Option<ParametresLus> {
    let type_fiche = TypeFiche::try_from(p.type_fiche.octet()).ok()?;
    let audience = match &p.audience {
        AudiencePublication::Membres => Audience::Membres,
        AudiencePublication::Invites => Audience::Invites,
        AudiencePublication::Nommes(cogs) => Audience::Nommes(
            cogs.iter()
                .map(|c| CogId::new(c))
                .collect::<Option<Vec<_>>>()?,
        ),
    };
    Some(ParametresLus {
        type_fiche,
        langue: p.langue.clone(),
        licence: p.licence.clone(),
        audience,
    })
}

/// État KindMother ⇒ [`EtiquetteLue`] (le drapeau `publiable` de KindMother
/// n'est pas repris : la décision est recalculée par `maia-federee`).
fn etiquette_lue(e: &EtatEtiquette) -> EtiquetteLue {
    EtiquetteLue::depuis_hex(
        e.sensibilite == Sensibilite::Nationale,
        e.posee,
        e.empreinte_contenu.as_deref(),
        e.empreinte_actuelle.as_deref(),
    )
    .avec_donnee(e.empreinte_donnee.as_deref())
    .avec_parametres(e.parametres.as_ref().and_then(parametres_lus))
}

/// Adaptateur [`LecteurEtiquettes`] : relit l'étiquette à chaque attestation ;
/// une lecture impossible est un refus.
struct Lecteur<'a, S>(&'a S);

impl<S: SourceEtiquettes> LecteurEtiquettes for Lecteur<'_, S> {
    type Donnee = RefDonnee;

    fn lire(
        &self,
        donnee: &RefDonnee,
    ) -> impl Future<Output = Result<EtiquetteLue, ErreurFederee>> + Send {
        let source = self.0;
        async move {
            match source.lire(donnee).await {
                Ok(e) => Ok(etiquette_lue(&e)),
                Err(motif) => {
                    tracing::warn!(target: "maia::federee", %motif, "étiquette illisible à l'attestation : refus");
                    Err(ErreurFederee::NonPubliable)
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Journal des décisions
// ═══════════════════════════════════════════════════════════════════════════

/// Décision tracée (publication ou refus).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct EntreeJournal {
    /// Instant (secondes Unix).
    pub instant: u64,
    /// Utilisateur authentifié.
    pub utilisateur: String,
    /// `publiee` ou `refusee`.
    pub verdict: &'static str,
    /// Code du motif (`publiee` pour une publication).
    pub motif: &'static str,
    /// Précision (règle StrongFather, champ falsifié…).
    pub detail: String,
    /// Identifiant de Fiche, s'il a pu être calculé.
    pub fiche: Option<String>,
    /// Version, si calculée.
    pub version: Option<u64>,
    /// Empreinte SHA-256 des octets signés (hex), si scellée.
    pub empreinte: Option<String>,
}

// ═══════════════════════════════════════════════════════════════════════════
// État
// ═══════════════════════════════════════════════════════════════════════════

struct Actif<S> {
    utilisateur: String,
    source: S,
    identite: CleSignature,
    service: Mutex<ServiceRequete>,
}

/// État de la route de publication.
pub struct EtatPublicationHttp<S: SourceEtiquettes> {
    actif: Option<Actif<S>>,
    garde: GardeSignee,
    instance: String,
    limiteur_ip: RateLimiter,
    limiteur_utilisateur: RateLimiter,
    gouvernance: Arc<GouvernanceMaia>,
    securite: SecurityConfig,
    journal: Mutex<VecDeque<EntreeJournal>>,
}

impl<S: SourceEtiquettes> EtatPublicationHttp<S> {
    /// Route active pour `actif` = (utilisateur, lecteur d'étiquettes, clé
    /// d'identité du COG) et les clés de l'utilisateur ; sans l'un d'eux :
    /// désactivée. `instance` : identifiant lié dans l'usage signé (celui des
    /// routes mémoire).
    pub fn new(
        actif: Option<(String, S, CleSignature)>,
        cles_utilisateur: Vec<[u8; 32]>,
        instance: String,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let actif = actif.and_then(|(utilisateur, source, identite)| {
            let Some(cog) = cog_de_cle(&identite.cle_publique()) else {
                tracing::error!(target: "maia::federee", "clé d'identité sans cog_id valide : publication désactivée");
                return None;
            };
            Some(Actif {
                utilisateur,
                source,
                identite,
                service: Mutex::new(ServiceRequete::new(cog)),
            })
        });
        Self {
            actif,
            garde: GardeSignee::new(cles_utilisateur),
            instance,
            limiteur_ip: RateLimiter::new(RateLimitConfig {
                max_requests: DEBIT_IP_PAR_MINUTE,
                window: std::time::Duration::from_secs(60),
            }),
            limiteur_utilisateur: RateLimiter::new(RateLimitConfig {
                max_requests: PUBLICATIONS_PAR_HEURE,
                window: std::time::Duration::from_secs(3_600),
            }),
            gouvernance,
            securite,
            journal: Mutex::new(VecDeque::new()),
        }
    }

    /// Vrai si la route est servie.
    pub fn est_active(&self) -> bool {
        self.actif.is_some() && self.garde.est_configuree()
    }

    /// Usage signé attendu des requêtes de publication de `utilisateur`.
    pub fn usage(&self, utilisateur: &str) -> String {
        format!("{USAGE_FEDEREE}/{}/{utilisateur}", self.instance)
    }

    /// Dernière version publiée de `id` (0 : jamais).
    #[cfg(test)]
    fn derniere_version(&self, id: &str) -> u64 {
        self.actif
            .as_ref()
            .and_then(|a| a.service.lock().ok().map(|s| s.derniere_version(id)))
            .unwrap_or(0)
    }

    /// Copie du journal des décisions (plus récentes en dernier).
    pub fn journal(&self) -> Vec<EntreeJournal> {
        self.journal
            .lock()
            .map(|j| j.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn tracer(&self, e: EntreeJournal) {
        if e.verdict == "publiee" {
            tracing::info!(target: "maia::federee::audit", utilisateur = %e.utilisateur, fiche = ?e.fiche, version = ?e.version, empreinte = ?e.empreinte, detail = %e.detail, "Fiche originale publiée");
        } else {
            tracing::warn!(target: "maia::federee::audit", utilisateur = %e.utilisateur, motif = e.motif, detail = %e.detail, fiche = ?e.fiche, version = ?e.version, "publication refusée");
        }
        match self.journal.lock() {
            Ok(mut j) => {
                if j.len() >= CAPACITE_JOURNAL {
                    j.pop_front();
                }
                j.push_back(e);
            }
            Err(_) => {
                tracing::error!(target: "maia::federee::audit", "journal des publications indisponible (verrou)");
            }
        }
    }
}

impl EtatPublicationHttp<EtiquettesKindMother> {
    /// Configuration par l'environnement (voir l'en-tête du module) ; tout
    /// élément absent ou invalide ⇒ route désactivée (journalisé).
    pub fn depuis_env(
        instance: String,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let cles = crate::routes_memoire::analyser_env(ENV_CLES_UTILISATEUR);
        let actif = actif_depuis_env();
        let etat = Self::new(actif, cles, instance, gouvernance, securite);
        if etat.est_active() {
            tracing::info!("route de publication fédérée active (requêtes signées « {USAGE_FEDEREE}/<instance>/<utilisateur> »)");
        } else {
            tracing::info!("route de publication fédérée désactivée ({ENV_UTILISATEUR}, {ENV_CLES_UTILISATEUR}, {ENV_CLE_IDENTITE} ou {ENV_OPERATEUR} absent)");
        }
        etat
    }
}

fn env_non_vide(nom: &str) -> Option<String> {
    std::env::var(nom)
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

fn actif_depuis_env() -> Option<(String, EtiquettesKindMother, CleSignature)> {
    let utilisateur = env_non_vide(ENV_UTILISATEUR)?;
    let chemin = env_non_vide(ENV_CLE_IDENTITE)?;
    let operateur = env_non_vide(ENV_OPERATEUR)?;
    let identite = match miyauth::http::service::charger_cle(Path::new(&chemin)) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(target: "maia::federee", "{ENV_CLE_IDENTITE} ({chemin}) : {e} — publication désactivée");
            return None;
        }
    };
    let adresse = env_non_vide(ENV_KINDMOTHER).unwrap_or_else(|| KINDMOTHER_DEFAUT.to_string());
    match EtiquettesKindMother::charger(&adresse, &operateur, None) {
        Ok(source) => Some((utilisateur, source, identite)),
        Err(e) => {
            tracing::error!(target: "maia::federee", "justificatif KindMother « {operateur} » : {e} — publication désactivée");
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Routeur et garde
// ═══════════════════════════════════════════════════════════════════════════

/// Routeur de la publication (à fusionner avec le routeur principal).
pub fn routeur<S: SourceEtiquettes>(etat: Arc<EtatPublicationHttp<S>>) -> Router {
    Router::new()
        .route("/v1/federee/{utilisateur}/publier", post(publier::<S>))
        .layer(axum::middleware::from_fn_with_state(
            Arc::clone(&etat),
            garde::<S>,
        ))
        .with_state(etat)
}

fn refus_http(statut: StatusCode, motif: &str) -> Response {
    (
        statut,
        Json(serde_json::json!({ "error": "publication refusée", "reason": motif })),
    )
        .into_response()
}

async fn garde<S: SourceEtiquettes>(
    State(etat): State<Arc<EtatPublicationHttp<S>>>,
    req: Request,
    next: Next,
) -> Response {
    let mut reponse = garder(&etat, req, next).await;
    if let Ok(v) = axum::http::HeaderValue::from_str(&etat.instance) {
        reponse.headers_mut().insert(ENTETE_INSTANCE, v);
    }
    reponse
}

/// Débit par IP, route active, pas de cookie, origine, requête signée par une
/// clé de l'utilisateur avec l'usage de publication lié à l'instance.
async fn garder<S: SourceEtiquettes>(
    etat: &EtatPublicationHttp<S>,
    req: Request,
    next: Next,
) -> Response {
    let Some(peer) = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip())
    else {
        return refus_http(StatusCode::FORBIDDEN, "no_connect_info");
    };
    let ip =
        crate::security::resolve_client_ip(peer, req.headers(), &etat.securite.trusted_proxies)
            .to_string();
    if etat.limiteur_ip.check(&format!("federee:{ip}")).is_err() {
        tracing::warn!(target: "maia::federee", ip = %ip, "débit dépassé sur la route de publication");
        return refus_http(StatusCode::TOO_MANY_REQUESTS, "debit_depasse");
    }
    let Some(actif) = etat.actif.as_ref().filter(|_| etat.garde.est_configuree()) else {
        return refus_http(StatusCode::SERVICE_UNAVAILABLE, "publication_desactivee");
    };
    if req.headers().contains_key(axum::http::header::COOKIE) {
        return refus_http(StatusCode::BAD_REQUEST, "cookie_refuse");
    }
    if let crate::security::OriginCheck::Blocked(_) =
        crate::security::check_origin(Some(&ip), req.headers(), &etat.securite)
    {
        return refus_http(StatusCode::FORBIDDEN, "origin_blocked");
    }
    let usage = etat.usage(&actif.utilisateur);
    let maintenant = maintenant_unix();
    let h = req.headers();
    let v = |n: &str| h.get(n).and_then(|x| x.to_str().ok());
    let entetes = match etat.garde.controler_entetes(
        v(ENTETE_SIGNATAIRE),
        v(ENTETE_HORODATAGE),
        v(ENTETE_NONCE),
        v(ENTETE_SIGNATURE),
        maintenant,
    ) {
        Ok(e) => e,
        Err(r) => {
            tracing::warn!(target: "maia::federee", refus = ?r, "requête de publication refusée");
            return refus_http(StatusCode::UNAUTHORIZED, "signature_utilisateur_requise");
        }
    };
    let (parts, corps) = req.into_parts();
    let Ok(corps) = axum::body::to_bytes(corps, MAX_CORPS).await else {
        return refus_http(StatusCode::PAYLOAD_TOO_LARGE, "corps_trop_volumineux");
    };
    let chemin = parts
        .uri
        .path_and_query()
        .map_or_else(|| parts.uri.path(), |pq| pq.as_str());
    if let Err(r) = etat.garde.verifier(
        &entetes,
        &usage,
        parts.method.as_str(),
        chemin,
        &corps,
        maintenant,
    ) {
        tracing::warn!(target: "maia::federee", refus = ?r, "requête de publication refusée");
        return refus_http(StatusCode::UNAUTHORIZED, "signature_utilisateur_requise");
    }
    let mut parts = parts;
    parts
        .extensions
        .insert(UtilisateurAuthentifie(actif.utilisateur.clone()));
    next.run(Request::from_parts(parts, Body::from(corps)))
        .await
}

// ═══════════════════════════════════════════════════════════════════════════
// Proposition
// ═══════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum TypePropose {
    Fait,
    Definition,
    Procedure,
    Document,
    Tableau,
}

impl TypePropose {
    fn fiche(self) -> TypeFiche {
        match self {
            Self::Fait => TypeFiche::Fait,
            Self::Definition => TypeFiche::Definition,
            Self::Procedure => TypeFiche::Procedure,
            Self::Document => TypeFiche::Document,
            Self::Tableau => TypeFiche::Tableau,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum AudienceProposee {
    Membres,
    Invites,
    Nommes(Vec<String>),
}

impl AudienceProposee {
    fn egale(&self, a: &Audience) -> bool {
        match (self, a) {
            (Self::Membres, Audience::Membres) | (Self::Invites, Audience::Invites) => true,
            (Self::Nommes(p), Audience::Nommes(c)) => {
                p.len() == c.len() && p.iter().zip(c).all(|(x, y)| x == y.as_str())
            }
            _ => false,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RelationProposee {
    sujet: String,
    predicat: String,
    objet: String,
}

/// Proposition de MAIA ou de l'interface. Les champs structurels sont
/// facultatifs et seulement **comparés** aux valeurs calculées.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Proposition {
    donnee: RefDonnee,
    sujet: String,
    texte: String,
    #[serde(default)]
    relations: Vec<RelationProposee>,
    #[serde(default)]
    sources: Vec<String>,
    /// `locale` (défaut) ; toute autre valeur ⇒ chemin TAMR.
    #[serde(default)]
    provenance: Option<String>,
    /// Présent (même vide) ⇒ republication : chemin TAMR.
    #[serde(default)]
    reprises: Option<serde_json::Value>,
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    version: Option<u64>,
    #[serde(default)]
    date: Option<u64>,
    #[serde(default, rename = "type")]
    type_fiche: Option<TypePropose>,
    #[serde(default)]
    langue: Option<String>,
    #[serde(default)]
    licence: Option<String>,
    #[serde(default)]
    audience: Option<AudienceProposee>,
}

/// Refus motivé (code stable dans `reason`).
#[derive(Debug)]
struct Refus {
    statut: StatusCode,
    code: &'static str,
    detail: String,
    fiche: Option<String>,
    version: Option<u64>,
    empreinte: Option<String>,
}

impl Refus {
    fn new(statut: StatusCode, code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            statut,
            code,
            detail: detail.into(),
            fiche: None,
            version: None,
            empreinte: None,
        }
    }

    fn pour(mut self, fiche: &str, version: u64) -> Self {
        self.fiche = Some(fiche.to_string());
        self.version = Some(version);
        self
    }
}

/// Publication réussie.
struct Publiee {
    id: String,
    version: u64,
    date: u64,
    empreinte: String,
    octets: Vec<u8>,
    regle: String,
}

fn refus_federee(e: &ErreurFederee) -> Refus {
    let (statut, code) = match e {
        ErreurFederee::NonPubliable => (StatusCode::FORBIDDEN, "non_publiable"),
        ErreurFederee::Interdit(_) => (StatusCode::FORBIDDEN, "donnee_nationale"),
        ErreurFederee::ValidationRequise => (StatusCode::FORBIDDEN, "republication_tamr_requise"),
        ErreurFederee::NonDerivee(_) => (StatusCode::UNPROCESSABLE_ENTITY, "non_derivee"),
        ErreurFederee::Quota => (StatusCode::TOO_MANY_REQUESTS, "plafond_attestations"),
        ErreurFederee::VersionObsolete => (StatusCode::CONFLICT, "version_deja_publiee"),
        ErreurFederee::FicheNonLocale => (StatusCode::FORBIDDEN, "fiche_non_locale"),
        _ => (StatusCode::UNPROCESSABLE_ENTITY, "fiche_invalide"),
    };
    Refus::new(statut, code, e.to_string())
}

fn falsifie(champ: &str) -> Refus {
    Refus::new(
        StatusCode::UNPROCESSABLE_ENTITY,
        "champ_structurel_falsifie",
        format!("{champ} différent de la valeur calculée par le serveur"),
    )
}

fn nom_audience(a: &Audience) -> &'static str {
    match a {
        Audience::Membres => "membres",
        Audience::Invites => "invites",
        Audience::Nommes(_) => "nommes",
    }
}

/// Champs structurels annoncés par la proposition : comparés, jamais repris.
fn controler_annonces(
    p: &Proposition,
    id: &str,
    derniere: u64,
    version: u64,
    parametres: &ParametresLus,
    maintenant: u64,
) -> Result<(), Refus> {
    if p.id.as_deref().is_some_and(|x| x != id) {
        return Err(falsifie("id"));
    }
    match p.version {
        Some(v) if v <= derniere => {
            return Err(Refus::new(
                StatusCode::CONFLICT,
                "version_deja_publiee",
                format!("version {v} déjà publiée (dernière : {derniere})"),
            ))
        }
        Some(v) if v != version => return Err(falsifie("version")),
        _ => {}
    }
    if p.date
        .is_some_and(|d| d.abs_diff(maintenant) > FENETRE_DATE_SECS)
    {
        return Err(falsifie("date (hors de la fenêtre)"));
    }
    if p.type_fiche
        .is_some_and(|t| t.fiche() != parametres.type_fiche)
    {
        return Err(falsifie("type"));
    }
    if p.langue.as_deref().is_some_and(|l| l != parametres.langue) {
        return Err(falsifie("langue"));
    }
    if p.licence
        .as_deref()
        .is_some_and(|l| l != parametres.licence)
    {
        return Err(falsifie("licence"));
    }
    if p.audience
        .as_ref()
        .is_some_and(|a| !a.egale(&parametres.audience))
    {
        return Err(falsifie("audience"));
    }
    Ok(())
}

fn est_nationale_par_nom(base: &str) -> bool {
    base.to_ascii_lowercase().starts_with(PREFIXE_NATIONALE)
}

/// Intention StrongFather d'une publication originale.
fn intention_publication(
    utilisateur: &str,
    fiche: &Fiche,
    empreinte: &str,
    donnee: &str,
    base: &str,
) -> Intent {
    let mut data = std::collections::HashMap::new();
    data.insert("operation".into(), OPERATION_PUBLIER.into());
    data.insert("utilisateur".into(), utilisateur.into());
    data.insert("canal".into(), crate::tamr::CANAL_UTILISATEUR.into());
    data.insert("usage_signe".into(), USAGE_FEDEREE.into());
    data.insert("provenance".into(), "locale".into());
    data.insert("sensibilite".into(), "ordinaire".into());
    data.insert("attestee".into(), "true".into());
    data.insert("reprises".into(), fiche.reprises.len().to_string());
    data.insert("fiche".into(), fiche.id.clone());
    data.insert("version".into(), fiche.version.to_string());
    data.insert("empreinte".into(), empreinte.into());
    data.insert("donnee".into(), donnee.into());
    data.insert("base".into(), base.into());
    data.insert("type".into(), (fiche.type_fiche as u8).to_string());
    data.insert("langue".into(), fiche.langue.clone());
    data.insert("licence".into(), fiche.licence.clone());
    data.insert("audience".into(), nom_audience(&fiche.audience).into());
    let mut i = Intent::new(
        format!(
            "maia-federee-{}",
            crate::garde_etranger::identifiant_aleatoire()
        ),
        ActionType::Creation,
        "maia.federee.publication".to_string(),
        CallContext {
            caller_identity: utilisateur.to_string(),
            origin: ORIGINE_MAIA.to_string(),
            instance: "local".to_string(),
        },
        IntentData { content: data },
    );
    i.metadata
        .insert("requested_at".into(), maintenant_unix().to_string());
    i
}

/// Chaîne complète (voir l'en-tête du module) ; aucun effet avant la porte.
async fn traiter<S: SourceEtiquettes>(
    etat: &EtatPublicationHttp<S>,
    actif: &Actif<S>,
    corps: &[u8],
) -> Result<Publiee, Refus> {
    let u = &actif.utilisateur;
    if etat
        .limiteur_utilisateur
        .check(&format!("publier:{u}"))
        .is_err()
    {
        return Err(Refus::new(
            StatusCode::TOO_MANY_REQUESTS,
            "debit_utilisateur_depasse",
            format!("{PUBLICATIONS_PAR_HEURE} propositions par heure au plus"),
        ));
    }
    let p: Proposition = serde_json::from_slice(corps)
        .map_err(|e| Refus::new(StatusCode::BAD_REQUEST, "corps_invalide", e.to_string()))?;
    if p.reprises.is_some() || p.provenance.as_deref().is_some_and(|x| x != "locale") {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "republication_tamr_requise",
            "contenu étranger : republication par le seul chemin TAMR",
        ));
    }
    if est_nationale_par_nom(&p.donnee.base) {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "donnee_nationale",
            "mémoire de MAIA : jamais publiable",
        ));
    }
    p.donnee
        .check_shape()
        .map_err(|e| Refus::new(StatusCode::BAD_REQUEST, "donnee_invalide", e))?;
    let publieur = publieur().ok_or_else(|| {
        Refus::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "autorite_indisponible",
            "autorité des étiquettes indisponible",
        )
    })?;

    // Étiquette courante : base des champs structurels (relue par l'attestation).
    let etat_km = actif.source.lire(&p.donnee).await.map_err(|e| {
        Refus::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "kindmother_indisponible",
            e,
        )
    })?;
    let lue = etiquette_lue(&etat_km);
    if lue.nationale {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "donnee_nationale",
            "donnée nationale : jamais publiable",
        ));
    }
    let (Some(donnee_id), Some(parametres), true, Some(_)) = (
        lue.empreinte_donnee,
        lue.parametres.clone(),
        lue.posee,
        lue.empreinte_contenu
            .filter(|e| lue.empreinte_actuelle == Some(*e)),
    ) else {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "non_publiable",
            "donnée non étiquetée, étiquette retirée, contenu modifié depuis la pose ou pose sans paramètres",
        ));
    };

    // Champs structurels calculés par le serveur.
    let maintenant = maintenant_unix();
    let id = id_fiche_derivee(&donnee_id);
    let derniere = actif
        .service
        .lock()
        .map_err(|_| {
            Refus::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "service_indisponible",
                "verrou",
            )
        })?
        .derniere_version(&id);
    let version = derniere.checked_add(1).ok_or_else(|| {
        Refus::new(
            StatusCode::CONFLICT,
            "version_deja_publiee",
            "version maximale atteinte",
        )
        .pour(&id, derniere)
    })?;
    controler_annonces(&p, &id, derniere, version, &parametres, maintenant)
        .map_err(|r| r.pour(&id, version))?;

    let fiche = Fiche {
        id: id.clone(),
        emetteur: actif.identite.cle_publique(),
        sujet: p.sujet,
        type_fiche: parametres.type_fiche,
        texte: p.texte,
        relations: p
            .relations
            .into_iter()
            .map(|r| Relation {
                sujet: r.sujet,
                predicat: r.predicat,
                objet: r.objet,
            })
            .collect(),
        sources: p.sources,
        date: maintenant,
        version,
        langue: parametres.langue,
        licence: parametres.licence,
        audience: parametres.audience,
        reprises: Vec::new(),
    };
    let octets =
        sceller_fiche(&fiche, &actif.identite).map_err(|e| refus_federee(&e).pour(&id, version))?;
    // Empreinte de publication (celle des `reprises`), relue sur la Fiche scellée.
    let empreinte = hex::encode(
        verifier_fiche(&octets, maintenant)
            .map_err(|e| refus_federee(&e).pour(&id, version))?
            .empreinte(),
    );
    let avec = |r: Refus| Refus {
        empreinte: Some(empreinte.clone()),
        ..r.pour(&id, version)
    };

    // Attestation : étiquette relue, dérivation stricte, plafond par donnée.
    let capacite = publieur
        .attester(
            &Lecteur(&actif.source),
            &p.donnee,
            &octets,
            derniere,
            maintenant,
        )
        .await
        .map_err(|e| avec(refus_federee(&e)))?;

    // Porte des Cores : seule une décision ALLOW exécute la publication.
    let intent = intention_publication(
        u,
        &fiche,
        &empreinte,
        &hex::encode(donnee_id),
        &p.donnee.base,
    );
    let a_publier = octets.clone();
    let resultat = etat.gouvernance.executer(&intent, |jeton| {
        let regle = jeton.regle().to_string();
        actif
            .service
            .lock()
            .map_err(|_| ErreurFederee::Interdit("service de publication indisponible"))
            .and_then(|mut s| s.publier(a_publier, Publication::Originale(capacite), maintenant))
            .map(|()| regle)
    });
    match resultat {
        Ok(Ok(regle)) => Ok(Publiee {
            id: id.clone(),
            version,
            date: maintenant,
            empreinte: empreinte.clone(),
            octets,
            regle,
        }),
        Ok(Err(e)) => Err(avec(refus_federee(&e))),
        Err(motif) => Err(avec(Refus::new(
            StatusCode::FORBIDDEN,
            "refus_porte",
            motif,
        ))),
    }
}

/// `POST /v1/federee/{u}/publier`.
async fn publier<S: SourceEtiquettes>(
    State(etat): State<Arc<EtatPublicationHttp<S>>>,
    Extension(auth): Extension<UtilisateurAuthentifie>,
    Chemin(u): Chemin<String>,
    corps: Bytes,
) -> Response {
    let Some(actif) = etat.actif.as_ref() else {
        return refus_http(StatusCode::SERVICE_UNAVAILABLE, "publication_desactivee");
    };
    let instant = maintenant_unix();
    if u != actif.utilisateur || auth.0 != u {
        etat.tracer(EntreeJournal {
            instant,
            utilisateur: auth.0.clone(),
            verdict: "refusee",
            motif: "autre_utilisateur",
            detail: format!("chemin pour {u}"),
            fiche: None,
            version: None,
            empreinte: None,
        });
        return refus_http(StatusCode::FORBIDDEN, "autre_utilisateur");
    }
    match traiter(&etat, actif, &corps).await {
        Ok(p) => {
            etat.tracer(EntreeJournal {
                instant,
                utilisateur: u,
                verdict: "publiee",
                motif: "publiee",
                detail: format!("règle {}", p.regle),
                fiche: Some(p.id.clone()),
                version: Some(p.version),
                empreinte: Some(p.empreinte.clone()),
            });
            Json(serde_json::json!({
                "publiee": {
                    "id": p.id,
                    "version": p.version,
                    "date": p.date,
                    "empreinte": p.empreinte,
                    "fiche": base64::engine::general_purpose::STANDARD.encode(&p.octets),
                }
            }))
            .into_response()
        }
        Err(r) => {
            etat.tracer(EntreeJournal {
                instant,
                utilisateur: u,
                verdict: "refusee",
                motif: r.code,
                detail: r.detail.clone(),
                fiche: r.fiche.clone(),
                version: r.version,
                empreinte: r.empreinte.clone(),
            });
            (
                r.statut,
                Json(serde_json::json!({
                    "error": "publication refusée",
                    "reason": r.code,
                    "detail": r.detail,
                })),
            )
                .into_response()
        }
    }
}

#[cfg(test)]
#[allow(clippy::too_many_lines, clippy::many_single_char_names)]
mod tests {
    use super::*;
    use axum::http::header;
    use ed25519_dalek::{Signer, SigningKey};
    use kindmother::etiquette::{
        encode_hex, ActionEtiquette, IntentionEtiquette, TypePublication, DOMAINE_CONTENU,
        VERSION_INTENTION,
    };
    use kindmother::intent::{SchemaMigration, TypedWriteIntent};
    use miyauth::http::service::entetes_signees;
    use sha2::{Digest, Sha256};
    use std::path::PathBuf;
    use tower::ServiceExt;

    const TEXTE: &str = "Paris est la capitale de la France.";
    const INSTANCE: &str = "instance-essai";

    /// Clé HTTP de l'utilisateur (requêtes signées).
    fn alice_http() -> CleSignature {
        CleSignature::depuis_graine(&[7_u8; 32])
    }

    /// Clé d'étiquette de l'utilisateur (intentions KindMother).
    fn alice_etiquette() -> SigningKey {
        SigningKey::from_bytes(&[11; 32])
    }

    fn usage() -> String {
        format!("{USAGE_FEDEREE}/{INSTANCE}/alice")
    }

    fn empreinte(texte: &str) -> String {
        let mut h = Sha256::new();
        h.update(DOMAINE_CONTENU);
        h.update(texte.as_bytes());
        encode_hex(&h.finalize())
    }

    /// KindMother réel (serveur TCP), opérateur `jaykonta`, table `notes`.
    struct Banc {
        dir: PathBuf,
        cle_serveur: [u8; 32],
        km: KindMotherClient,
        etat: Arc<EtatPublicationHttp<EtiquettesKindMother>>,
        app: Router,
    }

    impl Drop for Banc {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    async fn banc_avec(gouvernance: GouvernanceMaia, identite: Option<CleSignature>) -> Banc {
        let dir = std::env::temp_dir().join(format!(
            "maia-federee-{}",
            crate::garde_etranger::identifiant_aleatoire()
        ));
        let utilisateurs = dir.join("credentials").join("utilisateurs");
        std::fs::create_dir_all(&utilisateurs).expect("dossier");
        std::fs::write(
            utilisateurs.join("alice.pub"),
            format!(
                "{}\n",
                encode_hex(alice_etiquette().verifying_key().as_bytes())
            ),
        )
        .expect("clé");
        std::fs::write(utilisateurs.join("alice.bases"), "jaykonta\nmaia-alice\n").expect("bases");
        std::fs::write(dir.join("maia-utilisateurs"), "alice\n").expect("maia");
        std::fs::write(dir.join("bases-nationales"), "jaykoa\n").expect("nationales");
        let server = kindmother_service::KindMotherServer::new(&dir)
            .await
            .expect("serveur");
        let cle_serveur = server.server_public_key();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().expect("addr").to_string();
        tokio::spawn(server.serve_listener(listener));
        let cred = dir.join("credentials");
        let km = KindMotherClient::connect(
            &addr,
            &OperatorCredential::load_from_dir("jaykonta", &cred).expect("cred"),
            "jaykonta",
        )
        .await
        .expect("connexion");
        km.apply_migrations(vec![SchemaMigration::new(
            1,
            "notes",
            &["CREATE TABLE IF NOT EXISTS notes (id TEXT PRIMARY KEY, body TEXT NOT NULL)"],
        )])
        .await
        .expect("migration");
        let source = EtiquettesKindMother::charger(&addr, "jaykonta", Some(&cred)).expect("source");
        let actif = identite.map(|c| ("alice".to_string(), source, c));
        let etat = Arc::new(EtatPublicationHttp::new(
            actif,
            vec![alice_http().cle_publique()],
            INSTANCE.to_string(),
            Arc::new(gouvernance),
            SecurityConfig::default(),
        ));
        let app = routeur(Arc::clone(&etat));
        Banc {
            dir,
            cle_serveur,
            km,
            etat,
            app,
        }
    }

    async fn banc() -> Banc {
        banc_avec(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some(CleSignature::generer()),
        )
        .await
    }

    impl Banc {
        /// Nouvelle donnée (identifiant unique : plafonds d'attestation distincts par test).
        async fn donnee(&self, texte: &str) -> RefDonnee {
            let id = format!("n{}", crate::garde_etranger::identifiant_aleatoire());
            self.km
                .submit(
                    TypedWriteIntent::insert("notes")
                        .database("jaykonta")
                        .value("id", id.as_str())
                        .value("body", texte),
                )
                .await
                .expect("insert");
            RefDonnee::new("jaykonta", "notes", "body").cle("id", id.as_str())
        }

        async fn etiqueter(&self, d: &RefDonnee, action: ActionEtiquette, texte: &str, t: u64) {
            let parametres = (action == ActionEtiquette::Poser).then(|| ParametresPublication {
                type_fiche: TypePublication::Fait,
                langue: "fr".into(),
                licence: "CC-BY-4.0".into(),
                audience: AudiencePublication::Membres,
            });
            let cle = alice_etiquette();
            let mut i = IntentionEtiquette {
                version: VERSION_INTENTION,
                parametres,
                action,
                donnee: d.clone(),
                utilisateur: "alice".into(),
                empreinte_contenu: (action == ActionEtiquette::Poser).then(|| empreinte(texte)),
                horodatage: t,
                cle_publique: encode_hex(cle.verifying_key().as_bytes()),
                signature: "00".repeat(64),
            };
            let m = i.message_signe(&self.cle_serveur).expect("message");
            i.signature = encode_hex(&cle.sign(&m).to_bytes());
            self.km.etiqueter(i).await.expect("étiquette");
        }

        async fn poser(&self, d: &RefDonnee) {
            self.etiqueter(d, ActionEtiquette::Poser, TEXTE, maintenant_unix())
                .await;
        }

        async fn appel(
            &self,
            chemin: &str,
            corps: &str,
            signe: bool,
        ) -> (StatusCode, serde_json::Value) {
            let mut b = axum::http::Request::builder()
                .method("POST")
                .uri(chemin)
                .header(header::CONTENT_TYPE, "application/json");
            if signe {
                for (n, v) in
                    entetes_signees(&alice_http(), &usage(), "POST", chemin, corps.as_bytes())
                {
                    b = b.header(n, v);
                }
            }
            let mut r = b.body(Body::from(corps.to_string())).expect("requête");
            r.extensions_mut()
                .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40002))));
            let rep = self.app.clone().oneshot(r).await.expect("réponse");
            let s = rep.status();
            let o = axum::body::to_bytes(rep.into_body(), 1 << 20)
                .await
                .expect("corps");
            (
                s,
                serde_json::from_slice(&o).unwrap_or(serde_json::Value::Null),
            )
        }

        async fn proposer(&self, p: serde_json::Value) -> (StatusCode, serde_json::Value) {
            self.appel("/v1/federee/alice/publier", &p.to_string(), true)
                .await
        }

        fn dernier_motif(&self) -> &'static str {
            self.etat.journal().last().map_or("", |e| e.motif)
        }
    }

    /// Proposition dérivée exacte de `TEXTE` pour la donnée `d`.
    fn proposition(d: &RefDonnee) -> serde_json::Value {
        serde_json::json!({
            "donnee": d,
            "sujet": "capitale de la France",
            "texte": TEXTE,
            "relations": [{ "sujet": "Paris", "predicat": "capitale de", "objet": "France" }],
        })
    }

    fn avec(mut p: serde_json::Value, cle: &str, v: serde_json::Value) -> serde_json::Value {
        p[cle] = v;
        p
    }

    fn id_attendu(d: &RefDonnee) -> String {
        let mut m = b"miyukini/kindmother/etiquette/donnee/v1\0".to_vec();
        m.extend_from_slice(&d.canonical_bytes());
        let e: [u8; 32] = Sha256::digest(&m).into();
        id_fiche_derivee(&e)
    }

    #[tokio::test]
    async fn publication_reussie_puis_double_publication_refusee() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(s, StatusCode::OK, "{v}");
        let id = id_attendu(&d);
        assert_eq!(v["publiee"]["id"], id.as_str());
        assert_eq!(v["publiee"]["version"], 1);
        // Fiche signée par le COG, champs calculés par le serveur et paramètres de l'utilisateur.
        let octets = base64::engine::general_purpose::STANDARD
            .decode(v["publiee"]["fiche"].as_str().expect("fiche"))
            .expect("base64");
        let f = maia_federee::verifier_fiche(&octets, maintenant_unix()).expect("fiche");
        assert_eq!(f.fiche().id, id);
        assert_eq!(f.fiche().texte, TEXTE);
        assert_eq!(f.fiche().licence, "CC-BY-4.0");
        assert_eq!(f.fiche().audience, Audience::Membres);
        assert!(f.fiche().date.abs_diff(maintenant_unix()) <= 5);
        assert_eq!(
            hex::encode(f.empreinte()),
            v["publiee"]["empreinte"].as_str().expect("e")
        );
        assert_eq!(b.etat.derniere_version(&id), 1);
        let j = b.etat.journal();
        assert_eq!(
            (j[0].verdict, j[0].fiche.as_deref()),
            ("publiee", Some(id.as_str()))
        );
        assert!(
            j[0].detail.contains("maia-federee-publier"),
            "{}",
            j[0].detail
        );

        // Même version annoncée de nouveau : refus, rien n'est publié.
        let (s, v) = b.proposer(avec(proposition(&d), "version", 1.into())).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::CONFLICT, Some("version_deja_publiee"))
        );
        assert_eq!(b.etat.derniere_version(&id), 1);
        assert_eq!(b.dernier_motif(), "version_deja_publiee");
        // Version suivante (annoncée juste, avec l'id juste) : publiée.
        let p = avec(
            avec(proposition(&d), "version", 2.into()),
            "id",
            id.as_str().into(),
        );
        let (s, v) = b.proposer(p).await;
        assert_eq!(
            (s, v["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(2)),
            "{v}"
        );
        // Concurrence : le service lui-même refuse une version déjà publiée.
        let mut service = ServiceRequete::new(cog_de_cle(&f.fiche().emetteur).expect("cog"));
        assert_eq!(service.derniere_version(&id), 0);
        assert!(matches!(
            service.publier(
                octets.clone(),
                Publication::Originale(
                    publieur()
                        .expect("publieur")
                        .attester(
                            &Lecteur(&b.etat.actif.as_ref().expect("actif").source),
                            &d,
                            &octets,
                            0,
                            maintenant_unix()
                        )
                        .await
                        .expect("capacité")
                ),
                maintenant_unix()
            ),
            Ok(())
        ));
        let encore = publieur()
            .expect("publieur")
            .attester(
                &Lecteur(&b.etat.actif.as_ref().expect("actif").source),
                &d,
                &octets,
                0,
                maintenant_unix(),
            )
            .await
            .expect("capacité");
        assert_eq!(
            service.publier(octets, Publication::Originale(encore), maintenant_unix()),
            Err(ErreurFederee::VersionObsolete)
        );
    }

    #[tokio::test]
    async fn refus_non_etiquetee_retiree_contenu_modifie_texte_different() {
        let b = banc().await;
        // Non étiquetée.
        let d = b.donnee(TEXTE).await;
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("non_publiable")),
            "{v}"
        );
        // Posée puis retirée.
        let t = maintenant_unix();
        b.etiqueter(&d, ActionEtiquette::Poser, TEXTE, t).await;
        b.etiqueter(&d, ActionEtiquette::Retirer, TEXTE, t + 1)
            .await;
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("non_publiable")),
            "{v}"
        );
        // Contenu modifié après la pose.
        let d2 = b.donnee(TEXTE).await;
        b.poser(&d2).await;
        let cle = d2.cle.get("id").cloned().expect("clé");
        let kindmother::intent::SqlValue::Text(cle) = cle else {
            panic!("texte")
        };
        b.km.submit(
            TypedWriteIntent::update("notes")
                .database("jaykonta")
                .key("id", cle.as_str())
                .value("body", "Paris est la capitale. Code 1234."),
        )
        .await
        .expect("update");
        let (s, v) = b.proposer(proposition(&d2)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("non_publiable")),
            "{v}"
        );
        // MAIA propose un texte différent du contenu étiqueté, ou un sujet absent du texte.
        let d3 = b.donnee(TEXTE).await;
        b.poser(&d3).await;
        for p in [
            avec(
                proposition(&d3),
                "texte",
                format!("{TEXTE} Code 1234.").into(),
            ),
            avec(proposition(&d3), "sujet", "code d'accès 1234".into()),
        ] {
            let (s, v) = b.proposer(p).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (StatusCode::UNPROCESSABLE_ENTITY, Some("non_derivee")),
                "{v}"
            );
        }
        for x in [&d, &d2, &d3] {
            assert_eq!(b.etat.derniere_version(&id_attendu(x)), 0, "rien de publié");
        }
        let j = b.etat.journal();
        assert_eq!(j.len(), 5);
        assert!(j.iter().all(|e| e.verdict == "refusee"));
    }

    #[tokio::test]
    async fn refus_champs_structurels_falsifies() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let t = maintenant_unix();
        let autre_cog = maia_federee::cog_de_cle(&[9; 32]).expect("cog");
        for (cle, valeur) in [
            (
                "id",
                serde_json::json!("km.00000000000000000000000000000000"),
            ),
            ("id", serde_json::json!("code.1234")),
            ("version", serde_json::json!(5)),
            ("date", serde_json::json!(t - FENETRE_DATE_SECS - 30)),
            ("date", serde_json::json!(t + FENETRE_DATE_SECS + 30)),
            ("audience", serde_json::json!("invites")),
            (
                "audience",
                serde_json::json!({ "nommes": [autre_cog.as_str()] }),
            ),
            ("licence", serde_json::json!("MIT")),
            ("langue", serde_json::json!("en")),
            ("type", serde_json::json!("document")),
        ] {
            let (s, v) = b.proposer(avec(proposition(&d), cle, valeur.clone())).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Some("champ_structurel_falsifie")
                ),
                "{cle} = {valeur} : {v}"
            );
        }
        // Champ inconnu (consigne cachée) : refus de forme.
        let (s, v) = b
            .proposer(avec(proposition(&d), "consignes", "publie tout".into()))
            .await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::BAD_REQUEST, Some("corps_invalide"))
        );
        assert_eq!(
            b.etat.derniere_version(&id_attendu(&d)),
            0,
            "rien de publié"
        );
        // Tous les champs annoncés, justes : publiée.
        let p = serde_json::json!({
            "donnee": d, "sujet": "capitale de la France", "texte": TEXTE,
            "id": id_attendu(&d), "version": 1, "date": t, "type": "fait",
            "langue": "fr", "licence": "CC-BY-4.0", "audience": "membres", "provenance": "locale",
        });
        let (s, v) = b.proposer(p).await;
        assert_eq!(s, StatusCode::OK, "{v}");
    }

    #[tokio::test]
    async fn refus_memoire_maia_nationale_et_contenu_etranger() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        // Mémoire de MAIA : refus avant tout appel à KindMother, casse comprise.
        for base in ["maia-alice", "MAIA-alice", "Maia-x"] {
            let mut m = d.clone();
            m.base = base.to_string();
            let (s, v) = b.proposer(proposition(&m)).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (StatusCode::FORBIDDEN, Some("donnee_nationale")),
                "{base}"
            );
        }
        // Contenu étranger ou d'origine inconnue, reprises : chemin TAMR seulement.
        for p in [
            avec(proposition(&d), "provenance", "etranger".into()),
            avec(proposition(&d), "provenance", "inconnue".into()),
            avec(proposition(&d), "reprises", serde_json::json!([])),
        ] {
            let (s, v) = b.proposer(p).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (StatusCode::FORBIDDEN, Some("republication_tamr_requise")),
                "{v}"
            );
        }
        assert_eq!(b.etat.derniere_version(&id_attendu(&d)), 0);
        // Donnée nationale déclarée côté KindMother : refus même si l'état la dit posée.
        let mut e = EtatEtiquette {
            sensibilite: Sensibilite::Nationale,
            posee: true,
            publiable: true,
            empreinte_contenu: Some(empreinte(TEXTE)),
            empreinte_actuelle: Some(empreinte(TEXTE)),
            utilisateur: None,
            horodatage: None,
            seq: None,
            parametres: None,
            empreinte_donnee: Some("ab".repeat(32)),
        };
        assert!(etiquette_lue(&e).nationale);
        // COG nommé invalide dans les paramètres : paramètres absents (jamais publiable).
        e.parametres = Some(ParametresPublication {
            type_fiche: TypePublication::Fait,
            langue: "fr".into(),
            licence: "MIT".into(),
            audience: AudiencePublication::Nommes(vec!["pas-un-cog".into()]),
        });
        assert_eq!(etiquette_lue(&e).parametres, None);
    }

    #[tokio::test]
    async fn plafond_d_attestations_par_donnee() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        for v in 1..=maia_federee::MAX_ATTESTATIONS_PAR_PERIODE {
            let (s, r) = b.proposer(proposition(&d)).await;
            assert_eq!(
                (s, r["publiee"]["version"].as_u64()),
                (StatusCode::OK, Some(v as u64)),
                "{r}"
            );
        }
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::TOO_MANY_REQUESTS, Some("plafond_attestations"))
        );
        assert_eq!(b.dernier_motif(), "plafond_attestations");
        // Autre donnée : plafond distinct.
        let d2 = b.donnee(TEXTE).await;
        b.poser(&d2).await;
        assert_eq!(b.proposer(proposition(&d2)).await.0, StatusCode::OK);
    }

    #[tokio::test]
    async fn porte_des_cores_refus_par_defaut() {
        for g in [
            crate::tamr::tests::gouvernance(crate::tamr::tests::POLITIQUE_SANS_TAMR),
            GouvernanceMaia::indisponible("essai"),
        ] {
            let b = banc_avec(g, Some(CleSignature::generer())).await;
            let d = b.donnee(TEXTE).await;
            b.poser(&d).await;
            let (s, v) = b.proposer(proposition(&d)).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (StatusCode::FORBIDDEN, Some("refus_porte")),
                "{v}"
            );
            assert_eq!(
                b.etat.derniere_version(&id_attendu(&d)),
                0,
                "rien de publié"
            );
            let j = b.etat.journal();
            assert_eq!(j[0].motif, "refus_porte");
            assert!(j[0].empreinte.is_some());
        }
    }

    #[tokio::test]
    async fn authentification_utilisateur_debit_et_desactivation() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let corps = proposition(&d).to_string();
        // Sans signature, ou signée pour l'usage mémoire : 401.
        assert_eq!(
            b.appel("/v1/federee/alice/publier", &corps, false).await.0,
            StatusCode::UNAUTHORIZED
        );
        let chemin = "/v1/federee/alice/publier";
        let mut r = axum::http::Request::builder().method("POST").uri(chemin);
        let memoire = format!("maia/memoire/{INSTANCE}/alice");
        for (n, v) in entetes_signees(&alice_http(), &memoire, "POST", chemin, corps.as_bytes()) {
            r = r.header(n, v);
        }
        let mut r = r.body(Body::from(corps.clone())).expect("r");
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40002))));
        assert_eq!(
            b.app.clone().oneshot(r).await.expect("r").status(),
            StatusCode::UNAUTHORIZED
        );
        // Autre utilisateur : 403, tracé.
        let (s, v) = b.appel("/v1/federee/bob/publier", &corps, true).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("autre_utilisateur"))
        );
        assert_eq!(b.dernier_motif(), "autre_utilisateur");
        // Débit par utilisateur : au-delà de PUBLICATIONS_PAR_HEURE propositions, 429.
        for _ in 0..PUBLICATIONS_PAR_HEURE {
            let (s, _) = b.appel("/v1/federee/alice/publier", "{}", true).await;
            assert_eq!(s, StatusCode::BAD_REQUEST);
        }
        let (s, v) = b.appel("/v1/federee/alice/publier", &corps, true).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (
                StatusCode::TOO_MANY_REQUESTS,
                Some("debit_utilisateur_depasse")
            )
        );
        assert_eq!(b.etat.derniere_version(&id_attendu(&d)), 0);

        // Sans clé d'identité du COG : désactivée.
        let off = banc_avec(crate::gouvernance::tests::gouvernance_de_base(), None).await;
        assert!(!off.etat.est_active());
        assert_eq!(
            off.appel(chemin, &corps, true).await.0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
}
