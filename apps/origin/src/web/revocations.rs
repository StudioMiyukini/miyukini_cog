//! @id     origin_web_revocations
//! @do     suivre_revocations_cogs_reconnus
//! @role   securite
//! @layer  app
//! @human  origin — révocations des COG reconnus : suivi vérifié de leur registre (`SuiviRevocationsRegistre`), synchro par morceaux authentifiés (enveloppe signée), bornée par COG et globalement, fraîcheur ≤ 15 min, refus sinon.

//! Révocations des COG reconnus vues d'Origin (lot 2 §8.2, A-003, A-007).
//!
//! Chaque COG reconnu dont le registre est déclaré (`[[forum.cogs_reconnus]]` :
//! `registre`, `cle_journal`) a ici un [`SuiviRevocationsRegistre`]
//! (`miyuregistre`, fonctionnalité `miyauth`) : le **même** type que `MiyuCloud`
//! et les façades — miroir vérifié (tête signée par la clé de journal épinglée,
//! seuil 1, racine recalculée, monotonie, équivoque), liste des entrées
//! `Revocation`, **fraîcheur 15 min imposée par le type**, `RwLock` (les
//! vérifications ne prennent que la lecture ; tête, racine et sceaux sont
//! vérifiés hors du verrou d'écriture ; rejeu à l'identique sans recalcul).
//!
//! # Transport : `POST /api/auth/revocations/sync` (à consigner par amendement)
//!
//! Indépendant du store du forum (l'admin en dépend aussi) : la route existe
//! dès qu'Origin tourne. Le COG **pousse** JSON
//! `{cog_id, depuis, tete?, entrees, echeance, signature}` (titres et
//! signature en base64 URL).
//!
//! ## Enveloppe (chaque envoi, morceau ou tête)
//!
//! Chaque envoi est authentifié par une signature Ed25519 de la **clé de
//! journal épinglée** du COG (la même que pour ses têtes) sur
//! ([`octets_enveloppe`]) :
//!
//! ```text
//! "miyukini/origin/revocations/morceau/v1" ‖ 0x00     (DOMAINE_ENVELOPPE)
//! ‖ u16 BE longueur(cog_id) ‖ cog_id (UTF-8)
//! ‖ u64 BE depuis
//! ‖ u64 BE nombre d'entrées
//! ‖ SHA-256( ‖ pour chaque entrée : u32 BE longueur ‖ octets )   (32 o)
//! ‖ u8 tête présente (0 | 1) ‖ SHA-256(tête) ou 32 × 0x00         (33 o)
//! ‖ u64 BE echeance (secondes Unix)
//! ```
//!
//! - `echeance` : courte, au plus [`DUREE_MAX_ENVELOPPE_SECS`] (15 min) après
//!   l'heure d'Origin (+ [`TOLERANCE_HORLOGE_SECS`] de dérive) ; dépassée ⇒ `401`.
//! - La signature (64 o, `verify_strict`) est vérifiée **avant toute
//!   modification d'état** et **avant** la limite de débit par COG : un envoi
//!   non signé, mal signé ou signé par une autre clé est refusé (`401`) sans
//!   toucher à l'attente ni au quota du COG.
//! - Anti-rejeu : l'empreinte du message signé est retenue jusqu'à son
//!   échéance ; un rejeu est **sans effet** (`200`, `rejeu: true`, état
//!   courant rendu, rien de modifié).
//!
//! ## Morceaux et tête
//!
//! - **Morceau** : un envoi **sans** `tete` ; ses entrées (au plus
//!   [`MAX_ENTREES_SYNCHRO`] par envoi, corps ≤ 2 Mio) s'accumulent **en
//!   attente** pour ce COG. `depuis` doit valoir `taille du miroir + entrées
//!   en attente` ; sinon `409` rend `taille` (miroir) et `attendu` (position à
//!   reprendre) **sans purger** l'attente. Bornes par COG : miroir + attente ≤
//!   [`MAX_ENTREES_SUIVI`] entrées (1 048 576) et ≤ [`MAX_OCTETS_SUIVI`] octets
//!   (256 Mio), comme `MiyuCloud` ; au-delà, purge et `413`. Bornes globales
//!   (somme des attentes de tous les COG) : [`MAX_ENTREES_ATTENTE_GLOBALE`]
//!   et [`MAX_OCTETS_ATTENTE_GLOBALE`] ; au-delà, `503` sans purge (réessayer).
//!   Une attente sans tête depuis plus de 15 min est purgée.
//! - **Tête** : l'envoi qui porte `tete` valide le tout (attente + ses propres
//!   entrées) : racine reconstruite = racine signée. Refus du registre ⇒
//!   attente purgée, rien d'autre n'est modifié. Sans attente, `depuis` ≤
//!   taille du miroir est toléré (chevauchement).
//! - Rejeu de tête à l'identique (même tête, rien de neuf, nouvelle enveloppe)
//!   ⇒ `200`, `rejeu: true`, sans recalcul.
//! - Débit : par IP avant lecture du corps ; par COG **après** authentification
//!   (un tiers ne peut pas épuiser le quota d'un COG). Nombre d'entrées
//!   contrôlé **avant** tout décodage base64 ; refus journalisés **agrégés**
//!   (un résumé par minute au plus, pas un `warn!` par requête).
//!
//! Seul le détenteur de la clé de journal peut donc modifier l'attente d'un
//! COG (l'enrichir ou la faire purger).
//!
//! # Fail-closed
//!
//! COG sans registre déclaré, jamais synchronisé, tête de plus de 15 min, datée
//! dans le futur, verrou empoisonné ⇒ erreur de révocation, donc **refus** du
//! passeport ou du visa par miyauth. Miroir en mémoire : après redémarrage, le
//! COG renvoie son registre depuis l'index 0 (par morceaux au besoin).

use std::collections::BTreeMap;
use std::sync::{Mutex, PoisonError};

use ed25519_dalek::{Signature, VerifyingKey};
use miyauth::http::b64_decoder;
use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
use miyauth::{CibleRevocation, CogId, ErreurRevocation, SourceRevocation};
use miyuregistre::{RefusSuivi, SuiviRevocationsRegistre, MAX_ENTREES_SUIVI, MAX_OCTETS_SUIVI};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tracing::warn;

use super::server::RouteResponse;
use crate::config::ForumConfig;

/// Route de synchronisation des registres de révocation.
pub const ROUTE_REVOCATIONS: &str = "/api/auth/revocations/sync";
/// Taille maximale du corps d'un envoi (tête + entrées, base64).
pub const TAILLE_MAX_CORPS_REVOCATIONS: usize = 2 * 1024 * 1024;
/// Nombre maximal d'entrées par envoi (un morceau).
pub const MAX_ENTREES_SYNCHRO: usize = 4096;
/// Durée de vie d'une attente sans tête (15 min).
pub const DUREE_ATTENTE_SECS: u64 = FRAICHEUR_MAX_DEFAUT_SECS;
/// Fenêtre d'agrégation des refus journalisés.
pub const FENETRE_JOURNAL_REFUS_SECS: u64 = 60;
/// Domaine de séparation de l'enveloppe signée d'un envoi.
pub const DOMAINE_ENVELOPPE: &[u8] = b"miyukini/origin/revocations/morceau/v1";
/// Échéance maximale d'une enveloppe, comptée depuis l'heure d'Origin (15 min).
pub const DUREE_MAX_ENVELOPPE_SECS: u64 = 15 * 60;
/// Dérive d'horloge tolérée sur l'échéance (COG en avance).
pub const TOLERANCE_HORLOGE_SECS: u64 = 120;
/// Enveloppes retenues (anti-rejeu) par COG jusqu'à leur échéance.
pub const MAX_ENVELOPPES_VUES: usize = 8192;
/// Budget global de l'attente : entrées (somme sur tous les COG).
pub const MAX_ENTREES_ATTENTE_GLOBALE: u64 = 2_097_152;
/// Budget global de l'attente : octets (somme sur tous les COG, 512 Mio).
pub const MAX_OCTETS_ATTENTE_GLOBALE: u64 = 512 * 1024 * 1024;

/// Registre déclaré d'un COG reconnu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistreReconnu {
    /// COG reconnu.
    pub cog: CogId,
    /// Identifiant de son registre (32 o).
    pub registre: [u8; 32],
    /// Clé publique de journal : signe ses têtes, ses entrées de révocation et
    /// les enveloppes de ses envois.
    pub cle_journal: [u8; 32],
}

/// Refus d'une synchronisation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RefusSynchro {
    /// COG sans registre déclaré.
    CogInconnu,
    /// Enveloppe absente, mal formée, ou signature invalide pour la clé de journal.
    NonAuthentifie,
    /// Échéance de l'enveloppe dépassée ou trop lointaine.
    EnveloppeExpiree,
    /// Position incohérente : reprendre à `attendu` (attente conservée).
    Position {
        /// Taille actuelle du miroir.
        taille: u64,
        /// Position attendue du prochain envoi (miroir + attente).
        attendu: u64,
    },
    /// Trop d'entrées dans un envoi.
    TropDEntrees,
    /// Registre ou attente au-delà des bornes par COG (attente purgée).
    Bornes,
    /// Budget global de l'attente épuisé (attente conservée, réessayer).
    BudgetGlobal,
    /// Trop d'enveloppes retenues pour ce COG (anti-rejeu saturé, réessayer).
    TropDEnvois,
    /// Tête ou entrées refusées par le registre (signature, fraîcheur, racine…).
    Registre(String),
    /// État indisponible.
    EtatIndisponible,
}

impl RefusSynchro {
    /// Motif court (journal agrégé, réponse).
    pub fn motif(&self) -> &'static str {
        match self {
            Self::CogInconnu => "cog_inconnu",
            Self::NonAuthentifie => "non_authentifie",
            Self::EnveloppeExpiree => "enveloppe_expiree",
            Self::Position { .. } => "position",
            Self::TropDEntrees => "trop_d_entrees",
            Self::Bornes => "bornes",
            Self::BudgetGlobal => "budget_global",
            Self::TropDEnvois => "trop_d_envois",
            Self::Registre(_) => "rejected",
            Self::EtatIndisponible => "unavailable",
        }
    }

    /// Statut HTTP de la réponse.
    pub fn statut(&self) -> u16 {
        match self {
            Self::CogInconnu => 403,
            Self::NonAuthentifie | Self::EnveloppeExpiree => 401,
            Self::Position { .. } => 409,
            Self::TropDEntrees | Self::Bornes => 413,
            Self::TropDEnvois => 429,
            Self::Registre(_) => 400,
            Self::BudgetGlobal | Self::EtatIndisponible => 503,
        }
    }
}

impl std::fmt::Display for RefusSynchro {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CogInconnu => f.write_str("COG sans registre déclaré"),
            Self::NonAuthentifie => f.write_str("enveloppe non authentifiée"),
            Self::EnveloppeExpiree => f.write_str("échéance de l'enveloppe hors fenêtre"),
            Self::Position { taille, attendu } => {
                write!(f, "position incorrecte (taille du miroir : {taille}, attendue : {attendu})")
            }
            Self::TropDEntrees => write!(f, "plus de {MAX_ENTREES_SYNCHRO} entrées"),
            Self::Bornes => f.write_str("registre au-delà des bornes"),
            Self::BudgetGlobal => f.write_str("budget global de l'attente épuisé"),
            Self::TropDEnvois => f.write_str("trop d'enveloppes en cours pour ce COG"),
            Self::Registre(e) => write!(f, "registre : {e}"),
            Self::EtatIndisponible => f.write_str("état indisponible"),
        }
    }
}

/// Synchronisation acceptée (morceau mis en attente, ou tête appliquée).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Synchronise {
    /// Taille du miroir.
    pub taille: u64,
    /// Entrées en attente (après le miroir) ; 0 après une tête.
    pub en_attente: u64,
    /// Horodatage de la tête vérifiée la plus récente (aucune : jamais synchronisé).
    pub a_jour_au: Option<u64>,
    /// Révocations nouvelles prises en compte.
    pub ajoutees: usize,
    /// Tête déjà appliquée ou enveloppe rejouée : rien de modifié.
    pub rejeu: bool,
}

// ---------------------------------------------------------------------------
// Enveloppe
// ---------------------------------------------------------------------------

/// Empreinte des entrées d'un envoi : SHA-256 des entrées concaténées, chacune
/// préfixée de sa longueur (u32 BE) pour fixer les frontières.
pub fn empreinte_entrees(entrees: &[Vec<u8>]) -> [u8; 32] {
    let mut h = Sha256::new();
    for e in entrees {
        // Entrées bornées par le corps (≤ 2 Mio) : la longueur tient sur u32.
        h.update(u32::try_from(e.len()).unwrap_or(u32::MAX).to_be_bytes());
        h.update(e);
    }
    h.finalize().into()
}

/// Octets signés par la clé de journal du COG pour un envoi (voir l'en-tête du module).
pub fn octets_enveloppe(cog: &CogId, depuis: u64, tete: Option<&[u8]>, entrees: &[Vec<u8>], echeance: u64) -> Vec<u8> {
    let id = cog.as_str().as_bytes();
    let mut m = Vec::with_capacity(DOMAINE_ENVELOPPE.len() + id.len() + 100);
    m.extend_from_slice(DOMAINE_ENVELOPPE);
    m.push(0);
    // Un CogId est court (validé par `CogId::new`) : la longueur tient sur u16.
    m.extend_from_slice(&u16::try_from(id.len()).unwrap_or(u16::MAX).to_be_bytes());
    m.extend_from_slice(id);
    m.extend_from_slice(&depuis.to_be_bytes());
    m.extend_from_slice(&(entrees.len() as u64).to_be_bytes());
    m.extend_from_slice(&empreinte_entrees(entrees));
    if let Some(t) = tete {
        m.push(1);
        m.extend_from_slice(&Sha256::digest(t));
    } else {
        m.push(0);
        m.extend_from_slice(&[0u8; 32]);
    }
    m.extend_from_slice(&echeance.to_be_bytes());
    m
}

/// Envoi dont l'enveloppe a été vérifiée (construit seulement par
/// [`RevocationsReconnues::authentifier`]).
#[derive(Debug)]
pub struct EnvoiAuthentifie {
    cog: CogId,
    depuis: u64,
    tete: Option<Vec<u8>>,
    entrees: Vec<Vec<u8>>,
    echeance: u64,
    /// SHA-256 du message signé (anti-rejeu).
    empreinte: [u8; 32],
}

impl EnvoiAuthentifie {
    /// COG authentifié.
    pub fn cog(&self) -> &CogId {
        &self.cog
    }
}

// ---------------------------------------------------------------------------
// État
// ---------------------------------------------------------------------------

/// Budget global de l'attente (somme sur tous les COG).
#[derive(Debug)]
struct BudgetGlobal {
    max_entrees: u64,
    max_octets: u64,
    /// (entrées, octets) réservés.
    utilise: Mutex<(u64, u64)>,
}

impl BudgetGlobal {
    fn new(max_entrees: u64, max_octets: u64) -> Self {
        Self { max_entrees, max_octets, utilise: Mutex::new((0, 0)) }
    }

    fn reserver(&self, entrees: u64, octets: u64) -> bool {
        let mut u = self.utilise.lock().unwrap_or_else(PoisonError::into_inner);
        let (e, o) = (u.0.saturating_add(entrees), u.1.saturating_add(octets));
        if e > self.max_entrees || o > self.max_octets {
            return false;
        }
        *u = (e, o);
        true
    }

    fn liberer(&self, entrees: u64, octets: u64) {
        let mut u = self.utilise.lock().unwrap_or_else(PoisonError::into_inner);
        *u = (u.0.saturating_sub(entrees), u.1.saturating_sub(octets));
    }

    fn utilise(&self) -> (u64, u64) {
        *self.utilise.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

/// Entrées reçues sans tête, en attente de validation.
#[derive(Debug, Default)]
struct Attente {
    entrees: Vec<Vec<u8>>,
    /// Octets réservés au budget global (= somme des longueurs des entrées).
    octets: u64,
    /// Taille du miroir quand l'attente a commencé.
    base: u64,
    /// Heure du premier morceau.
    debut: u64,
}

impl Attente {
    /// Vide l'attente, rend ses entrées et libère sa part du budget global.
    fn vider(&mut self, budget: &BudgetGlobal) -> Vec<Vec<u8>> {
        budget.liberer(self.entrees.len() as u64, self.octets);
        std::mem::take(self).entrees
    }

    fn purger(&mut self, budget: &BudgetGlobal) {
        drop(self.vider(budget));
    }

    fn perimee(&self, maintenant: u64) -> bool {
        !self.entrees.is_empty() && maintenant > self.debut.saturating_add(DUREE_ATTENTE_SECS)
    }
}

/// État mutable d'un COG (sérialise ses synchronisations).
#[derive(Debug, Default)]
struct EtatCog {
    attente: Attente,
    /// Empreintes des enveloppes traitées → échéance (anti-rejeu).
    vues: BTreeMap<[u8; 32], u64>,
}

#[derive(Debug)]
struct SuiviCog {
    suivi: SuiviRevocationsRegistre,
    /// Clé de journal épinglée (vérifie les enveloppes).
    cle_journal: VerifyingKey,
    /// Jamais pris par les vérifications de révocation.
    etat: Mutex<EtatCog>,
}

/// Journal agrégé des refus : un résumé par fenêtre au plus.
#[derive(Debug, Default)]
pub struct JournalRefus {
    etat: Mutex<(BTreeMap<&'static str, u64>, u64)>,
}

impl JournalRefus {
    /// Note un refus de motif `motif` ; rend le résumé à journaliser quand la
    /// fenêtre ([`FENETRE_JOURNAL_REFUS_SECS`]) ouverte par le premier refus est close.
    pub fn noter(&self, motif: &'static str, maintenant: u64) -> Option<String> {
        let mut etat = self.etat.lock().unwrap_or_else(PoisonError::into_inner);
        let (comptes, debut) = &mut *etat;
        if comptes.is_empty() {
            *debut = maintenant;
        }
        *comptes.entry(motif).or_default() += 1;
        if maintenant < debut.saturating_add(FENETRE_JOURNAL_REFUS_SECS) {
            return None;
        }
        let total: u64 = comptes.values().sum();
        let detail: Vec<String> = comptes.iter().map(|(m, n)| format!("{m}={n}")).collect();
        let resume = format!("{total} refus en {} s ({})", maintenant.saturating_sub(*debut), detail.join(", "));
        comptes.clear();
        Some(resume)
    }
}

/// Révocations des COG reconnus (un suivi par COG, tiré de son registre vérifié).
#[derive(Debug)]
pub struct RevocationsReconnues {
    /// Fixée à la construction : aucun verrou global.
    cogs: BTreeMap<CogId, SuiviCog>,
    refus: JournalRefus,
    budget: BudgetGlobal,
}

impl Default for RevocationsReconnues {
    fn default() -> Self {
        Self {
            cogs: BTreeMap::new(),
            refus: JournalRefus::default(),
            budget: BudgetGlobal::new(MAX_ENTREES_ATTENTE_GLOBALE, MAX_OCTETS_ATTENTE_GLOBALE),
        }
    }
}

impl RevocationsReconnues {
    /// Suivi des registres déclarés. Doublon de COG ou clé invalide ⇒ erreur.
    pub fn new(registres: &[RegistreReconnu]) -> Result<Self, String> {
        Self::avec_budget(registres, MAX_ENTREES_ATTENTE_GLOBALE, MAX_OCTETS_ATTENTE_GLOBALE)
    }

    /// Comme [`Self::new`], avec un budget global d'attente explicite.
    pub fn avec_budget(registres: &[RegistreReconnu], max_entrees: u64, max_octets: u64) -> Result<Self, String> {
        let mut cogs = BTreeMap::new();
        for r in registres {
            let suivi = SuiviRevocationsRegistre::new(r.cog.clone(), r.registre, &r.cle_journal)
                .map_err(|e| format!("cle_journal ({}) : {e}", r.cog))?;
            let cle_journal =
                VerifyingKey::from_bytes(&r.cle_journal).map_err(|e| format!("cle_journal ({}) : {e}", r.cog))?;
            let ancien = cogs.insert(r.cog.clone(), SuiviCog { suivi, cle_journal, etat: Mutex::new(EtatCog::default()) });
            if ancien.is_some() {
                return Err(format!("registre déclaré deux fois pour {}", r.cog));
            }
        }
        Ok(Self { cogs, refus: JournalRefus::default(), budget: BudgetGlobal::new(max_entrees, max_octets) })
    }

    /// Registres déclarés dans `[[forum.cogs_reconnus]]` (`registre` + `cle_journal`).
    /// Un seul des deux champs, valeur invalide ⇒ erreur (aucune entrée ignorée).
    pub fn depuis_config(config: &ForumConfig) -> Result<Self, String> {
        let hex32 = |texte: &str, champ: &str| -> Result<[u8; 32], String> {
            hex::decode(texte.trim())
                .ok()
                .and_then(|v| v.try_into().ok())
                .ok_or_else(|| format!("forum.cogs_reconnus.{champ} : 32 octets hexadécimaux attendus"))
        };
        let mut registres = Vec::new();
        for c in &config.cogs_reconnus {
            let cog = CogId::new(c.cog_id.trim()).ok_or_else(|| format!("forum.cogs_reconnus : cog_id invalide « {} »", c.cog_id))?;
            match (c.registre.as_deref(), c.cle_journal.as_deref()) {
                (None, None) => {}
                (Some(registre), Some(cle)) => registres.push(RegistreReconnu {
                    cog,
                    registre: hex32(registre, "registre")?,
                    cle_journal: hex32(cle, "cle_journal")?,
                }),
                _ => return Err(format!("forum.cogs_reconnus ({cog}) : `registre` et `cle_journal` vont ensemble")),
            }
        }
        Self::new(&registres)
    }

    /// Vrai si le registre de `cog` est suivi.
    pub fn suit(&self, cog: &CogId) -> bool {
        self.cogs.contains_key(cog)
    }

    /// Taille du miroir de `cog`.
    pub fn taille(&self, cog: &CogId) -> Option<u64> {
        self.cogs.get(cog)?.suivi.taille()
    }

    /// Entrées en attente pour `cog` (sans tête).
    pub fn en_attente(&self, cog: &CogId) -> Option<u64> {
        let etat = self.cogs.get(cog)?.etat.lock().ok()?;
        Some(etat.attente.entrees.len() as u64)
    }

    /// Budget global réservé : (entrées, octets) en attente, tous COG confondus.
    pub fn attente_globale(&self) -> (u64, u64) {
        self.budget.utilise()
    }

    /// Note un refus de la route ; journalise un résumé agrégé au plus une fois par fenêtre.
    pub fn noter_refus(&self, motif: &'static str, maintenant: u64) {
        if let Some(resume) = self.refus.noter(motif, maintenant) {
            warn!("Synchro des révocations : {resume}");
        }
    }

    /// Vérifie l'enveloppe d'un envoi (échéance, signature de la clé de journal
    /// de `cog`). **Aucun état n'est lu ni modifié** hormis la clé épinglée.
    #[allow(clippy::too_many_arguments)]
    pub fn authentifier(
        &self,
        cog: &CogId,
        depuis: u64,
        tete: Option<Vec<u8>>,
        entrees: Vec<Vec<u8>>,
        echeance: u64,
        signature: &[u8; 64],
        maintenant: u64,
    ) -> Result<EnvoiAuthentifie, RefusSynchro> {
        if entrees.len() > MAX_ENTREES_SYNCHRO {
            return Err(RefusSynchro::TropDEntrees);
        }
        let suivi_cog = self.cogs.get(cog).ok_or(RefusSynchro::CogInconnu)?;
        let limite = maintenant.saturating_add(DUREE_MAX_ENVELOPPE_SECS).saturating_add(TOLERANCE_HORLOGE_SECS);
        if echeance < maintenant || echeance > limite {
            return Err(RefusSynchro::EnveloppeExpiree);
        }
        let message = octets_enveloppe(cog, depuis, tete.as_deref(), &entrees, echeance);
        suivi_cog
            .cle_journal
            .verify_strict(&message, &Signature::from_bytes(signature))
            .map_err(|_| RefusSynchro::NonAuthentifie)?;
        Ok(EnvoiAuthentifie {
            cog: cog.clone(),
            depuis,
            tete,
            entrees,
            echeance,
            empreinte: Sha256::digest(&message).into(),
        })
    }

    /// Purge les attentes périmées des **autres** COG (sans attendre leur verrou).
    fn balayer_perimees(&self, sauf: &CogId, maintenant: u64) {
        for (c, s) in &self.cogs {
            if c == sauf {
                continue;
            }
            if let Ok(mut etat) = s.etat.try_lock() {
                if etat.attente.perimee(maintenant) {
                    etat.attente.purger(&self.budget);
                }
            }
        }
    }

    /// Synchronise un envoi authentifié : morceau (`tete` absente, entrées
    /// mises en attente) ou tête (attente + entrées validées ensemble).
    ///
    /// Atomicité : le miroir et la liste du COG sont modifiés ensemble ou pas
    /// du tout ([`SuiviRevocationsRegistre::appliquer`]). Sur refus, seule une
    /// tête refusée par le registre ou un dépassement des bornes du COG purge
    /// l'attente ; une position incohérente ou le budget global la conservent.
    /// Une enveloppe déjà traitée est sans effet (`rejeu`).
    pub fn synchroniser(&self, envoi: &EnvoiAuthentifie, maintenant: u64) -> Result<Synchronise, RefusSynchro> {
        if envoi.echeance < maintenant {
            return Err(RefusSynchro::EnveloppeExpiree);
        }
        let suivi_cog = self.cogs.get(&envoi.cog).ok_or(RefusSynchro::CogInconnu)?;
        let mut etat = suivi_cog.etat.lock().map_err(|_| RefusSynchro::EtatIndisponible)?;
        let suivi = &suivi_cog.suivi;
        let taille = suivi.taille().ok_or(RefusSynchro::EtatIndisponible)?;
        let EtatCog { attente, vues } = &mut *etat;
        if attente.perimee(maintenant) || (!attente.entrees.is_empty() && attente.base != taille) {
            attente.purger(&self.budget);
        }
        let en_attente = attente.entrees.len() as u64;
        let attendu = taille.saturating_add(en_attente);

        // Anti-rejeu : une enveloppe ne sert qu'une fois, quelle qu'en soit l'issue.
        vues.retain(|_, echeance| *echeance >= maintenant);
        if vues.contains_key(&envoi.empreinte) {
            return Ok(Synchronise { taille, en_attente, a_jour_au: suivi.a_jour_au(), ajoutees: 0, rejeu: true });
        }
        if vues.len() >= MAX_ENVELOPPES_VUES {
            return Err(RefusSynchro::TropDEnvois);
        }
        vues.insert(envoi.empreinte, envoi.echeance);

        let entrees = &envoi.entrees;
        let Some(tete) = envoi.tete.as_deref() else {
            // Morceau : position exacte, bornes par COG puis globales.
            if envoi.depuis != attendu {
                return Err(RefusSynchro::Position { taille, attendu });
            }
            let ajout: u64 = entrees.iter().fold(0u64, |t, e| t.saturating_add(e.len() as u64));
            let octets = attente.octets.saturating_add(ajout);
            if attendu.saturating_add(entrees.len() as u64) > MAX_ENTREES_SUIVI || octets > MAX_OCTETS_SUIVI {
                attente.purger(&self.budget);
                return Err(RefusSynchro::Bornes);
            }
            let n = entrees.len() as u64;
            if !self.budget.reserver(n, ajout) {
                self.balayer_perimees(&envoi.cog, maintenant);
                if !self.budget.reserver(n, ajout) {
                    return Err(RefusSynchro::BudgetGlobal);
                }
            }
            if attente.entrees.is_empty() {
                attente.base = taille;
                attente.debut = maintenant;
            }
            attente.entrees.extend_from_slice(entrees);
            attente.octets = octets;
            return Ok(Synchronise {
                taille,
                en_attente: attente.entrees.len() as u64,
                a_jour_au: suivi.a_jour_au(),
                ajoutees: 0,
                rejeu: false,
            });
        };

        // Tête : valide l'attente et les entrées de cet envoi, ensemble.
        let resultat = if attente.entrees.is_empty() {
            suivi.appliquer(tete, envoi.depuis, entrees, maintenant)
        } else if envoi.depuis != attendu {
            return Err(RefusSynchro::Position { taille, attendu });
        } else {
            let mut toutes = attente.vider(&self.budget);
            toutes.extend_from_slice(entrees);
            suivi.appliquer(tete, taille, &toutes, maintenant)
        };
        attente.purger(&self.budget);
        match resultat {
            Ok(a) => Ok(Synchronise {
                taille: a.taille,
                en_attente: 0,
                a_jour_au: Some(a.a_jour_au),
                ajoutees: a.ajoutees,
                rejeu: a.rejeu,
            }),
            Err(RefusSuivi::Couverture { taille, .. }) => Err(RefusSynchro::Position { taille, attendu: taille }),
            Err(RefusSuivi::Bornes) => Err(RefusSynchro::Bornes),
            Err(RefusSuivi::EtatIndisponible) => Err(RefusSynchro::EtatIndisponible),
            Err(e @ RefusSuivi::Registre(_)) => Err(RefusSynchro::Registre(e.to_string())),
        }
    }
}

impl SourceRevocation for RevocationsReconnues {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        // Aucun verrou global : seul le RwLock (lecture) du suivi du COG est pris.
        self.cogs.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?.suivi.est_revoque(emetteur, cible, maintenant_secs)
    }
}

// ---------------------------------------------------------------------------
// Route HTTP
// ---------------------------------------------------------------------------

/// Corps JSON de `POST /api/auth/revocations/sync` (poussé par un COG reconnu).
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevocationsSyncRequest {
    /// COG dont le registre est poussé.
    pub cog_id: String,
    /// Index de la première entrée envoyée.
    pub depuis: u64,
    /// Tête de registre scellée (base64 URL) ; absente : morceau mis en attente.
    #[serde(default)]
    pub tete: Option<String>,
    /// Entrées `depuis..` (base64 URL), au plus [`MAX_ENTREES_SYNCHRO`].
    pub entrees: Vec<String>,
    /// Échéance de l'enveloppe (secondes Unix, ≤ 15 min).
    #[serde(default)]
    pub echeance: Option<u64>,
    /// Signature Ed25519 (base64 URL, 64 o) de [`octets_enveloppe`] par la clé de journal.
    #[serde(default)]
    pub signature: Option<String>,
}

/// Requête analysée (COG valide, nombre d'entrées borné), pas encore décodée.
#[derive(Debug)]
pub struct RequeteSynchro {
    /// COG visé (non authentifié à ce stade).
    pub cog: CogId,
    requete: RevocationsSyncRequest,
}

fn reponse(statut: u16, corps: &serde_json::Value) -> RouteResponse {
    let ligne = match statut {
        200 => "200 OK",
        400 => "400 Bad Request",
        401 => "401 Unauthorized",
        403 => "403 Forbidden",
        409 => "409 Conflict",
        413 => "413 Payload Too Large",
        429 => "429 Too Many Requests",
        503 => "503 Service Unavailable",
        _ => "500 Internal Server Error",
    };
    RouteResponse::Normal {
        status: ligne.to_string(),
        content_type: "application/json; charset=utf-8".to_string(),
        body: corps.to_string(),
    }
}

/// Réponse de refus `statut` de motif `motif` (noté au journal agrégé).
pub fn refus(revocations: &RevocationsReconnues, statut: u16, motif: &'static str, maintenant: u64) -> RouteResponse {
    revocations.noter_refus(motif, maintenant);
    reponse(statut, &serde_json::json!({ "ok": false, "error": motif }))
}

fn reponse_refus(revocations: &RevocationsReconnues, e: &RefusSynchro, maintenant: u64) -> RouteResponse {
    revocations.noter_refus(e.motif(), maintenant);
    let mut corps = serde_json::json!({ "ok": false, "error": e.motif() });
    if let RefusSynchro::Position { taille, attendu } = e {
        corps["taille"] = (*taille).into();
        corps["attendu"] = (*attendu).into();
    }
    reponse(e.statut(), &corps)
}

/// Clé du limiteur de débit pour `cog` (16 premiers octets de SHA-256).
/// À n'utiliser que pour un COG **authentifié** ([`EnvoiAuthentifie::cog`]).
pub fn cle_debit(cog: &CogId) -> [u8; 16] {
    let h = Sha256::digest(format!("origin/revocations/sync/{cog}").as_bytes());
    let mut cle = [0u8; 16];
    cle.copy_from_slice(&h[..16]);
    cle
}

/// Analyse le corps : JSON, `cog_id`, nombre d'entrées (**avant** tout décodage base64).
pub fn analyser(revocations: &RevocationsReconnues, body: &[u8], maintenant: u64) -> Result<RequeteSynchro, RouteResponse> {
    if body.len() > TAILLE_MAX_CORPS_REVOCATIONS {
        return Err(refus(revocations, 413, "payload_too_large", maintenant));
    }
    let Ok(requete) = serde_json::from_slice::<RevocationsSyncRequest>(body) else {
        return Err(refus(revocations, 400, "invalid_json", maintenant));
    };
    if requete.entrees.len() > MAX_ENTREES_SYNCHRO {
        return Err(refus(revocations, 413, "trop_d_entrees", maintenant));
    }
    let Some(cog) = CogId::new(requete.cog_id.trim()) else {
        return Err(refus(revocations, 400, "cog_id", maintenant));
    };
    if !revocations.suit(&cog) {
        return Err(refus(revocations, 403, "cog_inconnu", maintenant));
    }
    Ok(RequeteSynchro { cog, requete })
}

/// Décode une requête analysée et vérifie son enveloppe (calcul lourd : hors
/// de la boucle async). Aucun état n'est modifié.
pub fn authentifier_requete(
    revocations: &RevocationsReconnues,
    r: &RequeteSynchro,
    maintenant: u64,
) -> Result<EnvoiAuthentifie, RouteResponse> {
    let non_authentifie = |m| reponse_refus(revocations, &RefusSynchro::NonAuthentifie, m);
    let (Some(echeance), Some(signature)) = (r.requete.echeance, r.requete.signature.as_deref()) else {
        return Err(non_authentifie(maintenant));
    };
    let Some(signature) = b64_decoder(signature).and_then(|s| <[u8; 64]>::try_from(s).ok()) else {
        return Err(non_authentifie(maintenant));
    };
    let tete = match r.requete.tete.as_deref().map(b64_decoder) {
        None => None,
        Some(Some(t)) => Some(t),
        Some(None) => return Err(refus(revocations, 400, "base64", maintenant)),
    };
    let Some(entrees) = r.requete.entrees.iter().map(|e| b64_decoder(e)).collect::<Option<Vec<Vec<u8>>>>() else {
        return Err(refus(revocations, 400, "base64", maintenant));
    };
    revocations
        .authentifier(&r.cog, r.requete.depuis, tete, entrees, echeance, &signature, maintenant)
        .map_err(|e| reponse_refus(revocations, &e, maintenant))
}

/// Applique un envoi authentifié (calcul lourd : hors de la boucle async).
pub fn appliquer_envoi(revocations: &RevocationsReconnues, envoi: &EnvoiAuthentifie, maintenant: u64) -> RouteResponse {
    match revocations.synchroniser(envoi, maintenant) {
        Ok(s) => reponse(
            200,
            &serde_json::json!({
                "ok": true,
                "taille": s.taille,
                "en_attente": s.en_attente,
                "a_jour_au": s.a_jour_au,
                "ajoutees": s.ajoutees,
                "rejeu": s.rejeu,
            }),
        ),
        Err(e) => reponse_refus(revocations, &e, maintenant),
    }
}

/// [`analyser`], [`authentifier_requete`] puis [`appliquer_envoi`] (sans
/// limitation de débit : voir `web::server`).
pub fn route_synchro(revocations: &RevocationsReconnues, body: &[u8], maintenant: u64) -> RouteResponse {
    let envoi = match analyser(revocations, body, maintenant).and_then(|r| authentifier_requete(revocations, &r, maintenant)) {
        Ok(e) => e,
        Err(refus) => return refus,
    };
    appliquer_envoi(revocations, &envoi, maintenant)
}

/// Banc d'essai : COG reconnu tenant un journal (tests seulement).
#[cfg(test)]
pub(crate) mod banc {
    use super::*;
    use ed25519_dalek::{Signer, SigningKey};
    use miyuregistre::{cible_serie, Cible, Corps, EntreeRegistre, Journal, StockageMemoire};
    use miyusceau::{sceller, CleSignature};

    /// Échéance des enveloppes du banc (10 min).
    pub(crate) const VALIDITE: u64 = 600;

    /// Signe l'enveloppe d'un envoi avec `cle`.
    pub(crate) fn signer(
        cle: &CleSignature,
        cog: &CogId,
        depuis: u64,
        tete: Option<&[u8]>,
        entrees: &[Vec<u8>],
        echeance: u64,
    ) -> [u8; 64] {
        SigningKey::from_bytes(&cle.graine()).sign(&octets_enveloppe(cog, depuis, tete, entrees, echeance)).to_bytes()
    }

    /// Registre d'un COG reconnu (côté COG).
    pub(crate) struct RegistreCog {
        pub(crate) cog: CogId,
        pub(crate) registre: [u8; 32],
        pub(crate) cle_journal: CleSignature,
        pub(crate) journal: Journal<StockageMemoire>,
    }

    impl RegistreCog {
        pub(crate) fn new(cog: CogId) -> Self {
            let registre = [0x52; 32];
            Self {
                journal: Journal::ouvrir(registre, StockageMemoire::new(), StockageMemoire::new()).unwrap(),
                cog,
                registre,
                cle_journal: CleSignature::generer(),
            }
        }

        pub(crate) fn declaration(&self) -> RegistreReconnu {
            RegistreReconnu { cog: self.cog.clone(), registre: self.registre, cle_journal: self.cle_journal.cle_publique() }
        }

        /// Inscrit une révocation signée par `cle` (la clé de journal par défaut).
        pub(crate) fn revoquer(&mut self, cible: &CibleRevocation, cle: Option<&CleSignature>, t: u64) {
            let cible = match cible {
                CibleRevocation::Cle(kid) => Cible::Cle(*kid),
                CibleRevocation::Serie(s) => cible_serie(s),
            };
            let e = EntreeRegistre { inscrite_a: t, corps: Corps::Revocation { cible, motif: "test".into() } };
            let octets = sceller(&e, cle.unwrap_or(&self.cle_journal)).unwrap();
            self.journal.ajouter(&octets).unwrap();
        }

        /// Tête émise à `t` et entrées depuis `depuis`.
        pub(crate) fn publier(&mut self, depuis: u64, t: u64) -> (Vec<u8>, Vec<Vec<u8>>) {
            let tete = self.journal.emettre_tete(&[&self.cle_journal], t, 86_400).unwrap();
            (tete, self.journal.entrees(depuis, u64::MAX).unwrap())
        }

        /// Envoi authentifié (enveloppe signée par la clé de journal, échéance `t + VALIDITE`).
        pub(crate) fn envoi(
            &self,
            origin: &RevocationsReconnues,
            depuis: u64,
            tete: Option<&[u8]>,
            entrees: &[Vec<u8>],
            t: u64,
        ) -> Result<EnvoiAuthentifie, RefusSynchro> {
            let echeance = t + VALIDITE;
            let sig = signer(&self.cle_journal, &self.cog, depuis, tete, entrees, echeance);
            origin.authentifier(&self.cog, depuis, tete.map(<[u8]>::to_vec), entrees.to_vec(), echeance, &sig, t)
        }

        /// Comme [`Self::envoi`], sans copier les entrées.
        pub(crate) fn envoi_vec(
            &self,
            origin: &RevocationsReconnues,
            depuis: u64,
            entrees: Vec<Vec<u8>>,
            t: u64,
        ) -> Result<EnvoiAuthentifie, RefusSynchro> {
            let echeance = t + VALIDITE;
            let sig = signer(&self.cle_journal, &self.cog, depuis, None, &entrees, echeance);
            origin.authentifier(&self.cog, depuis, None, entrees, echeance, &sig, t)
        }

        /// Envoie (authentifie puis synchronise) à `t`.
        pub(crate) fn envoyer(
            &self,
            origin: &RevocationsReconnues,
            depuis: u64,
            tete: Option<&[u8]>,
            entrees: &[Vec<u8>],
            t: u64,
        ) -> Result<Synchronise, RefusSynchro> {
            origin.synchroniser(&self.envoi(origin, depuis, tete, entrees, t)?, t)
        }

        /// Corps JSON signé de la route.
        pub(crate) fn corps_json(&self, depuis: u64, tete: Option<&[u8]>, entrees: &[Vec<u8>], t: u64) -> String {
            let b64 = |o: &[u8]| miyauth::http::b64_encoder(o);
            let echeance = t + VALIDITE;
            let sig = signer(&self.cle_journal, &self.cog, depuis, tete, entrees, echeance);
            let mut corps = serde_json::json!({
                "cog_id": self.cog.as_str(),
                "depuis": depuis,
                "entrees": entrees.iter().map(|e| b64(e)).collect::<Vec<_>>(),
                "echeance": echeance,
                "signature": b64(&sig),
            });
            if let Some(t) = tete {
                corps["tete"] = b64(t).into();
            }
            corps.to_string()
        }

        /// Synchronise `origin` (tête à `t`).
        pub(crate) fn pousser(&mut self, origin: &RevocationsReconnues, t: u64) -> Result<Synchronise, RefusSynchro> {
            let depuis = origin.taille(&self.cog).unwrap_or(0);
            let (tete, entrees) = self.publier(depuis, t);
            self.envoyer(origin, depuis, Some(&tete), &entrees, t)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::banc::{signer, RegistreCog, VALIDITE};
    use super::*;
    use miyauth::revocation::FRAICHEUR_MAX_DEFAUT_SECS;
    use miyusceau::CleSignature;

    const T0: u64 = 1_800_000_000;

    fn cog(c: char) -> CogId {
        CogId::new(&c.to_string().repeat(26)).unwrap()
    }

    fn statut(rep: &RouteResponse) -> String {
        match rep {
            RouteResponse::Normal { status, .. } => status.clone(),
            _ => String::new(),
        }
    }

    fn json(rep: &RouteResponse) -> serde_json::Value {
        match rep {
            RouteResponse::Normal { body, .. } => serde_json::from_str(body).unwrap(),
            _ => serde_json::Value::Null,
        }
    }

    #[test]
    fn revocations_du_registre_appliquees() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        let cible = CibleRevocation::Cle([7; 32]);
        // Jamais synchronisé : liste absente (refus).
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Err(ErreurRevocation::ListeAbsente));

        r.revoquer(&cible, None, T0);
        r.revoquer(&CibleRevocation::Serie([4; 16]), None, T0);
        // Signée par une clé non épinglée : ignorée.
        let intrus = CleSignature::generer();
        r.revoquer(&CibleRevocation::Cle([8; 32]), Some(&intrus), T0);
        let s = r.pousser(&origin, T0).unwrap();
        assert_eq!((s.taille, s.a_jour_au, s.ajoutees), (3, Some(T0), 2));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Serie([4; 16]), T0), Ok(true));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([8; 32]), T0), Ok(false));
        // COG non déclaré : absente.
        assert_eq!(origin.est_revoque(&cog('z'), &cible, T0), Err(ErreurRevocation::ListeAbsente));

        // Synchronisation incrémentale.
        r.revoquer(&CibleRevocation::Cle([9; 32]), None, T0 + 60);
        assert_eq!(r.pousser(&origin, T0 + 60).unwrap().ajoutees, 1);
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([9; 32]), T0 + 60), Ok(true));
    }

    #[test]
    fn declaration_depuis_config() {
        use crate::config::{CogReconnuConfig, ForumConfig};
        let entree = |registre: Option<&str>, cle: Option<&str>| CogReconnuConfig {
            cog_id: "h".repeat(26),
            cle_emission: "11".repeat(32),
            registre: registre.map(str::to_string),
            cle_journal: cle.map(str::to_string),
        };
        let config = |e: CogReconnuConfig| ForumConfig { cogs_reconnus: vec![e], ..Default::default() };
        let cle = hex::encode(CleSignature::generer().cle_publique());
        let r = RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&cle)))).unwrap();
        assert!(r.suit(&cog('h')));
        // Sans registre déclaré : non suivi (ses passeports seront refusés).
        assert!(!RevocationsReconnues::depuis_config(&config(entree(None, None))).unwrap().suit(&cog('h')));
        // Un seul des deux champs, ou valeur invalide : erreur.
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), None))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(None, Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some("52"), Some(&cle)))).is_err());
        assert!(RevocationsReconnues::depuis_config(&config(entree(Some(&"52".repeat(32)), Some(&"00".repeat(32))))).is_err());
    }

    #[test]
    fn perimee_ou_invalide_refusee() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        r.pousser(&origin, T0).unwrap();
        let cible = CibleRevocation::Cle([1; 32]);
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS), Ok(false));
        // Au-delà de 15 min sans nouvelle tête : périmée, refus.
        assert!(matches!(
            origin.est_revoque(&r.cog, &cible, T0 + FRAICHEUR_MAX_DEFAUT_SECS + 1),
            Err(ErreurRevocation::Perimee { .. })
        ));

        // Mauvaise position : 409 avec la taille.
        let (tete, entrees) = r.publier(0, T0 + 10);
        assert_eq!(r.envoyer(&origin, 5, Some(&tete), &entrees, T0 + 10), Err(RefusSynchro::Position { taille: 0, attendu: 0 }));
        // Tête signée par une autre clé que la clé de journal : refusée dès l'enveloppe.
        let mut faux = RegistreCog::new(r.cog.clone());
        let (tete_f, entrees_f) = faux.publier(0, T0 + 20);
        assert_eq!(faux.envoyer(&origin, 0, Some(&tete_f), &entrees_f, T0 + 20), Err(RefusSynchro::NonAuthentifie));
        // Enveloppe authentique, tête d'une autre clé : refus du registre, état inchangé.
        assert!(matches!(r.envoyer(&origin, 0, Some(&tete_f), &entrees_f, T0 + 20), Err(RefusSynchro::Registre(_))));
        // Entrée omise : position à reprendre ; entrée altérée (racine différente) : refus.
        r.revoquer(&cible, None, T0 + 30);
        let (tete, entrees) = r.publier(0, T0 + 30);
        assert_eq!(r.envoyer(&origin, 0, Some(&tete), &[], T0 + 30), Err(RefusSynchro::Position { taille: 0, attendu: 0 }));
        let mut alterees = entrees.clone();
        alterees[0] = b"faux".to_vec();
        assert!(matches!(r.envoyer(&origin, 0, Some(&tete), &alterees, T0 + 30), Err(RefusSynchro::Registre(_))));
        assert_eq!(origin.est_revoque(&r.cog, &cible, T0 + 30), Ok(false));
        // COG inconnu, trop d'entrées.
        let z = RegistreCog::new(cog('z'));
        assert_eq!(z.envoi(&origin, 0, Some(&tete), &[], T0).unwrap_err(), RefusSynchro::CogInconnu);
        let trop = vec![Vec::new(); MAX_ENTREES_SYNCHRO + 1];
        assert_eq!(r.envoi(&origin, 0, Some(&tete), &trop, T0).unwrap_err(), RefusSynchro::TropDEntrees);
        // Déclaration en double refusée.
        assert!(RevocationsReconnues::new(&[r.declaration(), r.declaration()]).is_err());
    }

    /// Pousse `entrees` par morceaux de `pas` puis la tête seule.
    fn pousser_par_morceaux(
        origin: &RevocationsReconnues,
        r: &RegistreCog,
        depuis: u64,
        tete: &[u8],
        entrees: &[Vec<u8>],
        pas: usize,
        t: u64,
    ) -> Result<Synchronise, RefusSynchro> {
        let mut position = depuis;
        for morceau in entrees.chunks(pas) {
            let s = r.envoyer(origin, position, None, morceau, t)?;
            position += morceau.len() as u64;
            assert_eq!(s.taille + s.en_attente, position);
        }
        r.envoyer(origin, position, Some(tete), &[], t)
    }

    #[test]
    fn registre_de_5000_entrees_synchronise_par_morceaux_authentifies() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..5000u32 {
            let mut kid = [0u8; 32];
            kid[..4].copy_from_slice(&i.to_be_bytes());
            r.revoquer(&CibleRevocation::Cle(kid), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        assert_eq!(entrees.len(), 5000);
        // D'un seul envoi : impossible (borne par envoi).
        assert_eq!(r.envoi(&origin, 0, Some(&tete), &entrees, T0).unwrap_err(), RefusSynchro::TropDEntrees);
        let s = pousser_par_morceaux(&origin, &r, 0, &tete, &entrees, MAX_ENTREES_SYNCHRO, T0).unwrap();
        assert_eq!((s.taille, s.en_attente, s.ajoutees, s.a_jour_au), (5000, 0, 5000, Some(T0)));
        assert_eq!(origin.attente_globale(), (0, 0), "budget global rendu");
        let mut derniere = [0u8; 32];
        derniere[..4].copy_from_slice(&4999u32.to_be_bytes());
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle(derniere), T0), Ok(true));
        // Rejeu de tête à l'identique (nouvelle enveloppe) : 200, rien de recalculé.
        let s = r.envoyer(&origin, 5000, Some(&tete), &[], T0 + 1).unwrap();
        assert!(s.rejeu);
        assert_eq!((s.taille, s.ajoutees), (5000, 0));
    }

    #[test]
    fn morceau_non_authentifie_refuse_sans_purge() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..10u8 {
            r.revoquer(&CibleRevocation::Cle([i; 32]), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        assert_eq!(r.envoyer(&origin, 0, None, &entrees[..4], T0).unwrap().en_attente, 4);
        let echeance = T0 + VALIDITE;
        let tenter = |depuis: u64, morceau: &[Vec<u8>], sig: &[u8; 64], echeance: u64| {
            origin.authentifier(&r.cog, depuis, None, morceau.to_vec(), echeance, sig, T0).map(|_| ())
        };
        // Non signé (signature nulle), mal signé (autre contenu), signé par une autre clé :
        // refusés à l'authentification ; aucun ne peut purger (mauvaise position 6) ni enrichir.
        assert_eq!(tenter(6, &entrees[6..], &[0u8; 64], echeance), Err(RefusSynchro::NonAuthentifie));
        let sig_autre_contenu = signer(&r.cle_journal, &r.cog, 4, None, &entrees[4..6], echeance);
        assert_eq!(tenter(6, &entrees[6..], &sig_autre_contenu, echeance), Err(RefusSynchro::NonAuthentifie));
        let intrus = CleSignature::generer();
        let sig_intrus = signer(&intrus, &r.cog, 6, None, &entrees[6..], echeance);
        assert_eq!(tenter(6, &entrees[6..], &sig_intrus, echeance), Err(RefusSynchro::NonAuthentifie));
        let sig_intrus = signer(&intrus, &r.cog, 4, None, &[b"faux".to_vec()], echeance);
        assert_eq!(tenter(4, &[b"faux".to_vec()], &sig_intrus, echeance), Err(RefusSynchro::NonAuthentifie));
        // Échéance trop lointaine ou dépassée : refus, même bien signée.
        let loin = T0 + DUREE_MAX_ENVELOPPE_SECS + TOLERANCE_HORLOGE_SECS + 1;
        let sig = signer(&r.cle_journal, &r.cog, 4, None, &entrees[4..6], loin);
        assert_eq!(tenter(4, &entrees[4..6], &sig, loin), Err(RefusSynchro::EnveloppeExpiree));
        let sig = signer(&r.cle_journal, &r.cog, 4, None, &entrees[4..6], T0 - 1);
        assert_eq!(tenter(4, &entrees[4..6], &sig, T0 - 1), Err(RefusSynchro::EnveloppeExpiree));
        assert_eq!(origin.en_attente(&r.cog), Some(4), "attente intacte");

        // Par la route : sans enveloppe, mal signée ⇒ 401, attente intacte.
        let b64 = |o: &[u8]| miyauth::http::b64_encoder(o);
        let sans = serde_json::json!({"cog_id": r.cog.as_str(), "depuis": 9, "entrees": [b64(b"x")]}).to_string();
        assert!(statut(&route_synchro(&origin, sans.as_bytes(), T0)).starts_with("401"));
        let mal = serde_json::json!({
            "cog_id": r.cog.as_str(), "depuis": 9, "entrees": [b64(b"x")], "echeance": echeance, "signature": b64(&sig_intrus),
        })
        .to_string();
        assert!(statut(&route_synchro(&origin, mal.as_bytes(), T0)).starts_with("401"));
        assert_eq!(origin.en_attente(&r.cog), Some(4), "attente intacte");

        // Morceau authentifié à une mauvaise position : 409 avec la position attendue, sans purge.
        assert_eq!(r.envoyer(&origin, 6, None, &entrees[6..], T0), Err(RefusSynchro::Position { taille: 0, attendu: 4 }));
        assert_eq!(origin.en_attente(&r.cog), Some(4));
        // Reprise à `attendu` : le flot authentique aboutit.
        r.envoyer(&origin, 4, None, &entrees[4..8], T0).unwrap();
        let s = r.envoyer(&origin, 8, Some(&tete), &entrees[8..], T0).unwrap();
        assert_eq!((s.taille, s.ajoutees, s.en_attente), (10, 10, 0));
    }

    #[test]
    fn rejeu_d_enveloppe_sans_effet() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..6u8 {
            r.revoquer(&CibleRevocation::Cle([i; 32]), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        let morceau = r.envoi(&origin, 0, None, &entrees[..3], T0).unwrap();
        assert_eq!(origin.synchroniser(&morceau, T0).unwrap().en_attente, 3);
        // Même enveloppe rejouée : sans effet (pas de doublon dans l'attente, pas de 409).
        let s = origin.synchroniser(&morceau, T0 + 1).unwrap();
        assert!(s.rejeu);
        assert_eq!((s.taille, s.en_attente), (0, 3));
        // Même enveloppe reçue de nouveau par la route : idem.
        let corps = r.corps_json(0, None, &entrees[..3], T0);
        let rep = route_synchro(&origin, corps.as_bytes(), T0);
        assert!(statut(&rep).starts_with("200"));
        assert_eq!(json(&rep)["rejeu"], true);
        assert_eq!(origin.en_attente(&r.cog), Some(3));
        // Le flot continue ; la tête rejouée après application est sans effet.
        r.envoyer(&origin, 3, None, &entrees[3..], T0).unwrap();
        let fin = r.envoi(&origin, 6, Some(&tete), &[], T0).unwrap();
        assert_eq!(origin.synchroniser(&fin, T0).unwrap().ajoutees, 6);
        let s = origin.synchroniser(&fin, T0 + 2).unwrap();
        assert!(s.rejeu);
        assert_eq!((s.taille, s.ajoutees), (6, 0));
        // Enveloppe échue : refusée même à l'application.
        assert_eq!(origin.synchroniser(&fin, T0 + VALIDITE + 1), Err(RefusSynchro::EnveloppeExpiree));
    }

    #[test]
    fn morceau_authentifie_incoherent() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        for i in 0..10u8 {
            r.revoquer(&CibleRevocation::Cle([i; 32]), None, T0);
        }
        let (tete, entrees) = r.publier(0, T0);
        // Morceau falsifié mais signé par le COG : la tête refuse le tout, l'attente est purgée.
        r.envoyer(&origin, 0, None, &entrees[..5], T0).unwrap();
        r.envoyer(&origin, 5, None, &[b"faux".to_vec()], T0).unwrap();
        assert!(matches!(r.envoyer(&origin, 6, Some(&tete), &entrees[6..], T0), Err(RefusSynchro::Registre(_))));
        assert_eq!(origin.taille(&r.cog), Some(0));
        assert_eq!(origin.en_attente(&r.cog), Some(0));
        assert_eq!(origin.attente_globale(), (0, 0));
        // Même morceau, même enveloppe : rejeu sans effet (rien n'est remis en attente).
        assert!(r.envoyer(&origin, 0, None, &entrees[..5], T0).unwrap().rejeu);
        assert_eq!(origin.en_attente(&r.cog), Some(0));
        // Tête avant la fin des morceaux : position à reprendre, attente conservée.
        r.envoyer(&origin, 0, None, &entrees[..5], T0 + 1).unwrap();
        assert_eq!(r.envoyer(&origin, 3, Some(&tete), &[], T0 + 1), Err(RefusSynchro::Position { taille: 0, attendu: 5 }));
        assert_eq!(origin.en_attente(&r.cog), Some(5));
        // Attente abandonnée plus de 15 min : purgée au morceau suivant.
        let tard = T0 + 1 + DUREE_ATTENTE_SECS + 1;
        assert_eq!(r.envoyer(&origin, 5, None, &entrees[5..], tard), Err(RefusSynchro::Position { taille: 0, attendu: 0 }));
        assert_eq!(origin.attente_globale(), (0, 0));
        // Puis tout en règle : accepté (dernière partie portée par l'envoi de la tête).
        r.envoyer(&origin, 0, None, &entrees[..5], tard).unwrap();
        let s = r.envoyer(&origin, 5, Some(&tete), &entrees[5..], tard).unwrap();
        assert_eq!((s.taille, s.ajoutees, s.en_attente), (10, 10, 0));
    }

    #[test]
    fn attente_bornee_par_cog() {
        let r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        // Octets : 256 Mio au plus en attente. Pages nulles non touchées : rien n'est
        // réellement occupé, le refus a lieu avant toute copie.
        r.envoyer(&origin, 0, None, &[vec![1u8]], T0).unwrap();
        let gros: Vec<Vec<u8>> = (0..MAX_ENTREES_SYNCHRO).map(|_| vec![0u8; 64 * 1024]).collect();
        let envoi = r.envoi_vec(&origin, 1, gros, T0).unwrap();
        assert_eq!(origin.synchroniser(&envoi, T0), Err(RefusSynchro::Bornes));
        drop(envoi);
        // Purgée : on repart de 0.
        assert_eq!(r.envoyer(&origin, 1, None, &[], T0), Err(RefusSynchro::Position { taille: 0, attendu: 0 }));
        assert_eq!(origin.attente_globale(), (0, 0));
        // Entrées : miroir + attente ≤ 1 048 576.
        let petit = vec![vec![1u8]; MAX_ENTREES_SYNCHRO];
        let mut position = 0;
        while position + (MAX_ENTREES_SYNCHRO as u64) <= MAX_ENTREES_SUIVI {
            r.envoyer(&origin, position, None, &petit, T0).unwrap();
            position += MAX_ENTREES_SYNCHRO as u64;
        }
        assert_eq!(r.envoyer(&origin, position, None, &petit[..1], T0), Err(RefusSynchro::Bornes));
        assert_eq!(origin.attente_globale(), (0, 0));
    }

    #[test]
    fn attente_bornee_globalement() {
        let (a, b, c) = (RegistreCog::new(cog('a')), RegistreCog::new(cog('b')), RegistreCog::new(cog('c')));
        // Budget global : 10 entrées, 1 000 octets.
        let origin = RevocationsReconnues::avec_budget(&[a.declaration(), b.declaration(), c.declaration()], 10, 1000).unwrap();
        let e = |n: usize, l: usize| vec![vec![7u8; l]; n];
        a.envoyer(&origin, 0, None, &e(6, 10), T0).unwrap();
        assert_eq!(origin.attente_globale(), (6, 60));
        // B dépasserait le budget en entrées : 503 sans purge ni effet sur A.
        assert_eq!(b.envoyer(&origin, 0, None, &e(5, 10), T0), Err(RefusSynchro::BudgetGlobal));
        b.envoyer(&origin, 0, None, &e(4, 10), T0).unwrap();
        assert_eq!(origin.attente_globale(), (10, 100));
        assert_eq!((origin.en_attente(&a.cog), origin.en_attente(&b.cog)), (Some(6), Some(4)));
        // Octets : C seul, mais au-delà de 1 000 octets cumulés.
        assert_eq!(c.envoyer(&origin, 0, None, &e(1, 1), T0), Err(RefusSynchro::BudgetGlobal));
        // Les attentes de A et B périment : le budget est récupéré au besoin (balayage).
        let tard = T0 + DUREE_ATTENTE_SECS + 1;
        c.envoyer(&origin, 0, None, &e(1, 901), tard).unwrap();
        assert_eq!(origin.attente_globale(), (1, 901));
        assert_eq!(c.envoyer(&origin, 1, None, &e(1, 100), tard), Err(RefusSynchro::BudgetGlobal));
        assert_eq!(origin.en_attente(&c.cog), Some(1), "attente conservée");
    }

    #[test]
    fn refus_journalises_par_agregat() {
        let j = JournalRefus::default();
        assert_eq!(j.noter("position", T0), None);
        for _ in 0..100 {
            assert_eq!(j.noter("rejected", T0 + 10), None);
        }
        let resume = j.noter("position", T0 + FENETRE_JOURNAL_REFUS_SECS).unwrap();
        assert!(resume.starts_with("102 refus"), "{resume}");
        assert!(resume.contains("position=2") && resume.contains("rejected=100"), "{resume}");
        assert_eq!(j.noter("position", T0 + 61), None, "nouvelle fenêtre");
    }

    #[test]
    fn route_controle_avant_decodage() {
        let mut r = RegistreCog::new(cog('h'));
        let origin = RevocationsReconnues::new(&[r.declaration()]).unwrap();
        // Trop d'entrées : refus avant de décoder (chaînes invalides en base64).
        let corps = serde_json::json!({
            "cog_id": r.cog.as_str(), "depuis": 0, "entrees": vec!["!!"; MAX_ENTREES_SYNCHRO + 1],
        })
        .to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("413"));
        // COG inconnu : 403 avant décodage.
        let corps = serde_json::json!({"cog_id": cog('z').as_str(), "depuis": 0, "entrees": ["!!"]}).to_string();
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("403"));
        assert!(statut(&route_synchro(&origin, b"{}", T0)).starts_with("400"));
        // Morceau puis tête, par la route.
        r.revoquer(&CibleRevocation::Cle([3; 32]), None, T0);
        let (tete, entrees) = r.publier(0, T0);
        let corps = r.corps_json(0, None, &entrees[..1], T0);
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("200"));
        // Mauvaise position : 409 avec `taille` et `attendu`.
        let rep = route_synchro(&origin, r.corps_json(0, Some(&tete), &[], T0).as_bytes(), T0);
        assert!(statut(&rep).starts_with("409"));
        assert_eq!((json(&rep)["taille"].as_u64(), json(&rep)["attendu"].as_u64()), (Some(0), Some(1)));
        let corps = r.corps_json(1, Some(&tete), &[], T0);
        assert!(statut(&route_synchro(&origin, corps.as_bytes(), T0)).starts_with("200"));
        assert_eq!(origin.est_revoque(&r.cog, &CibleRevocation::Cle([3; 32]), T0), Ok(true));
        assert_ne!(cle_debit(&r.cog), cle_debit(&cog('z')));
    }
}
