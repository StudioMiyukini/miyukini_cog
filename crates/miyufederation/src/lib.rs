//! @id     miyufederation_lib
//! @do     declarer_modules_federation
//! @role   securite
//! @layer  toolkit
//! @human  miyufederation — fédération COG : candidature hashcash, statuts, blacklist sur preuves et quorum, note locale, anti-éclipse, sans Origin.

//! # MiyuFédération — admission et sanctions fédérales (lot 6)
//!
//! Spécification : `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §16 et
//! §18, `docs/securite/ARCHITECTURE-lot2-identite-signee.md` §4.1, §8.3, §10,
//! `docs/securite/MODELE-MENACES-COG.md` EX-30 à EX-38 ;
//! `docs/implementation/PLAN-IMPLEMENTATION.md` §8.
//!
//! - [`calcul`] : preuve de calcul légère (hashcash SHA-256) liée à la racine
//!   et au jour (EX-38)
//! - [`titres`] : candidature (certificat `0x06`, nature `0x43`), confirmation
//!   (reconnaissance `0x07`), accusation (certificat `0x06`, nature `0x42`)
//! - [`preuve`] : preuve de faute signée par le mis en cause (équivoque de têtes)
//! - [`federation`] : vue fédérale locale — statuts candidat → invité (≥ 2) →
//!   membre (≥ 4 pendant 7 j) → révoqué ; témoins anti-éclipse ; accusation au
//!   quorum de 6 ; révocation d'identité ; fraîcheur de la vue (EX-31..33, EX-35)
//! - [`note`] : note locale des pairs, jamais partagée (EX-37)
//! - [`eclipse`] : carnet de pairs diversifiés, anciens conservés (EX-37)
//!
//! **Sans Origin** (EX-30) : Origin n'est qu'un témoin facultatif.
//! **Aucune destruction à distance** (EX-33) : seule sanction, la révocation
//! d'identité ([`Sanction`]).
//!
//! Aucune dépendance réseau : la crate décide ; le transport (commérage
//! `inv` / `getdata`, WireGuard) reste hors de son périmètre.

pub mod calcul;
pub mod eclipse;
pub mod erreurs;
pub mod federation;
pub mod note;
pub mod preuve;
pub mod statut;
pub mod titres;

pub use eclipse::CarnetPairs;
pub use erreurs::ErreurFederation;
pub use federation::{
    Federation, Parametres, RevocationFederale, RevocationsFederation, PERIODE_QUOTA_SECS, QUOTA_CONFIRMATIONS,
    TEMOINS_MIN,
};
pub use note::{NoteLocale, Observation, SEUIL_BANNISSEMENT};
pub use preuve::Preuve;
pub use statut::{
    Sanction, Statut, CONFIRMATIONS_INVITE, CONFIRMATIONS_MEMBRE, DUREE_CONFIRMATION_MAX_SECS, DUREE_MEMBRE_SECS,
    QUORUM_BLACKLIST,
};
pub use titres::{Accusation, Candidature, Confirmation, NATURE_ACCUSATION, NATURE_CANDIDATURE};
