//! @id     miyufrontiere_lib
//! @do     cabler_composants_lot3
//! @role   integration
//! @layer  operateur
//! @human  miyufrontiere — câblage du lot 3 : Douane → Vigies, WorrySentinel → Portail + Douane, Portail → CDR.

//! # MiyuFrontière — câblage du lot 3 « Frontières »
//!
//! Les composants du lot 3 sont écrits indépendants les uns des autres
//! (`docs/implementation/PLAN-IMPLEMENTATION.md` §5 « Reste » : câblage) ;
//! ce crate est le **seul** à les connaître ensemble, ce qui évite tout cycle
//! et garde chaque crate dans son rôle (décision D1 : les Cores gouvernent,
//! des opérateurs exécutent) :
//!
//! - [`douane_vigies::SourceDouane`] : événements de l'IPS de la Douane
//!   (`miyudouane::EvenementDouane`) → `miyuvigie::SourceEvenements` ;
//! - [`coupure::CoupureFrontiere`] : ordres de coupure de WorrySentinel
//!   (`worrysentinel::CoupureVoie`) → Portail (`miyuportail::CoupureVoie`)
//!   **et** IPS de la Douane (`DecisionSentinelle::CouperVoie`), table voie ↔
//!   service explicite, voie inconnue = erreur (fail-closed) ;
//! - [`cdr::CdrPortail`] : désarmement du Portail (`miyuportail::Desarmement`)
//!   par `miyucdr`.
//!
//! Chaîne complète (critère de sortie du lot 3, niveau bibliothèque) :
//! `tests/bout_en_bout.rs`.

#![forbid(unsafe_code)]

pub mod cdr;
pub mod coupure;
pub mod douane_vigies;

pub use cdr::{CdrPortail, MotifRefusCdr};
pub use coupure::{CoupureFrontiere, Horloge, IpsPartage};
pub use douane_vigies::{zone_vigie, AttributionVoies, SourceDouane};
