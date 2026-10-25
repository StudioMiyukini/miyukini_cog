//! @id     miyuregistre_editeur
//! @do     editer_registre_revocations_cog
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — éditeur côté COG du registre de révocation : tête fraîche signée par la clé de journal, dépôt MiyuCloud atomique, envois signés par morceaux vers Origin (A-012, A-013, A-014).

//! Éditeur du registre de révocation d'un COG (fonctionnalité `editeur`).
//!
//! Les vérificateurs (Origin, MiyuCloud, façades) suivent le registre d'un COG
//! par [`crate::SuiviRevocationsRegistre`] ; ce module **produit** ce flux côté
//! COG à partir du journal local ([`crate::Journal`]) et de la clé de journal :
//!
//! 1. [`Instantane::emettre`] : émet une tête **fraîche** (horodatage = maintenant,
//!    jamais en recul, validité [`VALIDITE_TETE_SECS`]) couvrant toutes les
//!    entrées du journal, et en prend un instantané (tête + entrées).
//! 2. [`depot::DepotRevocations`] : dépôt lu par MiyuCloud (`tete.sceau` +
//!    `entrees.mrgj`), écriture atomique, protocole d'ordre documenté.
//! 3. [`origin::PublicationOrigin`] : envois signés (enveloppe A-014,
//!    [`crate::enveloppe`]) par morceaux bornés vers Origin, reprise sur `409`,
//!    arrêt sur `401` / `403`.
//!
//! # Cadence
//!
//! La fraîcheur des révocations est de 15 min (A-007), mesurée sur la tête
//! vérifiée : l'éditeur doit émettre et publier une tête **au moins toutes les
//! [`INTERVALLE_MAX_SECS`]** (5 min), ce qui laisse deux publications manquées
//! avant péremption chez les vérificateurs. Chaque cycle émet une tête, même
//! sans entrée nouvelle (≈ 200 o ajoutés au stockage des têtes par cycle).
//!
//! # Écrivain unique
//!
//! Le journal n'a qu'un écrivain à la fois ([`crate::StockageFichier`]) :
//! l'appelant le garantit (le binaire `miyuregistre-publier` prend un verrou
//! `flock` exclusif sur `<journal>/verrou`, que tout autre écrivain du journal
//! doit prendre aussi ; pris avec un délai borné, [`verrou::DELAI_VERROU`],
//! jamais d'attente infinie).

use miyusceau::{CleSignature, Empreinte, TOLERANCE_HORLOGE_SECS};

use crate::erreurs::ErreurJournal;
use crate::journal::Journal;
use crate::revocation::FRAICHEUR_REVOCATIONS_SECS;
use crate::stockage::Stockage;

#[cfg(unix)]
pub mod cle;
#[cfg(unix)]
pub mod depot;
#[cfg(feature = "publier")]
pub mod http;
pub mod origin;
#[cfg(unix)]
pub mod verrou;

/// Validité annoncée des têtes émises : la fraîcheur des révocations (15 min).
pub const VALIDITE_TETE_SECS: u64 = FRAICHEUR_REVOCATIONS_SECS;
/// Intervalle maximal entre deux publications (5 min, fraîcheur 15 min).
pub const INTERVALLE_MAX_SECS: u64 = 5 * 60;

/// Tête fraîche et entrées qu'elle couvre (état publié par un cycle).
#[derive(Debug, Clone)]
pub struct Instantane {
    /// Sceau de la tête (signée par la clé de journal).
    pub tete: Vec<u8>,
    /// Nombre d'entrées couvertes par la tête.
    pub taille: u64,
    /// Racine Merkle signée.
    pub racine: Empreinte,
    /// Horodatage d'émission de la tête.
    pub emise_a: u64,
    /// Entrées `0..taille` (sceaux de type `0x04`).
    pub entrees: Vec<Vec<u8>>,
}

/// Émission d'une tête refusée.
#[derive(Debug, thiserror::Error)]
pub enum ErreurEmission {
    /// Refus du journal.
    #[error(transparent)]
    Journal(#[from] ErreurJournal),
    /// L'horloge locale a reculé au-delà de [`TOLERANCE_HORLOGE_SECS`] : la
    /// dernière tête est datée après `maintenant + tolérance`. Émettre exigerait
    /// une tête datée du futur (refusée par les vérificateurs) ; l'opérateur
    /// doit corriger l'horloge.
    #[error(
        "horloge en recul : dernière tête émise à {derniere}, maintenant {maintenant} \
         (tolérance {TOLERANCE_HORLOGE_SECS} s) — corriger l'horloge"
    )]
    HorlogeEnRecul {
        /// Horodatage de la dernière tête émise.
        derniere: u64,
        /// Horloge locale.
        maintenant: u64,
    },
}

impl Instantane {
    /// Émet une tête couvrant tout `journal`, signée par `cle` (clé de
    /// journal), puis lit les entrées.
    ///
    /// Datation : `emise_a = max(maintenant, dernière tête)` — le journal
    /// refuse un horodatage en recul — mais **jamais** au-delà de
    /// `maintenant + TOLERANCE_HORLOGE_SECS` (la tolérance d'horloge des
    /// vérificateurs, `miyusceau`) : une tête précédente plus récente que cela
    /// (saut d'horloge en arrière) ⇒ [`ErreurEmission::HorlogeEnRecul`], aucune
    /// tête émise. Un léger recul (dans la tolérance) reste toléré.
    pub fn emettre<S: Stockage>(journal: &mut Journal<S>, cle: &CleSignature, maintenant: u64) -> Result<Self, ErreurEmission> {
        let emise_a = match journal.derniere_tete() {
            Some(d) if d.emise_a > maintenant.saturating_add(TOLERANCE_HORLOGE_SECS) => {
                return Err(ErreurEmission::HorlogeEnRecul { derniere: d.emise_a, maintenant });
            }
            Some(d) => d.emise_a.max(maintenant),
            None => maintenant,
        };
        let tete = journal.emettre_tete(&[cle], emise_a, VALIDITE_TETE_SECS)?;
        let taille = journal.taille();
        let entrees = journal.entrees(0, taille).map_err(ErreurJournal::from)?;
        if entrees.len() as u64 != taille {
            return Err(ErreurJournal::StockageIncoherent.into());
        }
        Ok(Self { tete, taille, racine: journal.racine(), emise_a, entrees })
    }
}

#[cfg(all(test, feature = "miyauth"))]
mod tests {
    use super::*;
    use crate::entree::{Cible, Corps, EntreeRegistre};
    use crate::stockage::StockageMemoire;
    use crate::suivi_revocations::SuiviRevocationsRegistre;
    use miyauth::{CibleRevocation, CogId, SourceRevocation};
    use miyusceau::sceller;

    #[test]
    fn instantane_applicable_et_jamais_en_recul() {
        let cle = CleSignature::generer();
        let mut journal = Journal::ouvrir([3; 32], StockageMemoire::new(), StockageMemoire::new()).unwrap();
        let e = EntreeRegistre { inscrite_a: 1, corps: Corps::Revocation { cible: Cible::Cle([5; 32]), motif: "x".into() } };
        journal.ajouter(&sceller(&e, &cle).unwrap()).unwrap();
        let t0 = 1_800_000_000;
        let premier = Instantane::emettre(&mut journal, &cle, t0).unwrap();
        assert_eq!((premier.taille, premier.emise_a, premier.entrees.len()), (1, t0, 1));
        // Léger recul (dans la tolérance) : tête datée de la précédente.
        let second = Instantane::emettre(&mut journal, &cle, t0 - 10).unwrap();
        assert_eq!(second.emise_a, t0);
        // B5 : saut d'horloge en arrière au-delà de la tolérance ⇒ refus
        // explicite, aucune tête datée au-delà de maintenant + tolérance.
        let taille_tetes = journal.derniere_tete().map(|t| t.emise_a);
        let recul = t0 - TOLERANCE_HORLOGE_SECS - 1;
        assert!(matches!(
            Instantane::emettre(&mut journal, &cle, recul),
            Err(ErreurEmission::HorlogeEnRecul { derniere, maintenant }) if derniere == t0 && maintenant == recul
        ));
        assert_eq!(journal.derniere_tete().map(|t| t.emise_a), taille_tetes, "aucune tête émise");
        // Limite exacte : dernière = maintenant + tolérance ⇒ accepté.
        let limite = Instantane::emettre(&mut journal, &cle, t0 - TOLERANCE_HORLOGE_SECS).unwrap();
        assert_eq!(limite.emise_a, t0);
        let cog = CogId::new(&"q".repeat(26)).unwrap();
        let suivi = SuiviRevocationsRegistre::new(cog.clone(), [3; 32], &cle.cle_publique()).unwrap();
        suivi.appliquer(&premier.tete, 0, &premier.entrees, t0).unwrap();
        suivi.appliquer(&second.tete, 0, &second.entrees, t0).unwrap();
        assert_eq!(suivi.est_revoque(&cog, &CibleRevocation::Cle([5; 32]), t0 + 1), Ok(true));
    }
}
