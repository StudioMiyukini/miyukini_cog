//! @id     miyuregistre_lib
//! @do     declarer_modules_registre
//! @role   securite
//! @layer  toolkit
//! @human  miyuregistre — registre Merkle local et fédéral : entrées typées, journal append-only, têtes signées, Median-Time-Past.

//! # MiyuRegistre — registre Merkle local et fédéral (lot 4)
//!
//! Spécification : `docs/securite/ARCHITECTURE-lot2-identite-signee.md` §8,
//! §8.1, §8.2, §8.3 (amendement A-006) ; `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §7, §17 ;
//! exigences EX-22 (Median-Time-Past) et EX-34 (registre fédéral contresigné).
//!
//! Briques réutilisées de `miyusceau` : format Sceau v1.1, `TeteRegistre`,
//! `EtatRegistre` (fraîcheur, monotonie, équivoque), arbre de Merkle RFC 9162.
//!
//! - [`entree`] : entrées typées (publication, révocation, émission, décret,
//!   événement), Sceau de type `0x04` ;
//! - [`stockage`] : journal append-only indépendant du support, fichier
//!   crash-safe (`fsync`, récupération de fin tronquée) ;
//! - [`journal`] : côté émetteur (ajout, têtes signées, preuves) ;
//! - [`suivi`] : côté vérificateur (têtes + Median-Time-Past, miroir intégral vérifié) ;
//! - [`temps`] : Median-Time-Past (11 têtes) et horloge de confiance monotone ;
//! - [`federal`] : têtes des COG fédérés contresignées par ≥ 3 membres
//!   témoins, Origin = un témoin au plus (A-007) ;
//! - `revocation` (fonctionnalité `miyauth`) : `SourceRevocation` de miyauth
//!   adossée à un miroir vérifié (lot 2 §8.2) ;
//! - `suivi_revocations` (fonctionnalité `miyauth`) : suivi **unique** du
//!   registre de révocation d'un COG (miroir + liste, fraîcheur ≤ 15 min
//!   imposée), utilisé par Origin, MiyuCloud et les façades ;
//! - `enveloppe` (fonctionnalité `miyauth`) : format partagé de l'enveloppe
//!   signée des envois de révocation vers Origin (A-014) ;
//! - `editeur` (fonctionnalité `editeur`) : côté COG, publication du registre
//!   de révocation (dépôt MiyuCloud, envois signés vers Origin) ; binaire
//!   `miyuregistre-publier` (fonctionnalité `publier`).
//!
//! Fail-closed : toute donnée non vérifiée, incohérente ou hors politique est
//! un refus ; aucun état n'est modifié par une opération refusée.

// C-10 : aucun unwrap / expect / panic! / todo! / unimplemented! / unreachable! hors tests.
#![cfg_attr(
    not(test),
    deny(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unreachable
    )
)]

#[cfg(feature = "editeur")]
pub mod editeur;
pub mod entree;
#[cfg(feature = "miyauth")]
pub mod enveloppe;
pub mod erreurs;
pub mod federal;
pub mod journal;
#[cfg(feature = "miyauth")]
pub mod revocation;
pub mod stockage;
pub mod suivi;
#[cfg(feature = "miyauth")]
pub mod suivi_revocations;
pub mod temps;

pub use entree::{identite_sceau, Categorie, Cible, Corps, EntreeRegistre, Publication};
pub use erreurs::{ErreurJournal, ErreurStockage, ErreurTemps};
pub use federal::{verifier_temoins, RegistreFederal, TEMOINS_MIN};
#[cfg(feature = "miyauth")]
pub use revocation::{cible_serie, controler_fraicheur, RevocationsRegistre, FRAICHEUR_REVOCATIONS_SECS};
#[cfg(feature = "miyauth")]
pub use suivi_revocations::{
    Application, RefusSuivi, SuiviRevocationsRegistre, MAX_ENTREES_SUIVI, MAX_OCTETS_SUIVI,
};
pub use journal::{controler_structure_entree, Journal};
pub use stockage::{Stockage, StockageFichier, StockageMemoire, TAILLE_MAX_ENREGISTREMENT};
pub use suivi::{MiroirRegistre, SuiviTetes, SynchroPreparee, TetePreparee};
pub use temps::{HorlogeConfiance, TempsMedian, FENETRE_MTP};
