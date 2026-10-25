//! @id     maia_tamr
//! @do     confirmer_actions_sensibles_par_approbation_tamr
//! @role   security
//! @layer  app
//! @human  MAIA — chemin de confirmation TAMR : intervention APPROVAL sur une action exacte
//! @human  (validation d'un candidat sensible, republication d'octets exacts), tranchée par
//! @human  StrongFather ; refus, StrongFather absent ou délai dépassé ⇒ refus (A-009, D2).

//! # Confirmation TAMR (A-009, D2, INV-TAMR-5)
//!
//! TAMR encadre l'intervention humaine **APPROVAL** ; il ne décide jamais :
//! la décision est rendue par StrongFather (porte obligatoire
//! [`GouvernanceMaia::executer`]).
//!
//! Déroulé (deux requêtes séparées de l'utilisateur authentifié) :
//!
//! 1. [`RegistreApprobations::demander`] fige l'**action exacte** (candidat
//!    complet, ou octets exacts d'une Fiche à republier) et rend une demande
//!    `EN_ATTENTE` : identifiant d'intervention, empreinte SHA-256 de l'action,
//!    description, durée de vie ([`DUREE_VIE_SECS`]) ;
//! 2. [`RegistreApprobations::approuver`] consomme la demande (une seule
//!    réponse, R-APPR-2), vérifie le délai (R-APPR-4 : expiration ⇒ refus),
//!    l'empreinte présentée par l'approbateur et son identité, puis soumet
//!    l'intention `APPROVAL` à StrongFather. Seule une décision ALLOW couvrant
//!    cette intention exacte produit une [`ApprobationAccordee`].
//!
//! [`ApprobationAccordee`] n'a ni champ public ni constructeur public : elle
//! ne naît que dans la fermeture de la porte StrongFather, avec le jeton
//! [`bondingbrother::DecisionAutorisee`] (infalsifiable). C'est le **seul**
//! endroit du pont qui construit `Validation::UtilisateurEtTamr` et
//! `ValidationUtilisateur::confirmee_par_tamr` : il détient les autorités
//! scellées (`maia_memoire::AutoriteTamr`, `maia_federee::AutoriteConfirmation`,
//! jetons uniques par processus institués au démarrage, L1).
//!
//! ## Approbation signée (revue de sécurité du lot 7, F7)
//!
//! L'approbation (étape 2) n'est recevable que si la requête HTTP est signée
//! avec l'usage [`USAGE_APPROBATION`] par une clé épinglée dans
//! [`ENV_CLES_APPROBATION`], **distincte** des clés `maia/memoire` (vérifié
//! par `routes_memoire`) ; liste vide ⇒ approbations refusées. La garde HTTP
//! produit une [`SignatureApprobation`] (identifiant de la clé signataire),
//! portée par l'intention (`data.cle_approbation`, `data.usage_signe`,
//! `data.canal = 'utilisateur_signe'`). **Limite restante** : StrongFather ne
//! revérifie pas la signature lui-même ; ces attributs sont posés par le pont
//! après vérification — la règle `maia-tamr-approbation` reste une défense en
//! profondeur, pas une preuve cryptographique indépendante.
//!
//! ## Action exacte (F8)
//!
//! Pour une correction, la **version courante de la note cible** (texte,
//! provenance, importance, liens, sources) est figée dans l'action, affichée
//! et liée à l'empreinte ; `maia-memoire` refuse la validation si la cible a
//! changé depuis la demande (`ConfirmationTamr::version_cible`).
//!
//! ## Intention StrongFather
//!
//! - `action` : `modification` (valider un candidat) ou `creation` (republier)
//! - `subject` : `maia.memoire.<proprietaire>` ou `maia.federee.republication`
//! - `caller` : identifiant de l'approbateur ; `origin` : `maia`
//! - `data.operation = 'tamr.approbation'`, `data.intervention = 'APPROVAL'`,
//!   `data.objet` (`memoire.valider_candidat` / `federee.republier`),
//!   `data.intervention_id`, `data.proprietaire`, `data.approbateur`,
//!   `data.approbateur_est_proprietaire`, `data.empreinte` (SHA-256 hexadécimal
//!   de l'action), `data.canal` (`utilisateur_signe`), `data.usage_signe`
//!   ([`USAGE_APPROBATION`]), `data.cle_approbation` (identifiant de la clé
//!   signataire), et pour un candidat `data.domaine`, `data.sensible`,
//!   `data.provenance`, `data.type_unite`, `data.version_cible`
//! - métadonnées : `requested_at`, `resolved_at` (secondes Unix)
//!
//! Sans politique `tamr.approbation` dans `maia.toml` : refus par défaut.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use bondingbrother::strongfather::{ActionType, CallContext, IntentData};
use bondingbrother::{DecisionAutorisee, Intent};
use maia_federee::{AutoriteConfirmation, ServiceRequete, ValidationUtilisateur};
use maia_memoire::{AutoriteTamr, Candidat, ConfirmationTamr, Note, Validation};
use sha2::{Digest, Sha256};

use crate::garde_etranger;
use crate::gouvernance::{GouvernanceMaia, ORIGINE_MAIA};

/// Durée de vie d'une demande d'approbation (secondes) ; au-delà : EXPIRÉ ⇒ refus.
pub const DUREE_VIE_SECS: u64 = 300;
/// Demandes d'approbation en attente au plus (C-13).
pub const CAPACITE_MAX: usize = 32;
/// Valeur de `data.operation` des intentions TAMR.
pub const OPERATION_APPROBATION: &str = "tamr.approbation";
/// Type d'intervention TAMR (contrat des types d'intervention, §3).
pub const TYPE_INTERVENTION: &str = "APPROVAL";
/// Canal unique d'approbation : requête signée par une clé d'approbation de l'utilisateur (F7).
pub const CANAL_UTILISATEUR: &str = "utilisateur_signe";
/// Usage signé (préfixe) des requêtes d'approbation TAMR (F7), distinct de `maia/memoire`.
pub const USAGE_APPROBATION: &str = "maia/tamr-approbation";
/// Clés publiques d'approbation de l'utilisateur (base64 URL ou hex, séparées par des
/// virgules) ; vide ⇒ approbations refusées ; une clé aussi épinglée pour `maia/memoire` ⇒
/// approbations refusées.
pub const ENV_CLES_APPROBATION: &str = "MAIA_TAMR_CLES_APPROBATION";

/// Autorités scellées (L1) : jetons uniques par processus de `maia-memoire` et
/// `maia-federee`, détenus par ce module seul.
struct Autorites {
    tamr: AutoriteTamr,
    confirmation: AutoriteConfirmation,
}

static AUTORITES: OnceLock<Option<Autorites>> = OnceLock::new();

fn autorites() -> Option<&'static Autorites> {
    AUTORITES
        .get_or_init(|| {
            if let (Some(tamr), Some(confirmation)) = (AutoriteTamr::instituer(), AutoriteConfirmation::instituer()) {
                return Some(Autorites { tamr, confirmation });
            }
            tracing::error!(target: "maia::tamr", "autorités TAMR déjà prises par un autre module : approbations refusées");
            None
        })
        .as_ref()
}

/// Institue les autorités TAMR au démarrage (avant tout autre code) ; `false`
/// si elles ont été prises ailleurs (toute approbation sera alors refusée).
pub fn instituer_autorites() -> bool {
    autorites().is_some()
}

/// Preuve, posée par la garde HTTP **après vérification**, qu'une requête
/// d'approbation a été signée avec [`USAGE_APPROBATION`] par une clé épinglée
/// d'approbation (F7). Porte l'identifiant de la clé signataire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignatureApprobation {
    cle: String,
}

impl SignatureApprobation {
    /// Signature vérifiée par la clé publique `cle_publique` (identifiant :
    /// SHA-256 hexadécimal de la clé).
    pub(crate) fn verifiee(cle_publique: &[u8; 32]) -> Self {
        Self {
            cle: hex::encode(Sha256::digest(cle_publique)),
        }
    }

    /// Identifiant de la clé signataire.
    pub fn cle(&self) -> &str {
        &self.cle
    }
}
/// Domaine de l'empreinte d'une action approuvable.
const DOMAINE_EMPREINTE: &[u8] = b"maia/tamr/approbation/v1\0";

/// Action exacte soumise à approbation.
#[derive(Debug, Clone, PartialEq)]
pub enum ActionTamr {
    /// Validation d'un candidat (branche sensible) de la mémoire de `proprietaire`.
    ValiderCandidat {
        /// Utilisateur propriétaire de la mémoire.
        proprietaire: String,
        /// Candidat complet, figé au moment de la demande.
        candidat: Candidat,
        /// Version courante de la note cible au moment de la demande (`None` :
        /// nouvelle note) ; affichée et liée à l'empreinte (F8).
        cible_actuelle: Option<Box<Note>>,
    },
    /// Republication d'une donnée étrangère : octets signés exacts de la Fiche.
    Republier {
        /// Utilisateur qui republie.
        proprietaire: String,
        /// Octets exacts de la Fiche à publier.
        octets: Vec<u8>,
    },
}

impl ActionTamr {
    /// Nom stable de l'objet (`data.objet`).
    pub fn objet(&self) -> &'static str {
        match self {
            Self::ValiderCandidat { .. } => "memoire.valider_candidat",
            Self::Republier { .. } => "federee.republier",
        }
    }

    /// Propriétaire de l'action.
    pub fn proprietaire(&self) -> &str {
        match self {
            Self::ValiderCandidat { proprietaire, .. } | Self::Republier { proprietaire, .. } => {
                proprietaire
            }
        }
    }

    /// Empreinte SHA-256 (hexadécimal) de l'action exacte : domaine, objet,
    /// propriétaire, puis candidat canonique (JSON) **et version courante de la
    /// cible** (F8), ou description de republication (empreinte des octets).
    /// Sérialisation impossible ⇒ erreur (jamais d'empreinte par défaut).
    pub fn empreinte(&self) -> Result<String, RefusTamr> {
        let mut h = Sha256::new();
        h.update(DOMAINE_EMPREINTE);
        h.update(self.objet().as_bytes());
        h.update([0]);
        h.update(self.proprietaire().as_bytes());
        h.update([0]);
        match self {
            Self::ValiderCandidat {
                candidat,
                cible_actuelle,
                ..
            } => {
                // Sérialisation déterministe (ordre des champs des structures).
                let k = serde_json::to_vec(candidat).map_err(|_| RefusTamr::Serialisation)?;
                let c = serde_json::to_vec(cible_actuelle).map_err(|_| RefusTamr::Serialisation)?;
                h.update(u64::try_from(k.len()).unwrap_or(u64::MAX).to_be_bytes());
                h.update(k);
                h.update(c);
            }
            Self::Republier { octets, .. } => {
                h.update(ServiceRequete::description_republication(octets).as_bytes());
            }
        }
        Ok(hex::encode(h.finalize()))
    }

    /// Description affichée à l'utilisateur (aucun texte généré par un modèle
    /// hors des valeurs citées) : candidat complet et, pour une correction,
    /// texte actuel de la cible (F8).
    pub fn description(&self) -> String {
        match self {
            Self::ValiderCandidat {
                candidat: k,
                cible_actuelle,
                ..
            } => {
                let liste = |v: Vec<&str>| {
                    if v.is_empty() {
                        "aucun".to_string()
                    } else {
                        v.join(", ")
                    }
                };
                let liens = liste(
                    k.liens
                        .iter()
                        .map(maia_memoire::IdMemoire::as_str)
                        .collect(),
                );
                let sources = liste(
                    k.sources
                        .iter()
                        .map(maia_memoire::EmpreinteSource::as_str)
                        .collect(),
                );
                let cible = match (&k.cible, cible_actuelle) {
                    (Some(id), Some(n)) => format!(
                        "corrige {} (version {}, texte actuel : « {} », provenance {}, importance {})",
                        id, n.version, n.texte, n.provenance.as_str(), n.importance
                    ),
                    (Some(id), None) => format!("corrige {id} (version actuelle inconnue)"),
                    (None, _) => "nouvelle note".to_string(),
                };
                format!(
                    "valider le candidat {} dans la branche sensible {} : « {} » — {} ; provenance {}, \
                     importance {}, confiance {}, liens : {}, sources : {}",
                    k.id,
                    k.branche.as_str(),
                    k.texte,
                    cible,
                    k.provenance.as_str(),
                    k.importance,
                    k.confiance,
                    liens,
                    sources
                )
            }
            Self::Republier { octets, .. } => ServiceRequete::description_republication(octets),
        }
    }

    fn action_strongfather(&self) -> ActionType {
        match self {
            Self::ValiderCandidat { .. } => ActionType::Modification,
            Self::Republier { .. } => ActionType::Creation,
        }
    }

    fn sujet(&self) -> String {
        match self {
            Self::ValiderCandidat { proprietaire, .. } => {
                format!("maia.memoire.{}", proprietaire.to_lowercase())
            }
            Self::Republier { .. } => "maia.federee.republication".to_string(),
        }
    }
}

/// Demande d'approbation présentée à l'utilisateur (état `EN_ATTENTE`).
#[derive(Debug, Clone, serde::Serialize)]
pub struct DemandeApprobation {
    /// Identifiant de l'intervention (128 bits aléatoires).
    pub intervention_id: String,
    /// Type d'intervention (`APPROVAL`).
    pub type_intervention: &'static str,
    /// Objet de l'action.
    pub objet: &'static str,
    /// Empreinte de l'action exacte, à renvoyer pour approuver.
    pub empreinte: String,
    /// Description exacte de l'action.
    pub description: String,
    /// Secondes restantes avant expiration.
    pub expire_dans_s: u64,
}

/// Refus du chemin TAMR (toujours fail-closed : rien n'est écrit ni publié).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum RefusTamr {
    /// Intervention inconnue ou déjà résolue (R-APPR-2).
    #[error("demande d'approbation inconnue ou déjà résolue")]
    Inconnue,
    /// Délai dépassé (EXPIRÉ ⇒ refus par défaut, R-APPR-4).
    #[error("demande d'approbation expirée (durée de vie {DUREE_VIE_SECS} s) : refus par défaut")]
    Expiree,
    /// L'empreinte présentée ne couvre pas l'action en attente.
    #[error("l'empreinte présentée ne correspond pas à l'action en attente")]
    EmpreinteDifferente,
    /// L'approbateur n'est pas le propriétaire de l'action.
    #[error("approbateur différent du propriétaire de l'action")]
    ApprobateurInvalide,
    /// Registre plein.
    #[error("trop de demandes d'approbation en attente ({CAPACITE_MAX})")]
    RegistrePlein,
    /// Verrou interne empoisonné.
    #[error("registre des approbations indisponible")]
    Indisponible,
    /// L'approbation accordée ne couvre pas l'action demandée.
    #[error("l'approbation accordée ne couvre pas cette action")]
    HorsPortee,
    /// StrongFather refuse ou est indisponible.
    #[error("{0}")]
    StrongFather(String),
    /// L'action ne se sérialise pas (empreinte impossible).
    #[error("action non sérialisable : refus")]
    Serialisation,
    /// Autorités TAMR indisponibles (prises par un autre module, L1).
    #[error("autorités TAMR indisponibles : refus")]
    AutoriteIndisponible,
    /// Approbation non signée par une clé d'approbation épinglée (F7).
    #[error("approbation non signée par une clé d'approbation épinglée")]
    SignatureRequise,
}

/// Approbation TAMR accordée par StrongFather pour une action exacte. Aucun
/// constructeur public : seule [`RegistreApprobations::approuver`] en produit,
/// à partir d'un jeton [`DecisionAutorisee`] couvrant l'intention exacte.
#[derive(Debug)]
pub struct ApprobationAccordee {
    action: ActionTamr,
    intervention: String,
    empreinte: String,
    regle: String,
}

impl ApprobationAccordee {
    /// Scelle une approbation : le jeton doit couvrir exactement `intent`,
    /// et `intent` porter l'empreinte `empreinte` (calculée une seule fois, à
    /// la demande) de `action`.
    fn sceller(
        jeton: &DecisionAutorisee,
        intent: &Intent,
        action: ActionTamr,
        intervention: String,
        empreinte: String,
    ) -> Option<Self> {
        let liee = intent.data.content.get("empreinte") == Some(&empreinte)
            && intent.data.content.get("intervention_id") == Some(&intervention);
        (jeton.couvre(intent) && liee).then(|| Self {
            action,
            intervention,
            empreinte,
            regle: jeton.regle().to_string(),
        })
    }

    /// Identifiant de l'intervention.
    pub fn intervention(&self) -> &str {
        &self.intervention
    }

    /// Empreinte de l'action approuvée.
    pub fn empreinte(&self) -> &str {
        &self.empreinte
    }

    /// Règle StrongFather qui a autorisé.
    pub fn regle(&self) -> &str {
        &self.regle
    }

    /// Action approuvée.
    pub fn action(&self) -> &ActionTamr {
        &self.action
    }

    /// `Validation::UtilisateurEtTamr` pour le candidat approuvé (identifiant
    /// et contenu exacts) ; toute autre action ⇒ refus.
    pub fn validation_memoire(self, candidat: maia_memoire::Uuid) -> Result<Validation, RefusTamr> {
        match self.action {
            ActionTamr::ValiderCandidat {
                candidat: k,
                cible_actuelle,
                ..
            } if k.id == candidat => {
                let a = autorites().ok_or(RefusTamr::AutoriteIndisponible)?;
                let version_cible = cible_actuelle.map(|n| n.version);
                Ok(Validation::UtilisateurEtTamr(ConfirmationTamr::nouvelle(
                    &a.tamr,
                    k,
                    version_cible,
                    &self.intervention,
                )))
            }
            _ => Err(RefusTamr::HorsPortee),
        }
    }

    /// `ValidationUtilisateur::confirmee_par_tamr` pour les octets exacts
    /// approuvés ; tout autre contenu ⇒ refus.
    pub fn validation_republication(
        self,
        octets: &[u8],
    ) -> Result<ValidationUtilisateur, RefusTamr> {
        match &self.action {
            ActionTamr::Republier {
                octets: approuves, ..
            } if approuves.as_slice() == octets => {
                let a = autorites().ok_or(RefusTamr::AutoriteIndisponible)?;
                Ok(ValidationUtilisateur::confirmee_par_tamr(
                    &a.confirmation,
                    &self.intervention,
                    &ServiceRequete::description_republication(octets),
                ))
            }
            _ => Err(RefusTamr::HorsPortee),
        }
    }
}

struct EnAttente {
    action: ActionTamr,
    empreinte: String,
    demandee: Instant,
    demandee_unix: u64,
}

/// Registre des demandes d'approbation (usage unique, expiration, capacité bornée).
pub struct RegistreApprobations {
    attente: Mutex<HashMap<String, EnAttente>>,
    duree: Duration,
}

impl std::fmt::Debug for RegistreApprobations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RegistreApprobations")
            .field("duree", &self.duree)
            .finish_non_exhaustive()
    }
}

impl Default for RegistreApprobations {
    fn default() -> Self {
        Self::avec_duree(Duration::from_secs(DUREE_VIE_SECS))
    }
}

fn unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

fn journaliser(intervention: &str, a: &EnAttente, approbateur: &str, resultat: &str, motif: &str) {
    tracing::info!(
        target: "maia::tamr",
        intervention_id = intervention,
        r#type = TYPE_INTERVENTION,
        approver_id = approbateur,
        requested_at = a.demandee_unix,
        resolved_at = unix(),
        result = resultat,
        subject = a.action.objet(),
        empreinte = %a.empreinte,
        motif,
        "intervention TAMR résolue"
    );
}

impl RegistreApprobations {
    /// Registre vide (durée de vie par défaut).
    pub fn new() -> Self {
        Self::default()
    }

    /// Registre avec une durée de vie donnée (tests).
    pub fn avec_duree(duree: Duration) -> Self {
        Self {
            attente: Mutex::new(HashMap::new()),
            duree,
        }
    }

    fn purger(&self, attente: &mut HashMap<String, EnAttente>) {
        let duree = self.duree;
        attente.retain(|id, a| {
            let vivant = a.demandee.elapsed() <= duree;
            if !vivant {
                journaliser(id, a, "", "EXPIRÉ", "délai dépassé");
            }
            vivant
        });
    }

    /// Fige l'action exacte et ouvre une demande d'approbation (`EN_ATTENTE`).
    pub fn demander(&self, action: ActionTamr) -> Result<DemandeApprobation, RefusTamr> {
        let mut attente = self.attente.lock().map_err(|_| RefusTamr::Indisponible)?;
        self.purger(&mut attente);
        if attente.len() >= CAPACITE_MAX {
            return Err(RefusTamr::RegistrePlein);
        }
        let intervention_id = garde_etranger::identifiant_aleatoire();
        let empreinte = action.empreinte()?;
        let demande = DemandeApprobation {
            intervention_id: intervention_id.clone(),
            type_intervention: TYPE_INTERVENTION,
            objet: action.objet(),
            empreinte: empreinte.clone(),
            description: action.description(),
            expire_dans_s: self.duree.as_secs(),
        };
        tracing::info!(target: "maia::tamr", intervention_id = %intervention_id, subject = action.objet(), "approbation TAMR demandée");
        attente.insert(
            intervention_id,
            EnAttente {
                action,
                empreinte,
                demandee: Instant::now(),
                demandee_unix: unix(),
            },
        );
        Ok(demande)
    }

    /// Refus explicite de l'utilisateur (REFUSÉ) : la demande est close.
    pub fn refuser(&self, intervention_id: &str, approbateur: &str) -> Result<(), RefusTamr> {
        let a = self.prendre(intervention_id)?;
        journaliser(
            intervention_id,
            &a,
            approbateur,
            "REFUSÉ",
            "refus de l'utilisateur",
        );
        Ok(())
    }

    fn prendre(&self, intervention_id: &str) -> Result<EnAttente, RefusTamr> {
        let mut attente = self.attente.lock().map_err(|_| RefusTamr::Indisponible)?;
        let a = attente.remove(intervention_id).ok_or(RefusTamr::Inconnue)?;
        if a.demandee.elapsed() > self.duree {
            journaliser(intervention_id, &a, "", "EXPIRÉ", "délai dépassé");
            return Err(RefusTamr::Expiree);
        }
        Ok(a)
    }

    /// Approbation de l'utilisateur `approbateur` pour l'intervention
    /// `intervention_id`, sur l'empreinte `empreinte` qu'il a vue, par une
    /// requête signée avec une clé d'approbation (`signature`, F7). La demande
    /// est consommée quelle que soit l'issue (une seule réponse). StrongFather
    /// tranche ; refus, StrongFather indisponible, délai dépassé ⇒ refus.
    /// L'empreinte de l'action est celle calculée à la demande (une seule fois).
    pub fn approuver(
        &self,
        intervention_id: &str,
        approbateur: &str,
        empreinte: &str,
        signature: &SignatureApprobation,
        gouvernance: &GouvernanceMaia,
    ) -> Result<ApprobationAccordee, RefusTamr> {
        let a = self.prendre(intervention_id)?;
        let refus = |r: RefusTamr| {
            journaliser(intervention_id, &a, approbateur, "REFUSÉ", &r.to_string());
            Err(r)
        };
        if empreinte.trim() != a.empreinte {
            return refus(RefusTamr::EmpreinteDifferente);
        }
        if approbateur.trim().is_empty() || approbateur != a.action.proprietaire() {
            return refus(RefusTamr::ApprobateurInvalide);
        }
        if signature.cle().is_empty() {
            return refus(RefusTamr::SignatureRequise);
        }
        if autorites().is_none() {
            return refus(RefusTamr::AutoriteIndisponible);
        }
        let intent = intention_approbation(
            intervention_id,
            &a.action,
            &a.empreinte,
            approbateur,
            signature,
            a.demandee_unix,
        );
        let action = a.action.clone();
        let accordee = gouvernance.executer(&intent, |jeton| {
            ApprobationAccordee::sceller(
                jeton,
                &intent,
                action,
                intervention_id.to_string(),
                a.empreinte.clone(),
            )
        });
        match accordee {
            Ok(Some(ok)) => {
                journaliser(intervention_id, &a, approbateur, "APPROUVÉ", ok.regle());
                Ok(ok)
            }
            Ok(None) => refus(RefusTamr::HorsPortee),
            Err(motif) => refus(RefusTamr::StrongFather(motif)),
        }
    }
}

/// Intention StrongFather d'une approbation TAMR (forme : voir l'en-tête du
/// module) ; `empreinte` : empreinte de `action` calculée à la demande.
pub fn intention_approbation(
    intervention_id: &str,
    action: &ActionTamr,
    empreinte: &str,
    approbateur: &str,
    signature: &SignatureApprobation,
    demandee: u64,
) -> Intent {
    let mut data = HashMap::new();
    data.insert("operation".into(), OPERATION_APPROBATION.into());
    data.insert("intervention".into(), TYPE_INTERVENTION.into());
    data.insert("objet".into(), action.objet().into());
    data.insert("intervention_id".into(), intervention_id.into());
    data.insert("proprietaire".into(), action.proprietaire().into());
    data.insert("approbateur".into(), approbateur.into());
    data.insert(
        "approbateur_est_proprietaire".into(),
        (approbateur == action.proprietaire()).to_string(),
    );
    data.insert("empreinte".into(), empreinte.into());
    data.insert("canal".into(), CANAL_UTILISATEUR.into());
    data.insert("usage_signe".into(), USAGE_APPROBATION.into());
    data.insert("cle_approbation".into(), signature.cle().into());
    if let ActionTamr::ValiderCandidat {
        candidat,
        cible_actuelle,
        ..
    } = action
    {
        data.insert(
            "version_cible".into(),
            cible_actuelle
                .as_ref()
                .map_or_else(String::new, |n| n.version.to_string()),
        );
        data.insert("domaine".into(), candidat.branche.domaine().into());
        data.insert("sensible".into(), candidat.sensible.to_string());
        data.insert("provenance".into(), candidat.provenance.as_str().into());
        data.insert(
            "type_unite".into(),
            candidat.type_unite.lettre().to_string(),
        );
    }
    let mut i = Intent::new(
        format!("maia-tamr-{}", garde_etranger::identifiant_aleatoire()),
        action.action_strongfather(),
        action.sujet(),
        CallContext {
            caller_identity: approbateur.to_string(),
            origin: ORIGINE_MAIA.to_string(),
            instance: "local".to_string(),
        },
        IntentData { content: data },
    );
    i.metadata
        .insert("requested_at".into(), demandee.to_string());
    i.metadata.insert("resolved_at".into(), unix().to_string());
    i
}

#[cfg(test)]
#[allow(clippy::assert_is_empty, clippy::many_single_char_names)]
pub(crate) mod tests {
    use super::*;
    use maia_memoire::{Branche, Provenance, TypeUnite};

    /// Politique d'approbation TAMR proposée pour `config/strongfather/maia.toml`.
    pub(crate) const POLITIQUE_TAMR: &str = r#"
version = 1

[[policy]]
id = "maia-tamr-approbation"
type = "permission"
effect = "authorize"
condition = "data.operation == 'tamr.approbation'"
rule = "data.intervention == 'APPROVAL' && data.canal == 'utilisateur_signe' && data.usage_signe == 'maia/tamr-approbation' && data.cle_approbation != '' && data.approbateur_est_proprietaire == 'true' && data.empreinte != ''"
[policy.target]
actions = ["modification", "creation"]
origins = ["maia"]
"#;

    /// Politiques valides **sans** règle TAMR ni règle `memoire.noter`.
    pub(crate) const POLITIQUE_SANS_TAMR: &str = r#"
version = 1

[[policy]]
id = "autre-chose"
type = "permission"
effect = "authorize"
condition = "data.operation == 'autre'"
rule = "data.operation == 'autre'"
[policy.target]
origins = ["maia"]
"#;

    /// Gouvernance de test chargée depuis `contenu` (fichier temporaire).
    pub(crate) fn gouvernance(contenu: &str) -> GouvernanceMaia {
        let dir = std::env::temp_dir().join(format!(
            "maia-tamr-{}",
            garde_etranger::identifiant_aleatoire()
        ));
        std::fs::create_dir_all(&dir).expect("tmp");
        let chemin = dir.join("maia.toml");
        std::fs::write(&chemin, contenu).expect("écriture");
        let g = GouvernanceMaia::depuis_fichier(&chemin);
        let _ = std::fs::remove_dir_all(&dir);
        g
    }

    pub(crate) fn candidat(texte: &str) -> Candidat {
        serde_json::from_value(serde_json::json!({
            "id": maia_memoire::Uuid::new_v4(), "cible": null, "branche": "sante.sommeil",
            "type_unite": "fait", "texte": texte, "importance": 5, "confiance": 9,
            "provenance": "utilisateur", "sensible": true, "propose_le": "2026-10-25T12:00:00Z"
        }))
        .expect("candidat")
    }

    fn action(texte: &str) -> ActionTamr {
        ActionTamr::ValiderCandidat {
            proprietaire: "alice".into(),
            candidat: candidat(texte),
            cible_actuelle: None,
        }
    }

    /// Signature d'approbation vérifiée (clé d'approbation de test).
    pub(crate) fn sig() -> SignatureApprobation {
        SignatureApprobation::verifiee(&[0xA5; 32])
    }

    #[test]
    fn approbation_accordee_liee_a_l_action_exacte() {
        let g = gouvernance(POLITIQUE_TAMR);
        assert!(g.est_operationnelle());
        let r = RegistreApprobations::new();
        let a = action("Dort 6 h");
        let ActionTamr::ValiderCandidat { candidat: k, .. } = a.clone() else {
            unreachable!()
        };
        let d = r.demander(a).expect("demande");
        assert_eq!(d.type_intervention, "APPROVAL");
        let ok = r
            .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
            .expect("accordée");
        assert_eq!(ok.regle(), "maia-tamr-approbation");
        assert_eq!(ok.empreinte(), d.empreinte);
        // Usage unique (R-APPR-2).
        assert_eq!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
                .unwrap_err(),
            RefusTamr::Inconnue
        );
        // L'approbation ne couvre pas un autre candidat ni une republication.
        let autre = maia_memoire::Uuid::new_v4();
        let d2 = r
            .demander(ActionTamr::ValiderCandidat {
                proprietaire: "alice".into(),
                candidat: k.clone(),
                cible_actuelle: None,
            })
            .expect("d2");
        let ok2 = r
            .approuver(&d2.intervention_id, "alice", &d2.empreinte, &sig(), &g)
            .expect("ok2");
        assert_eq!(
            ok2.validation_memoire(autre).unwrap_err(),
            RefusTamr::HorsPortee
        );
        assert!(matches!(
            ok.validation_republication(b"x"),
            Err(RefusTamr::HorsPortee)
        ));
        let d3 = r
            .demander(ActionTamr::ValiderCandidat {
                proprietaire: "alice".into(),
                candidat: k.clone(),
                cible_actuelle: None,
            })
            .expect("d3");
        let v = r
            .approuver(&d3.intervention_id, "alice", &d3.empreinte, &sig(), &g)
            .expect("ok3")
            .validation_memoire(k.id)
            .expect("v");
        let Validation::UtilisateurEtTamr(conf) = v else {
            panic!("TAMR attendu")
        };
        assert!(conf.couvre(&k));
    }

    #[test]
    fn refus_strongfather_absent_empreinte_approbateur_delai() {
        let r = RegistreApprobations::new();
        // Politiques absentes : refus (fail-closed).
        let absente =
            GouvernanceMaia::depuis_fichier(std::env::temp_dir().join("maia-tamr-absent.toml"));
        let d = r.demander(action("a")).expect("d");
        assert!(matches!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &absente),
            Err(RefusTamr::StrongFather(_))
        ));
        // Politiques valides sans règle TAMR : refus par défaut.
        let sans = gouvernance(POLITIQUE_SANS_TAMR);
        assert!(sans.est_operationnelle());
        let d = r.demander(action("a")).expect("d");
        let e = r
            .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &sans)
            .unwrap_err();
        assert!(
            matches!(&e, RefusTamr::StrongFather(m) if m.contains("défaut")),
            "{e}"
        );
        let g = gouvernance(POLITIQUE_TAMR);
        // Empreinte d'une autre action.
        let d = r.demander(action("a")).expect("d");
        let autre = action("b").empreinte().expect("empreinte");
        assert_eq!(
            r.approuver(&d.intervention_id, "alice", &autre, &sig(), &g)
                .unwrap_err(),
            RefusTamr::EmpreinteDifferente
        );
        assert_eq!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
                .unwrap_err(),
            RefusTamr::Inconnue,
            "consommée"
        );
        // Autre utilisateur.
        let d = r.demander(action("a")).expect("d");
        assert_eq!(
            r.approuver(&d.intervention_id, "bob", &d.empreinte, &sig(), &g)
                .unwrap_err(),
            RefusTamr::ApprobateurInvalide
        );
        // Délai dépassé.
        let court = RegistreApprobations::avec_duree(Duration::ZERO);
        let d = court.demander(action("a")).expect("d");
        std::thread::sleep(Duration::from_millis(5));
        assert_eq!(
            court
                .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
                .unwrap_err(),
            RefusTamr::Expiree
        );
        // Refus explicite.
        let d = r.demander(action("a")).expect("d");
        r.refuser(&d.intervention_id, "alice").expect("refus");
        assert_eq!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
                .unwrap_err(),
            RefusTamr::Inconnue
        );
    }

    #[test]
    fn republication_octets_exacts_sous_tamr() {
        use maia_federee::{autoriser, Effet, Provenance as P};
        let g = gouvernance(POLITIQUE_TAMR);
        let r = RegistreApprobations::new();
        let octets = b"fiche signee".to_vec();
        let description = ServiceRequete::description_republication(&octets);
        let d = r
            .demander(ActionTamr::Republier {
                proprietaire: "alice".into(),
                octets: octets.clone(),
            })
            .expect("d");
        let ok = r
            .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
            .expect("ok");
        assert!(matches!(
            ok.validation_memoire(maia_memoire::Uuid::new_v4()),
            Err(RefusTamr::HorsPortee)
        ));
        let d = r
            .demander(ActionTamr::Republier {
                proprietaire: "alice".into(),
                octets: octets.clone(),
            })
            .expect("d");
        let ok = r
            .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
            .expect("ok");
        assert_eq!(ok.action().objet(), "federee.republier");
        let v = ok.validation_republication(&octets).expect("v");
        assert!(v.par_tamr());
        assert!(autoriser(P::Etranger, Effet::Publication, Some(&v), &description).is_ok());
        assert!(autoriser(
            P::Etranger,
            Effet::Publication,
            Some(&v),
            &ServiceRequete::description_republication(b"autre")
        )
        .is_err());
        // Octets différents de ceux approuvés : refus.
        let d = r
            .demander(ActionTamr::Republier {
                proprietaire: "alice".into(),
                octets,
            })
            .expect("d");
        let ok = r
            .approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g)
            .expect("ok");
        assert_eq!(
            ok.validation_republication(b"autre").unwrap_err(),
            RefusTamr::HorsPortee
        );
    }

    #[test]
    fn intention_porte_l_empreinte_et_l_approbation() {
        let a = action("a");
        let e = a.empreinte().expect("empreinte");
        let i = intention_approbation("iv1", &a, &e, "alice", &sig(), 7);
        assert_eq!(i.action_type.as_str(), "modification");
        assert_eq!(i.subject, "maia.memoire.alice");
        assert_eq!(i.call_context.caller_identity, "alice");
        assert_eq!(i.data.content["intervention"], "APPROVAL");
        assert_eq!(i.data.content["empreinte"], e);
        assert_eq!(i.data.content["sensible"], "true");
        // F7 : canal signé, usage distinct, identifiant de la clé signataire.
        assert_eq!(i.data.content["canal"], "utilisateur_signe");
        assert_eq!(i.data.content["usage_signe"], USAGE_APPROBATION);
        assert_eq!(i.data.content["cle_approbation"], sig().cle());
        assert_eq!(sig().cle().len(), 64);
        assert_ne!(action("b").empreinte().expect("b"), e);
    }

    #[test]
    fn approbation_sans_cle_d_approbation_refusee() {
        // F7 : sans identifiant de clé d'approbation, StrongFather n'est même pas consulté.
        let g = gouvernance(POLITIQUE_TAMR);
        let r = RegistreApprobations::new();
        let d = r.demander(action("a")).expect("d");
        let vide = SignatureApprobation { cle: String::new() };
        assert_eq!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &vide, &g)
                .unwrap_err(),
            RefusTamr::SignatureRequise
        );
        // L'ancienne règle (attributs calculés, sans signature) n'autorise plus rien.
        let ancienne = POLITIQUE_TAMR.replace(
            "data.canal == 'utilisateur_signe' && data.usage_signe == 'maia/tamr-approbation' && data.cle_approbation != ''",
            "data.canal == 'utilisateur'",
        );
        let g2 = gouvernance(&ancienne);
        let d = r.demander(action("a")).expect("d");
        assert!(matches!(
            r.approuver(&d.intervention_id, "alice", &d.empreinte, &sig(), &g2),
            Err(RefusTamr::StrongFather(_))
        ));
    }

    #[test]
    fn description_et_empreinte_lient_la_version_de_la_cible() {
        // F8 : cible, texte actuel, provenance, importance, liens, sources affichés ;
        // la version courante de la cible est liée à l'empreinte.
        let note = |version: u32, texte: &str| -> Note {
            serde_json::from_value(serde_json::json!({
                "id": "sante.sommeil.f1", "version": version, "texte": texte, "importance": 6, "confiance": 9,
                "provenance": "utilisateur", "auteur": "utilisateur",
                "cree_le": "2026-10-25T12:00:00Z", "enregistre_le": "2026-10-25T12:00:00Z"
            }))
            .expect("note")
        };
        let mut k = candidat("Dort 7 h");
        k.cible = Some(maia_memoire::IdMemoire::parse("sante.sommeil.f1").expect("id"));
        k.sources = vec![maia_memoire::EmpreinteSource::parse(&"ab".repeat(32)).expect("src")];
        let v1 = ActionTamr::ValiderCandidat {
            proprietaire: "alice".into(),
            candidat: k.clone(),
            cible_actuelle: Some(Box::new(note(1, "Dort 6 h"))),
        };
        let v2 = ActionTamr::ValiderCandidat {
            proprietaire: "alice".into(),
            candidat: k,
            cible_actuelle: Some(Box::new(note(2, "Dort 8 h"))),
        };
        let d = v1.description();
        for attendu in [
            "sante.sommeil.f1",
            "version 1",
            "texte actuel : « Dort 6 h »",
            "« Dort 7 h »",
            "provenance utilisateur",
            "importance 5",
            "liens : aucun",
            &"ab".repeat(32),
        ] {
            assert!(d.contains(attendu), "{attendu} absent de : {d}");
        }
        assert_ne!(v1.empreinte().expect("v1"), v2.empreinte().expect("v2"));
        // La confirmation produite porte la version approuvée.
        let g = gouvernance(POLITIQUE_TAMR);
        let r = RegistreApprobations::new();
        let ActionTamr::ValiderCandidat { candidat: kk, .. } = v1.clone() else {
            unreachable!()
        };
        let dm = r.demander(v1).expect("d");
        let ok = r
            .approuver(&dm.intervention_id, "alice", &dm.empreinte, &sig(), &g)
            .expect("ok");
        let Validation::UtilisateurEtTamr(conf) = ok.validation_memoire(kk.id).expect("v") else {
            panic!("TAMR attendu")
        };
        assert_eq!(conf.version_cible(), Some(1));
    }
}
