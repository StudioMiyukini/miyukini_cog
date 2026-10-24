//! @id     miyauth_revocation
//! @do     verifier_revocation_appareil
//! @role   securite
//! @layer  toolkit
//! @human  miyauth — révocation d'appareil, de passeport et de clé d'émission (EX-14), fail-closed.

//! Révocations (EX-14).
//!
//! La doc gelée (§8) place les révocations dans le **registre Merkle** du COG
//! (`Revocation(serial | package_id | kid)`, tête signée par la clé de journal).
//! Le registre est en cours dans `miyusceau` : miyauth ne dépend donc que d'une
//! interface indépendante du stockage, [`SourceRevocation`], qui **échoue fermé**
//! quand la liste est absente, périmée ou datée dans le futur.
//!
//! Point d'intégration : une implémentation adossée au registre construira une
//! [`ListeRevocation`] (ou implémentera le trait) à partir des entrées
//! `Revocation` vérifiées, avec `a_jour_au` = horodatage de la **tête de registre
//! vérifiée** (fraîcheur EX-03 / EX-35, horloge EX-22).

use std::collections::{BTreeMap, HashSet};
use std::fmt;

use miyusceau::{empreinte_cle, Kid, TOLERANCE_HORLOGE_SECS};

use crate::titres::{CogId, Serie};

/// Fraîcheur maximale par défaut d'une liste de révocation (choix fail-closed :
/// 15 minutes, durée d'un visa — un appareil révoqué ne peut pas obtenir de
/// nouveau visa au-delà de ce délai).
pub const FRAICHEUR_MAX_DEFAUT_SECS: u64 = 15 * 60;

/// Ce qui peut être révoqué.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CibleRevocation {
    /// Un titre précis (passeport ou visa), par son numéro de série.
    Serie(Serie),
    /// Une clé : clé d'appareil (révoque tous ses passeports) ou clé d'émission.
    Cle(Kid),
}

impl CibleRevocation {
    /// Cible « appareil » à partir de sa clé publique (révocation en un geste).
    pub fn appareil(cle_appareil: &[u8; 32]) -> Self {
        Self::Cle(empreinte_cle(cle_appareil))
    }
}

/// Motif d'indisponibilité du statut de révocation. Toujours traité comme un refus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErreurRevocation {
    /// Aucune liste pour ce COG émetteur.
    ListeAbsente,
    /// Liste trop ancienne : `a_jour_au + fraicheur_max < maintenant`.
    Perimee {
        /// Horodatage de la liste.
        a_jour_au: u64,
        /// Heure de la vérification.
        maintenant: u64,
    },
    /// Liste datée dans le futur au-delà de la tolérance d'horloge.
    HorlogeIncoherente,
    /// Source indisponible (stockage, réseau…).
    Indisponible(String),
}

impl fmt::Display for ErreurRevocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ListeAbsente => write!(f, "aucune liste de révocation pour ce COG"),
            Self::Perimee { a_jour_au, maintenant } => {
                write!(f, "liste de révocation périmée (à jour au {a_jour_au}, maintenant {maintenant})")
            }
            Self::HorlogeIncoherente => write!(f, "liste de révocation datée dans le futur"),
            Self::Indisponible(m) => write!(f, "source de révocation indisponible : {m}"),
        }
    }
}

impl std::error::Error for ErreurRevocation {}

/// Source du statut de révocation, indépendante du stockage.
///
/// Contrat : `Ok(false)` seulement si la source **sait** que la cible n'est pas
/// révoquée avec une information fraîche ; tout doute est une erreur (refus).
pub trait SourceRevocation {
    /// Statut de `cible` émise sous l'autorité de `emetteur`, à l'instant `maintenant_secs`.
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation>;
}

/// Liste de révocation d'un COG à un instant donné (instantané du registre).
#[derive(Debug, Clone)]
pub struct ListeRevocation {
    cog: CogId,
    a_jour_au: u64,
    fraicheur_max: u64,
    cibles: HashSet<CibleRevocation>,
}

impl ListeRevocation {
    /// Liste vide pour `cog`, à jour au `a_jour_au` (secondes Unix).
    pub fn new(cog: CogId, a_jour_au: u64, fraicheur_max: u64) -> Self {
        Self { cog, a_jour_au, fraicheur_max, cibles: HashSet::new() }
    }

    /// Révoque une cible (geste unique côté Central : [`CibleRevocation::appareil`]).
    pub fn revoquer(&mut self, cible: CibleRevocation) -> bool {
        self.cibles.insert(cible)
    }

    /// Met à jour l'horodatage (nouvelle tête de registre vérifiée). Refuse un recul.
    pub fn rafraichir(&mut self, a_jour_au: u64) -> bool {
        if a_jour_au < self.a_jour_au {
            return false;
        }
        self.a_jour_au = a_jour_au;
        true
    }

    /// COG concerné.
    pub fn cog(&self) -> &CogId {
        &self.cog
    }

    /// Nombre de cibles révoquées.
    pub fn len(&self) -> usize {
        self.cibles.len()
    }

    /// Vrai si aucune révocation.
    pub fn is_empty(&self) -> bool {
        self.cibles.is_empty()
    }

    fn controler_fraicheur(&self, maintenant: u64) -> Result<(), ErreurRevocation> {
        if self.a_jour_au > maintenant.saturating_add(TOLERANCE_HORLOGE_SECS) {
            return Err(ErreurRevocation::HorlogeIncoherente);
        }
        if self.a_jour_au.saturating_add(self.fraicheur_max) < maintenant {
            return Err(ErreurRevocation::Perimee { a_jour_au: self.a_jour_au, maintenant });
        }
        Ok(())
    }
}

impl SourceRevocation for ListeRevocation {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        if emetteur != &self.cog {
            return Err(ErreurRevocation::ListeAbsente);
        }
        self.controler_fraicheur(maintenant_secs)?;
        Ok(self.cibles.contains(cible))
    }
}

/// Ensemble de listes, une par COG (son propre COG + COG reconnus, EX-35).
#[derive(Debug, Clone, Default)]
pub struct RevocationsFederees {
    listes: BTreeMap<CogId, ListeRevocation>,
}

impl RevocationsFederees {
    /// Ensemble vide (refuse tout).
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute ou remplace la liste d'un COG.
    pub fn inserer(&mut self, liste: ListeRevocation) {
        self.listes.insert(liste.cog.clone(), liste);
    }

    /// Accès mutable à la liste d'un COG (révocation locale).
    pub fn liste_mut(&mut self, cog: &CogId) -> Option<&mut ListeRevocation> {
        self.listes.get_mut(cog)
    }
}

impl SourceRevocation for RevocationsFederees {
    fn est_revoque(&self, emetteur: &CogId, cible: &CibleRevocation, maintenant_secs: u64)
        -> Result<bool, ErreurRevocation> {
        self.listes.get(emetteur).ok_or(ErreurRevocation::ListeAbsente)?.est_revoque(emetteur, cible, maintenant_secs)
    }
}
