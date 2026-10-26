//! @id     miyucloud_infrastructure_services_miyu_auth_service
//! @do     authentifier_visa_et_cle_appareil
//! @role   securite
//! @layer  app
//! @human  miyucloud — visa + preuve de possession (clients natifs) et requêtes signées par clé d'appareil.

//! Authentification sans secret partagé des clients natifs (inventaire #18 et #21, lot 2).
//!
//! # 1. Visa + preuve de possession (`MiyuVisa`)
//!
//! Schéma commun [`miyauth::http`] : le client présente
//! `Authorization: MiyuVisa <b64url(visa)>.<b64url(passeport)>` et
//! `X-Miyu-PoP: <b64url(id du défi)>.<b64url(signature)>`.
//!
//! - Le défi est émis par MiyuCloud (`POST /api/auth/miyu/defi`, audience
//!   `miyucloud`, 60 s, **usage unique**) et consommé avant toute vérification.
//! - La preuve est liée à la requête : `liaison_requete(méthode, chemin sans
//!   requête, corps)` (TLS terminé par un mandataire : pas d'exporteur TLS).
//! - Ancre de confiance (`MIYUCLOUD_MIYU_COG_ID` + l'un des deux modes) :
//!   - **chaîne (A-022, A-024)** : `MIYUCLOUD_MIYU_RACINE` = clé publique
//!     d'identité du COG (indice 0 ; `cog_id` = son empreinte, contrôlé) et
//!     `MIYUCLOUD_MIYU_DELEGATIONS` = délégations `0x45` en cours, au format de
//!     l'en-tête `X-Miyu-Delegations` (base64 URL séparées par `.`), en
//!     principe `emission` (indice 1, passeports) et `visas` (indice 5, visas).
//!     La racine connue active le **mode strict** : seules les clés déléguées
//!     signent, un visa signé par l'indice 1 est refusé. Une délégation
//!     invalide ou expirée au démarrage ⇒ refus de démarrer. Les délégations
//!     présentées avec un visa (`X-Miyu-Delegations`) sont acceptées par
//!     [`MiyuAuthService::verifier_visa_avec_delegations`] ;
//!   - **transition** : `MIYUCLOUD_MIYU_CLE_EMISSION` = clé d'émission épinglée
//!     directement (passeport et visa scellés par elle).
//!
//!   Niveau souverain ; aucune ancre ⇒ schéma désactivé (refus).
//! - Révocation : liste tirée du **registre vérifié** du COG d'origine
//!   ([`super::miyu_revocations`], `miyuregistre::RevocationsRegistre`) :
//!   passeport, visa, clé d'appareil et clé d'émission sont contrôlés à chaque
//!   requête ; fraîcheur **≤ 15 min** (A-007) ; liste absente ou périmée ⇒
//!   **refus**. Sans source configurée, tout visa est refusé (fail-closed).
//!   Côté MiyuCloud, retirer la liaison coupe aussi l'accès immédiatement ;
//!   un visa dont la clé d'appareil a été **retirée** du compte (révoquée dans
//!   MiyuCloud) est refusé lui aussi.
//! - Délégations présentées (`X-Miyu-Delegations`) : vérifiées contre la racine
//!   épinglée **dès le précontrôle** et pour la preuve complète (A-029).
//! - Défis et nonces **persistants** ([`MiyuAuthService::avec_etat_persistant`],
//!   `miyauth::http::persistant`) : un défi consommé ou un nonce vu reste
//!   refusé après redémarrage et entre instances partageant le dossier.
//! - Compte : l'identité prouvée (COG, sujet par paire) n'ouvre un compte que
//!   s'il a été **lié explicitement** par l'utilisateur connecté
//!   (`POST /api/auth/miyu/liaisons`). Jamais de création implicite.
//!
//! # 2. Requête signée par clé d'appareil (fin du flux RFC 8628)
//!
//! L'appareil génère une paire Ed25519, envoie sa clé publique au début du flux
//! d'appareil ; à l'approbation, la clé est enregistrée pour l'utilisateur (aucun
//! jeton n'est délivré). Chaque requête porte les en-têtes de [`miyauth::http`] §2
//! (`X-Miyu-Signataire`, `X-Miyu-Horodatage`, `X-Miyu-Nonce`, `X-Miyu-Signature`)
//! avec l'usage [`USAGE_APPAREIL`] ; le chemin signé est le chemin **et** la
//! requête tels qu'envoyés. Ordre fail-closed : clé enregistrée et active,
//! horodatage ± 300 s et nonce non vu **avant** la lecture du corps, puis signature.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use miyauth::http::{
    DefiEmis, EntetesSignees, MagasinDefis, MagasinDefisFichier, MagasinDefisMemoire, RefusHttp,
    RegistreNonces, RequeteHttp, analyser_autorisation_visa, analyser_delegations, analyser_pop,
    verifier_requete_signee, verifier_requete_visa_avec_delegations,
};
use miyauth::{AnnuaireEmetteurs, CogId, PolitiqueIdentite};
use miyusceau::NiveauConfiance;
use uuid::Uuid;

use super::miyu_liaison_appareil::{DemandeLiaisonAppareil, RefusLiaison};
use super::miyu_revocations::RevocationsMiyuCloud;
use crate::common::errors::{DomainError, ErrorKind};

/// Audience des visas acceptés par MiyuCloud.
pub const AUDIENCE_MIYUCLOUD: &str = "miyucloud";
/// Usage (séparation de domaine) des requêtes signées par clé d'appareil.
pub const USAGE_APPAREIL: &str = "miyucloud/appareil";
/// Corps maximal lu pour vérifier une preuve liée au corps (visa ou signature).
/// Les gros envois passent par l'API d'envoi par morceaux.
pub const TAILLE_MAX_CORPS_PROUVE: usize = 16 * 1024 * 1024;

/// Secondes Unix courantes.
pub fn maintenant() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

/// Lit une clé publique Ed25519 : 64 caractères hexadécimaux ou base64 URL (43).
pub fn analyser_cle_publique(texte: &str) -> Option<[u8; 32]> {
    let t = texte.trim();
    if t.len() == 64 {
        return hex::decode(t).ok().and_then(|v| v.try_into().ok());
    }
    miyauth::http::b64_decoder(t).and_then(|v| v.try_into().ok())
}

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Ancre du COG d'origine (émetteur des passeports et visas acceptés).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AncreCog {
    /// `cog_id` du COG d'origine.
    pub cog_id: CogId,
    /// Transition : clé publique d'émission épinglée directement.
    pub cle_emission: Option<[u8; 32]>,
    /// Chaîne : clé publique d'identité (racine) du COG ⇒ mode strict.
    pub racine: Option<[u8; 32]>,
    /// Chaîne : délégations `0x45` configurées (sceaux complets).
    pub delegations: Vec<Vec<u8>>,
}

impl AncreCog {
    /// Lit `MIYUCLOUD_MIYU_COG_ID` avec `MIYUCLOUD_MIYU_CLE_EMISSION`
    /// (transition) **ou** `MIYUCLOUD_MIYU_RACINE` (+ `MIYUCLOUD_MIYU_DELEGATIONS`).
    ///
    /// Tout absent : `Ok(None)` (schéma visa désactivé). Combinaison incomplète,
    /// les deux modes à la fois ou valeur invalide : erreur (fail-closed, le
    /// serveur refuse de démarrer).
    pub fn depuis_env() -> Result<Option<Self>, String> {
        Self::depuis_valeurs_chaine(
            std::env::var("MIYUCLOUD_MIYU_COG_ID").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_CLE_EMISSION").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_RACINE").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_DELEGATIONS").ok().as_deref(),
        )
    }

    /// Variante pure de [`Self::depuis_env`], mode de transition seulement.
    pub fn depuis_valeurs(cog: Option<&str>, cle: Option<&str>) -> Result<Option<Self>, String> {
        Self::depuis_valeurs_chaine(cog, cle, None, None)
    }

    /// Variante pure de [`Self::depuis_env`].
    pub fn depuis_valeurs_chaine(
        cog: Option<&str>,
        cle: Option<&str>,
        racine: Option<&str>,
        delegations: Option<&str>,
    ) -> Result<Option<Self>, String> {
        let nettoyer = |v: Option<&str>| v.map(str::trim).filter(|s| !s.is_empty()).map(str::to_string);
        let (cog, cle, racine, delegations) =
            (nettoyer(cog), nettoyer(cle), nettoyer(racine), nettoyer(delegations));
        let cog_id = match (&cog, &cle, &racine) {
            (None, None, None) if delegations.is_none() => return Ok(None),
            (Some(c), Some(_), None) | (Some(c), None, Some(_)) => {
                CogId::new(c).ok_or_else(|| format!("MIYUCLOUD_MIYU_COG_ID invalide « {c} »"))?
            }
            (Some(_), Some(_), Some(_)) => {
                return Err("MIYUCLOUD_MIYU_CLE_EMISSION et MIYUCLOUD_MIYU_RACINE s'excluent".to_string());
            }
            _ => {
                return Err(
                    "MIYUCLOUD_MIYU_COG_ID va avec MIYUCLOUD_MIYU_CLE_EMISSION ou MIYUCLOUD_MIYU_RACINE"
                        .to_string(),
                );
            }
        };
        let cle_emission = match cle {
            None => None,
            Some(k) => Some(analyser_cle_publique(&k).ok_or(
                "MIYUCLOUD_MIYU_CLE_EMISSION : clé publique Ed25519 (64 hex ou base64 URL) attendue",
            )?),
        };
        let racine = match racine {
            None => None,
            Some(k) => {
                let r = analyser_cle_publique(&k).ok_or(
                    "MIYUCLOUD_MIYU_RACINE : clé publique Ed25519 (64 hex ou base64 URL) attendue",
                )?;
                if miyusceau::cog_id_depuis_cle(&r) != cog_id.as_str() {
                    return Err("MIYUCLOUD_MIYU_RACINE : cog_id différent de MIYUCLOUD_MIYU_COG_ID".to_string());
                }
                Some(r)
            }
        };
        let delegations = match delegations {
            None => Vec::new(),
            Some(_) if racine.is_none() => {
                return Err("MIYUCLOUD_MIYU_DELEGATIONS exige MIYUCLOUD_MIYU_RACINE".to_string());
            }
            Some(d) => miyauth::http::analyser_delegations(&d)
                .map_err(|_| "MIYUCLOUD_MIYU_DELEGATIONS : délégations base64 URL séparées par « . »")?,
        };
        Ok(Some(Self {
            cog_id,
            cle_emission,
            racine,
            delegations,
        }))
    }

    /// Annuaire d'émetteurs de l'ancre ; racine ⇒ délégations vérifiées à
    /// `maintenant` et **mode strict**.
    pub fn annuaire(&self, maintenant: u64) -> Result<AnnuaireEmetteurs, String> {
        let mut annuaire = AnnuaireEmetteurs::new();
        match (self.cle_emission, self.racine) {
            (Some(cle), None) => {
                annuaire
                    .ajouter(self.cog_id.clone(), &cle, NiveauConfiance::Souverain)
                    .map_err(|e| format!("MIYUCLOUD_MIYU_CLE_EMISSION : {e}"))?;
            }
            (None, Some(racine)) => {
                annuaire
                    .epingler_racine(self.cog_id.clone(), &racine, NiveauConfiance::Souverain)
                    .map_err(|e| format!("MIYUCLOUD_MIYU_RACINE : {e}"))?;
                for d in &self.delegations {
                    annuaire
                        .ajouter_delegation(d, maintenant)
                        .map_err(|e| format!("MIYUCLOUD_MIYU_DELEGATIONS : {e}"))?;
                }
                annuaire.exiger_delegations();
            }
            _ => return Err("ancre du COG incomplète".to_string()),
        }
        Ok(annuaire)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Stockage
// ═══════════════════════════════════════════════════════════════════════════════

/// Identité Miyukini liée à un compte.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IdentiteLiee {
    pub id: Uuid,
    pub user_id: Uuid,
    pub cog_id: String,
    /// Sujet par paire (hexadécimal).
    pub sujet: String,
    pub label: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Clé d'appareil enregistrée (fin du flux d'appareil).
#[derive(Debug, Clone, serde::Serialize)]
pub struct CleAppareil {
    pub id: Uuid,
    pub user_id: Uuid,
    /// Clé publique (base64 URL).
    pub public_key: String,
    pub client_name: String,
    pub scopes: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Stockage des liaisons d'identité et des clés d'appareil.
pub trait MiyuIdentityStore: Send + Sync + 'static {
    /// Compte lié à `(cog_id, sujet)`.
    fn user_for_identity(
        &self,
        cog_id: &str,
        sujet: &[u8; 32],
    ) -> impl std::future::Future<Output = Result<Option<Uuid>, DomainError>> + Send;
    /// Lie `(cog_id, sujet)` à `user_id`. Déjà lié à un autre compte : erreur.
    fn link_identity(
        &self,
        user_id: Uuid,
        cog_id: &str,
        sujet: &[u8; 32],
        label: &str,
    ) -> impl std::future::Future<Output = Result<IdentiteLiee, DomainError>> + Send;
    /// Liaisons d'un compte.
    fn list_identities(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<IdentiteLiee>, DomainError>> + Send;
    /// Retire une liaison du compte. Vrai si elle existait.
    fn unlink_identity(
        &self,
        user_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DomainError>> + Send;
    /// Enregistre une clé d'appareil. Clé déjà connue : erreur.
    fn register_device_key(
        &self,
        user_id: Uuid,
        public_key: &[u8; 32],
        client_name: &str,
        scopes: &str,
    ) -> impl std::future::Future<Output = Result<CleAppareil, DomainError>> + Send;
    /// Clé d'appareil **active** correspondant à `public_key`.
    fn active_device_key(
        &self,
        public_key: &[u8; 32],
    ) -> impl std::future::Future<Output = Result<Option<CleAppareil>, DomainError>> + Send;
    /// Clés actives d'un compte.
    fn list_device_keys(
        &self,
        user_id: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<CleAppareil>, DomainError>> + Send;
    /// Révoque une clé du compte. Vrai si elle existait.
    fn revoke_device_key(
        &self,
        user_id: Uuid,
        id: Uuid,
    ) -> impl std::future::Future<Output = Result<bool, DomainError>> + Send;
    /// Vrai si `public_key` a été enregistrée puis **révoquée** (retirée du
    /// compte) : les visas portant cette clé d'appareil sont refusés.
    fn device_key_revoked(
        &self,
        public_key: &[u8; 32],
    ) -> impl std::future::Future<Output = Result<bool, DomainError>> + Send;
    /// Rotation **atomique** : enregistre `nouvelle` pour `user_id` avec le nom
    /// et les portées de la clé `ancienne_id` (active, du même compte), puis
    /// révoque celle-ci. `None` si l'ancienne n'est pas une clé active du
    /// compte ; nouvelle clé déjà connue : erreur (rien n'est changé).
    fn rotate_device_key(
        &self,
        user_id: Uuid,
        ancienne_id: Uuid,
        nouvelle: &[u8; 32],
    ) -> impl std::future::Future<Output = Result<Option<CleAppareil>, DomainError>> + Send;
}

// ═══════════════════════════════════════════════════════════════════════════════
// Refus
// ═══════════════════════════════════════════════════════════════════════════════

/// Refus d'authentification (toute variante = refus).
#[derive(Debug)]
pub enum RefusMiyu {
    /// Refus du schéma (en-têtes, défi, titres, signature, rejeu…).
    Http(RefusHttp),
    /// Schéma visa non configuré (aucune ancre épinglée).
    VisaDesactive,
    /// Identité prouvée mais liée à aucun compte.
    IdentiteNonLiee,
    /// Clé d'appareil inconnue ou révoquée.
    AppareilInconnu,
    /// Visa portant une clé d'appareil retirée du compte.
    AppareilRetire,
    /// Demande de liaison d'appareil refusée.
    Liaison(RefusLiaison),
    /// Clé à lier déjà enregistrée (ou demande déjà utilisée).
    CleDejaLiee,
    /// Portées demandées invalides.
    PorteesInvalides,
    /// Stockage indisponible.
    Stockage(DomainError),
}

impl RefusMiyu {
    /// Statut HTTP conseillé.
    pub fn statut(&self) -> u16 {
        match self {
            Self::Http(r) => r.statut(),
            Self::Stockage(_) => 503,
            Self::Liaison(_) | Self::PorteesInvalides => 400,
            Self::CleDejaLiee => 409,
            _ => 401,
        }
    }
}

impl std::fmt::Display for RefusMiyu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http(r) => write!(f, "{r}"),
            Self::VisaDesactive => write!(f, "visa non accepté : aucun COG d'origine épinglé"),
            Self::IdentiteNonLiee => write!(f, "identité Miyukini liée à aucun compte"),
            Self::AppareilInconnu => write!(f, "clé d'appareil inconnue ou révoquée"),
            Self::AppareilRetire => write!(f, "clé d'appareil retirée du compte"),
            Self::Liaison(r) => write!(f, "{r}"),
            Self::CleDejaLiee => write!(f, "clé d'appareil déjà liée ou demande déjà utilisée"),
            Self::PorteesInvalides => write!(f, "portées invalides (webdav, caldav, carddav, api)"),
            Self::Stockage(_) => write!(f, "stockage d'identités indisponible"),
        }
    }
}

impl From<RefusHttp> for RefusMiyu {
    fn from(r: RefusHttp) -> Self {
        Self::Http(r)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Service
// ═══════════════════════════════════════════════════════════════════════════════

struct HoteVisa {
    cog: CogId,
    annuaire: AnnuaireEmetteurs,
    /// Révocations du registre du COG d'origine ([`RevocationsMiyuCloud`]).
    revocations: RevocationsMiyuCloud,
}

impl HoteVisa {
    fn politique(&self) -> PolitiqueIdentite<'_> {
        PolitiqueIdentite {
            cog_local: self.cog.clone(),
            annuaire: &self.annuaire,
            revocations: &self.revocations,
        }
    }
}

/// Portées de visa reconnues par MiyuCloud (mêmes noms que les portées des
/// mots de passe d'application). Toute autre portée n'accorde rien.
pub const PORTEES_VISA: &[&str] = &["webdav", "caldav", "carddav", "api"];

/// Portées d'un visa → liste `a,b` comprise par `app_password_allows`
/// (fail-closed : portée inconnue ignorée ; `all` n'est pas reconnue).
pub fn portees_visa_csv(portees: &[String]) -> String {
    let mut retenues: Vec<&str> = Vec::new();
    for p in portees {
        if let Some(k) = PORTEES_VISA.iter().find(|k| **k == p.as_str())
            && !retenues.contains(k)
        {
            retenues.push(k);
        }
    }
    retenues.join(",")
}

/// Portées d'une clé d'appareil liée (`a,b`, au moins une, toutes connues) ;
/// `None` sinon (fail-closed : jamais de portée ignorée en silence ici).
pub fn portees_appareil(scopes: &str) -> Option<String> {
    let mut retenues: Vec<&str> = Vec::new();
    for p in scopes.split(',').map(str::trim).filter(|p| !p.is_empty()) {
        let k = PORTEES_VISA.iter().find(|k| **k == p)?;
        if !retenues.contains(k) {
            retenues.push(k);
        }
    }
    (!retenues.is_empty()).then(|| retenues.join(","))
}

/// Identité prouvée par un visa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentiteVisa {
    pub cog_id: String,
    pub sujet: [u8; 32],
    /// Clé d'appareil du visa (celle qui a signé la preuve de possession).
    pub cle_appareil: [u8; 32],
    /// Portées MiyuCloud accordées par le visa (`webdav,api`…), peut être vide.
    pub portees: String,
}

/// Requête vue par l'authentification (indépendante d'axum).
#[derive(Debug, Clone, Copy)]
pub struct RequeteProuvee<'a> {
    pub methode: &'a str,
    /// Chemin sans requête (liaison de la preuve de visa).
    pub chemin: &'a str,
    /// Chemin et requête tels qu'envoyés (requêtes signées).
    pub chemin_et_requete: &'a str,
    pub corps: &'a [u8],
}

/// Service d'authentification Miyukini de MiyuCloud.
pub struct MiyuAuthService<S: MiyuIdentityStore> {
    store: Arc<S>,
    hote: Option<HoteVisa>,
    defis: Box<dyn MagasinDefis>,
    nonces: RegistreNonces,
}

impl<S: MiyuIdentityStore> MiyuAuthService<S> {
    /// Service adossé à `store`, avec l'ancre `ancre` (ou sans schéma visa).
    pub fn new(store: Arc<S>, ancre: Option<AncreCog>) -> Result<Self, String> {
        let hote = match ancre {
            None => None,
            Some(a) => {
                let annuaire = a.annuaire(maintenant())?;
                Some(HoteVisa {
                    // Fail-closed : sans source branchée, tout visa est refusé.
                    revocations: RevocationsMiyuCloud::absentes(),
                    cog: a.cog_id,
                    annuaire,
                })
            }
        };
        Ok(Self {
            store,
            hote,
            defis: Box::new(MagasinDefisMemoire::default()),
            nonces: RegistreNonces::default(),
        })
    }

    /// Défis et nonces **persistants et partagés** dans `dossier` (sous-dossiers
    /// `defis` et `nonces`, créés `0700` ; refus s'ils sont des liens ou ouverts
    /// aux « autres ») : rejeu refusé après redémarrage et entre instances.
    pub fn avec_etat_persistant(mut self, dossier: &Path) -> Result<Self, String> {
        self.defis = Box::new(
            MagasinDefisFichier::ouvrir_defaut(&dossier.join("defis"))
                .map_err(|e| format!("défis persistants ({}) : {e}", dossier.display()))?,
        );
        self.nonces = RegistreNonces::persistant(
            &dossier.join("nonces"),
            miyauth::http::CAPACITE_NONCES_DEFAUT,
            miyauth::http::FENETRE_HORODATAGE_SECS,
        )
        .map_err(|e| format!("nonces persistants ({}) : {e}", dossier.display()))?;
        Ok(self)
    }

    /// Branche la source de révocations (registre vérifié du COG d'origine).
    /// Sans appel, tout visa est refusé (liste absente). Sans schéma visa :
    /// sans effet.
    pub fn avec_revocations(mut self, revocations: RevocationsMiyuCloud) -> Self {
        if let Some(h) = self.hote.as_mut() {
            h.revocations = revocations;
        }
        self
    }

    /// Stockage sous-jacent.
    pub fn store(&self) -> &Arc<S> {
        &self.store
    }

    /// Vrai si un COG d'origine est épinglé.
    pub fn visa_active(&self) -> bool {
        self.hote.is_some()
    }

    /// `cog_id` du COG d'origine épinglé.
    pub fn cog_id(&self) -> Option<&str> {
        self.hote.as_ref().map(|h| h.cog.as_str())
    }

    /// Émet un défi de possession (audience `miyucloud`, usage unique).
    pub fn emettre_defi(&self, maintenant: u64) -> Result<DefiEmis, RefusMiyu> {
        if self.hote.is_none() {
            return Err(RefusMiyu::VisaDesactive);
        }
        Ok(self.defis.emettre(AUDIENCE_MIYUCLOUD, maintenant)?)
    }

    /// Précontrôle **avant lecture du corps** : schéma actif, en-têtes bien
    /// formés, délégations présentées (`X-Miyu-Delegations`) vérifiées contre
    /// la racine épinglée, visa et passeport scellés par une ancre épinglée,
    /// audience, fenêtres et révocation. Ne consomme pas le défi (la preuve
    /// liée au corps est vérifiée ensuite par [`Self::verifier_visa_avec_delegations`]).
    pub fn precontroler_visa(
        &self,
        autorisation: Option<&str>,
        pop: Option<&str>,
        delegations: Option<&str>,
        maintenant: u64,
    ) -> Result<(), RefusMiyu> {
        let hote = self.hote.as_ref().ok_or(RefusMiyu::VisaDesactive)?;
        let (visa, passeport) = analyser_autorisation_visa(autorisation.unwrap_or(""))?;
        analyser_pop(pop.unwrap_or(""))?;
        let presentees = delegations
            .map(analyser_delegations)
            .transpose()?
            .unwrap_or_default();
        let refs: Vec<&[u8]> = presentees.iter().map(Vec::as_slice).collect();
        let annuaire = hote
            .annuaire
            .avec_delegations(&refs, maintenant)
            .map_err(|e| RefusMiyu::Http(RefusHttp::Identite(e)))?;
        let politique = PolitiqueIdentite {
            cog_local: hote.cog.clone(),
            annuaire: &annuaire,
            revocations: &hote.revocations,
        };
        miyauth::verifier_visa(&visa, &passeport, AUDIENCE_MIYUCLOUD, &politique, maintenant)
            .map_err(|e| RefusMiyu::Http(RefusHttp::Identite(e)))?;
        Ok(())
    }

    /// Vérifie `MiyuVisa` + `X-Miyu-PoP` (défi consommé d'abord) et rend
    /// l'identité prouvée, sans consulter le stockage.
    pub fn verifier_visa(
        &self,
        requete: RequeteProuvee<'_>,
        autorisation: Option<&str>,
        pop: Option<&str>,
        maintenant: u64,
    ) -> Result<IdentiteVisa, RefusMiyu> {
        self.verifier_visa_avec_delegations(requete, autorisation, pop, None, maintenant)
    }

    /// Comme [`Self::verifier_visa`], avec les délégations `0x45` présentées
    /// (`X-Miyu-Delegations`, valeur brute) : vérifiées contre la racine épinglée.
    pub fn verifier_visa_avec_delegations(
        &self,
        requete: RequeteProuvee<'_>,
        autorisation: Option<&str>,
        pop: Option<&str>,
        delegations: Option<&str>,
        maintenant: u64,
    ) -> Result<IdentiteVisa, RefusMiyu> {
        let hote = self.hote.as_ref().ok_or(RefusMiyu::VisaDesactive)?;
        let req = RequeteHttp {
            methode: requete.methode,
            chemin: requete.chemin,
            corps: requete.corps,
            autorisation,
            pop,
            liaison: None,
        };
        let acces = verifier_requete_visa_avec_delegations(
            &req,
            delegations,
            AUDIENCE_MIYUCLOUD,
            &hote.politique(),
            self.defis.as_ref(),
            maintenant,
        )?;
        Ok(IdentiteVisa {
            cog_id: acces.cog_origine().as_str().to_string(),
            sujet: *acces.sujet(),
            cle_appareil: acces.acces().visa().cle_appareil,
            portees: portees_visa_csv(acces.acces().visa().portees.as_slice()),
        })
    }

    /// Visa + PoP (+ délégations présentées) puis liaison : rend le compte
    /// MiyuCloud et les portées du visa (à appliquer au chemin par l'appelant).
    /// Clé d'appareil du visa retirée du compte : refus.
    pub async fn authentifier_visa(
        &self,
        requete: RequeteProuvee<'_>,
        autorisation: Option<&str>,
        pop: Option<&str>,
        delegations: Option<&str>,
        maintenant: u64,
    ) -> Result<(Uuid, String), RefusMiyu> {
        let identite =
            self.verifier_visa_avec_delegations(requete, autorisation, pop, delegations, maintenant)?;
        if self
            .store
            .device_key_revoked(&identite.cle_appareil)
            .await
            .map_err(RefusMiyu::Stockage)?
        {
            return Err(RefusMiyu::AppareilRetire);
        }
        let user = self
            .store
            .user_for_identity(&identite.cog_id, &identite.sujet)
            .await
            .map_err(RefusMiyu::Stockage)?
            .ok_or(RefusMiyu::IdentiteNonLiee)?;
        Ok((user, identite.portees))
    }

    /// Étape 1 d'une requête signée (avant lecture du corps) : en-têtes,
    /// horodatage, clé d'appareil enregistrée et active, nonce non vu.
    pub async fn preparer_appareil(
        &self,
        signataire: Option<&str>,
        horodatage: Option<&str>,
        nonce: Option<&str>,
        signature: Option<&str>,
        maintenant: u64,
    ) -> Result<(EntetesSignees, CleAppareil), RefusMiyu> {
        let entetes = EntetesSignees::analyser(signataire, horodatage, nonce, signature)?;
        entetes.controler_horodatage(maintenant, self.nonces.fenetre())?;
        let cle = self
            .store
            .active_device_key(&entetes.signataire)
            .await
            .map_err(RefusMiyu::Stockage)?
            .ok_or(RefusMiyu::AppareilInconnu)?;
        if self.nonces.deja_vu(&entetes.kid(), &entetes.nonce) {
            return Err(RefusMiyu::Http(RefusHttp::Rejeu));
        }
        Ok((entetes, cle))
    }

    /// Étape 2 d'une requête signée : signature sur le corps lu, nonce enregistré.
    pub fn verifier_appareil(
        &self,
        entetes: &EntetesSignees,
        requete: RequeteProuvee<'_>,
        maintenant: u64,
    ) -> Result<(), RefusMiyu> {
        Ok(verifier_requete_signee(
            entetes,
            USAGE_APPAREIL,
            requete.methode,
            requete.chemin_et_requete,
            requete.corps,
            &self.nonces,
            maintenant,
        )?)
    }

    /// Vérifie une demande de liaison `MLA2` pour le COG épinglé et brûle sa
    /// preuve (usage unique, registre de nonces persistant).
    fn verifier_demande(&self, texte: &str, maintenant: u64) -> Result<DemandeLiaisonAppareil, RefusMiyu> {
        let hote = self.hote.as_ref().ok_or(RefusMiyu::VisaDesactive)?;
        let demande = DemandeLiaisonAppareil::decoder(texte).map_err(RefusMiyu::Liaison)?;
        demande.verifier(&hote.cog, maintenant).map_err(RefusMiyu::Liaison)?;
        let kid = miyauth::passeport::kid_appareil(&demande.cle);
        self.nonces
            .consommer(kid, &demande.preuve, maintenant)
            .map_err(|r| match r {
                RefusHttp::Rejeu => RefusMiyu::CleDejaLiee,
                autre => RefusMiyu::Http(autre),
            })?;
        Ok(demande)
    }

    /// Lie une nouvelle clé d'appareil au compte `user_id` depuis une **session
    /// de confiance** (navigateur) : demande `MLA2` simple (possession prouvée
    /// par la nouvelle clé), portées explicites. Une demande de rotation est
    /// refusée ici (elle passe par [`Self::faire_tourner_appareil`]).
    pub async fn lier_appareil(
        &self,
        user_id: Uuid,
        demande: &str,
        nom: &str,
        portees: &str,
        maintenant: u64,
    ) -> Result<CleAppareil, RefusMiyu> {
        let portees = portees_appareil(portees).ok_or(RefusMiyu::PorteesInvalides)?;
        // Forme contrôlée avant de brûler la preuve.
        if DemandeLiaisonAppareil::decoder(demande)
            .map_err(RefusMiyu::Liaison)?
            .autorisation
            .is_some()
        {
            return Err(RefusMiyu::Liaison(RefusLiaison::Format));
        }
        let d = self.verifier_demande(demande, maintenant)?;
        let nom: String = nom
            .trim()
            .chars()
            .filter(|c| !c.is_control())
            .take(255)
            .collect();
        let nom = if nom.is_empty() { "Appareil".to_string() } else { nom };
        self.store
            .register_device_key(user_id, &d.cle, &nom, &portees)
            .await
            .map_err(|e| match e.kind {
                ErrorKind::AlreadyExists => RefusMiyu::CleDejaLiee,
                _ => RefusMiyu::Stockage(e),
            })
    }

    /// Rotation demandée par l'appareil lui-même, requête **signée par
    /// l'ancienne clé** `signataire` (déjà authentifiée par le middleware) :
    /// la demande `MLA2` de rotation doit être autorisée par cette même clé ;
    /// la nouvelle reçoit son nom et ses portées, l'ancienne est révoquée
    /// dans la même transaction.
    pub async fn faire_tourner_appareil(
        &self,
        signataire: &CleAppareil,
        demande: &str,
        maintenant: u64,
    ) -> Result<CleAppareil, RefusMiyu> {
        let lue = DemandeLiaisonAppareil::decoder(demande).map_err(RefusMiyu::Liaison)?;
        let Some((ancienne, _)) = lue.autorisation else {
            return Err(RefusMiyu::Liaison(RefusLiaison::Format));
        };
        if miyauth::http::b64_encoder(&ancienne) != signataire.public_key {
            // Requête signée par une autre clé que celle qui autorise la rotation.
            return Err(RefusMiyu::Liaison(RefusLiaison::Signature));
        }
        let d = self.verifier_demande(demande, maintenant)?;
        match self
            .store
            .rotate_device_key(signataire.user_id, signataire.id, &d.cle)
            .await
        {
            Ok(Some(c)) => Ok(c),
            Ok(None) => Err(RefusMiyu::AppareilInconnu),
            Err(e) if e.kind == ErrorKind::AlreadyExists => Err(RefusMiyu::CleDejaLiee),
            Err(e) => Err(RefusMiyu::Stockage(e)),
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Stockage mémoire (tests)
// ═══════════════════════════════════════════════════════════════════════════════

/// Clé d'appareil en mémoire : (fiche, clé publique, active).
type EntreeCle = (CleAppareil, [u8; 32], bool);

/// Stockage en mémoire (tests et outils) : mêmes règles que PostgreSQL.
#[derive(Debug, Default)]
pub struct MemoireIdentites {
    identites: std::sync::Mutex<HashMap<Uuid, (IdentiteLiee, [u8; 32])>>,
    cles: std::sync::Mutex<HashMap<Uuid, EntreeCle>>,
}

fn erreur_conflit(message: &str) -> DomainError {
    DomainError::new(
        ErrorKind::AlreadyExists,
        "MiyuIdentity",
        message.to_string(),
    )
}

impl MiyuIdentityStore for MemoireIdentites {
    async fn user_for_identity(
        &self,
        cog_id: &str,
        sujet: &[u8; 32],
    ) -> Result<Option<Uuid>, DomainError> {
        let m = self.identites.lock().unwrap_or_else(|e| e.into_inner());
        Ok(m.values()
            .find(|(i, s)| i.cog_id == cog_id && s == sujet)
            .map(|(i, _)| i.user_id))
    }

    async fn link_identity(
        &self,
        user_id: Uuid,
        cog_id: &str,
        sujet: &[u8; 32],
        label: &str,
    ) -> Result<IdentiteLiee, DomainError> {
        let mut m = self.identites.lock().unwrap_or_else(|e| e.into_inner());
        if m.values().any(|(i, s)| i.cog_id == cog_id && s == sujet) {
            return Err(erreur_conflit("identity already linked"));
        }
        let i = IdentiteLiee {
            id: Uuid::new_v4(),
            user_id,
            cog_id: cog_id.to_string(),
            sujet: hex::encode(sujet),
            label: label.to_string(),
            created_at: chrono::Utc::now(),
            last_used_at: None,
        };
        m.insert(i.id, (i.clone(), *sujet));
        Ok(i)
    }

    async fn list_identities(&self, user_id: Uuid) -> Result<Vec<IdentiteLiee>, DomainError> {
        let m = self.identites.lock().unwrap_or_else(|e| e.into_inner());
        Ok(m.values()
            .filter(|(i, _)| i.user_id == user_id)
            .map(|(i, _)| i.clone())
            .collect())
    }

    async fn unlink_identity(&self, user_id: Uuid, id: Uuid) -> Result<bool, DomainError> {
        let mut m = self.identites.lock().unwrap_or_else(|e| e.into_inner());
        if m.get(&id).is_some_and(|(i, _)| i.user_id == user_id) {
            m.remove(&id);
            return Ok(true);
        }
        Ok(false)
    }

    async fn register_device_key(
        &self,
        user_id: Uuid,
        public_key: &[u8; 32],
        client_name: &str,
        scopes: &str,
    ) -> Result<CleAppareil, DomainError> {
        let mut m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        if m.values().any(|(_, k, _)| k == public_key) {
            return Err(erreur_conflit("device key already registered"));
        }
        let c = CleAppareil {
            id: Uuid::new_v4(),
            user_id,
            public_key: miyauth::http::b64_encoder(public_key),
            client_name: client_name.to_string(),
            scopes: scopes.to_string(),
            created_at: chrono::Utc::now(),
            last_used_at: None,
        };
        m.insert(c.id, (c.clone(), *public_key, true));
        Ok(c)
    }

    async fn active_device_key(
        &self,
        public_key: &[u8; 32],
    ) -> Result<Option<CleAppareil>, DomainError> {
        let m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        Ok(m.values()
            .find(|(_, k, actif)| *actif && k == public_key)
            .map(|(c, _, _)| c.clone()))
    }

    async fn list_device_keys(&self, user_id: Uuid) -> Result<Vec<CleAppareil>, DomainError> {
        let m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        Ok(m.values()
            .filter(|(c, _, actif)| *actif && c.user_id == user_id)
            .map(|(c, _, _)| c.clone())
            .collect())
    }

    async fn revoke_device_key(&self, user_id: Uuid, id: Uuid) -> Result<bool, DomainError> {
        let mut m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        match m.get_mut(&id) {
            Some((c, _, actif)) if c.user_id == user_id && *actif => {
                *actif = false;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    async fn device_key_revoked(&self, public_key: &[u8; 32]) -> Result<bool, DomainError> {
        let m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        Ok(m.values().any(|(_, k, actif)| !*actif && k == public_key))
    }

    async fn rotate_device_key(
        &self,
        user_id: Uuid,
        ancienne_id: Uuid,
        nouvelle: &[u8; 32],
    ) -> Result<Option<CleAppareil>, DomainError> {
        // Un seul verrou : vérification, insertion et révocation indivisibles.
        let mut m = self.cles.lock().unwrap_or_else(|e| e.into_inner());
        let Some((ancienne, _, true)) = m.get(&ancienne_id).cloned() else {
            return Ok(None);
        };
        if ancienne.user_id != user_id {
            return Ok(None);
        }
        if m.values().any(|(_, k, _)| k == nouvelle) {
            return Err(erreur_conflit("device key already registered"));
        }
        let c = CleAppareil {
            id: Uuid::new_v4(),
            user_id,
            public_key: miyauth::http::b64_encoder(nouvelle),
            client_name: ancienne.client_name.clone(),
            scopes: ancienne.scopes.clone(),
            created_at: chrono::Utc::now(),
            last_used_at: None,
        };
        m.insert(c.id, (c.clone(), *nouvelle, true));
        if let Some((_, _, actif)) = m.get_mut(&ancienne_id) {
            *actif = false;
        }
        Ok(Some(c))
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::infrastructure::services::miyu_revocations::FRAICHEUR_REVOCATIONS_SECS;
    use crate::infrastructure::services::miyu_revocations::banc::RegistreCog;
    use miyauth::http::{
        b64_encoder, formater_autorisation_visa, formater_pop, liaison_requete, signer_pop,
        signer_requete,
    };
    use miyauth::{CibleRevocation, ErreurRevocation, RefusIdentite};
    use miyauth::{
        DemandePasseport, DemandeVisa, Portees, SecretTitulaire, emettre_passeport, emettre_visa,
        prouver_possession, verifier_passeport,
    };
    use miyusceau::CleSignature;

    /// Révocations vues par le COG émetteur au moment d'émettre (test).
    struct VueCogEmetteur;

    impl miyauth::SourceRevocation for VueCogEmetteur {
        fn est_revoque(
            &self,
            _emetteur: &CogId,
            _cible: &CibleRevocation,
            _maintenant: u64,
        ) -> Result<bool, ErreurRevocation> {
            Ok(false)
        }
    }

    /// COG d'origine (avec son registre), titulaire et appareil de test.
    pub(crate) struct Monde {
        pub cle_cog: CleSignature,
        pub cog: CogId,
        pub secret: SecretTitulaire,
        pub appareil: CleSignature,
        pub registre: RegistreCog,
        pub revocations: RevocationsMiyuCloud,
        pub service: MiyuAuthService<MemoireIdentites>,
    }

    impl Monde {
        /// Monde dont les révocations sont relevées (tête fraîche, aucune).
        pub(crate) fn new() -> Self {
            let mut m = Self::sans_releve();
            m.registre.relever(&m.revocations, maintenant());
            m
        }

        /// Monde dont la source de révocations n'a encore rien relevé.
        pub(crate) fn sans_releve() -> Self {
            let cle_cog = CleSignature::generer();
            let cog = CogId::new(&miyuwebway_participant::identite::cog_id_depuis_cle(
                &cle_cog.cle_publique(),
            ))
            .unwrap();
            let registre = RegistreCog::new();
            let revocations = registre.source(&cog);
            let service = MiyuAuthService::new(
                Arc::new(MemoireIdentites::default()),
                Some(AncreCog {
                    cog_id: cog.clone(),
                    cle_emission: Some(cle_cog.cle_publique()),
                    racine: None,
                    delegations: Vec::new(),
                }),
            )
            .unwrap()
            .avec_revocations(revocations.clone());
            Self {
                cle_cog,
                cog,
                secret: SecretTitulaire::generer(),
                appareil: CleSignature::generer(),
                registre,
                revocations,
                service,
            }
        }

        /// Passeport (émis par `emetteur`, clé `cle`) et visa `miyucloud` du COG d'origine.
        pub(crate) fn titres_par(&self, cle: &CleSignature, audience: &str) -> (Vec<u8>, Vec<u8>) {
            let t = maintenant();
            let demande = DemandePasseport {
                secret: &self.secret,
                destination: self.cog.clone(),
                cle_appareil: self.appareil.cle_publique(),
                portees: Portees::new(["fichiers"]).unwrap(),
                pas_avant: t - 10,
                pas_apres: t + 3600,
            };
            let passeport = emettre_passeport(&self.cog, cle, &demande).unwrap();
            // Possession prouvée côté COG (défi quelconque) pour émettre le visa :
            // vue du COG émetteur (sa clé, ses révocations à l'émission), pas
            // celle de MiyuCloud.
            let mut annuaire = AnnuaireEmetteurs::new();
            annuaire
                .ajouter(
                    self.cog.clone(),
                    &cle.cle_publique(),
                    NiveauConfiance::Souverain,
                )
                .unwrap();
            let pol = PolitiqueIdentite {
                cog_local: self.cog.clone(),
                annuaire: &annuaire,
                revocations: &VueCogEmetteur,
            };
            let pv = verifier_passeport(&passeport, &pol, t).unwrap();
            let defi = [7u8; 32];
            let liaison = [9u8; 32];
            let sig = signer_pop(&self.appareil, &defi, "cog", &liaison);
            let possession = prouver_possession(pv, &defi, "cog", &liaison, &sig).unwrap();
            let visa = emettre_visa(
                &self.cog,
                cle,
                &possession,
                &DemandeVisa {
                    audience: audience.to_string(),
                    portees: Portees::new(["fichiers"]).unwrap(),
                    pas_avant: t - 10,
                    pas_apres: t + 600,
                },
            )
            .unwrap();
            (visa, passeport)
        }

        pub(crate) fn titres(&self) -> (Vec<u8>, Vec<u8>) {
            self.titres_par(&self.cle_cog, AUDIENCE_MIYUCLOUD)
        }

        /// En-têtes `(Authorization, X-Miyu-PoP)` pour une requête.
        pub(crate) fn entetes_visa(
            &self,
            titres: &(Vec<u8>, Vec<u8>),
            methode: &str,
            chemin: &str,
            corps: &[u8],
        ) -> (String, String) {
            let d = self.service.emettre_defi(maintenant()).unwrap();
            let liaison = liaison_requete(methode, chemin, corps);
            let sig = signer_pop(&self.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
            (
                formater_autorisation_visa(&titres.0, &titres.1),
                formater_pop(&d.id, &sig),
            )
        }
    }

    fn req<'a>(methode: &'a str, chemin: &'a str, corps: &'a [u8]) -> RequeteProuvee<'a> {
        RequeteProuvee {
            methode,
            chemin,
            chemin_et_requete: chemin,
            corps,
        }
    }

    fn bloquer<F: std::future::Future>(f: F) -> F::Output {
        tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap()
            .block_on(f)
    }

    #[test]
    fn anchor_config_is_all_or_nothing() {
        assert_eq!(AncreCog::depuis_valeurs(None, None).unwrap(), None);
        assert!(AncreCog::depuis_valeurs(Some("x"), None).is_err());
        assert!(AncreCog::depuis_valeurs(None, Some(&"11".repeat(32))).is_err());
        let cle = CleSignature::generer().cle_publique();
        let cog = miyuwebway_participant::identite::cog_id_depuis_cle(&cle);
        let a = AncreCog::depuis_valeurs(Some(&cog), Some(&hex::encode(cle)))
            .unwrap()
            .unwrap();
        assert_eq!(a.cle_emission, Some(cle));
        let b = AncreCog::depuis_valeurs(Some(&cog), Some(&b64_encoder(&cle)))
            .unwrap()
            .unwrap();
        assert_eq!(a, b);
        assert!(AncreCog::depuis_valeurs(Some(&cog), Some("pas une clé")).is_err());
    }

    #[test]
    fn anchor_chain_mode_is_strict_and_checks_delegations() {
        use miyusceau::DelegationCle;
        let identite = CleSignature::generer();
        let racine = identite.cle_publique();
        let cog = miyusceau::cog_id_depuis_cle(&racine);
        let visas = CleSignature::generer();
        let t = maintenant();
        let delegation = |debut: u64, fin: u64| {
            DelegationCle::nouvelle(&cog, 5, 0, visas.cle_publique(), debut, fin)
                .unwrap()
                .sceller(&identite)
                .unwrap()
        };
        let valide = b64_encoder(&delegation(t - 60, t + 86_400));
        let a = AncreCog::depuis_valeurs_chaine(Some(&cog), None, Some(&hex::encode(racine)), Some(&valide))
            .unwrap()
            .unwrap();
        let annuaire = a.annuaire(t).unwrap();
        assert!(annuaire.est_strict() && annuaire.delegation(&visas.kid()).is_some());
        // Les deux modes à la fois, racine d'un autre COG, délégations sans racine : refus.
        let cle = hex::encode(CleSignature::generer().cle_publique());
        assert!(AncreCog::depuis_valeurs_chaine(Some(&cog), Some(&cle), Some(&hex::encode(racine)), None).is_err());
        let autre = miyusceau::cog_id_depuis_cle(&CleSignature::generer().cle_publique());
        assert!(AncreCog::depuis_valeurs_chaine(Some(&autre), None, Some(&hex::encode(racine)), None).is_err());
        assert!(AncreCog::depuis_valeurs_chaine(Some(&cog), Some(&cle), None, Some(&valide)).is_err());
        // Délégation expirée au démarrage : refus de démarrer.
        let expiree = b64_encoder(&delegation(t - 86_400, t - 600));
        let b = AncreCog::depuis_valeurs_chaine(Some(&cog), None, Some(&hex::encode(racine)), Some(&expiree))
            .unwrap()
            .unwrap();
        assert!(b.annuaire(t).is_err());
    }

    #[test]
    fn visa_with_fresh_pop_is_accepted_for_linked_account() {
        let m = Monde::new();
        let titres = m.titres();
        let user = Uuid::new_v4();
        // Non liée : identité prouvée mais refusée.
        let (a, p) = m.entetes_visa(&titres, "GET", "/api/files", b"");
        let r = bloquer(m.service.authentifier_visa(
            req("GET", "/api/files", b""),
            Some(&a),
            Some(&p),
            None,
            maintenant(),
        ));
        assert!(matches!(r, Err(RefusMiyu::IdentiteNonLiee)));
        // Liaison puis accès.
        let (a, p) = m.entetes_visa(&titres, "POST", "/api/auth/miyu/liaisons", b"");
        let id = m
            .service
            .verifier_visa(
                req("POST", "/api/auth/miyu/liaisons", b""),
                Some(&a),
                Some(&p),
                maintenant(),
            )
            .unwrap();
        assert_eq!(id.cog_id, m.cog.as_str());
        bloquer(
            m.service
                .store()
                .link_identity(user, &id.cog_id, &id.sujet, "test"),
        )
        .unwrap();
        let (a, p) = m.entetes_visa(&titres, "PUT", "/api/files/x", b"contenu");
        let r = bloquer(m.service.authentifier_visa(
            req("PUT", "/api/files/x", b"contenu"),
            Some(&a),
            Some(&p),
            None,
            maintenant(),
        ));
        let (u, portees) = r.unwrap();
        assert_eq!(u, user);
        // Visa de test : portée « fichiers », inconnue de MiyuCloud ⇒ rien.
        assert_eq!(portees, "");
    }

    #[test]
    fn replayed_pop_is_refused() {
        let m = Monde::new();
        let titres = m.titres();
        let (a, p) = m.entetes_visa(&titres, "GET", "/api/files", b"");
        let r = req("GET", "/api/files", b"");
        assert!(
            m.service
                .verifier_visa(r, Some(&a), Some(&p), maintenant())
                .is_ok()
        );
        // Même requête rejouée : défi déjà consommé.
        let rejeu = m.service.verifier_visa(r, Some(&a), Some(&p), maintenant());
        assert!(matches!(
            rejeu,
            Err(RefusMiyu::Http(RefusHttp::DefiInvalide))
        ));
    }

    #[test]
    fn proof_is_bound_to_method_path_and_body() {
        let m = Monde::new();
        let titres = m.titres();
        for (meth, chemin, corps) in [
            ("DELETE", "/api/files/x", &b"a"[..]),
            ("PUT", "/api/files/y", &b"a"[..]),
            ("PUT", "/api/files/x", &b"b"[..]),
        ] {
            let (a, p) = m.entetes_visa(&titres, "PUT", "/api/files/x", b"a");
            assert!(
                m.service
                    .verifier_visa(req(meth, chemin, corps), Some(&a), Some(&p), maintenant())
                    .is_err(),
                "{meth} {chemin}"
            );
        }
    }

    #[test]
    fn visa_without_pop_or_from_unpinned_cog_is_refused() {
        let m = Monde::new();
        let titres = m.titres();
        let (a, _) = m.entetes_visa(&titres, "GET", "/api/files", b"");
        assert!(
            m.service
                .verifier_visa(req("GET", "/api/files", b""), Some(&a), None, maintenant())
                .is_err()
        );
        // Titres scellés par une clé non épinglée.
        let intrus = CleSignature::generer();
        let faux = m.titres_par(&intrus, AUDIENCE_MIYUCLOUD);
        let (a, p) = m.entetes_visa(&faux, "GET", "/api/files", b"");
        assert!(
            m.service
                .verifier_visa(
                    req("GET", "/api/files", b""),
                    Some(&a),
                    Some(&p),
                    maintenant()
                )
                .is_err()
        );
        // Visa d'une autre audience.
        let autre = m.titres_par(&m.cle_cog, "forum");
        let (a, p) = m.entetes_visa(&autre, "GET", "/api/files", b"");
        assert!(
            m.service
                .verifier_visa(
                    req("GET", "/api/files", b""),
                    Some(&a),
                    Some(&p),
                    maintenant()
                )
                .is_err()
        );
    }

    #[test]
    fn unlinked_identity_loses_access_immediately() {
        let m = Monde::new();
        let titres = m.titres();
        let user = Uuid::new_v4();
        let (a, p) = m.entetes_visa(&titres, "GET", "/x", b"");
        let id = m
            .service
            .verifier_visa(req("GET", "/x", b""), Some(&a), Some(&p), maintenant())
            .unwrap();
        let lien = bloquer(
            m.service
                .store()
                .link_identity(user, &id.cog_id, &id.sujet, "t"),
        )
        .unwrap();
        // Un autre compte ne peut pas lier la même identité.
        assert!(
            bloquer(
                m.service
                    .store()
                    .link_identity(Uuid::new_v4(), &id.cog_id, &id.sujet, "t")
            )
            .is_err()
        );
        assert!(bloquer(m.service.store().unlink_identity(user, lien.id)).unwrap());
        let (a, p) = m.entetes_visa(&titres, "GET", "/x", b"");
        let r = bloquer(m.service.authentifier_visa(
            req("GET", "/x", b""),
            Some(&a),
            Some(&p),
            None,
            maintenant(),
        ));
        assert!(matches!(r, Err(RefusMiyu::IdentiteNonLiee)));
    }

    #[test]
    fn visa_scopes_map_fail_closed() {
        use crate::interfaces::middleware::auth::app_password_allows;
        let v = |l: &[&str]| portees_visa_csv(&l.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        assert_eq!(
            v(&["api", "webdav", "api", "inconnue", "all"]),
            "api,webdav"
        );
        assert_eq!(v(&["fichiers"]), "");
        // Visa sans portée correspondante : refusé partout.
        assert!(!app_password_allows("/api/files", &v(&["fichiers"])));
        assert!(!app_password_allows("/webdav/x", &v(&["api"])));
        assert!(app_password_allows("/api/files", &v(&["api"])));
        // Jamais la gestion des secrets ni l'administration.
        assert!(!app_password_allows("/api/admin/users", &v(&["api"])));
        assert!(!app_password_allows(
            "/api/auth/app-passwords",
            &v(&["api"])
        ));
    }

    #[test]
    fn visa_scheme_disabled_without_anchor() {
        let s = MiyuAuthService::new(Arc::new(MemoireIdentites::default()), None).unwrap();
        assert!(!s.visa_active());
        assert!(matches!(
            s.emettre_defi(maintenant()),
            Err(RefusMiyu::VisaDesactive)
        ));
    }

    #[test]
    fn device_signed_request_accepted_then_replay_and_revocation_refused() {
        let m = Monde::new();
        let user = Uuid::new_v4();
        let cle = bloquer(m.service.store().register_device_key(
            user,
            &m.appareil.cle_publique(),
            "rclone",
            "webdav",
        ))
        .unwrap();
        let t = maintenant();
        let e = signer_requete(
            &m.appareil,
            USAGE_APPAREIL,
            "PUT",
            "/webdav/a?x=1",
            b"corps",
            t,
        );
        let h = e.en_entetes();
        let (sig, hor, non, sgn) = (&h[0].1, &h[1].1, &h[2].1, &h[3].1);
        let requete = RequeteProuvee {
            methode: "PUT",
            chemin: "/webdav/a",
            chemin_et_requete: "/webdav/a?x=1",
            corps: b"corps",
        };
        let (entetes, trouvee) =
            bloquer(
                m.service
                    .preparer_appareil(Some(sig), Some(hor), Some(non), Some(sgn), t),
            )
            .unwrap();
        assert_eq!(trouvee.id, cle.id);
        m.service.verifier_appareil(&entetes, requete, t).unwrap();
        // Rejeu : refusé dès l'étape 1 (nonce vu).
        let r = bloquer(
            m.service
                .preparer_appareil(Some(sig), Some(hor), Some(non), Some(sgn), t),
        );
        assert!(matches!(r, Err(RefusMiyu::Http(RefusHttp::Rejeu))));
        // Corps modifié : signature invalide.
        let e2 = signer_requete(
            &m.appareil,
            USAGE_APPAREIL,
            "PUT",
            "/webdav/a?x=1",
            b"corps",
            t,
        );
        let h2 = e2.en_entetes();
        let (en2, _) = bloquer(m.service.preparer_appareil(
            Some(&h2[0].1),
            Some(&h2[1].1),
            Some(&h2[2].1),
            Some(&h2[3].1),
            t,
        ))
        .unwrap();
        let alteree = RequeteProuvee {
            corps: b"autre",
            ..requete
        };
        assert!(m.service.verifier_appareil(&en2, alteree, t).is_err());
        // Horodatage hors fenêtre.
        let vieux = signer_requete(&m.appareil, USAGE_APPAREIL, "GET", "/webdav/", b"", t - 600);
        let hv = vieux.en_entetes();
        assert!(
            bloquer(m.service.preparer_appareil(
                Some(&hv[0].1),
                Some(&hv[1].1),
                Some(&hv[2].1),
                Some(&hv[3].1),
                t
            ))
            .is_err()
        );
        // Clé révoquée : refusée.
        assert!(bloquer(m.service.store().revoke_device_key(user, cle.id)).unwrap());
        let e3 = signer_requete(&m.appareil, USAGE_APPAREIL, "GET", "/webdav/", b"", t);
        let h3 = e3.en_entetes();
        let r = bloquer(m.service.preparer_appareil(
            Some(&h3[0].1),
            Some(&h3[1].1),
            Some(&h3[2].1),
            Some(&h3[3].1),
            t,
        ));
        assert!(matches!(r, Err(RefusMiyu::AppareilInconnu)));
    }

    #[test]
    fn unknown_device_key_is_refused() {
        let m = Monde::new();
        let t = maintenant();
        let e = signer_requete(
            &CleSignature::generer(),
            USAGE_APPAREIL,
            "GET",
            "/api/x",
            b"",
            t,
        );
        let h = e.en_entetes();
        let r = bloquer(m.service.preparer_appareil(
            Some(&h[0].1),
            Some(&h[1].1),
            Some(&h[2].1),
            Some(&h[3].1),
            t,
        ));
        assert!(matches!(r, Err(RefusMiyu::AppareilInconnu)));
    }

    /// Visa présenté avec en-têtes et défi frais : résultat de `verifier_visa`.
    fn presenter(m: &Monde) -> Result<IdentiteVisa, RefusMiyu> {
        let titres = m.titres();
        let (a, p) = m.entetes_visa(&titres, "GET", "/api/files", b"");
        m.service.verifier_visa(
            req("GET", "/api/files", b""),
            Some(&a),
            Some(&p),
            maintenant(),
        )
    }

    fn refus_revocation(r: &Result<IdentiteVisa, RefusMiyu>) -> Option<ErreurRevocation> {
        match r {
            Err(RefusMiyu::Http(RefusHttp::Identite(RefusIdentite::Revocation(e)))) => {
                Some(e.clone())
            }
            _ => None,
        }
    }

    #[test]
    fn visa_accepted_with_fresh_revocation_list() {
        let m = Monde::new();
        assert!(presenter(&m).is_ok());
    }

    #[test]
    fn revoked_device_key_in_registry_is_refused() {
        let mut m = Monde::new();
        assert!(presenter(&m).is_ok());
        let t = maintenant();
        let CibleRevocation::Cle(kid) = CibleRevocation::appareil(&m.appareil.cle_publique())
        else {
            unreachable!()
        };
        m.registre.revoquer_cle(kid, None, t);
        m.registre.relever(&m.revocations, t);
        let r = presenter(&m);
        assert!(
            matches!(
                r,
                Err(RefusMiyu::Http(RefusHttp::Identite(
                    RefusIdentite::Revoque(_)
                )))
            ),
            "appareil révoqué au registre : refus ({r:?})"
        );
    }

    #[test]
    fn revocation_list_absent_or_stale_is_refused() {
        // Jamais relevée : liste absente ⇒ refus.
        let m = Monde::sans_releve();
        let r = presenter(&m);
        assert_eq!(
            refus_revocation(&r),
            Some(ErreurRevocation::ListeAbsente),
            "{r:?}"
        );

        // Tête vieille de 15 min + 1 s : périmée ⇒ refus.
        let mut m = Monde::sans_releve();
        let t = maintenant();
        let (tete, entrees) = m.registre.publier(t - FRAICHEUR_REVOCATIONS_SECS - 1);
        m.revocations
            .appliquer_releve(&tete, 0, &entrees, t)
            .unwrap();
        let r = presenter(&m);
        assert!(
            matches!(refus_revocation(&r), Some(ErreurRevocation::Perimee { .. })),
            "{r:?}"
        );
    }

    #[test]
    fn visa_refused_without_configured_revocation_source() {
        let m = Monde::new();
        let service = MiyuAuthService::new(
            Arc::new(MemoireIdentites::default()),
            Some(AncreCog {
                cog_id: m.cog.clone(),
                cle_emission: Some(m.cle_cog.cle_publique()),
                racine: None,
                delegations: Vec::new(),
            }),
        )
        .unwrap();
        let titres = m.titres();
        let d = service.emettre_defi(maintenant()).unwrap();
        let liaison = liaison_requete("GET", "/api/files", b"");
        let sig = signer_pop(&m.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
        let (a, p) = (
            formater_autorisation_visa(&titres.0, &titres.1),
            formater_pop(&d.id, &sig),
        );
        let r = service.verifier_visa(
            req("GET", "/api/files", b""),
            Some(&a),
            Some(&p),
            maintenant(),
        );
        assert_eq!(
            refus_revocation(&r),
            Some(ErreurRevocation::ListeAbsente),
            "{r:?}"
        );
        // Le précontrôle (avant lecture du corps) refuse aussi.
        assert!(
            service
                .precontroler_visa(Some(&a), Some(&p), None, maintenant())
                .is_err()
        );
    }

    // ── A-029 / I6 : état persistant, délégations présentées, clés d'appareil ──

    /// Dossier d'état propre au test (sous `TMPDIR`).
    pub(crate) fn dossier_etat(nom: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!(
            "miyucloud-etat-{nom}-{}-{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        d.join("etat")
    }

    /// Même ancre et mêmes révocations que `m`, état persistant dans `dossier`
    /// (nouvelle instance : « redémarrage » ou seconde instance).
    fn instance(m: &Monde, dossier: &std::path::Path) -> MiyuAuthService<MemoireIdentites> {
        MiyuAuthService::new(
            Arc::new(MemoireIdentites::default()),
            Some(AncreCog {
                cog_id: m.cog.clone(),
                cle_emission: Some(m.cle_cog.cle_publique()),
                racine: None,
                delegations: Vec::new(),
            }),
        )
        .unwrap()
        .avec_revocations(m.revocations.clone())
        .avec_etat_persistant(dossier)
        .unwrap()
    }

    #[test]
    fn persistent_challenge_is_single_use_across_restart_and_instances() {
        let m = Monde::new();
        let dossier = dossier_etat("defis");
        let a = instance(&m, &dossier);
        let titres = m.titres();
        // Défi émis par A, consommé par B (autre instance, même dossier).
        let d = a.emettre_defi(maintenant()).unwrap();
        let liaison = liaison_requete("GET", "/api/files", b"");
        let sig = signer_pop(&m.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
        let (aut, pop) = (
            formater_autorisation_visa(&titres.0, &titres.1),
            formater_pop(&d.id, &sig),
        );
        let b = instance(&m, &dossier);
        assert!(
            b.verifier_visa(req("GET", "/api/files", b""), Some(&aut), Some(&pop), maintenant())
                .is_ok()
        );
        // Rejeu après « redémarrage » (nouvelle instance) : défi déjà consommé.
        drop((a, b));
        let c = instance(&m, &dossier);
        let r = c.verifier_visa(req("GET", "/api/files", b""), Some(&aut), Some(&pop), maintenant());
        assert!(matches!(r, Err(RefusMiyu::Http(RefusHttp::DefiInvalide))), "{r:?}");
        let _ = std::fs::remove_dir_all(dossier.parent().unwrap());
    }

    #[test]
    fn persistent_nonce_refuses_replay_after_restart() {
        let m = Monde::new();
        let dossier = dossier_etat("nonces");
        let a = instance(&m, &dossier);
        let user = Uuid::new_v4();
        bloquer(a.store().register_device_key(user, &m.appareil.cle_publique(), "t", "webdav")).unwrap();
        let t = maintenant();
        let e = signer_requete(&m.appareil, USAGE_APPAREIL, "GET", "/webdav/", b"", t);
        let h = e.en_entetes();
        let (en, _) = bloquer(a.preparer_appareil(
            Some(&h[0].1),
            Some(&h[1].1),
            Some(&h[2].1),
            Some(&h[3].1),
            t,
        ))
        .unwrap();
        let r = RequeteProuvee {
            methode: "GET",
            chemin: "/webdav/",
            chemin_et_requete: "/webdav/",
            corps: b"",
        };
        a.verifier_appareil(&en, r, t).unwrap();
        // Nouvelle instance (même dossier, même clé enregistrée) : rejeu refusé.
        let b = MiyuAuthService::new(a.store().clone(), None)
            .unwrap()
            .avec_etat_persistant(&dossier)
            .unwrap();
        drop(a);
        let rejeu = bloquer(b.preparer_appareil(
            Some(&h[0].1),
            Some(&h[1].1),
            Some(&h[2].1),
            Some(&h[3].1),
            t,
        ));
        assert!(matches!(rejeu, Err(RefusMiyu::Http(RefusHttp::Rejeu))), "{rejeu:?}");
        assert_eq!(b.verifier_appareil(&en, r, t).err().map(|e| e.statut()), Some(401));
        let _ = std::fs::remove_dir_all(dossier.parent().unwrap());
    }

    #[test]
    fn persistent_state_refuses_world_writable_directory() {
        use std::os::unix::fs::PermissionsExt;
        let dossier = dossier_etat("droits");
        std::fs::create_dir_all(dossier.join("defis")).unwrap();
        std::fs::set_permissions(dossier.join("defis"), std::fs::Permissions::from_mode(0o777)).unwrap();
        let s = MiyuAuthService::new(Arc::new(MemoireIdentites::default()), None).unwrap();
        assert!(s.avec_etat_persistant(&dossier).is_err());
        let _ = std::fs::remove_dir_all(dossier.parent().unwrap());
    }

    /// COG en mode chaîné : racine, clé `emission` (indice 1) et clé `visas` (5).
    struct Chaine {
        identite: CleSignature,
        emission: CleSignature,
        visas: CleSignature,
        cog: CogId,
    }

    impl Chaine {
        fn new() -> Self {
            let identite = CleSignature::generer();
            let cog = CogId::new(&miyusceau::cog_id_depuis_cle(&identite.cle_publique())).unwrap();
            Self {
                identite,
                emission: CleSignature::generer(),
                visas: CleSignature::generer(),
                cog,
            }
        }

        fn delegation(&self, indice: u32, cle: &CleSignature) -> Vec<u8> {
            let t = maintenant();
            miyusceau::DelegationCle::nouvelle(
                self.cog.as_str(),
                indice,
                0,
                cle.cle_publique(),
                t - 60,
                t + 86_400,
            )
            .unwrap()
            .sceller(&self.identite)
            .unwrap()
        }
    }

    #[test]
    fn presented_delegations_are_checked_in_precheck_and_proof() {
        let c = Chaine::new();
        let mut registre = RegistreCog::new();
        let revocations = registre.source(&c.cog);
        registre.relever(&revocations, maintenant());
        // MiyuCloud ne connaît que la délégation `emission` : celle de `visas`
        // arrive avec la requête (X-Miyu-Delegations).
        let ancre = AncreCog::depuis_valeurs_chaine(
            Some(c.cog.as_str()),
            None,
            Some(&hex::encode(c.identite.cle_publique())),
            Some(&b64_encoder(&c.delegation(1, &c.emission))),
        )
        .unwrap()
        .unwrap();
        let service = MiyuAuthService::new(Arc::new(MemoireIdentites::default()), Some(ancre))
            .unwrap()
            .avec_revocations(revocations);
        // Titres : passeport par `emission`, visa par `visas` (mode chaîné).
        let mut m = Monde::new();
        m.cog = c.cog.clone();
        let t = maintenant();
        let passeport = emettre_passeport(
            &c.cog,
            &c.emission,
            &DemandePasseport {
                secret: &m.secret,
                destination: c.cog.clone(),
                cle_appareil: m.appareil.cle_publique(),
                portees: Portees::new(["api"]).unwrap(),
                pas_avant: t - 10,
                pas_apres: t + 3600,
            },
        )
        .unwrap();
        let mut vue = AnnuaireEmetteurs::new();
        vue.ajouter(c.cog.clone(), &c.emission.cle_publique(), NiveauConfiance::Souverain)
            .unwrap();
        let pol = PolitiqueIdentite {
            cog_local: c.cog.clone(),
            annuaire: &vue,
            revocations: &VueCogEmetteur,
        };
        let pv = verifier_passeport(&passeport, &pol, t).unwrap();
        let sig = signer_pop(&m.appareil, &[7; 32], "cog", &[9; 32]);
        let possession = prouver_possession(pv, &[7; 32], "cog", &[9; 32], &sig).unwrap();
        let visa = emettre_visa(
            &c.cog,
            &c.visas,
            &possession,
            &DemandeVisa {
                audience: AUDIENCE_MIYUCLOUD.to_string(),
                portees: Portees::new(["api"]).unwrap(),
                pas_avant: t - 10,
                pas_apres: t + 600,
            },
        )
        .unwrap();
        let entetes = |svc: &MiyuAuthService<MemoireIdentites>| {
            let d = svc.emettre_defi(maintenant()).unwrap();
            let liaison = liaison_requete("GET", "/api/files", b"");
            let sig = signer_pop(&m.appareil, &d.defi, AUDIENCE_MIYUCLOUD, &liaison);
            (formater_autorisation_visa(&visa, &passeport), formater_pop(&d.id, &sig))
        };
        let deleg_visas = b64_encoder(&c.delegation(5, &c.visas));
        // Sans la délégation `visas` : refus au précontrôle et à la preuve (strict).
        let (a, p) = entetes(&service);
        assert!(service.precontroler_visa(Some(&a), Some(&p), None, t).is_err());
        assert!(
            service
                .verifier_visa_avec_delegations(req("GET", "/api/files", b""), Some(&a), Some(&p), None, t)
                .is_err()
        );
        // Avec la délégation présentée : accepté.
        let (a, p) = entetes(&service);
        service
            .precontroler_visa(Some(&a), Some(&p), Some(&deleg_visas), t)
            .unwrap();
        let id = service
            .verifier_visa_avec_delegations(
                req("GET", "/api/files", b""),
                Some(&a),
                Some(&p),
                Some(&deleg_visas),
                t,
            )
            .unwrap();
        assert_eq!(id.portees, "api");
        assert_eq!(id.cle_appareil, m.appareil.cle_publique());
        // Délégation scellée par une autre racine : refus.
        let intrus = Chaine::new();
        let fausse = b64_encoder(&intrus.delegation(5, &c.visas));
        let (a, p) = entetes(&service);
        assert!(service.precontroler_visa(Some(&a), Some(&p), Some(&fausse), t).is_err());
        // En-tête mal formé : refus.
        assert!(service.precontroler_visa(Some(&a), Some(&p), Some("!!"), t).is_err());
    }

    #[test]
    fn visa_refused_once_its_device_key_is_removed_from_the_account() {
        let m = Monde::new();
        let titres = m.titres();
        let user = Uuid::new_v4();
        let store = m.service.store();
        let (a, p) = m.entetes_visa(&titres, "GET", "/x", b"");
        let id = m
            .service
            .verifier_visa(req("GET", "/x", b""), Some(&a), Some(&p), maintenant())
            .unwrap();
        bloquer(store.link_identity(user, &id.cog_id, &id.sujet, "t")).unwrap();
        let cle = bloquer(store.register_device_key(user, &m.appareil.cle_publique(), "tel", "api"))
            .unwrap();
        let (a, p) = m.entetes_visa(&titres, "GET", "/x", b"");
        assert!(
            bloquer(m.service.authentifier_visa(req("GET", "/x", b""), Some(&a), Some(&p), None, maintenant()))
                .is_ok()
        );
        // Retrait de l'appareil : son visa ne passe plus.
        assert!(bloquer(store.revoke_device_key(user, cle.id)).unwrap());
        let (a, p) = m.entetes_visa(&titres, "GET", "/x", b"");
        let r = bloquer(m.service.authentifier_visa(req("GET", "/x", b""), Some(&a), Some(&p), None, maintenant()));
        assert!(matches!(r, Err(RefusMiyu::AppareilRetire)), "{r:?}");
    }

    use crate::infrastructure::services::miyu_liaison_appareil::tests::demande as demande_mla2;

    fn graine(c: &CleSignature) -> [u8; 32] {
        c.graine()
    }

    #[test]
    fn device_link_requires_fresh_possession_and_is_single_use() {
        let m = Monde::new();
        let user = Uuid::new_v4();
        let t = maintenant();
        let nouvelle = CleSignature::generer();
        let texte = demande_mla2(&m.cog, &graine(&nouvelle), None, t + 120).encoder();
        let cle = bloquer(m.service.lier_appareil(user, &texte, " Téléphone\n", "api,webdav,api", t))
            .unwrap();
        assert_eq!(cle.public_key, b64_encoder(&nouvelle.cle_publique()));
        assert_eq!(cle.client_name, "Téléphone");
        assert_eq!(cle.scopes, "api,webdav");
        // Rejeu : refusé (preuve brûlée).
        let r = bloquer(m.service.lier_appareil(Uuid::new_v4(), &texte, "x", "api", t));
        assert!(matches!(r, Err(RefusMiyu::CleDejaLiee)), "{r:?}");
        // Portées inconnues ou vides : refus, preuve non brûlée.
        let autre = CleSignature::generer();
        let texte2 = demande_mla2(&m.cog, &graine(&autre), None, t + 120).encoder();
        for portees in ["", "all", "api,admin"] {
            let r = bloquer(m.service.lier_appareil(user, &texte2, "x", portees, t));
            assert!(matches!(r, Err(RefusMiyu::PorteesInvalides)), "{portees}");
        }
        assert!(bloquer(m.service.lier_appareil(user, &texte2, "", "caldav", t)).is_ok());
        // Demande d'un autre COG, expirée ou de rotation : refus.
        let ailleurs = CogId::new(&miyusceau::cog_id_depuis_cle(&CleSignature::generer().cle_publique()))
            .unwrap();
        let k = CleSignature::generer();
        for texte in [
            demande_mla2(&ailleurs, &graine(&k), None, t + 60).encoder(),
            demande_mla2(&m.cog, &graine(&k), None, t - 1).encoder(),
            demande_mla2(&m.cog, &graine(&k), Some(&graine(&m.appareil)), t + 60).encoder(),
        ] {
            let r = bloquer(m.service.lier_appareil(user, &texte, "x", "api", t));
            assert!(matches!(r, Err(RefusMiyu::Liaison(_))), "{r:?}");
        }
        assert_eq!(bloquer(m.service.store().list_device_keys(user)).unwrap().len(), 2);
        // Sans COG épinglé : liaison impossible.
        let sans = MiyuAuthService::new(Arc::new(MemoireIdentites::default()), None).unwrap();
        let texte = demande_mla2(&m.cog, &graine(&k), None, t + 60).encoder();
        assert!(matches!(
            bloquer(sans.lier_appareil(user, &texte, "x", "api", t)),
            Err(RefusMiyu::VisaDesactive)
        ));
    }

    #[test]
    fn rotation_signed_by_old_key_replaces_and_revokes_it() {
        let m = Monde::new();
        let user = Uuid::new_v4();
        let t = maintenant();
        let store = m.service.store();
        let ancienne = bloquer(store.register_device_key(user, &m.appareil.cle_publique(), "tel", "webdav"))
            .unwrap();
        let nouvelle = CleSignature::generer();
        let rotation = demande_mla2(&m.cog, &graine(&nouvelle), Some(&graine(&m.appareil)), t + 60).encoder();
        // Requête signée par une autre clé que l'autorisante : refus.
        let tierce = bloquer(store.register_device_key(user, &CleSignature::generer().cle_publique(), "pc", "api"))
            .unwrap();
        assert!(matches!(
            bloquer(m.service.faire_tourner_appareil(&tierce, &rotation, t)),
            Err(RefusMiyu::Liaison(RefusLiaison::Signature))
        ));
        // Demande simple (sans autorisation) : refus.
        let simple = demande_mla2(&m.cog, &graine(&nouvelle), None, t + 60).encoder();
        assert!(matches!(
            bloquer(m.service.faire_tourner_appareil(&ancienne, &simple, t)),
            Err(RefusMiyu::Liaison(RefusLiaison::Format))
        ));
        let cle = bloquer(m.service.faire_tourner_appareil(&ancienne, &rotation, t)).unwrap();
        assert_eq!(cle.client_name, "tel");
        assert_eq!(cle.scopes, "webdav");
        // L'ancienne est révoquée, la nouvelle active.
        assert!(bloquer(store.active_device_key(&m.appareil.cle_publique())).unwrap().is_none());
        assert!(bloquer(store.device_key_revoked(&m.appareil.cle_publique())).unwrap());
        assert!(bloquer(store.active_device_key(&nouvelle.cle_publique())).unwrap().is_some());
        // Rejeu de la rotation : refusé.
        assert!(bloquer(m.service.faire_tourner_appareil(&ancienne, &rotation, t)).is_err());
        // Nouvelle rotation par l'ancienne clé (déjà révoquée) : refusée.
        let encore = demande_mla2(
            &m.cog,
            &graine(&CleSignature::generer()),
            Some(&graine(&m.appareil)),
            t + 60,
        )
        .encoder();
        assert!(matches!(
            bloquer(m.service.faire_tourner_appareil(&ancienne, &encore, t)),
            Err(RefusMiyu::AppareilInconnu)
        ));
    }

    #[test]
    fn device_scopes_are_strict() {
        assert_eq!(portees_appareil("webdav, api,webdav").as_deref(), Some("webdav,api"));
        assert_eq!(portees_appareil(" , "), None);
        assert_eq!(portees_appareil("webdav,all"), None);
    }
}
