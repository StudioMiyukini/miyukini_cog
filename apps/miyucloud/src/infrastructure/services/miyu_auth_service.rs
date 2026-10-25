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
//! - Ancre de confiance : la **clé d'émission épinglée du COG d'origine**
//!   (`MIYUCLOUD_MIYU_COG_ID` + `MIYUCLOUD_MIYU_CLE_EMISSION`), niveau souverain :
//!   passeport **et** visa doivent être scellés par ce COG. Aucune clé épinglée ⇒
//!   schéma désactivé (refus).
//! - Révocation : liste tirée du **registre vérifié** du COG d'origine
//!   ([`super::miyu_revocations`], `miyuregistre::RevocationsRegistre`) :
//!   passeport, visa, clé d'appareil et clé d'émission sont contrôlés à chaque
//!   requête ; fraîcheur **≤ 15 min** (A-007) ; liste absente ou périmée ⇒
//!   **refus**. Sans source configurée, tout visa est refusé (fail-closed).
//!   Côté MiyuCloud, retirer la liaison coupe aussi l'accès immédiatement.
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
use std::sync::Arc;

use miyauth::http::{
    DefiEmis, EntetesSignees, MagasinDefis, MagasinDefisMemoire, RefusHttp, RegistreNonces,
    RequeteHttp, analyser_autorisation_visa, analyser_pop, verifier_requete_signee,
    verifier_requete_visa,
};
use miyauth::{AnnuaireEmetteurs, CogId, PolitiqueIdentite};
use miyusceau::NiveauConfiance;
use uuid::Uuid;

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
    /// Clé publique d'émission épinglée.
    pub cle_emission: [u8; 32],
}

impl AncreCog {
    /// Lit `MIYUCLOUD_MIYU_COG_ID` et `MIYUCLOUD_MIYU_CLE_EMISSION`.
    ///
    /// Les deux absents : `Ok(None)` (schéma visa désactivé). Un seul présent ou
    /// une valeur invalide : erreur (fail-closed, le serveur refuse de démarrer).
    pub fn depuis_env() -> Result<Option<Self>, String> {
        Self::depuis_valeurs(
            std::env::var("MIYUCLOUD_MIYU_COG_ID").ok().as_deref(),
            std::env::var("MIYUCLOUD_MIYU_CLE_EMISSION").ok().as_deref(),
        )
    }

    /// Variante pure de [`Self::depuis_env`].
    pub fn depuis_valeurs(cog: Option<&str>, cle: Option<&str>) -> Result<Option<Self>, String> {
        let cog = cog.map(str::trim).filter(|s| !s.is_empty());
        let cle = cle.map(str::trim).filter(|s| !s.is_empty());
        match (cog, cle) {
            (None, None) => Ok(None),
            (Some(c), Some(k)) => {
                let cog_id = CogId::new(c)
                    .ok_or_else(|| format!("MIYUCLOUD_MIYU_COG_ID invalide « {c} »"))?;
                let cle_emission = analyser_cle_publique(k).ok_or(
                    "MIYUCLOUD_MIYU_CLE_EMISSION : clé publique Ed25519 (64 hex ou base64 URL) attendue",
                )?;
                Ok(Some(Self {
                    cog_id,
                    cle_emission,
                }))
            }
            _ => Err(
                "MIYUCLOUD_MIYU_COG_ID et MIYUCLOUD_MIYU_CLE_EMISSION vont ensemble".to_string(),
            ),
        }
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
    /// Stockage indisponible.
    Stockage(DomainError),
}

impl RefusMiyu {
    /// Statut HTTP conseillé.
    pub fn statut(&self) -> u16 {
        match self {
            Self::Http(r) => r.statut(),
            Self::Stockage(_) => 503,
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

/// Identité prouvée par un visa.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdentiteVisa {
    pub cog_id: String,
    pub sujet: [u8; 32],
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
    defis: MagasinDefisMemoire,
    nonces: RegistreNonces,
}

impl<S: MiyuIdentityStore> MiyuAuthService<S> {
    /// Service adossé à `store`, avec l'ancre `ancre` (ou sans schéma visa).
    pub fn new(store: Arc<S>, ancre: Option<AncreCog>) -> Result<Self, String> {
        let hote = match ancre {
            None => None,
            Some(a) => {
                let mut annuaire = AnnuaireEmetteurs::new();
                annuaire
                    .ajouter(
                        a.cog_id.clone(),
                        &a.cle_emission,
                        NiveauConfiance::Souverain,
                    )
                    .map_err(|e| format!("MIYUCLOUD_MIYU_CLE_EMISSION : {e}"))?;
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
            defis: MagasinDefisMemoire::default(),
            nonces: RegistreNonces::default(),
        })
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
    /// formés, visa et passeport scellés par une ancre épinglée, audience,
    /// fenêtres et révocation. Ne consomme pas le défi (la preuve liée au corps
    /// est vérifiée ensuite par [`Self::verifier_visa`]).
    pub fn precontroler_visa(
        &self,
        autorisation: Option<&str>,
        pop: Option<&str>,
        maintenant: u64,
    ) -> Result<(), RefusMiyu> {
        let hote = self.hote.as_ref().ok_or(RefusMiyu::VisaDesactive)?;
        let (visa, passeport) = analyser_autorisation_visa(autorisation.unwrap_or(""))?;
        analyser_pop(pop.unwrap_or(""))?;
        miyauth::verifier_visa(
            &visa,
            &passeport,
            AUDIENCE_MIYUCLOUD,
            &hote.politique(),
            maintenant,
        )
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
        let hote = self.hote.as_ref().ok_or(RefusMiyu::VisaDesactive)?;
        let req = RequeteHttp {
            methode: requete.methode,
            chemin: requete.chemin,
            corps: requete.corps,
            autorisation,
            pop,
            liaison: None,
        };
        let acces = verifier_requete_visa(
            &req,
            AUDIENCE_MIYUCLOUD,
            &hote.politique(),
            &self.defis,
            maintenant,
        )?;
        Ok(IdentiteVisa {
            cog_id: acces.cog_origine().as_str().to_string(),
            sujet: *acces.sujet(),
            portees: portees_visa_csv(acces.acces().visa().portees.as_slice()),
        })
    }

    /// Visa + PoP puis liaison : rend le compte MiyuCloud et les portées du
    /// visa (à appliquer au chemin par l'appelant).
    pub async fn authentifier_visa(
        &self,
        requete: RequeteProuvee<'_>,
        autorisation: Option<&str>,
        pop: Option<&str>,
        maintenant: u64,
    ) -> Result<(Uuid, String), RefusMiyu> {
        let identite = self.verifier_visa(requete, autorisation, pop, maintenant)?;
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
                    cle_emission: cle_cog.cle_publique(),
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
        assert_eq!(a.cle_emission, cle);
        let b = AncreCog::depuis_valeurs(Some(&cog), Some(&b64_encoder(&cle)))
            .unwrap()
            .unwrap();
        assert_eq!(a, b);
        assert!(AncreCog::depuis_valeurs(Some(&cog), Some("pas une clé")).is_err());
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
                cle_emission: m.cle_cog.cle_publique(),
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
                .precontroler_visa(Some(&a), Some(&p), maintenant())
                .is_err()
        );
    }
}
