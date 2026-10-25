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
//! - [`federal`] : têtes des COG fédérés contresignées par Origin ;
//! - `revocation` (fonctionnalité `miyauth`) : `SourceRevocation` de miyauth
//!   adossée à un miroir vérifié (lot 2 §8.2).
//!
//! Fail-closed : toute donnée non vérifiée, incohérente ou hors politique est
//! un refus ; aucun état n'est modifié par une opération refusée.

pub mod entree;
pub mod erreurs;
pub mod federal;
pub mod journal;
#[cfg(feature = "miyauth")]
pub mod revocation;
pub mod stockage;
pub mod suivi;
pub mod temps;

pub use entree::{identite_sceau, Categorie, Cible, Corps, EntreeRegistre, Publication};
pub use erreurs::{ErreurJournal, ErreurStockage, ErreurTemps};
pub use federal::RegistreFederal;
#[cfg(feature = "miyauth")]
pub use revocation::{cible_serie, RevocationsRegistre};
pub use journal::{controler_structure_entree, Journal};
pub use stockage::{Stockage, StockageFichier, StockageMemoire, TAILLE_MAX_ENREGISTREMENT};
pub use suivi::{MiroirRegistre, SuiviTetes, TetePreparee};
pub use temps::{HorlogeConfiance, TempsMedian, FENETRE_MTP};
