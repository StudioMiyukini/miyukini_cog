//! @id     maia_routes_federee
//! @do     publier_fiche_derivee_d_une_donnee_etiquetee
//! @role   security
//! @layer  app
//! @human  MAIA — routes de publication des Fiches fédérées (EX-47, EX-57, A-009, A-016, A-017, A-022 §4) :
//! @human  Fiche originale dérivée d'une donnée que l'utilisateur a étiquetée `publiable` ; republication
//! @human  d'une Fiche étrangère seulement après approbation TAMR des octets exacts ; Fiches signées par
//! @human  la clé `fiches` déléguée (0x45) ; StrongFather tranche ; état durable dans KindMother.

//! # Publication des Fiches (EX-47, A-009, A-015 à A-018, A-022 §4, A-024)
//!
//! | Méthode | Chemin | Signée par | Effet |
//! |---|---|---|---|
//! | POST | `/v1/federee/{u}/publier` | clé de l'utilisateur | propose une Fiche dérivée d'une donnée étiquetée ; publiée si tout passe |
//! | POST | `/v1/federee/{u}/republier` | clé de l'utilisateur | republication d'une donnée étrangère : Fiche calculée, **202** + demande d'approbation TAMR |
//! | POST | `/v1/federee/{u}/republier/{iv}` | clé d'**approbation** TAMR | approbation (`{"empreinte"}`) ⇒ StrongFather ⇒ publication |
//! | DELETE | `/v1/federee/{u}/republier/{iv}` | clé de l'utilisateur | refus de l'utilisateur, demande close |
//!
//! ## Clé des Fiches (A-022 §4, A-024)
//!
//! Les Fiches originales et republiées sont signées par la **clé d'usage
//! `fiches`** (indice 6) et portent sa délégation `0x45` signée par
//! l'identité du COG ([`SignataireFiches::deleguee`] ; vérification de chaîne
//! dans `maia_federee::fiche`). Configuration : [`ENV_CLE_FICHES`] (graine,
//! `0600`), [`ENV_DELEGATION_FICHES`] (sceau `0x45`, ≤ 1 Kio, fichier
//! régulier), [`ENV_IDENTITE_PUBLIQUE`] (clé publique d'identité) ; la
//! délégation doit être valide au démarrage et à chaque signature (sinon 503
//! `delegation_fiches_invalide` : la renouveler par la cérémonie
//! `miyuracine deleguer --usage fiches`, puis redémarrer le pont).
//!
//! **Transition** : sans clé `fiches`, [`ENV_CLE_IDENTITE`] (clé d'identité
//! dans le pont, A-017) signe encore, journalisé comme transitoire ; une clé
//! `fiches` configurée mais invalide désactive la route (jamais de repli sur
//! l'identité) et rend [`ENV_CLE_IDENTITE`] ignorée. Les Fiches déjà publiées
//! sous l'identité restent lisibles et rechargées ; changer d'**identité**
//! bloque la route (état incohérent, 503), changer de clé ou de génération
//! `fiches` non.
//!
//! ## Republication d'une donnée étrangère (A-009, chemin TAMR)
//!
//! 1. `POST …/republier` `{"fiches":[base64…], "sujet"?, "audience"?, "licence"?}` :
//!    débit (même plafond horaire que les propositions, compteur distinct) ;
//!    chaque source (1 à 16) est vérifiée (chaîne de signature), **étrangère**
//!    (jamais une Fiche du COG local) et son émetteur **admis** par la vue
//!    fédérale ([`EtatPublicationHttp::avec_vue_federale`] ; absente ⇒ 503
//!    `vue_federale_indisponible`) ; aucune donnée KindMother n'est lue (champ
//!    `donnee` refusé : jamais `maia-*`). Le serveur calcule la Fiche
//!    ([`maia_federee::deriver_reprise`] : contenu des sources, `reprises`,
//!    licence identique, audience au plus aussi large, `id` = `rp.` + 32 hex
//!    de l'ensemble des sources, version = dernière + 1), la scelle, puis fige
//!    ses **octets exacts** dans une demande TAMR ([`ActionTamr::Republier`]) ;
//!    réponse 202 `{"approbation":{intervention_id, empreinte, description…},
//!    "fiche":{id, version, date, empreinte, reprises, fiche}}`. Rien n'est publié.
//! 2. `POST …/republier/{iv}` `{"empreinte"}`, signée par une clé de
//!    `MAIA_TAMR_CLES_APPROBATION` (usage `maia/tamr-approbation/<instance>/<u>`,
//!    distincte des clés de l'utilisateur, F7) : la demande est consommée
//!    (usage unique) ; délai ([`DUREE_VIE_SECS`]), empreinte, approbateur =
//!    propriétaire, puis StrongFather `tamr.approbation` ; les sources sont
//!    **revérifiées** (vue fédérale comprise), [`ServiceRequete::controler`]
//!    revérifie la validation TAMR des octets exacts, les `reprises`, la licence,
//!    l'audience, la dérivation et la version ; puis la porte
//!    `federee.republier` (politiques `maia-federee-republier`,
//!    `maia-federee-republier-sans-tamr`) ; sur ALLOW seulement : écriture
//!    durable (même table qu'A-018 ; `donnee` = empreinte hex de l'ensemble des
//!    sources) puis inscription. Une Fiche republiée est citée mais ne compte
//!    jamais pour la corroboration.
//!
//! Codes propres : 202 demande ; 400 `sources_invalides`, `corps_invalide` ;
//! 401 `signature_approbation_requise` ; 403 `source_locale`,
//! `source_non_admise`, `audience_elargie`, `empreinte_differente`,
//! `approbation_expiree`, `approbateur_invalide`, `refus_tamr`, `refus_porte`,
//! `autre_utilisateur` ; 404 `approbation_inconnue` ; 409
//! `version_deja_publiee` ; 422 `source_invalide`, `licence_differente`,
//! `non_derivee` ; 429 ; 503 `vue_federale_indisponible`,
//! `approbations_desactivees`, `tamr_indisponible`, `delegation_fiches_invalide`.
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
//!   ([`ServiceRequete::derniere_version`], état **rechargé de KindMother**) ;
//! - `date` = horloge du serveur ;
//! - `type`, `langue`, `licence`, `audience` = paramètres **signés par
//!   l'utilisateur** avec la pose de l'étiquette (intention v2) ;
//! - émetteur = clé d'identité du COG, signature par le serveur (clé `fiches`).
//!
//! Si la proposition **annonce** l'un de ces champs, il est comparé à la
//! valeur calculée : écart ⇒ refus `champ_structurel_falsifie` (date : hors de
//! ±[`FENETRE_DATE_SECS`] s ; version déjà publiée : `version_deja_publiee`).
//!
//! ## Propriétaire de l'étiquette
//!
//! L'étiquette doit avoir été **posée par l'utilisateur authentifié** de la
//! route (`EtatEtiquette::utilisateur`), sinon refus 403 `non_publiable` (même
//! code qu'une donnée non étiquetée : la route ne révèle pas qu'un autre
//! utilisateur a étiqueté la donnée). Le propriétaire est **revérifié** par
//! [`PublieurOriginal::attester`] à la relecture de l'étiquette : s'il a changé
//! entre-temps, refus `non_publiable`. La propriété de la **base** (fichier
//! `<u>.bases`, A-016) est vérifiée par KindMother à la pose : une étiquette
//! posée par l'utilisateur l'a été sur une base dont il est propriétaire
//! (le client KindMother n'expose pas la liste `.bases`).
//!
//! ## Ordre de la chaîne
//!
//! 1. débit, forme, refus sans appel à KindMother (ci-dessous) ;
//! 2. lecture de l'étiquette (non `nationale`, posée **par l'utilisateur**,
//!    contenu inchangé, paramètres présents) ;
//! 3. **section critique** (verrou unique de publication, tenu jusqu'à la fin) :
//!    état rechargé de KindMother si nécessaire, version calculée, plafond
//!    d'attestations contrôlé **sans être décompté**, Fiche scellée,
//!    attestation ([`PublieurOriginal::attester`] : étiquette relue,
//!    propriétaire revérifié, dérivation stricte), contrôle sans effet du
//!    service ([`ServiceRequete::controler`]) ;
//! 4. porte StrongFather (refus par défaut) ; **seule une décision ALLOW**
//!    exécute l'effet : écriture durable dans KindMother **puis** inscription
//!    en mémoire ([`ServiceRequete::publier`], qui consomme l'attestation).
//!
//! Un refus de la porte ne consomme donc aucune attestation ; deux requêtes
//! concurrentes sont sérialisées par le verrou : ni version dupliquée, ni
//! plafond dépassé.
//!
//! ## Persistance (A-017, formats à consigner par amendement)
//!
//! KindMother est le seul écrivain. Base **dédiée** [`BASE_PUBLICATIONS_DEFAUT`]
//! (`cog-federee`, surchargeable par [`ENV_PUBLICATIONS`]), dont l'opérateur
//! KindMother homonyme est propriétaire. Elle n'est **pas** `nationale` (le nom
//! ne commence pas par `maia-` ; un nom `maia-*` est refusé au démarrage) et ne
//! doit figurer dans **aucun** fichier `<u>.bases` (jamais étiquetable par un
//! utilisateur) ; une proposition dont `donnee.base` est cette base est
//! refusée (`base_reservee`). Table [`TABLE_PUBLICATIONS`], une ligne par
//! version publiée, clé primaire `(fiche, version)` :
//!
//! | colonne | type | contenu |
//! |---|---|---|
//! | `fiche` | TEXT | `id` de la Fiche (`km.<32 hex>`) |
//! | `version` | INTEGER ≥ 1 | version publiée |
//! | `donnee` | TEXT | empreinte (hex) de la référence de donnée étiquetée |
//! | `empreinte` | TEXT | empreinte (hex) de publication des octets signés |
//! | `utilisateur` | TEXT | utilisateur qui a proposé (propriétaire de l'étiquette) |
//! | `publiee_a` | INTEGER | instant de publication = attestation consommée (s Unix) |
//! | `octets` | TEXT | octets signés de la Fiche (base64 standard) |
//!
//! Une publication = **une** intention d'insertion typée (atomique : version,
//! Fiche et attestation dans la même ligne). L'insertion échoue si `(fiche,
//! version)` existe déjà : une version n'est enregistrée qu'une fois, jamais
//! avec un autre contenu. Ordre : écriture KindMother **avant** l'état en
//! mémoire et avant toute réponse. Crash avant l'écriture : rien n'a été rendu
//! ni servi, la version reste libre. Crash ou réponse perdue après
//! l'écriture : l'état est marqué « à recharger », la requête suivante relit
//! KindMother et exige la version suivante. Au démarrage
//! ([`EtatPublicationHttp::precharger`]) et après toute erreur de
//! persistance, l'état est rechargé (Fiches revérifiées, versions contiguës,
//! plafond glissant reconstitué par [`PublieurOriginal::restaurer`]) ;
//! KindMother indisponible ou état incohérent ⇒ 503 (fail-closed), jamais de
//! reprise à la version 1.
//!
//! ## Refus sans appel à KindMother
//!
//! - base `maia-*` (mémoire de MAIA, comparée sans casse) ⇒ `donnee_nationale` ;
//! - base de l'état de publication ⇒ `base_reservee` ;
//! - `provenance` autre que `locale`, ou champ `reprises` présent ⇒
//!   `republication_tamr_requise` : une donnée étrangère ne se republie que par
//!   `…/republier` (chemin TAMR, ci-dessus), jamais par `…/publier`.
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
//! - clé des Fiches : [`ENV_CLE_FICHES`], [`ENV_DELEGATION_FICHES`],
//!   [`ENV_IDENTITE_PUBLIQUE`] ; à défaut (transitoire) [`ENV_CLE_IDENTITE`] ;
//! - republication : `MAIA_TAMR_CLES_APPROBATION` et une vue fédérale ;
//! - `MAIA_FEDEREE_OPERATEUR` : opérateur KindMother qui lit les étiquettes
//!   (justificatif à l'emplacement standard, droit de lecture sur les bases
//!   publiables) ; `KINDMOTHER_ADDR` (défaut `127.0.0.1:50051`) ;
//! - justificatif de l'opérateur de l'état de publication ([`ENV_PUBLICATIONS`],
//!   défaut `cog-federee`) à l'emplacement standard.
//!
//! ## Limites (à consigner par amendement)
//!
//! - l'opérateur / la base `cog-federee` doit être déclaré côté KindMother
//!   (`kindmother-service`, opérateurs par défaut) : sans lui, la route reste
//!   désactivée ;
//! - le journal des décisions reste en mémoire (la trace durable est le
//!   journal `maia::federee::audit` et la table des publications) ;
//! - la vue fédérale n'est pas encore câblée dans le pont : en production la
//!   republication répond 503 `vue_federale_indisponible` ;
//! - demandes de republication en attente gardées **en mémoire** (perdues au
//!   redémarrage : l'utilisateur redemande) ;
//! - aucune révocation de délégation `0x45` ; délégation renouvelée ⇒
//!   redémarrage du pont ;
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
    base_valide, AudiencePublication, EtatEtiquette, ParametresPublication, RefDonnee, Sensibilite,
    PREFIXE_NATIONALE,
};
use kindmother::intent::{SchemaMigration, SqlValue, TypedSelect, TypedWriteIntent};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use maia_federee::fiche::MAX_REPRISES;
use maia_federee::{
    deriver_reprise, empreinte_ensemble, id_fiche_derivee, id_fiche_reprise,
    verifier_derivation_reprise, verifier_fiche, Audience, AutoriteEtiquettes, ChoixReprise,
    ErreurFederee, EtiquetteLue, Fiche, FicheVerifiee, LecteurEtiquettes, ParametresLus,
    Publication, PublieurOriginal, Relation, ServiceRequete, SignataireFiches, TypeFiche,
    VueFederale, FENETRE_DATE_SECS, MAX_DELEGATION,
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
use crate::tamr::{
    ActionTamr, RefusTamr, RegistreApprobations, SignatureApprobation, CAPACITE_MAX,
    DUREE_VIE_SECS, ENV_CLES_APPROBATION, TYPE_INTERVENTION, USAGE_APPROBATION,
};

/// Usage signé (préfixe) des requêtes de publication ; usage complet :
/// `maia/federee/<instance>/<utilisateur>`.
pub const USAGE_FEDEREE: &str = "maia/federee";
/// **Transitoire** (A-022 §4) : graine de la clé d'identité du COG (chemin
/// d'un fichier de 32 octets, `0600`), utilisée seulement si la clé `fiches`
/// n'est pas configurée ; ignorée sinon.
pub const ENV_CLE_IDENTITE: &str = "MAIA_FEDEREE_CLE_IDENTITE";
/// Graine de la clé d'usage `fiches` (indice 6, A-022 §4) : chemin d'un
/// fichier de 32 octets, `0600` (`miyuracine deriver --exporter-graine-usage fiches`).
pub const ENV_CLE_FICHES: &str = "MAIA_FEDEREE_CLE_FICHES";
/// Délégation `0x45` de la clé `fiches` (chemin du sceau écrit par
/// `miyuracine deleguer --usage fiches`).
pub const ENV_DELEGATION_FICHES: &str = "MAIA_FEDEREE_DELEGATION_FICHES";
/// Clé publique d'identité du COG (hex ou base64 URL, 32 octets) : émetteur
/// des Fiches et signataire attendu de la délégation.
pub const ENV_IDENTITE_PUBLIQUE: &str = "MAIA_FEDEREE_IDENTITE_PUBLIQUE";
/// Opérateur KindMother qui lit les étiquettes `publiable`.
pub const ENV_OPERATEUR: &str = "MAIA_FEDEREE_OPERATEUR";
/// Opérateur KindMother (et base homonyme) de l'état de publication.
pub const ENV_PUBLICATIONS: &str = "MAIA_FEDEREE_PUBLICATIONS_OPERATEUR";
/// Base (et opérateur propriétaire) de l'état de publication par défaut.
pub const BASE_PUBLICATIONS_DEFAUT: &str = "cog-federee";
/// Table des versions publiées (voir l'en-tête du module).
pub const TABLE_PUBLICATIONS: &str = "fiches_publiees";
/// Lignes lues par page au rechargement.
const PAGE_RECHARGEMENT: u32 = 64;
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
/// Valeur de `data.operation` de l'intention StrongFather d'une republication.
pub const OPERATION_REPUBLIER: &str = "federee.republier";

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
// Connexion KindMother (partagée par la lecture des étiquettes et l'état)
// ═══════════════════════════════════════════════════════════════════════════

/// Connexion authentifiée d'un opérateur KindMother, ouverte au premier usage
/// et rouverte après une erreur de transport.
pub struct ConnexionKindMother {
    adresse: String,
    operateur: String,
    base: String,
    justificatif: OperatorCredential,
    client: tokio::sync::Mutex<Option<Arc<KindMotherClient>>>,
}

impl std::fmt::Debug for ConnexionKindMother {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConnexionKindMother")
            .field("adresse", &self.adresse)
            .field("operateur", &self.operateur)
            .field("base", &self.base)
            .finish_non_exhaustive()
    }
}

fn erreur_de_transport(e: &ClientError) -> bool {
    matches!(
        e,
        ClientError::Transport(_)
            | ClientError::Timeout(_)
            | ClientError::Connection(_)
            | ClientError::NotConnected
    )
}

impl ConnexionKindMother {
    /// Connexion de l'opérateur `operateur` (justificatif déjà chargé), base par
    /// défaut `base`.
    pub fn new(
        adresse: &str,
        operateur: &str,
        base: &str,
        justificatif: OperatorCredential,
    ) -> Self {
        Self {
            adresse: adresse.to_string(),
            operateur: operateur.to_string(),
            base: base.to_string(),
            justificatif,
            client: tokio::sync::Mutex::new(None),
        }
    }

    /// Justificatif `operateur` lu dans `credentials`, sinon à l'emplacement standard.
    pub fn charger(
        adresse: &str,
        operateur: &str,
        base: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, String> {
        let justificatif = match credentials {
            Some(d) => OperatorCredential::load_from_dir(operateur, d),
            None => OperatorCredential::load(operateur),
        }
        .map_err(|e| e.to_string())?;
        Ok(Self::new(adresse, operateur, base, justificatif))
    }

    async fn client(&self) -> Result<Arc<KindMotherClient>, ClientError> {
        let mut c = self.client.lock().await;
        if let Some(x) = c.as_ref() {
            return Ok(Arc::clone(x));
        }
        let x = Arc::new(
            KindMotherClient::connect(&self.adresse, &self.justificatif, &self.base).await?,
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

    /// Exécute `f` ; une erreur de transport ferme la connexion et, si
    /// `rejouable`, `f` est rejoué une fois sur une connexion neuve (jamais
    /// pour une écriture : son issue serait ambiguë).
    async fn executer<T, F, Fut>(&self, rejouable: bool, f: F) -> Result<T, String>
    where
        F: Fn(Arc<KindMotherClient>) -> Fut,
        Fut: Future<Output = Result<T, ClientError>>,
    {
        let mut derniere = String::from("KindMother injoignable");
        for tentative in 0..2 {
            let c = match self.client().await {
                Ok(c) => c,
                Err(e) => {
                    derniere = e.to_string();
                    if rejouable && tentative == 0 && erreur_de_transport(&e) {
                        continue;
                    }
                    return Err(derniere);
                }
            };
            match f(Arc::clone(&c)).await {
                Ok(v) => return Ok(v),
                Err(e) if erreur_de_transport(&e) => {
                    tracing::warn!(target: "maia::federee", operateur = %self.operateur, erreur = %e, "connexion KindMother perdue : rouverte");
                    self.abandonner(&c).await;
                    derniere = e.to_string();
                    if !rejouable {
                        return Err(derniere);
                    }
                }
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(derniere)
    }
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

/// Lecteur d'étiquettes adossé à un opérateur KindMother authentifié.
#[derive(Debug)]
pub struct EtiquettesKindMother(ConnexionKindMother);

impl EtiquettesKindMother {
    /// Lecteur de l'opérateur `operateur` (justificatif déjà chargé).
    pub fn new(adresse: &str, operateur: &str, justificatif: OperatorCredential) -> Self {
        Self(ConnexionKindMother::new(
            adresse,
            operateur,
            operateur,
            justificatif,
        ))
    }

    /// Justificatif `operateur` lu dans `credentials`, sinon à l'emplacement standard.
    pub fn charger(
        adresse: &str,
        operateur: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, String> {
        ConnexionKindMother::charger(adresse, operateur, operateur, credentials).map(Self)
    }
}

impl SourceEtiquettes for EtiquettesKindMother {
    async fn lire(&self, donnee: &RefDonnee) -> Result<EtatEtiquette, String> {
        self.0
            .executer(true, |c| {
                let d = donnee.clone();
                async move { c.lire_etiquette(d).await }
            })
            .await
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// État durable de publication (KindMother, A-017)
// ═══════════════════════════════════════════════════════════════════════════

/// Une version publiée, telle qu'enregistrée (une ligne de [`TABLE_PUBLICATIONS`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationDurable {
    /// Identifiant de la Fiche.
    pub fiche: String,
    /// Version publiée.
    pub version: u64,
    /// Empreinte (hex) de la référence de donnée étiquetée.
    pub donnee: String,
    /// Empreinte (hex) de publication des octets signés.
    pub empreinte: String,
    /// Utilisateur qui a proposé.
    pub utilisateur: String,
    /// Instant de publication (attestation consommée).
    pub publiee_a: u64,
    /// Octets signés de la Fiche.
    pub octets: Vec<u8>,
}

/// Stockage durable de l'état de publication.
pub trait MagasinPublications: Send + Sync + 'static {
    /// Nom de la base (refusée comme source de publication).
    fn base(&self) -> &str;
    /// Toutes les versions publiées ; erreur : motif lisible.
    fn charger(&self) -> impl Future<Output = Result<Vec<PublicationDurable>, String>> + Send;
    /// Enregistre **atomiquement** une version publiée ; échoue si `(fiche,
    /// version)` existe déjà. Erreur : issue inconnue (l'appelant recharge).
    fn enregistrer(
        &self,
        p: &PublicationDurable,
    ) -> impl Future<Output = Result<(), String>> + Send;
}

/// État de publication dans la base KindMother dédiée (voir l'en-tête du module).
#[derive(Debug)]
pub struct PublicationsKindMother(ConnexionKindMother);

/// Migrations de [`TABLE_PUBLICATIONS`] (v1 ; toute évolution : nouvelle étape).
fn migrations_publications() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "maia-federee : versions publiees (A-017)",
        &["CREATE TABLE IF NOT EXISTS fiches_publiees (\
            fiche TEXT NOT NULL, \
            version INTEGER NOT NULL CHECK (version >= 1), \
            donnee TEXT NOT NULL, \
            empreinte TEXT NOT NULL, \
            utilisateur TEXT NOT NULL, \
            publiee_a INTEGER NOT NULL, \
            octets TEXT NOT NULL, \
            PRIMARY KEY (fiche, version))"],
    )]
}

/// Base admise pour l'état de publication : nom valide, jamais `nationale`.
fn base_publications_admise(base: &str) -> bool {
    base_valide(base) && !est_nationale_par_nom(base)
}

impl PublicationsKindMother {
    /// État dans la base `operateur` (dont l'opérateur homonyme est
    /// propriétaire) ; base `maia-*` ou invalide ⇒ erreur.
    pub fn charger(
        adresse: &str,
        operateur: &str,
        credentials: Option<&Path>,
    ) -> Result<Self, String> {
        if !base_publications_admise(operateur) {
            return Err(format!(
                "base de publication « {operateur} » refusée (invalide ou nationale)"
            ));
        }
        ConnexionKindMother::charger(adresse, operateur, operateur, credentials).map(Self)
    }
}

fn texte_de(
    ligne: &std::collections::HashMap<String, serde_json::Value>,
    c: &str,
) -> Result<String, String> {
    ligne
        .get(c)
        .and_then(serde_json::Value::as_str)
        .map(str::to_string)
        .ok_or_else(|| format!("colonne {c} absente ou mal typée"))
}

fn entier_de(
    ligne: &std::collections::HashMap<String, serde_json::Value>,
    c: &str,
) -> Result<u64, String> {
    ligne
        .get(c)
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| format!("colonne {c} absente ou mal typée"))
}

fn publication_de(
    ligne: &std::collections::HashMap<String, serde_json::Value>,
) -> Result<PublicationDurable, String> {
    Ok(PublicationDurable {
        fiche: texte_de(ligne, "fiche")?,
        version: entier_de(ligne, "version")?,
        donnee: texte_de(ligne, "donnee")?,
        empreinte: texte_de(ligne, "empreinte")?,
        utilisateur: texte_de(ligne, "utilisateur")?,
        publiee_a: entier_de(ligne, "publiee_a")?,
        octets: base64::engine::general_purpose::STANDARD
            .decode(texte_de(ligne, "octets")?)
            .map_err(|e| format!("octets : {e}"))?,
    })
}

fn entier_sql(v: u64) -> Result<SqlValue, String> {
    i64::try_from(v)
        .map(SqlValue::Integer)
        .map_err(|_| "entier hors bornes".to_string())
}

impl MagasinPublications for PublicationsKindMother {
    fn base(&self) -> &str {
        &self.0.base
    }

    async fn charger(&self) -> Result<Vec<PublicationDurable>, String> {
        self.0
            .executer(true, |c| async move {
                c.apply_migrations(migrations_publications()).await
            })
            .await?;
        let mut toutes = Vec::new();
        let mut decalage: u32 = 0;
        loop {
            let mut q = TypedSelect::from(TABLE_PUBLICATIONS)
                .order_by("fiche", false)
                .order_by("version", false)
                .limit(PAGE_RECHARGEMENT);
            q.offset = Some(decalage);
            let lignes = self
                .0
                .executer(true, |c| {
                    let q = q.clone();
                    async move { c.select(q).await }
                })
                .await?;
            let n = lignes.len();
            for l in &lignes {
                toutes.push(publication_de(l)?);
            }
            if n < PAGE_RECHARGEMENT as usize {
                return Ok(toutes);
            }
            decalage = decalage
                .checked_add(PAGE_RECHARGEMENT)
                .ok_or_else(|| "trop de publications".to_string())?;
        }
    }

    async fn enregistrer(&self, p: &PublicationDurable) -> Result<(), String> {
        let intention = TypedWriteIntent::insert(TABLE_PUBLICATIONS)
            .value("fiche", p.fiche.as_str())
            .value("version", entier_sql(p.version)?)
            .value("donnee", p.donnee.as_str())
            .value("empreinte", p.empreinte.as_str())
            .value("utilisateur", p.utilisateur.as_str())
            .value("publiee_a", entier_sql(p.publiee_a)?)
            .value(
                "octets",
                base64::engine::general_purpose::STANDARD.encode(&p.octets),
            )
            .purpose("maia-federee : version publiee (A-017)");
        self.0
            .executer(false, |c| {
                let i = intention.clone();
                async move { c.submit(i).await }
            })
            .await
            .map(|_| ())
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
    .avec_proprietaire(e.utilisateur.as_deref())
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

struct Actif<S, M> {
    utilisateur: String,
    source: S,
    magasin: M,
    /// Clé `fiches` déléguée (ou identité, transitoire) : signe les Fiches.
    signataire: SignataireFiches,
    cog: CogId,
    /// Verrou unique de publication ; `None` : état à (re)charger de KindMother.
    publication: tokio::sync::Mutex<Option<ServiceRequete>>,
}

/// Vue fédérale partagée (admission des émetteurs des Fiches republiées).
pub type VuePartagee = Arc<dyn VueFederale + Send + Sync>;

/// Republication en attente d'approbation TAMR : octets exacts figés et
/// sources (octets signés) à revérifier au moment de publier.
struct RepublicationEnAttente {
    utilisateur: String,
    octets: Vec<u8>,
    sources: Vec<Vec<u8>>,
    id: String,
    version: u64,
    ensemble: String,
    empreinte: String,
    demandee: std::time::Instant,
}

/// État de la route de publication.
pub struct EtatPublicationHttp<S: SourceEtiquettes, M: MagasinPublications = PublicationsKindMother>
{
    actif: Option<Actif<S, M>>,
    garde: GardeSignee,
    /// Clés d'approbation TAMR (F7), distinctes des clés de l'utilisateur.
    garde_approbation: GardeSignee,
    instance: String,
    limiteur_ip: RateLimiter,
    limiteur_utilisateur: RateLimiter,
    gouvernance: Arc<GouvernanceMaia>,
    securite: SecurityConfig,
    journal: Mutex<VecDeque<EntreeJournal>>,
    /// Registre TAMR des republications (usage unique, expiration).
    approbations: RegistreApprobations,
    duree_approbation: std::time::Duration,
    republications: Mutex<std::collections::HashMap<String, RepublicationEnAttente>>,
    /// Vue fédérale ; absente ⇒ republication refusée (503).
    vue: Option<VuePartagee>,
}

fn indisponible(detail: impl Into<String>) -> Refus {
    Refus::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "kindmother_indisponible",
        detail,
    )
}

/// Reconstruit le service depuis les versions enregistrées (ordre `(fiche,
/// version)`) : chaque Fiche est revérifiée (chaîne de la clé `fiches`, ou
/// identité pour les Fiches de la transition) et doit concorder avec sa ligne ;
/// une Fiche republiée est restaurée par le chemin TAMR.
fn restaurer(
    publieur: &PublieurOriginal,
    cog: &CogId,
    mut lignes: Vec<PublicationDurable>,
    maintenant: u64,
) -> Result<ServiceRequete, String> {
    lignes.sort_by(|a, b| a.fiche.cmp(&b.fiche).then(a.version.cmp(&b.version)));
    let mut service = ServiceRequete::new(cog.clone());
    for l in lignes {
        let f = verifier_fiche(&l.octets, maintenant)
            .map_err(|e| format!("{} v{} : {e}", l.fiche, l.version))?;
        if f.fiche().id != l.fiche
            || f.fiche().version != l.version
            || hex::encode(f.empreinte()) != l.empreinte
        {
            return Err(format!("{} v{} : ligne discordante", l.fiche, l.version));
        }
        if f.est_reprise() {
            if !l.fiche.ends_with(l.donnee.get(..32).unwrap_or("-")) {
                return Err(format!(
                    "{} v{} : ensemble de sources discordant",
                    l.fiche, l.version
                ));
            }
            crate::tamr::restaurer_republication(&mut service, l.octets, maintenant)
        } else {
            publieur
                .restaurer(&mut service, l.octets, l.publiee_a, maintenant)
                .map_err(|e| e.to_string())
        }
        .map_err(|e| format!("{} v{} : {e}", l.fiche, l.version))?;
    }
    Ok(service)
}

impl<S: SourceEtiquettes, M: MagasinPublications> EtatPublicationHttp<S, M> {
    /// Route active pour `actif` = (utilisateur, lecteur d'étiquettes, état
    /// durable, signataire des Fiches) et les clés de l'utilisateur ; sans
    /// l'un d'eux : désactivée. `instance` : identifiant lié dans l'usage signé
    /// (celui des routes mémoire). L'état durable est chargé par
    /// [`Self::precharger`] ou à la première proposition. Republication :
    /// [`Self::avec_cles_approbation`], [`Self::avec_vue_federale`].
    pub fn new(
        actif: Option<(String, S, M, SignataireFiches)>,
        cles_utilisateur: Vec<[u8; 32]>,
        instance: String,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let actif = actif.and_then(|(utilisateur, source, magasin, signataire)| {
            let Some(cog) = signataire.cog() else {
                tracing::error!(target: "maia::federee", "clé d'identité sans cog_id valide : publication désactivée");
                return None;
            };
            if !base_publications_admise(magasin.base()) {
                tracing::error!(target: "maia::federee", base = magasin.base(), "base de l'état de publication refusée (invalide ou nationale) : publication désactivée");
                return None;
            }
            Some(Actif {
                utilisateur,
                source,
                magasin,
                signataire,
                cog,
                publication: tokio::sync::Mutex::new(None),
            })
        });
        let duree_approbation = std::time::Duration::from_secs(DUREE_VIE_SECS);
        Self {
            actif,
            garde: GardeSignee::new(cles_utilisateur),
            garde_approbation: GardeSignee::new(Vec::new()),
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
            approbations: RegistreApprobations::avec_duree(duree_approbation),
            duree_approbation,
            republications: Mutex::new(std::collections::HashMap::new()),
            vue: None,
        }
    }

    /// Clés d'approbation TAMR (F7) : `cles` doivent être **distinctes** des
    /// clés de l'utilisateur (`cles_utilisateur`), sinon aucune approbation.
    #[must_use]
    pub fn avec_cles_approbation(
        mut self,
        cles: Vec<[u8; 32]>,
        cles_utilisateur: &[[u8; 32]],
    ) -> Self {
        let cles = if cles.iter().any(|c| cles_utilisateur.contains(c)) {
            tracing::error!(target: "maia::federee", "{ENV_CLES_APPROBATION} : clé aussi épinglée pour « {USAGE_FEDEREE} » — republications refusées");
            Vec::new()
        } else {
            cles
        };
        self.garde_approbation = GardeSignee::new(cles);
        self
    }

    /// Vue fédérale qui admet les émetteurs des Fiches republiées.
    #[must_use]
    pub fn avec_vue_federale(mut self, vue: VuePartagee) -> Self {
        self.vue = Some(vue);
        self
    }

    /// Durée de vie des demandes d'approbation (essais).
    #[cfg(test)]
    #[must_use]
    fn avec_duree_approbation(mut self, duree: std::time::Duration) -> Self {
        self.approbations = RegistreApprobations::avec_duree(duree);
        self.duree_approbation = duree;
        self
    }

    /// Vrai si la route est servie.
    pub fn est_active(&self) -> bool {
        self.actif.is_some() && self.garde.est_configuree()
    }

    /// Usage signé attendu des requêtes de publication de `utilisateur`.
    pub fn usage(&self, utilisateur: &str) -> String {
        format!("{USAGE_FEDEREE}/{}/{utilisateur}", self.instance)
    }

    /// Charge l'état durable au démarrage ; `false` si la route est inactive
    /// ou si KindMother est indisponible (la route répond alors 503 et
    /// réessaie à chaque proposition).
    pub async fn precharger(&self) -> bool {
        let Some(actif) = self.actif.as_ref() else {
            return false;
        };
        let mut garde = actif.publication.lock().await;
        match Self::charger(actif, &mut garde).await {
            Ok(()) => true,
            Err(r) => {
                tracing::error!(target: "maia::federee", motif = r.code, detail = %r.detail, "état de publication non chargé : route en 503 jusqu'au rechargement");
                false
            }
        }
    }

    /// Recharge l'état si nécessaire (`None`) ; échec ⇒ 503, l'état reste à recharger.
    async fn charger(actif: &Actif<S, M>, garde: &mut Option<ServiceRequete>) -> Result<(), Refus> {
        if garde.is_some() {
            return Ok(());
        }
        let publieur = publieur().ok_or_else(|| {
            Refus::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "autorite_indisponible",
                "autorité des étiquettes indisponible",
            )
        })?;
        let lignes = actif.magasin.charger().await.map_err(|e| {
            tracing::error!(target: "maia::federee", base = actif.magasin.base(), erreur = %e, "état de publication illisible");
            indisponible(format!("état de publication illisible : {e}"))
        })?;
        let n = lignes.len();
        let service = restaurer(publieur, &actif.cog, lignes, maintenant_unix()).map_err(|e| {
            tracing::error!(target: "maia::federee::audit", base = actif.magasin.base(), erreur = %e, "état de publication incohérent : publication suspendue");
            Refus::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "etat_publication_incoherent",
                e,
            )
        })?;
        tracing::info!(target: "maia::federee", base = actif.magasin.base(), versions = n, "état de publication chargé");
        *garde = Some(service);
        Ok(())
    }

    /// Dernière version publiée de `id` (0 : jamais ; état non chargé : 0).
    #[cfg(test)]
    async fn derniere_version(&self, id: &str) -> u64 {
        match self.actif.as_ref() {
            Some(a) => a
                .publication
                .lock()
                .await
                .as_ref()
                .map_or(0, |s| s.derniere_version(id)),
            None => 0,
        }
    }

    /// Copie du journal des décisions (plus récentes en dernier).
    pub fn journal(&self) -> Vec<EntreeJournal> {
        self.journal
            .lock()
            .map(|j| j.iter().cloned().collect())
            .unwrap_or_default()
    }

    fn tracer(&self, e: EntreeJournal) {
        if e.verdict == "refusee" {
            tracing::warn!(target: "maia::federee::audit", utilisateur = %e.utilisateur, motif = e.motif, detail = %e.detail, fiche = ?e.fiche, version = ?e.version, "publication refusée");
        } else {
            tracing::info!(target: "maia::federee::audit", utilisateur = %e.utilisateur, motif = e.motif, fiche = ?e.fiche, version = ?e.version, empreinte = ?e.empreinte, detail = %e.detail, "Fiche publiée ou demande d'approbation");
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

impl EtatPublicationHttp<EtiquettesKindMother, PublicationsKindMother> {
    /// Configuration par l'environnement (voir l'en-tête du module) ; tout
    /// élément absent ou invalide ⇒ route désactivée (journalisé).
    pub fn depuis_env(
        instance: String,
        gouvernance: Arc<GouvernanceMaia>,
        securite: SecurityConfig,
    ) -> Self {
        let cles = crate::routes_memoire::analyser_env(ENV_CLES_UTILISATEUR);
        let approbation = crate::routes_memoire::analyser_env(ENV_CLES_APPROBATION);
        let actif = actif_depuis_env();
        let etat = Self::new(actif, cles.clone(), instance, gouvernance, securite)
            .avec_cles_approbation(approbation, &cles);
        if etat.est_active() {
            tracing::info!("route de publication fédérée active (requêtes signées « {USAGE_FEDEREE}/<instance>/<utilisateur> »)");
            // Aucune vue fédérale n'est encore câblée dans le pont : republication en 503.
            tracing::info!(
                "republication fédérée désactivée (vue fédérale non câblée dans le pont)"
            );
        } else {
            tracing::info!("route de publication fédérée désactivée ({ENV_UTILISATEUR}, {ENV_CLES_UTILISATEUR}, {ENV_CLE_FICHES} + {ENV_DELEGATION_FICHES} + {ENV_IDENTITE_PUBLIQUE} (ou, transitoire, {ENV_CLE_IDENTITE}), {ENV_OPERATEUR} ou justificatif de l'état de publication absent)");
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

/// Lit le sceau de délégation : fichier régulier (lien symbolique refusé),
/// au plus [`MAX_DELEGATION`] octets.
fn lire_delegation(chemin: &Path) -> Result<Vec<u8>, String> {
    let meta = std::fs::symlink_metadata(chemin).map_err(|e| e.to_string())?;
    if !meta.file_type().is_file() {
        return Err("fichier de délégation non régulier".into());
    }
    if meta.len() == 0 || meta.len() > MAX_DELEGATION as u64 {
        return Err(format!(
            "taille de la délégation hors bornes (1..={MAX_DELEGATION} octets)"
        ));
    }
    std::fs::read(chemin).map_err(|e| e.to_string())
}

/// Signataire des Fiches (A-022 §4, A-024) : clé `fiches` + délégation `0x45`
/// et identité publique ; à défaut, **transitoire**, la clé d'identité
/// ([`ENV_CLE_IDENTITE`]). Clé `fiches` configurée mais invalide ⇒ aucune
/// (jamais de repli silencieux sur l'identité).
fn signataire_depuis_env(maintenant: u64) -> Option<SignataireFiches> {
    if let Some(chemin) = env_non_vide(ENV_CLE_FICHES) {
        if env_non_vide(ENV_CLE_IDENTITE).is_some() {
            tracing::warn!(target: "maia::federee", "{ENV_CLE_IDENTITE} ignorée : la clé « fiches » est configurée (A-022 §4)");
        }
        let resultat = (|| {
            let cle = miyauth::http::service::charger_cle(Path::new(&chemin))
                .map_err(|e| format!("{ENV_CLE_FICHES} ({chemin}) : {e}"))?;
            let delegation = env_non_vide(ENV_DELEGATION_FICHES)
                .ok_or_else(|| format!("{ENV_DELEGATION_FICHES} absent"))
                .and_then(|c| {
                    lire_delegation(Path::new(&c))
                        .map_err(|e| format!("{ENV_DELEGATION_FICHES} ({c}) : {e}"))
                })?;
            let identite = env_non_vide(ENV_IDENTITE_PUBLIQUE)
                .ok_or_else(|| format!("{ENV_IDENTITE_PUBLIQUE} absent"))
                .and_then(|t| {
                    miyauth::http::service::analyser_cles_publiques(&[t])
                        .ok()
                        .and_then(|v| v.first().copied())
                        .ok_or_else(|| format!("{ENV_IDENTITE_PUBLIQUE} mal formée"))
                })?;
            SignataireFiches::deleguee(cle, identite, delegation, maintenant)
                .map_err(|e| e.to_string())
        })();
        return match resultat {
            Ok(s) => {
                tracing::info!(target: "maia::federee", fin = ?s.fin_delegation(), "Fiches signées par la clé « fiches » déléguée (0x45)");
                Some(s)
            }
            Err(e) => {
                tracing::error!(target: "maia::federee", "clé « fiches » : {e} — publication désactivée");
                None
            }
        };
    }
    let chemin = env_non_vide(ENV_CLE_IDENTITE)?;
    match miyauth::http::service::charger_cle(Path::new(&chemin)) {
        Ok(c) => {
            tracing::warn!(target: "maia::federee", "{ENV_CLE_IDENTITE} : Fiches signées par la clé d'identité (TRANSITOIRE, A-022 §4) — configurer {ENV_CLE_FICHES}, {ENV_DELEGATION_FICHES}, {ENV_IDENTITE_PUBLIQUE}");
            Some(SignataireFiches::identite(c))
        }
        Err(e) => {
            tracing::error!(target: "maia::federee", "{ENV_CLE_IDENTITE} ({chemin}) : {e} — publication désactivée");
            None
        }
    }
}

fn actif_depuis_env() -> Option<(
    String,
    EtiquettesKindMother,
    PublicationsKindMother,
    SignataireFiches,
)> {
    let utilisateur = env_non_vide(ENV_UTILISATEUR)?;
    let operateur = env_non_vide(ENV_OPERATEUR)?;
    let signataire = signataire_depuis_env(maintenant_unix())?;
    let adresse = env_non_vide(ENV_KINDMOTHER).unwrap_or_else(|| KINDMOTHER_DEFAUT.to_string());
    let source = match EtiquettesKindMother::charger(&adresse, &operateur, None) {
        Ok(s) => s,
        Err(e) => {
            tracing::error!(target: "maia::federee", "justificatif KindMother « {operateur} » : {e} — publication désactivée");
            return None;
        }
    };
    let base =
        env_non_vide(ENV_PUBLICATIONS).unwrap_or_else(|| BASE_PUBLICATIONS_DEFAUT.to_string());
    match PublicationsKindMother::charger(&adresse, &base, None) {
        Ok(magasin) => Some((utilisateur, source, magasin, signataire)),
        Err(e) => {
            tracing::error!(target: "maia::federee", "état de publication « {base} » : {e} — publication désactivée");
            None
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// Routeur et garde
// ═══════════════════════════════════════════════════════════════════════════

/// Routeur de la publication (à fusionner avec le routeur principal).
pub fn routeur<S: SourceEtiquettes, M: MagasinPublications>(
    etat: Arc<EtatPublicationHttp<S, M>>,
) -> Router {
    Router::new()
        .route("/v1/federee/{utilisateur}/publier", post(publier::<S, M>))
        .route(
            "/v1/federee/{utilisateur}/republier",
            post(demander_republication::<S, M>),
        )
        .route(
            "/v1/federee/{utilisateur}/republier/{intervention}",
            post(approuver_republication::<S, M>).delete(refuser_republication::<S, M>),
        )
        .layer(axum::middleware::from_fn_with_state(
            Arc::clone(&etat),
            garde::<S, M>,
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

async fn garde<S: SourceEtiquettes, M: MagasinPublications>(
    State(etat): State<Arc<EtatPublicationHttp<S, M>>>,
    req: Request,
    next: Next,
) -> Response {
    let mut reponse = garder(&etat, req, next).await;
    if let Ok(v) = axum::http::HeaderValue::from_str(&etat.instance) {
        reponse.headers_mut().insert(ENTETE_INSTANCE, v);
    }
    reponse
}

/// `POST /v1/federee/{u}/republier/{iv}` : approbation TAMR (F7).
fn est_approbation(methode: &axum::http::Method, chemin: &str) -> bool {
    let segments: Vec<&str> = chemin.split('/').collect();
    methode == axum::http::Method::POST
        && matches!(segments.as_slice(), ["", "v1", "federee", u, "republier", iv] if !u.is_empty() && !iv.is_empty())
}

/// Débit par IP, route active, pas de cookie, origine, requête signée par une
/// clé de l'utilisateur avec l'usage de publication lié à l'instance — ou,
/// pour une approbation de republication, par une clé d'approbation TAMR avec
/// l'usage `maia/tamr-approbation/<instance>/<utilisateur>` (F7).
async fn garder<S: SourceEtiquettes, M: MagasinPublications>(
    etat: &EtatPublicationHttp<S, M>,
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
    let approbation = est_approbation(req.method(), req.uri().path());
    let (garde, usage) = if approbation {
        if !etat.garde_approbation.est_configuree() {
            return refus_http(StatusCode::SERVICE_UNAVAILABLE, "approbations_desactivees");
        }
        (
            &etat.garde_approbation,
            format!(
                "{USAGE_APPROBATION}/{}/{}",
                etat.instance, actif.utilisateur
            ),
        )
    } else {
        (&etat.garde, etat.usage(&actif.utilisateur))
    };
    let requise = if approbation {
        "signature_approbation_requise"
    } else {
        "signature_utilisateur_requise"
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
            tracing::warn!(target: "maia::federee", refus = ?r, approbation, "requête de publication refusée");
            return refus_http(StatusCode::UNAUTHORIZED, requise);
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
    if let Err(r) = garde.verifier(
        &entetes,
        &usage,
        parts.method.as_str(),
        chemin,
        &corps,
        maintenant,
    ) {
        tracing::warn!(target: "maia::federee", refus = ?r, approbation, "requête de publication refusée");
        return refus_http(StatusCode::UNAUTHORIZED, requise);
    }
    let mut parts = parts;
    parts
        .extensions
        .insert(UtilisateurAuthentifie(actif.utilisateur.clone()));
    if approbation {
        parts
            .extensions
            .insert(SignatureApprobation::verifiee(&entetes.signataire));
    }
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
        ErreurFederee::Delegation(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "delegation_fiches_invalide",
        ),
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
#[allow(clippy::too_many_lines)]
async fn traiter<S: SourceEtiquettes, M: MagasinPublications>(
    etat: &EtatPublicationHttp<S, M>,
    actif: &Actif<S, M>,
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
    if p.donnee.base.eq_ignore_ascii_case(actif.magasin.base()) {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "base_reservee",
            "base de l'état de publication : jamais publiable",
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
    let etat_km = actif.source.lire(&p.donnee).await.map_err(indisponible)?;
    let lue = etiquette_lue(&etat_km);
    if lue.nationale {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "donnee_nationale",
            "donnée nationale : jamais publiable",
        ));
    }
    let (Some(donnee_id), Some(parametres), true, Some(_), true) = (
        lue.empreinte_donnee,
        lue.parametres.clone(),
        lue.posee,
        lue.empreinte_contenu
            .filter(|e| lue.empreinte_actuelle == Some(*e)),
        lue.proprietaire.as_deref() == Some(u.as_str()),
    ) else {
        return Err(Refus::new(
            StatusCode::FORBIDDEN,
            "non_publiable",
            "donnée non étiquetée par cet utilisateur, étiquette retirée, contenu modifié depuis la pose ou pose sans paramètres",
        ));
    };

    // Section critique : un seul calcul de version / plafond / publication à la fois.
    let mut garde = actif.publication.lock().await;
    EtatPublicationHttp::<S, M>::charger(actif, &mut garde).await?;
    let Some(service) = garde.as_ref() else {
        return Err(indisponible("état de publication non chargé"));
    };

    // Champs structurels calculés par le serveur.
    let maintenant = maintenant_unix();
    let id = id_fiche_derivee(&donnee_id);
    let derniere = service.derniere_version(&id);
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
    // Plafond contrôlé ici, décompté seulement par la publication effective.
    if service.plafond_atteint(&id, maintenant) {
        return Err(refus_federee(&ErreurFederee::Quota).pour(&id, version));
    }

    let fiche = Fiche {
        id: id.clone(),
        emetteur: actif.signataire.emetteur(),
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
    let octets = actif
        .signataire
        .sceller(&fiche, maintenant)
        .map_err(|e| refus_federee(&e).pour(&id, version))?;
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

    // Attestation : étiquette relue, propriétaire revérifié, dérivation stricte.
    let capacite = publieur
        .attester(
            &Lecteur(&actif.source),
            &p.donnee,
            u,
            &octets,
            derniere,
            maintenant,
        )
        .await
        .map_err(|e| avec(refus_federee(&e)))?;
    let publication = Publication::Originale(capacite);
    // Contrôle sans effet (version, plafond, capacité) avant la porte.
    service
        .controler(&octets, &publication, maintenant)
        .map_err(|e| avec(refus_federee(&e)))?;

    // Porte des Cores : seule une décision ALLOW exécute l'effet (durable, puis mémoire).
    let intent = intention_publication(
        u,
        &fiche,
        &empreinte,
        &hex::encode(donnee_id),
        &p.donnee.base,
    );
    let durable = PublicationDurable {
        fiche: id.clone(),
        version,
        donnee: hex::encode(donnee_id),
        empreinte: empreinte.clone(),
        utilisateur: u.clone(),
        publiee_a: maintenant,
        octets: octets.clone(),
    };
    let magasin = &actif.magasin;
    let a_publier = octets.clone();
    let effet = etat.gouvernance.executer(&intent, |jeton| {
        let regle = jeton.regle().to_string();
        let garde = &mut garde;
        async move {
            if let Err(e) = magasin.enregistrer(&durable).await {
                // Issue inconnue : l'état sera relu de KindMother avant toute autre publication.
                **garde = None;
                tracing::error!(target: "maia::federee::audit", fiche = %durable.fiche, version = durable.version, erreur = %e, "version non enregistrée (issue inconnue) : état à recharger");
                return Err(Refus::new(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "persistance_indisponible",
                    "écriture de l'état de publication impossible",
                ));
            }
            let inscrite = match garde.as_mut() {
                Some(s) => s.publier(a_publier, publication, maintenant),
                None => Err(ErreurFederee::Interdit("état de publication non chargé")),
            };
            if let Err(e) = inscrite {
                // Durable mais pas en mémoire : relire KindMother (qui fait foi).
                **garde = None;
                tracing::error!(target: "maia::federee::audit", fiche = %durable.fiche, version = durable.version, erreur = %e, "version durable non inscrite en mémoire : état à recharger");
                return Err(refus_federee(&e));
            }
            Ok(regle)
        }
    });
    let resultat = match effet {
        Ok(f) => f.await,
        Err(motif) => {
            return Err(avec(Refus::new(
                StatusCode::FORBIDDEN,
                "refus_porte",
                motif,
            )))
        }
    };
    match resultat {
        Ok(regle) => Ok(Publiee {
            id: id.clone(),
            version,
            date: maintenant,
            empreinte: empreinte.clone(),
            octets,
            regle,
        }),
        Err(r) => Err(avec(r)),
    }
}

/// `POST /v1/federee/{u}/publier`.
async fn publier<S: SourceEtiquettes, M: MagasinPublications>(
    State(etat): State<Arc<EtatPublicationHttp<S, M>>>,
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

// ═══════════════════════════════════════════════════════════════════════════
// Republication d'une donnée étrangère (A-009, chemin TAMR, A7)
// ═══════════════════════════════════════════════════════════════════════════

/// Demande de republication : Fiches étrangères (octets signés) et les deux
/// seuls choix de l'utilisateur (sujet, audience) ; la licence annoncée n'est
/// que comparée. Tout autre champ (dont `donnee` : aucune donnée KindMother,
/// jamais `maia-*`) ⇒ 400.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct PropositionReprise {
    /// Fiches étrangères à republier : octets signés, base64 standard (1 à 16).
    fiches: Vec<String>,
    #[serde(default)]
    sujet: Option<String>,
    #[serde(default)]
    audience: Option<AudienceProposee>,
    #[serde(default)]
    licence: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CorpsApprobation {
    empreinte: String,
}

fn audience_choisie(a: &AudienceProposee) -> Result<Audience, Refus> {
    Ok(match a {
        AudienceProposee::Membres => Audience::Membres,
        AudienceProposee::Invites => Audience::Invites,
        AudienceProposee::Nommes(c) => Audience::Nommes(
            c.iter()
                .map(|x| CogId::new(x))
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| {
                    Refus::new(
                        StatusCode::BAD_REQUEST,
                        "corps_invalide",
                        "COG nommé invalide",
                    )
                })?,
        ),
    })
}

/// Refus d'une republication (dérivation, contrôle du service).
fn refus_reprise(e: &ErreurFederee) -> Refus {
    let (statut, code) = match e {
        ErreurFederee::ChampInvalide("licence différente de la source") => {
            (StatusCode::UNPROCESSABLE_ENTITY, "licence_differente")
        }
        ErreurFederee::ChampInvalide("audience plus large que la source") => {
            (StatusCode::FORBIDDEN, "audience_elargie")
        }
        ErreurFederee::ValidationRequise => (StatusCode::FORBIDDEN, "republication_tamr_requise"),
        ErreurFederee::VersionObsolete => (StatusCode::CONFLICT, "version_deja_publiee"),
        ErreurFederee::FicheNonLocale => (StatusCode::FORBIDDEN, "fiche_non_locale"),
        ErreurFederee::Delegation(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            "delegation_fiches_invalide",
        ),
        _ => (StatusCode::UNPROCESSABLE_ENTITY, "non_derivee"),
    };
    Refus::new(statut, code, e.to_string())
}

/// Refus du chemin TAMR.
fn refus_tamr(r: &RefusTamr) -> Refus {
    let (statut, code) = match r {
        RefusTamr::Inconnue => (StatusCode::NOT_FOUND, "approbation_inconnue"),
        RefusTamr::Expiree => (StatusCode::FORBIDDEN, "approbation_expiree"),
        RefusTamr::EmpreinteDifferente => (StatusCode::FORBIDDEN, "empreinte_differente"),
        RefusTamr::ApprobateurInvalide => (StatusCode::FORBIDDEN, "approbateur_invalide"),
        RefusTamr::SignatureRequise => (StatusCode::UNAUTHORIZED, "signature_approbation_requise"),
        RefusTamr::StrongFather(_) | RefusTamr::HorsPortee => (StatusCode::FORBIDDEN, "refus_tamr"),
        RefusTamr::RegistrePlein
        | RefusTamr::Indisponible
        | RefusTamr::AutoriteIndisponible
        | RefusTamr::Serialisation => (StatusCode::SERVICE_UNAVAILABLE, "tamr_indisponible"),
    };
    Refus::new(statut, code, r.to_string())
}

/// Fiches sources : décodées, vérifiées (chaîne de signature), étrangères au
/// COG local, émetteur admis **maintenant** par la vue fédérale.
fn verifier_sources(
    vue: &dyn VueFederale,
    cog_local: &CogId,
    brutes: &[Vec<u8>],
    maintenant: u64,
) -> Result<Vec<FicheVerifiee>, Refus> {
    if brutes.is_empty() || brutes.len() > MAX_REPRISES {
        return Err(Refus::new(
            StatusCode::BAD_REQUEST,
            "sources_invalides",
            format!("1 à {MAX_REPRISES} Fiches sources"),
        ));
    }
    brutes
        .iter()
        .map(|b| {
            let v = verifier_fiche(b, maintenant).map_err(|e| {
                Refus::new(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "source_invalide",
                    e.to_string(),
                )
            })?;
            if v.cog() == cog_local {
                return Err(Refus::new(
                    StatusCode::FORBIDDEN,
                    "source_locale",
                    "une Fiche du COG local n'est pas une donnée étrangère",
                ));
            }
            vue.qualifier(v.cog(), maintenant).map_err(|m| {
                Refus::new(
                    StatusCode::FORBIDDEN,
                    "source_non_admise",
                    format!("{} : {m}", v.cog().as_str()),
                )
            })?;
            Ok(v)
        })
        .collect()
}

/// Contexte de l'approbation TAMR porté par l'intention de publication.
struct ContexteTamr {
    intervention: String,
    regle: String,
    empreinte: String,
    cle: String,
}

/// Intention StrongFather d'une republication approuvée par TAMR.
fn intention_republication(
    utilisateur: &str,
    fiche: &Fiche,
    empreinte: &str,
    ensemble: &str,
    tamr: &ContexteTamr,
) -> Intent {
    let mut data = std::collections::HashMap::new();
    for (k, v) in [
        ("operation", OPERATION_REPUBLIER.to_string()),
        ("utilisateur", utilisateur.to_string()),
        ("canal", crate::tamr::CANAL_UTILISATEUR.to_string()),
        ("usage_signe", USAGE_FEDEREE.to_string()),
        ("provenance", "etranger".to_string()),
        ("sensibilite", "ordinaire".to_string()),
        ("tamr", TYPE_INTERVENTION.to_string()),
        ("intervention_id", tamr.intervention.clone()),
        ("regle_tamr", tamr.regle.clone()),
        ("empreinte_tamr", tamr.empreinte.clone()),
        ("cle_approbation", tamr.cle.clone()),
        ("reprises", fiche.reprises.len().to_string()),
        // Posés après `ServiceRequete::controler` (défense en profondeur).
        ("licence_identique", "true".to_string()),
        ("audience_incluse", "true".to_string()),
        ("derivee", "true".to_string()),
        ("fiche", fiche.id.clone()),
        ("version", fiche.version.to_string()),
        ("empreinte", empreinte.to_string()),
        ("donnee", ensemble.to_string()),
        ("type", (fiche.type_fiche as u8).to_string()),
        ("langue", fiche.langue.clone()),
        ("licence", fiche.licence.clone()),
        ("audience", nom_audience(&fiche.audience).to_string()),
    ] {
        data.insert(k.to_string(), v);
    }
    let mut i = Intent::new(
        format!(
            "maia-federee-reprise-{}",
            crate::garde_etranger::identifiant_aleatoire()
        ),
        ActionType::Creation,
        "maia.federee.republication".to_string(),
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

/// Demande acceptée : approbation TAMR à présenter à l'utilisateur.
struct DemandeReprise {
    demande: crate::tamr::DemandeApprobation,
    id: String,
    version: u64,
    date: u64,
    empreinte: String,
    reprises: usize,
    octets: Vec<u8>,
}

/// Étape 1 : Fiche republiée calculée, scellée, figée dans une demande TAMR.
async fn traiter_demande<S: SourceEtiquettes, M: MagasinPublications>(
    etat: &EtatPublicationHttp<S, M>,
    actif: &Actif<S, M>,
    corps: &[u8],
) -> Result<DemandeReprise, Refus> {
    let u = &actif.utilisateur;
    if etat
        .limiteur_utilisateur
        .check(&format!("republier:{u}"))
        .is_err()
    {
        return Err(Refus::new(
            StatusCode::TOO_MANY_REQUESTS,
            "debit_utilisateur_depasse",
            format!("{PUBLICATIONS_PAR_HEURE} demandes de republication par heure au plus"),
        ));
    }
    let p: PropositionReprise = serde_json::from_slice(corps)
        .map_err(|e| Refus::new(StatusCode::BAD_REQUEST, "corps_invalide", e.to_string()))?;
    let vue = etat.vue.as_ref().ok_or_else(|| {
        Refus::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "vue_federale_indisponible",
            "aucune vue fédérale : émetteurs des sources non vérifiables",
        )
    })?;
    let brutes = p
        .fiches
        .iter()
        .map(|b| base64::engine::general_purpose::STANDARD.decode(b))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| Refus::new(StatusCode::BAD_REQUEST, "corps_invalide", e.to_string()))?;
    let maintenant = maintenant_unix();
    let sources = verifier_sources(vue.as_ref(), &actif.cog, &brutes, maintenant)?;
    if let Some(l) = &p.licence {
        if sources.iter().any(|s| &s.fiche().licence != l) {
            return Err(refus_reprise(&ErreurFederee::ChampInvalide(
                "licence différente de la source",
            )));
        }
    }
    let choix = ChoixReprise {
        sujet: p.sujet.clone(),
        audience: p.audience.as_ref().map(audience_choisie).transpose()?,
    };
    let ensemble = empreinte_ensemble(&sources);
    let id = id_fiche_reprise(&ensemble);

    // Section critique : version calculée sur l'état durable.
    let mut garde = actif.publication.lock().await;
    EtatPublicationHttp::<S, M>::charger(actif, &mut garde).await?;
    let Some(service) = garde.as_ref() else {
        return Err(indisponible("état de publication non chargé"));
    };
    let version = service
        .derniere_version(&id)
        .checked_add(1)
        .ok_or_else(|| {
            Refus::new(
                StatusCode::CONFLICT,
                "version_deja_publiee",
                "version maximale",
            )
        })?;
    let fiche = deriver_reprise(
        &sources,
        actif.signataire.emetteur(),
        &choix,
        version,
        maintenant,
    )
    .map_err(|e| refus_reprise(&e).pour(&id, version))?;
    let octets = actif
        .signataire
        .sceller(&fiche, maintenant)
        .map_err(|e| refus_reprise(&e).pour(&id, version))?;
    let v =
        verifier_fiche(&octets, maintenant).map_err(|e| refus_reprise(&e).pour(&id, version))?;
    verifier_derivation_reprise(&v, &sources).map_err(|e| refus_reprise(&e).pour(&id, version))?;
    drop(garde);

    // Action TAMR figée : octets exacts de la Fiche à publier.
    let empreinte = hex::encode(v.empreinte());
    let mut attente = etat
        .republications
        .lock()
        .map_err(|_| refus_tamr(&RefusTamr::Indisponible))?;
    let duree = etat.duree_approbation;
    attente.retain(|_, a| a.demandee.elapsed() <= duree);
    if attente.len() >= CAPACITE_MAX {
        return Err(refus_tamr(&RefusTamr::RegistrePlein).pour(&id, version));
    }
    let demande = etat
        .approbations
        .demander(ActionTamr::Republier {
            proprietaire: u.clone(),
            octets: octets.clone(),
        })
        .map_err(|r| refus_tamr(&r).pour(&id, version))?;
    attente.insert(
        demande.intervention_id.clone(),
        RepublicationEnAttente {
            utilisateur: u.clone(),
            octets: octets.clone(),
            sources: brutes,
            id: id.clone(),
            version,
            ensemble: hex::encode(ensemble),
            empreinte: empreinte.clone(),
            demandee: std::time::Instant::now(),
        },
    );
    Ok(DemandeReprise {
        demande,
        id,
        version,
        date: maintenant,
        empreinte,
        reprises: fiche.reprises.len(),
        octets,
    })
}

/// Étape 2 : approbation TAMR (StrongFather `tamr.approbation`), sources
/// revérifiées, contrôle du service, porte `federee.republier`, puis
/// écriture durable et inscription.
async fn traiter_approbation<S: SourceEtiquettes, M: MagasinPublications>(
    etat: &EtatPublicationHttp<S, M>,
    actif: &Actif<S, M>,
    signature: Option<SignatureApprobation>,
    intervention: &str,
    empreinte_vue: &str,
) -> Result<Publiee, Refus> {
    let u = &actif.utilisateur;
    let signature = signature.ok_or_else(|| refus_tamr(&RefusTamr::SignatureRequise))?;
    // Usage unique : l'attente est retirée quelle que soit l'issue.
    let attente = etat
        .republications
        .lock()
        .map_err(|_| refus_tamr(&RefusTamr::Indisponible))?
        .remove(intervention);
    let ok = etat
        .approbations
        .approuver(
            intervention,
            u,
            empreinte_vue,
            &signature,
            &etat.gouvernance,
        )
        .map_err(|r| refus_tamr(&r))?;
    let Some(attente) = attente.filter(|a| &a.utilisateur == u) else {
        return Err(refus_tamr(&RefusTamr::HorsPortee));
    };
    let (id, version) = (attente.id.clone(), attente.version);
    let avec = |r: Refus| Refus {
        empreinte: Some(attente.empreinte.clone()),
        ..r.pour(&id, version)
    };
    let tamr = ContexteTamr {
        intervention: ok.intervention().to_string(),
        regle: ok.regle().to_string(),
        empreinte: ok.empreinte().to_string(),
        cle: signature.cle().to_string(),
    };
    let validation = ok
        .validation_republication(&attente.octets)
        .map_err(|r| avec(refus_tamr(&r)))?;
    let maintenant = maintenant_unix();
    let vue = etat.vue.as_ref().ok_or_else(|| {
        avec(Refus::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "vue_federale_indisponible",
            "aucune vue fédérale",
        ))
    })?;
    let sources =
        verifier_sources(vue.as_ref(), &actif.cog, &attente.sources, maintenant).map_err(avec)?;

    let mut garde = actif.publication.lock().await;
    EtatPublicationHttp::<S, M>::charger(actif, &mut garde)
        .await
        .map_err(avec)?;
    let Some(service) = garde.as_ref() else {
        return Err(avec(indisponible("état de publication non chargé")));
    };
    // Contrôle sans effet : TAMR (octets exacts), reprises, licence, audience, dérivation, version.
    let v = service
        .controler(
            &attente.octets,
            &Publication::Reprise {
                sources: &sources,
                validation: &validation,
            },
            maintenant,
        )
        .map_err(|e| avec(refus_reprise(&e)))?;
    let intent =
        intention_republication(u, v.fiche(), &attente.empreinte, &attente.ensemble, &tamr);
    let durable = PublicationDurable {
        fiche: id.clone(),
        version,
        donnee: attente.ensemble.clone(),
        empreinte: attente.empreinte.clone(),
        utilisateur: u.clone(),
        publiee_a: maintenant,
        octets: attente.octets.clone(),
    };
    let magasin = &actif.magasin;
    let a_publier = attente.octets.clone();
    let (sources_ref, validation_ref) = (&sources, &validation);
    let effet = etat.gouvernance.executer(&intent, |jeton| {
        let regle = jeton.regle().to_string();
        let garde = &mut garde;
        async move {
            if let Err(e) = magasin.enregistrer(&durable).await {
                **garde = None;
                tracing::error!(target: "maia::federee::audit", fiche = %durable.fiche, version = durable.version, erreur = %e, "republication non enregistrée (issue inconnue) : état à recharger");
                return Err(Refus::new(
                    StatusCode::SERVICE_UNAVAILABLE,
                    "persistance_indisponible",
                    "écriture de l'état de publication impossible",
                ));
            }
            let publication = Publication::Reprise {
                sources: sources_ref,
                validation: validation_ref,
            };
            let inscrite = match garde.as_mut() {
                Some(s) => s.publier(a_publier, publication, maintenant),
                None => Err(ErreurFederee::Interdit("état de publication non chargé")),
            };
            if let Err(e) = inscrite {
                **garde = None;
                tracing::error!(target: "maia::federee::audit", fiche = %durable.fiche, version = durable.version, erreur = %e, "republication durable non inscrite en mémoire : état à recharger");
                return Err(refus_reprise(&e));
            }
            Ok(regle)
        }
    });
    let resultat = match effet {
        Ok(f) => f.await,
        Err(motif) => {
            return Err(avec(Refus::new(
                StatusCode::FORBIDDEN,
                "refus_porte",
                motif,
            )))
        }
    };
    match resultat {
        Ok(regle) => Ok(Publiee {
            id: id.clone(),
            version,
            date: v.fiche().date,
            empreinte: attente.empreinte.clone(),
            octets: attente.octets.clone(),
            regle: format!(
                "{regle} (TAMR : {}, intervention {})",
                tamr.regle, tamr.intervention
            ),
        }),
        Err(r) => Err(avec(r)),
    }
}

impl<S: SourceEtiquettes, M: MagasinPublications> EtatPublicationHttp<S, M> {
    /// Trace un refus et rend la réponse JSON correspondante.
    fn repondre_refus(&self, instant: u64, utilisateur: &str, r: Refus) -> Response {
        self.tracer(EntreeJournal {
            instant,
            utilisateur: utilisateur.to_string(),
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

    /// Actif si `u` est l'utilisateur de l'instance et l'authentifié ; sinon refus tracé.
    fn actif_pour(
        &self,
        auth: &UtilisateurAuthentifie,
        u: &str,
    ) -> Result<&Actif<S, M>, Box<Response>> {
        let Some(actif) = self.actif.as_ref() else {
            return Err(Box::new(refus_http(
                StatusCode::SERVICE_UNAVAILABLE,
                "publication_desactivee",
            )));
        };
        if u != actif.utilisateur || auth.0 != u {
            return Err(Box::new(self.repondre_refus(
                maintenant_unix(),
                &auth.0,
                Refus::new(
                    StatusCode::FORBIDDEN,
                    "autre_utilisateur",
                    format!("chemin pour {u}"),
                ),
            )));
        }
        Ok(actif)
    }
}

/// `POST /v1/federee/{u}/republier` : demande ⇒ 202 et approbation TAMR à donner.
async fn demander_republication<S: SourceEtiquettes, M: MagasinPublications>(
    State(etat): State<Arc<EtatPublicationHttp<S, M>>>,
    Extension(auth): Extension<UtilisateurAuthentifie>,
    Chemin(u): Chemin<String>,
    corps: Bytes,
) -> Response {
    let actif = match etat.actif_pour(&auth, &u) {
        Ok(a) => a,
        Err(r) => return *r,
    };
    let instant = maintenant_unix();
    match traiter_demande(&etat, actif, &corps).await {
        Ok(d) => {
            etat.tracer(EntreeJournal {
                instant,
                utilisateur: u,
                verdict: "en_attente",
                motif: "approbation_tamr_demandee",
                detail: format!("intervention {}", d.demande.intervention_id),
                fiche: Some(d.id.clone()),
                version: Some(d.version),
                empreinte: Some(d.empreinte.clone()),
            });
            (
                StatusCode::ACCEPTED,
                Json(serde_json::json!({
                    "approbation": d.demande,
                    "fiche": {
                        "id": d.id,
                        "version": d.version,
                        "date": d.date,
                        "empreinte": d.empreinte,
                        "reprises": d.reprises,
                        "fiche": base64::engine::general_purpose::STANDARD.encode(&d.octets),
                    }
                })),
            )
                .into_response()
        }
        Err(r) => etat.repondre_refus(instant, &u, r),
    }
}

/// `POST /v1/federee/{u}/republier/{iv}` : approbation TAMR (`{"empreinte"}`) ⇒ publication.
async fn approuver_republication<S: SourceEtiquettes, M: MagasinPublications>(
    State(etat): State<Arc<EtatPublicationHttp<S, M>>>,
    Extension(auth): Extension<UtilisateurAuthentifie>,
    signature: Option<Extension<SignatureApprobation>>,
    Chemin((u, intervention)): Chemin<(String, String)>,
    corps: Bytes,
) -> Response {
    let actif = match etat.actif_pour(&auth, &u) {
        Ok(a) => a,
        Err(r) => return *r,
    };
    let instant = maintenant_unix();
    let c: CorpsApprobation = match serde_json::from_slice(&corps) {
        Ok(c) => c,
        Err(e) => {
            return etat.repondre_refus(
                instant,
                &u,
                Refus::new(StatusCode::BAD_REQUEST, "corps_invalide", e.to_string()),
            )
        }
    };
    match traiter_approbation(
        &etat,
        actif,
        signature.map(|e| e.0),
        &intervention,
        &c.empreinte,
    )
    .await
    {
        Ok(p) => {
            etat.tracer(EntreeJournal {
                instant,
                utilisateur: u,
                verdict: "publiee",
                motif: "republiee",
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
                },
                "tamr": "approuve",
            }))
            .into_response()
        }
        Err(r) => etat.repondre_refus(instant, &u, r),
    }
}

/// `DELETE /v1/federee/{u}/republier/{iv}` : refus de l'utilisateur (clé de l'utilisateur).
async fn refuser_republication<S: SourceEtiquettes, M: MagasinPublications>(
    State(etat): State<Arc<EtatPublicationHttp<S, M>>>,
    Extension(auth): Extension<UtilisateurAuthentifie>,
    Chemin((u, intervention)): Chemin<(String, String)>,
) -> Response {
    if let Err(r) = etat.actif_pour(&auth, &u) {
        return *r;
    }
    if let Ok(mut a) = etat.republications.lock() {
        a.remove(&intervention);
    }
    match etat.approbations.refuser(&intervention, &auth.0) {
        Ok(()) => Json(serde_json::json!({ "tamr": "refuse" })).into_response(),
        Err(r) => etat.repondre_refus(maintenant_unix(), &u, refus_tamr(&r)),
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
    use maia_federee::cog_de_cle;
    use miyauth::http::service::entetes_signees;
    use miyusceau::DelegationCle;
    use sha2::{Digest, Sha256};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use tower::ServiceExt;

    const TEXTE: &str = "Paris est la capitale de la France.";
    const INSTANCE: &str = "instance-essai";
    const CHEMIN: &str = "/v1/federee/alice/publier";
    /// Opérateur propriétaire de la base de l'état de publication dans le banc :
    /// base non nationale, absente des `.bases` des utilisateurs (en production :
    /// `cog-federee`, à déclarer côté KindMother).
    const OPERATEUR_PUBLICATIONS: &str = "jayrdv";

    /// Clé HTTP de l'utilisateur (requêtes signées).
    fn alice_http() -> CleSignature {
        CleSignature::depuis_graine(&[7_u8; 32])
    }

    /// Clé d'approbation TAMR de l'utilisateur (distincte de sa clé HTTP, F7).
    fn alice_approbation() -> CleSignature {
        CleSignature::depuis_graine(&[8_u8; 32])
    }

    fn usage_approbation() -> String {
        format!("{USAGE_APPROBATION}/{INSTANCE}/alice")
    }

    /// Signataire des Fiches du COG d'identité `graine` : clé `fiches`
    /// dérivée de la graine, délégation `0x45` de l'identité (A-024).
    fn signataire(graine: &[u8; 32]) -> SignataireFiches {
        let identite = CleSignature::depuis_graine(graine);
        let g: [u8; 32] = Sha256::digest([graine.as_slice(), b"fiches"].concat()).into();
        let fiches = CleSignature::depuis_graine(&g);
        let t = maintenant_unix();
        let cog = cog_de_cle(&identite.cle_publique()).expect("cog");
        let d = DelegationCle::nouvelle(
            cog.as_str(),
            maia_federee::INDICE_FICHES,
            0,
            fiches.cle_publique(),
            t - 86_400,
            t + 30 * 86_400,
        )
        .expect("délégation")
        .sceller(&identite)
        .expect("sceau");
        SignataireFiches::deleguee(fiches, identite.cle_publique(), d, t).expect("signataire")
    }

    /// Vue fédérale d'essai : tout COG est membre, sauf ceux de `refuses`.
    #[derive(Default)]
    struct VueEssai {
        refuses: Vec<CogId>,
    }

    impl VueFederale for VueEssai {
        fn qualifier(&self, cog: &CogId, _: u64) -> Result<maia_federee::Qualite, String> {
            if self.refuses.contains(cog) {
                Err("statut Revoque".into())
            } else {
                Ok(maia_federee::Qualite::Membre)
            }
        }
    }

    /// Graine d'un COG étranger révoqué (refusé par la vue d'essai).
    const GRAINE_REVOQUEE: u8 = 66;

    fn vue_essai() -> VuePartagee {
        Arc::new(VueEssai {
            refuses: vec![cog_de_cle(
                &CleSignature::depuis_graine(&[GRAINE_REVOQUEE; 32]).cle_publique(),
            )
            .expect("cog")],
        })
    }

    /// Clé d'étiquette de l'utilisateur (intentions KindMother).
    fn alice_etiquette() -> SigningKey {
        SigningKey::from_bytes(&[11; 32])
    }

    /// Clé d'étiquette d'un autre utilisateur (propriétaire de la même base).
    fn bob_etiquette() -> SigningKey {
        SigningKey::from_bytes(&[13; 32])
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

    fn graine_aleatoire() -> [u8; 32] {
        Sha256::digest(crate::garde_etranger::identifiant_aleatoire().as_bytes()).into()
    }

    /// KindMother réel (serveur TCP), opérateur `jaykonta`, table `notes` ;
    /// état de publication dans la base `jayrdv`.
    struct Banc<
        S: SourceEtiquettes = EtiquettesKindMother,
        M: MagasinPublications = PublicationsKindMother,
    > {
        dir: PathBuf,
        addr: String,
        cle_serveur: [u8; 32],
        graine: [u8; 32],
        km: KindMotherClient,
        etat: Arc<EtatPublicationHttp<S, M>>,
        app: Router,
    }

    impl<S: SourceEtiquettes, M: MagasinPublications> Drop for Banc<S, M> {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    fn cred(dir: &Path) -> PathBuf {
        dir.join("credentials")
    }

    fn source(addr: &str, dir: &Path) -> EtiquettesKindMother {
        EtiquettesKindMother::charger(addr, "jaykonta", Some(&cred(dir))).expect("source")
    }

    fn magasin(addr: &str, dir: &Path) -> PublicationsKindMother {
        PublicationsKindMother::charger(addr, OPERATEUR_PUBLICATIONS, Some(&cred(dir)))
            .expect("magasin")
    }

    fn etat_de<S: SourceEtiquettes, M: MagasinPublications>(
        gouvernance: GouvernanceMaia,
        actif: Option<(String, S, M, SignataireFiches)>,
    ) -> (Arc<EtatPublicationHttp<S, M>>, Router) {
        etat_regle(gouvernance, actif, |e| e)
    }

    /// État de la route (clés d'approbation d'Alice, vue d'essai), ajusté par `f`.
    fn etat_regle<S: SourceEtiquettes, M: MagasinPublications>(
        gouvernance: GouvernanceMaia,
        actif: Option<(String, S, M, SignataireFiches)>,
        f: impl FnOnce(EtatPublicationHttp<S, M>) -> EtatPublicationHttp<S, M>,
    ) -> (Arc<EtatPublicationHttp<S, M>>, Router) {
        let etat = Arc::new(f(EtatPublicationHttp::new(
            actif,
            vec![alice_http().cle_publique()],
            INSTANCE.to_string(),
            Arc::new(gouvernance),
            SecurityConfig::default(),
        )
        .avec_cles_approbation(
            vec![alice_approbation().cle_publique()],
            &[alice_http().cle_publique()],
        )
        .avec_vue_federale(vue_essai())));
        let app = routeur(Arc::clone(&etat));
        (etat, app)
    }

    async fn banc_complet<S: SourceEtiquettes, M: MagasinPublications>(
        gouvernance: GouvernanceMaia,
        avec_identite: bool,
        f_source: impl FnOnce(EtiquettesKindMother) -> S,
        f_magasin: impl FnOnce(PublicationsKindMother) -> M,
    ) -> Banc<S, M> {
        let dir = std::env::temp_dir().join(format!(
            "maia-federee-{}",
            crate::garde_etranger::identifiant_aleatoire()
        ));
        let utilisateurs = dir.join("credentials").join("utilisateurs");
        std::fs::create_dir_all(&utilisateurs).expect("dossier");
        for (nom, cle) in [("alice", alice_etiquette()), ("bob", bob_etiquette())] {
            std::fs::write(
                utilisateurs.join(format!("{nom}.pub")),
                format!("{}\n", encode_hex(cle.verifying_key().as_bytes())),
            )
            .expect("clé");
        }
        std::fs::write(utilisateurs.join("alice.bases"), "jaykonta\nmaia-alice\n").expect("bases");
        std::fs::write(utilisateurs.join("bob.bases"), "jaykonta\n").expect("bases");
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
        let km = KindMotherClient::connect(
            &addr,
            &OperatorCredential::load_from_dir("jaykonta", cred(&dir)).expect("cred"),
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
        let graine = graine_aleatoire();
        let actif = avec_identite.then(|| {
            (
                "alice".to_string(),
                f_source(source(&addr, &dir)),
                f_magasin(magasin(&addr, &dir)),
                signataire(&graine),
            )
        });
        let (etat, app) = etat_de(gouvernance, actif);
        Banc {
            dir,
            addr,
            cle_serveur,
            graine,
            km,
            etat,
            app,
        }
    }

    async fn banc_avec(gouvernance: GouvernanceMaia, avec_identite: bool) -> Banc {
        banc_complet(gouvernance, avec_identite, |s| s, |m| m).await
    }

    async fn banc() -> Banc {
        banc_avec(crate::gouvernance::tests::gouvernance_de_base(), true).await
    }

    /// Requête `methode` signée par `cle` pour `usage` (ou non signée).
    async fn envoyer_avec(
        app: Router,
        methode: &str,
        chemin: &str,
        corps: &str,
        signature: Option<(&CleSignature, &str)>,
    ) -> (StatusCode, serde_json::Value) {
        let mut b = axum::http::Request::builder()
            .method(methode)
            .uri(chemin)
            .header(header::CONTENT_TYPE, "application/json");
        if let Some((cle, usage)) = signature {
            for (n, v) in entetes_signees(cle, usage, methode, chemin, corps.as_bytes()) {
                b = b.header(n, v);
            }
        }
        let mut r = b.body(Body::from(corps.to_string())).expect("requête");
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40003))));
        let rep = app.oneshot(r).await.expect("réponse");
        let s = rep.status();
        let o = axum::body::to_bytes(rep.into_body(), 1 << 20)
            .await
            .expect("corps");
        (
            s,
            serde_json::from_slice(&o).unwrap_or(serde_json::Value::Null),
        )
    }

    /// Requête signée par la clé HTTP d'Alice (ou non signée).
    async fn envoyer(
        app: Router,
        chemin: &str,
        corps: &str,
        signe: bool,
    ) -> (StatusCode, serde_json::Value) {
        let mut b = axum::http::Request::builder()
            .method("POST")
            .uri(chemin)
            .header(header::CONTENT_TYPE, "application/json");
        if signe {
            for (n, v) in entetes_signees(&alice_http(), &usage(), "POST", chemin, corps.as_bytes())
            {
                b = b.header(n, v);
            }
        }
        let mut r = b.body(Body::from(corps.to_string())).expect("requête");
        r.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 40002))));
        let rep = app.oneshot(r).await.expect("réponse");
        let s = rep.status();
        let o = axum::body::to_bytes(rep.into_body(), 1 << 20)
            .await
            .expect("corps");
        (
            s,
            serde_json::from_slice(&o).unwrap_or(serde_json::Value::Null),
        )
    }

    impl<S: SourceEtiquettes, M: MagasinPublications> Banc<S, M> {
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

        async fn etiqueter_par(
            &self,
            utilisateur: &str,
            cle: &SigningKey,
            d: &RefDonnee,
            action: ActionEtiquette,
            texte: &str,
            t: u64,
        ) {
            let parametres = (action == ActionEtiquette::Poser).then(|| ParametresPublication {
                type_fiche: TypePublication::Fait,
                langue: "fr".into(),
                licence: "CC-BY-4.0".into(),
                audience: AudiencePublication::Membres,
            });
            let mut i = IntentionEtiquette {
                version: VERSION_INTENTION,
                parametres,
                action,
                donnee: d.clone(),
                utilisateur: utilisateur.into(),
                empreinte_contenu: (action == ActionEtiquette::Poser).then(|| empreinte(texte)),
                horodatage: t,
                cle_publique: encode_hex(cle.verifying_key().as_bytes()),
                signature: "00".repeat(64),
            };
            let m = i.message_signe(&self.cle_serveur).expect("message");
            i.signature = encode_hex(&cle.sign(&m).to_bytes());
            self.km.etiqueter(i).await.expect("étiquette");
        }

        async fn etiqueter(&self, d: &RefDonnee, action: ActionEtiquette, texte: &str, t: u64) {
            self.etiqueter_par("alice", &alice_etiquette(), d, action, texte, t)
                .await;
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
            envoyer(self.app.clone(), chemin, corps, signe).await
        }

        async fn proposer(&self, p: serde_json::Value) -> (StatusCode, serde_json::Value) {
            self.appel(CHEMIN, &p.to_string(), true).await
        }

        fn dernier_motif(&self) -> &'static str {
            self.etat.journal().last().map_or("", |e| e.motif)
        }

        /// « Redémarrage » : nouvel état (mémoire vide), même KindMother, même
        /// clé d'identité du COG.
        fn redemarrer(
            &self,
            gouvernance: GouvernanceMaia,
        ) -> (
            Arc<EtatPublicationHttp<EtiquettesKindMother, PublicationsKindMother>>,
            Router,
        ) {
            etat_de(
                gouvernance,
                Some((
                    "alice".to_string(),
                    source(&self.addr, &self.dir),
                    magasin(&self.addr, &self.dir),
                    signataire(&self.graine),
                )),
            )
        }

        /// Versions enregistrées dans KindMother (lecture directe de la base).
        async fn lignes(&self) -> Vec<PublicationDurable> {
            magasin(&self.addr, &self.dir)
                .charger()
                .await
                .expect("lignes")
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
        // A-022 §4 : signée par la clé `fiches` déléguée, émetteur = identité du COG.
        assert!(matches!(
            f.signature(),
            maia_federee::SignatureFiche::Deleguee { generation: 0, .. }
        ));
        assert!(maia_federee::verifier_fiche_deleguee(&octets, maintenant_unix()).is_ok());
        assert_eq!(
            f.fiche().emetteur,
            CleSignature::depuis_graine(&b.graine).cle_publique()
        );
        assert_eq!(f.fiche().id, id);
        assert_eq!(f.fiche().texte, TEXTE);
        assert_eq!(f.fiche().licence, "CC-BY-4.0");
        assert_eq!(f.fiche().audience, Audience::Membres);
        assert!(f.fiche().date.abs_diff(maintenant_unix()) <= 5);
        assert_eq!(
            hex::encode(f.empreinte()),
            v["publiee"]["empreinte"].as_str().expect("e")
        );
        assert_eq!(b.etat.derniere_version(&id).await, 1);
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
        assert_eq!(b.etat.derniere_version(&id).await, 1);
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
                            "alice",
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
                "alice",
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
            assert_eq!(
                b.etat.derniere_version(&id_attendu(x)).await,
                0,
                "rien de publié"
            );
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
            b.etat.derniere_version(&id_attendu(&d)).await,
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
        assert_eq!(b.etat.derniere_version(&id_attendu(&d)).await, 0);
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
            let b = banc_avec(g, true).await;
            let d = b.donnee(TEXTE).await;
            b.poser(&d).await;
            let (s, v) = b.proposer(proposition(&d)).await;
            assert_eq!(
                (s, v["reason"].as_str()),
                (StatusCode::FORBIDDEN, Some("refus_porte")),
                "{v}"
            );
            assert_eq!(
                b.etat.derniere_version(&id_attendu(&d)).await,
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
        assert_eq!(b.etat.derniere_version(&id_attendu(&d)).await, 0);

        // Sans clé d'identité du COG : désactivée.
        let off = banc_avec(crate::gouvernance::tests::gouvernance_de_base(), false).await;
        assert!(!off.etat.est_active());
        assert_eq!(
            off.appel(chemin, &corps, true).await.0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }

    // ── A-017 : persistance, monotonie, plafond après la porte, propriétaire ──

    /// État de publication enregistré : `n` versions, contiguës, octets vérifiables.
    async fn verifier_lignes<S: SourceEtiquettes, M: MagasinPublications>(
        b: &Banc<S, M>,
        id: &str,
        n: u64,
    ) {
        let l: Vec<_> = b
            .lignes()
            .await
            .into_iter()
            .filter(|l| l.fiche == id)
            .collect();
        assert_eq!(
            l.iter().map(|l| l.version).collect::<Vec<_>>(),
            (1..=n).collect::<Vec<_>>()
        );
        for x in &l {
            let f = verifier_fiche(&x.octets, maintenant_unix()).expect("fiche");
            assert_eq!(
                (f.fiche().version, hex::encode(f.empreinte())),
                (x.version, x.empreinte.clone())
            );
            assert_eq!(x.utilisateur, "alice");
        }
    }

    #[tokio::test]
    async fn redemarrage_recharge_versions_et_plafond() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let id = id_attendu(&d);
        for v in 1..=2_u64 {
            let (s, r) = b.proposer(proposition(&d)).await;
            assert_eq!(
                (s, r["publiee"]["version"].as_u64()),
                (StatusCode::OK, Some(v)),
                "{r}"
            );
        }
        verifier_lignes(&b, &id, 2).await;

        // Redémarrage : mémoire vide, état relu de KindMother.
        let (etat2, app2) = b.redemarrer(crate::gouvernance::tests::gouvernance_de_base());
        assert!(etat2.precharger().await);
        assert_eq!(etat2.derniere_version(&id).await, 2);
        for deja in [1_u64, 2] {
            let (s, r) = envoyer(
                app2.clone(),
                CHEMIN,
                &avec(proposition(&d), "version", deja.into()).to_string(),
                true,
            )
            .await;
            assert_eq!(
                (s, r["reason"].as_str()),
                (StatusCode::CONFLICT, Some("version_deja_publiee")),
                "{r}"
            );
        }
        let (s, r) = envoyer(app2.clone(), CHEMIN, &proposition(&d).to_string(), true).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(3)),
            "{r}"
        );
        for v in 4..=maia_federee::MAX_ATTESTATIONS_PAR_PERIODE as u64 {
            let (s, r) = envoyer(app2.clone(), CHEMIN, &proposition(&d).to_string(), true).await;
            assert_eq!(
                (s, r["publiee"]["version"].as_u64()),
                (StatusCode::OK, Some(v)),
                "{r}"
            );
        }
        verifier_lignes(&b, &id, maia_federee::MAX_ATTESTATIONS_PAR_PERIODE as u64).await;

        // Second redémarrage, sans préchargement : le plafond glissant est rechargé lui aussi.
        let (etat3, app3) = b.redemarrer(crate::gouvernance::tests::gouvernance_de_base());
        let (s, r) = envoyer(app3, CHEMIN, &proposition(&d).to_string(), true).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::TOO_MANY_REQUESTS, Some("plafond_attestations")),
            "{r}"
        );
        assert_eq!(
            etat3.derniere_version(&id).await,
            maia_federee::MAX_ATTESTATIONS_PAR_PERIODE as u64
        );
    }

    #[tokio::test]
    async fn kindmother_indisponible_503_jamais_version_1() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        assert_eq!(b.proposer(proposition(&d)).await.0, StatusCode::OK);
        // État de publication sur une adresse morte (étiquettes lisibles).
        let mort = {
            let l = std::net::TcpListener::bind("127.0.0.1:0").expect("bind");
            l.local_addr().expect("addr").to_string()
        };
        let (etat, app) = etat_de(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                PublicationsKindMother::charger(&mort, OPERATEUR_PUBLICATIONS, Some(&cred(&b.dir)))
                    .expect("magasin"),
                signataire(&b.graine),
            )),
        );
        assert!(!etat.precharger().await);
        for _ in 0..2 {
            let (s, r) = envoyer(app.clone(), CHEMIN, &proposition(&d).to_string(), true).await;
            assert_eq!(
                (s, r["reason"].as_str()),
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Some("kindmother_indisponible")
                ),
                "{r}"
            );
        }
        assert_eq!(
            etat.journal().last().map(|e| e.motif),
            Some("kindmother_indisponible")
        );
        verifier_lignes(&b, &id_attendu(&d), 1).await;
        // KindMother de retour : la version suivante, jamais la 1.
        let (_, app) = b.redemarrer(crate::gouvernance::tests::gouvernance_de_base());
        let (s, r) = envoyer(app, CHEMIN, &proposition(&d).to_string(), true).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(2)),
            "{r}"
        );
        // Base de l'état de publication nationale : refusée.
        assert!(
            PublicationsKindMother::charger(&b.addr, "maia-alice", Some(&cred(&b.dir))).is_err()
        );
    }

    /// Magasin dont l'écriture échoue avant (rien d'écrit) ou après (écrit,
    /// réponse perdue : issue ambiguë, comme un crash).
    struct MagasinCapricieux {
        inner: PublicationsKindMother,
        echec_avant: AtomicBool,
        echec_apres: AtomicBool,
    }

    impl MagasinPublications for MagasinCapricieux {
        fn base(&self) -> &str {
            self.inner.base()
        }
        async fn charger(&self) -> Result<Vec<PublicationDurable>, String> {
            self.inner.charger().await
        }
        async fn enregistrer(&self, p: &PublicationDurable) -> Result<(), String> {
            if self.echec_avant.swap(false, Ordering::SeqCst) {
                return Err("transport".into());
            }
            self.inner.enregistrer(p).await?;
            if self.echec_apres.swap(false, Ordering::SeqCst) {
                return Err("réponse perdue".into());
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn ecriture_echouee_ou_ambigue_puis_rechargement() {
        let b = banc_complet(
            crate::gouvernance::tests::gouvernance_de_base(),
            true,
            |s| s,
            |m| MagasinCapricieux {
                inner: m,
                echec_avant: AtomicBool::new(false),
                echec_apres: AtomicBool::new(false),
            },
        )
        .await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let id = id_attendu(&d);
        let magasin = &b.etat.actif.as_ref().expect("actif").magasin;
        // Échec avant l'écriture : rien de durable, rien de publié, la version 1 reste libre.
        magasin.echec_avant.store(true, Ordering::SeqCst);
        let (s, r) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("persistance_indisponible")
            ),
            "{r}"
        );
        assert_eq!(b.lignes().await, Vec::new());
        let (s, r) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(1)),
            "{r}"
        );
        // Écrite puis réponse perdue : l'état est relu, la version 2 est déjà prise.
        magasin.echec_apres.store(true, Ordering::SeqCst);
        let (s, r) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("persistance_indisponible")
            ),
            "{r}"
        );
        let (s, r) = b.proposer(avec(proposition(&d), "version", 2.into())).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::CONFLICT, Some("version_deja_publiee")),
            "{r}"
        );
        let (s, r) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(3)),
            "{r}"
        );
        verifier_lignes(&b, &id, 3).await;
        // Une version n'est enregistrée qu'une fois, jamais avec un autre contenu.
        let mut double = b
            .lignes()
            .await
            .into_iter()
            .find(|l| l.fiche == id && l.version == 2)
            .expect("v2");
        double.octets.push(0);
        double.empreinte = "00".repeat(32);
        assert!(magasin.inner.enregistrer(&double).await.is_err());
        verifier_lignes(&b, &id, 3).await;
    }

    #[tokio::test]
    async fn refus_de_la_porte_ne_consomme_pas_d_attestation() {
        let b = banc_avec(
            crate::tamr::tests::gouvernance(crate::tamr::tests::POLITIQUE_SANS_TAMR),
            true,
        )
        .await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        for _ in 0..=maia_federee::MAX_ATTESTATIONS_PAR_PERIODE {
            let (s, r) = b.proposer(proposition(&d)).await;
            assert_eq!(
                (s, r["reason"].as_str()),
                (StatusCode::FORBIDDEN, Some("refus_porte")),
                "{r}"
            );
        }
        assert_eq!(b.lignes().await, Vec::new());
        // Porte qui autorise : le plafond est entier.
        let (_, app) = b.redemarrer(crate::gouvernance::tests::gouvernance_de_base());
        for v in 1..=maia_federee::MAX_ATTESTATIONS_PAR_PERIODE as u64 {
            let (s, r) = envoyer(app.clone(), CHEMIN, &proposition(&d).to_string(), true).await;
            assert_eq!(
                (s, r["publiee"]["version"].as_u64()),
                (StatusCode::OK, Some(v)),
                "{r}"
            );
        }
        let (s, r) = envoyer(app, CHEMIN, &proposition(&d).to_string(), true).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::TOO_MANY_REQUESTS, Some("plafond_attestations")),
            "{r}"
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn concurrence_ni_version_dupliquee_ni_plafond_depasse() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let corps = proposition(&d).to_string();
        let taches: Vec<_> = (0..12)
            .map(|_| {
                let app = b.app.clone();
                let corps = corps.clone();
                tokio::spawn(async move { envoyer(app, CHEMIN, &corps, true).await })
            })
            .collect();
        let mut versions = Vec::new();
        let mut plafond = 0;
        for t in taches {
            let (s, r) = t.await.expect("tâche");
            match s {
                StatusCode::OK => versions.push(r["publiee"]["version"].as_u64().expect("v")),
                StatusCode::TOO_MANY_REQUESTS => {
                    assert_eq!(r["reason"].as_str(), Some("plafond_attestations"), "{r}");
                    plafond += 1;
                }
                _ => panic!("{s} {r}"),
            }
        }
        versions.sort_unstable();
        let max = maia_federee::MAX_ATTESTATIONS_PAR_PERIODE as u64;
        assert_eq!(versions, (1..=max).collect::<Vec<_>>());
        assert_eq!(plafond, 12 - maia_federee::MAX_ATTESTATIONS_PAR_PERIODE);
        verifier_lignes(&b, &id_attendu(&d), max).await;
    }

    #[tokio::test]
    async fn donnee_etiquetee_par_un_autre_utilisateur_refusee() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        // Bob (propriétaire de la base lui aussi) pose l'étiquette ; Alice propose.
        b.etiqueter_par(
            "bob",
            &bob_etiquette(),
            &d,
            ActionEtiquette::Poser,
            TEXTE,
            maintenant_unix(),
        )
        .await;
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("non_publiable")),
            "{v}"
        );
        assert_eq!(b.etat.derniere_version(&id_attendu(&d)).await, 0);
        assert_eq!(b.lignes().await, Vec::new());
        // Base de l'état de publication : jamais source d'une Fiche.
        let mut r = d.clone();
        r.base = OPERATEUR_PUBLICATIONS.to_uppercase();
        let (s, v) = b.proposer(proposition(&r)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("base_reservee")),
            "{v}"
        );
    }

    /// Source dont le propriétaire relu change après la première lecture.
    struct SourceVolage {
        inner: EtiquettesKindMother,
        lectures: AtomicUsize,
    }

    impl SourceEtiquettes for SourceVolage {
        async fn lire(&self, donnee: &RefDonnee) -> Result<EtatEtiquette, String> {
            let mut e = self.inner.lire(donnee).await?;
            if self.lectures.fetch_add(1, Ordering::SeqCst) >= 1 {
                e.utilisateur = Some("bob".into());
            }
            Ok(e)
        }
    }

    #[tokio::test]
    async fn proprietaire_change_entre_la_route_et_l_attestation() {
        let b = banc_complet(
            crate::gouvernance::tests::gouvernance_de_base(),
            true,
            |s| SourceVolage {
                inner: s,
                lectures: AtomicUsize::new(0),
            },
            |m| m,
        )
        .await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        let (s, v) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, v["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("non_publiable")),
            "{v}"
        );
        let j = b.etat.journal();
        let e = j.last().expect("entrée");
        // Refus à l'attestation (Fiche déjà scellée), non à la première lecture.
        assert!(e.empreinte.is_some() && e.version == Some(1), "{e:?}");
        assert_eq!(
            b.etat
                .actif
                .as_ref()
                .expect("actif")
                .source
                .lectures
                .load(Ordering::SeqCst),
            2
        );
        assert_eq!(b.lignes().await, Vec::new());
    }
    // ── A-009 / A7 : republication d'une donnée étrangère (chemin TAMR) ──

    const CHEMIN_REPRISE: &str = "/v1/federee/alice/republier";

    /// Fiche étrangère du COG de graine `g` (signée par son identité : format
    /// de la transition, encore lisible).
    fn source_etrangere(g: u8, id: &str, licence: &str, audience: Audience) -> Vec<u8> {
        let cle = CleSignature::depuis_graine(&[g; 32]);
        let f = Fiche {
            id: id.into(),
            emetteur: cle.cle_publique(),
            sujet: "capitale de la France".into(),
            type_fiche: TypeFiche::Fait,
            texte: TEXTE.into(),
            relations: vec![Relation {
                sujet: "Paris".into(),
                predicat: "capitale de".into(),
                objet: "France".into(),
            }],
            sources: vec!["https://exemple.org/geo".into()],
            date: maintenant_unix() - 60,
            version: 1,
            langue: "fr".into(),
            licence: licence.into(),
            audience,
            reprises: Vec::new(),
        };
        maia_federee::sceller_fiche(&f, &cle).expect("source")
    }

    fn b64(o: &[u8]) -> String {
        base64::engine::general_purpose::STANDARD.encode(o)
    }

    fn reprise(sources: &[&[u8]]) -> serde_json::Value {
        serde_json::json!({ "fiches": sources.iter().map(|o| b64(o)).collect::<Vec<_>>() })
    }

    async fn demander(app: &Router, corps: &serde_json::Value) -> (StatusCode, serde_json::Value) {
        let u = usage();
        envoyer_avec(
            app.clone(),
            "POST",
            CHEMIN_REPRISE,
            &corps.to_string(),
            Some((&alice_http(), &u)),
        )
        .await
    }

    async fn approuver(
        app: &Router,
        chemin: &str,
        empreinte: &str,
    ) -> (StatusCode, serde_json::Value) {
        let u = usage_approbation();
        let corps = serde_json::json!({ "empreinte": empreinte }).to_string();
        envoyer_avec(
            app.clone(),
            "POST",
            chemin,
            &corps,
            Some((&alice_approbation(), &u)),
        )
        .await
    }

    fn chemin_approbation(v: &serde_json::Value) -> String {
        format!(
            "{CHEMIN_REPRISE}/{}",
            v["approbation"]["intervention_id"].as_str().expect("iv")
        )
    }

    fn empreinte_tamr(v: &serde_json::Value) -> String {
        v["approbation"]["empreinte"]
            .as_str()
            .expect("empreinte")
            .to_string()
    }

    #[tokio::test]
    async fn republication_approuvee_publiee_persistee_rechargee() {
        let b = banc().await;
        let src = source_etrangere(31, "geo.paris", "CC-BY-4.0", Audience::Membres);
        // 1. Demande : Fiche calculée par le serveur, figée dans une demande TAMR ; rien n'est publié.
        let (s, v) = demander(&b.app, &reprise(&[&src])).await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        assert_eq!(v["approbation"]["type_intervention"], "APPROVAL");
        assert_eq!(v["approbation"]["objet"], "federee.republier");
        let description = v["approbation"]["description"]
            .as_str()
            .expect("description");
        let octets = base64::engine::general_purpose::STANDARD
            .decode(v["fiche"]["fiche"].as_str().expect("fiche"))
            .expect("b64");
        assert!(description.starts_with(&ServiceRequete::description_republication(&octets)));
        assert!(
            description.contains(TEXTE) && description.contains("CC-BY-4.0"),
            "{description}"
        );
        let id = v["fiche"]["id"].as_str().expect("id").to_string();
        assert!(id.starts_with(maia_federee::PREFIXE_ID_REPRISE));
        assert_eq!(b.lignes().await, Vec::new());
        assert_eq!(b.dernier_motif(), "approbation_tamr_demandee");

        // 2. Approbation signée par la clé d'approbation, sur l'empreinte vue : publiée.
        let chemin = chemin_approbation(&v);
        let (s, r) = approuver(&b.app, &chemin, &empreinte_tamr(&v)).await;
        assert_eq!(s, StatusCode::OK, "{r}");
        assert_eq!(r["tamr"], "approuve");
        assert_eq!(r["publiee"]["id"], id.as_str());
        assert_eq!(
            r["publiee"]["fiche"], v["fiche"]["fiche"],
            "octets exacts approuvés"
        );
        let f = verifier_fiche(&octets, maintenant_unix()).expect("fiche");
        assert!(f.est_reprise());
        assert!(matches!(
            f.signature(),
            maia_federee::SignatureFiche::Deleguee { .. }
        ));
        let source = verifier_fiche(&src, maintenant_unix()).expect("source");
        assert_eq!(f.fiche().reprises, vec![maia_federee::Reprise::de(&source)]);
        assert_eq!(
            (f.fiche().licence.as_str(), &f.fiche().audience),
            ("CC-BY-4.0", &Audience::Membres)
        );
        let j = b.etat.journal();
        let e = j.last().expect("entrée");
        assert_eq!((e.verdict, e.motif), ("publiee", "republiee"));
        assert!(
            e.detail.contains("maia-federee-republier")
                && e.detail.contains("maia-tamr-approbation"),
            "{}",
            e.detail
        );
        // Persistée comme A-018 : une ligne, ensemble des sources dans `donnee`.
        let l = b.lignes().await;
        assert_eq!(l.len(), 1);
        assert_eq!((l[0].fiche.as_str(), l[0].version), (id.as_str(), 1));
        assert_eq!(
            l[0].donnee,
            hex::encode(maia_federee::empreinte_ensemble(&[source]))
        );
        // Usage unique.
        let (s, r) = approuver(&b.app, &chemin, &empreinte_tamr(&v)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::NOT_FOUND, Some("approbation_inconnue"))
        );

        // 3. Redémarrage : la republication est rechargée (chemin TAMR), version suivante = 2.
        let (etat2, app2) = b.redemarrer(crate::gouvernance::tests::gouvernance_de_base());
        assert!(etat2.precharger().await);
        assert_eq!(etat2.derniere_version(&id).await, 1);
        let (s, v) = demander(&app2, &reprise(&[&src])).await;
        assert_eq!(
            (s, v["fiche"]["version"].as_u64()),
            (StatusCode::ACCEPTED, Some(2)),
            "{v}"
        );
        let (s, r) = approuver(&app2, &chemin_approbation(&v), &empreinte_tamr(&v)).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(2)),
            "{r}"
        );
        // Jamais comptée pour la corroboration (A-009).
        assert!(verifier_fiche(&octets, maintenant_unix())
            .expect("f")
            .est_reprise());
    }

    #[tokio::test]
    async fn republication_refusee_sans_approbation_autre_empreinte_refus_expiree() {
        let b = banc().await;
        let src = source_etrangere(32, "geo.paris", "CC-BY-4.0", Audience::Invites);
        // Sans approbation : rien n'est publié ; la route ordinaire refuse les reprises.
        let (s, v) = demander(&b.app, &reprise(&[&src])).await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        assert_eq!(b.lignes().await, Vec::new());
        let d = b.donnee(TEXTE).await;
        let (s, r) = b
            .proposer(avec(
                proposition(&d),
                "reprises",
                serde_json::json!([b64(&src)]),
            ))
            .await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("republication_tamr_requise"))
        );
        // Autre empreinte : refus, et la demande est consommée.
        let chemin = chemin_approbation(&v);
        let (s, r) = approuver(&b.app, &chemin, &"00".repeat(32)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("empreinte_differente")),
            "{r}"
        );
        let (s, r) = approuver(&b.app, &chemin, &empreinte_tamr(&v)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::NOT_FOUND, Some("approbation_inconnue"))
        );
        // Refus explicite de l'utilisateur (clé de l'utilisateur) : close la demande.
        let (_, v) = demander(&b.app, &reprise(&[&src])).await;
        let chemin = chemin_approbation(&v);
        let u = usage();
        let (s, r) = envoyer_avec(
            b.app.clone(),
            "DELETE",
            &chemin,
            "",
            Some((&alice_http(), &u)),
        )
        .await;
        assert_eq!(
            (s, r["tamr"].as_str()),
            (StatusCode::OK, Some("refuse")),
            "{r}"
        );
        let (s, _) = approuver(&b.app, &chemin, &empreinte_tamr(&v)).await;
        assert_eq!(s, StatusCode::NOT_FOUND);
        assert_eq!(b.lignes().await, Vec::new());

        // Délai expiré (R-APPR-4) : refus par défaut.
        let (_, app) = etat_regle(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                magasin(&b.addr, &b.dir),
                signataire(&b.graine),
            )),
            |e| e.avec_duree_approbation(std::time::Duration::ZERO),
        );
        let (s, v) = demander(&app, &reprise(&[&src])).await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        let (s, r) = approuver(&app, &chemin_approbation(&v), &empreinte_tamr(&v)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("approbation_expiree")),
            "{r}"
        );
        assert_eq!(b.lignes().await, Vec::new());
    }

    #[tokio::test]
    async fn republication_refus_licence_audience_et_sources() {
        let b = banc().await;
        let src = source_etrangere(33, "geo.paris", "CC-BY-4.0", Audience::Membres);
        let mit = source_etrangere(34, "geo.lyon", "MIT", Audience::Membres);
        let attendu = |s: StatusCode, r: &serde_json::Value, statut: StatusCode, code: &str| {
            assert_eq!((s, r["reason"].as_str()), (statut, Some(code)), "{r}");
        };
        // Licence différente entre les sources, ou annoncée différente de la source.
        let (s, r) = demander(&b.app, &reprise(&[&src, &mit])).await;
        attendu(
            s,
            &r,
            StatusCode::UNPROCESSABLE_ENTITY,
            "licence_differente",
        );
        let (s, r) = demander(&b.app, &avec(reprise(&[&src]), "licence", "MIT".into())).await;
        attendu(
            s,
            &r,
            StatusCode::UNPROCESSABLE_ENTITY,
            "licence_differente",
        );
        // Audience élargie (membres ⇒ invités).
        let (s, r) = demander(
            &b.app,
            &avec(reprise(&[&src]), "audience", "invites".into()),
        )
        .await;
        attendu(s, &r, StatusCode::FORBIDDEN, "audience_elargie");
        // Sujet absent des sources.
        let (s, r) = demander(
            &b.app,
            &avec(reprise(&[&src]), "sujet", "code d'accès 1234".into()),
        )
        .await;
        attendu(s, &r, StatusCode::UNPROCESSABLE_ENTITY, "non_derivee");
        // Jamais de données KindMother (donc jamais `maia-*`) : champ refusé.
        let donnee = serde_json::json!({ "base": "maia-alice", "table": "notes", "colonne": "body", "cle": { "id": "n1" } });
        let (s, r) = demander(&b.app, &avec(reprise(&[&src]), "donnee", donnee)).await;
        attendu(s, &r, StatusCode::BAD_REQUEST, "corps_invalide");
        // Fiche du COG local présentée comme étrangère.
        let identite = CleSignature::depuis_graine(&b.graine).cle_publique();
        let mut locale = verifier_fiche(&src, maintenant_unix())
            .expect("f")
            .fiche()
            .clone();
        locale.emetteur = identite;
        let locale = signataire(&b.graine)
            .sceller(&locale, maintenant_unix())
            .expect("locale");
        let (s, r) = demander(&b.app, &reprise(&[&locale])).await;
        attendu(s, &r, StatusCode::FORBIDDEN, "source_locale");
        // Émetteur non admis par la vue fédérale.
        let revoque =
            source_etrangere(GRAINE_REVOQUEE, "geo.paris", "CC-BY-4.0", Audience::Membres);
        let (s, r) = demander(&b.app, &reprise(&[&revoque])).await;
        attendu(s, &r, StatusCode::FORBIDDEN, "source_non_admise");
        // Source altérée, ou aucune source.
        let mut alteree = src.clone();
        if let Some(o) = alteree.last_mut() {
            *o ^= 1;
        }
        let (s, r) = demander(&b.app, &reprise(&[&alteree])).await;
        attendu(s, &r, StatusCode::UNPROCESSABLE_ENTITY, "source_invalide");
        let (s, r) = demander(&b.app, &reprise(&[])).await;
        attendu(s, &r, StatusCode::BAD_REQUEST, "sources_invalides");
        assert_eq!(b.lignes().await, Vec::new());
        // Audience restreinte : COG nommés ⊂ invités admis ; ⊄ membres (un nommé peut être invité).
        let nomme = serde_json::json!({ "nommes": [cog_de_cle(&[9; 32]).expect("cog").as_str()] });
        let (s, r) = demander(&b.app, &avec(reprise(&[&src]), "audience", nomme.clone())).await;
        attendu(s, &r, StatusCode::FORBIDDEN, "audience_elargie");
        let invites = source_etrangere(37, "geo.paris", "CC-BY-4.0", Audience::Invites);
        let (s, v) = demander(&b.app, &avec(reprise(&[&invites]), "audience", nomme)).await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
    }

    #[tokio::test]
    async fn republication_autre_utilisateur_cles_vue_et_debit() {
        let b = banc().await;
        let src = source_etrangere(35, "geo.paris", "CC-BY-4.0", Audience::Membres);
        let corps = reprise(&[&src]).to_string();
        let u = usage();
        // Autre utilisateur dans le chemin : 403.
        let (s, r) = envoyer_avec(
            b.app.clone(),
            "POST",
            "/v1/federee/bob/republier",
            &corps,
            Some((&alice_http(), &u)),
        )
        .await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("autre_utilisateur"))
        );
        let (_, v) = demander(&b.app, &reprise(&[&src])).await;
        let iv = v["approbation"]["intervention_id"]
            .as_str()
            .expect("iv")
            .to_string();
        let corps_a = serde_json::json!({ "empreinte": empreinte_tamr(&v) }).to_string();
        // Approbation signée par la clé ordinaire de l'utilisateur (pas d'approbation) : 401.
        let (s, r) = envoyer_avec(
            b.app.clone(),
            "POST",
            &chemin_approbation(&v),
            &corps_a,
            Some((&alice_http(), &u)),
        )
        .await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::UNAUTHORIZED,
                Some("signature_approbation_requise")
            )
        );
        // Approbation pour un autre utilisateur : 403, rien n'est publié.
        let ua = usage_approbation();
        let (s, r) = envoyer_avec(
            b.app.clone(),
            "POST",
            &format!("/v1/federee/bob/republier/{iv}"),
            &corps_a,
            Some((&alice_approbation(), &ua)),
        )
        .await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("autre_utilisateur"))
        );
        // TAMR : l'approbateur doit être le propriétaire de l'action.
        let d = b
            .etat
            .approbations
            .demander(ActionTamr::Republier {
                proprietaire: "alice".into(),
                octets: src.clone(),
            })
            .expect("d");
        let sig = SignatureApprobation::verifiee(&alice_approbation().cle_publique());
        assert_eq!(
            b.etat
                .approbations
                .approuver(
                    &d.intervention_id,
                    "bob",
                    &d.empreinte,
                    &sig,
                    &b.etat.gouvernance
                )
                .unwrap_err(),
            RefusTamr::ApprobateurInvalide
        );
        assert_eq!(b.lignes().await, Vec::new());
        // Clé d'approbation = clé de l'utilisateur : approbations désactivées.
        let (_, app) = etat_regle(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                magasin(&b.addr, &b.dir),
                signataire(&b.graine),
            )),
            |e| {
                e.avec_cles_approbation(
                    vec![alice_http().cle_publique()],
                    &[alice_http().cle_publique()],
                )
            },
        );
        let (s, r) = envoyer_avec(
            app,
            "POST",
            &chemin_approbation(&v),
            &corps_a,
            Some((&alice_http(), &ua)),
        )
        .await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("approbations_desactivees")
            )
        );
        // Sans vue fédérale : émetteurs non vérifiables, 503.
        let sans_vue = Arc::new(EtatPublicationHttp::new(
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                magasin(&b.addr, &b.dir),
                signataire(&b.graine),
            )),
            vec![alice_http().cle_publique()],
            INSTANCE.to_string(),
            Arc::new(crate::gouvernance::tests::gouvernance_de_base()),
            SecurityConfig::default(),
        ));
        let (s, r) = demander(&routeur(sans_vue), &reprise(&[&src])).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Some("vue_federale_indisponible")
            )
        );
        // Débit par utilisateur : au-delà de PUBLICATIONS_PAR_HEURE demandes, 429.
        for _ in 1..PUBLICATIONS_PAR_HEURE {
            let (s, _) = demander(&b.app, &serde_json::json!({})).await;
            assert_eq!(s, StatusCode::BAD_REQUEST);
        }
        let (s, r) = demander(&b.app, &reprise(&[&src])).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (
                StatusCode::TOO_MANY_REQUESTS,
                Some("debit_utilisateur_depasse")
            )
        );
    }

    #[tokio::test]
    async fn republication_refusee_par_la_porte_malgre_tamr() {
        // TAMR autorisé, mais aucune règle `federee.republier` : refus par défaut.
        let b = banc_avec(
            crate::tamr::tests::gouvernance(crate::tamr::tests::POLITIQUE_TAMR),
            true,
        )
        .await;
        let src = source_etrangere(36, "geo.paris", "CC-BY-4.0", Audience::Membres);
        let (s, v) = demander(&b.app, &reprise(&[&src])).await;
        assert_eq!(s, StatusCode::ACCEPTED, "{v}");
        let (s, r) = approuver(&b.app, &chemin_approbation(&v), &empreinte_tamr(&v)).await;
        assert_eq!(
            (s, r["reason"].as_str()),
            (StatusCode::FORBIDDEN, Some("refus_porte")),
            "{r}"
        );
        assert_eq!(b.lignes().await, Vec::new());
    }

    #[tokio::test]
    async fn transition_fiches_signees_par_l_identite_relues_puis_cle_fiches() {
        let b = banc().await;
        let d = b.donnee(TEXTE).await;
        b.poser(&d).await;
        // Configuration transitoire (A-017) : la clé d'identité signe.
        let (_, app_identite) = etat_de(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                magasin(&b.addr, &b.dir),
                SignataireFiches::identite(CleSignature::depuis_graine(&b.graine)),
            )),
        );
        let (s, r) = envoyer(app_identite, CHEMIN, &proposition(&d).to_string(), true).await;
        assert_eq!(s, StatusCode::OK, "{r}");
        let id = id_attendu(&d);
        let l = b.lignes().await;
        assert_eq!(
            verifier_fiche(&l[0].octets, maintenant_unix())
                .expect("v1")
                .signature(),
            &maia_federee::SignatureFiche::Identite
        );
        // Clé `fiches` configurée (même identité) : l'état ancien se recharge, la suite est déléguée.
        let (s, r) = b.proposer(proposition(&d)).await;
        assert_eq!(
            (s, r["publiee"]["version"].as_u64()),
            (StatusCode::OK, Some(2)),
            "{r}"
        );
        verifier_lignes(&b, &id, 2).await;
        let l = b.lignes().await;
        let v2 = l.iter().find(|x| x.version == 2).expect("v2");
        assert!(matches!(
            verifier_fiche(&v2.octets, maintenant_unix())
                .expect("v2")
                .signature(),
            maia_federee::SignatureFiche::Deleguee { .. }
        ));
        // Autre identité (changement de clé d'identité du COG) : état incohérent, 503.
        let (etat, _) = etat_de(
            crate::gouvernance::tests::gouvernance_de_base(),
            Some((
                "alice".to_string(),
                source(&b.addr, &b.dir),
                magasin(&b.addr, &b.dir),
                signataire(&[99; 32]),
            )),
        );
        assert!(!etat.precharger().await);
    }
}
