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
//!   (reconnaissance `0x07`), accusation (certificat `0x06`, nature `0x42`),
//!   recours accepté (certificat `0x06`, nature `0x44`, à consigner par amendement)
//! - [`preuve`] : preuve de faute signée par le mis en cause (équivoque de têtes)
//! - [`federation`] : vue fédérale locale — statuts candidat → invité (≥ 2) →
//!   membre (≥ 4 pendant 7 j) → suspendu → révoqué ; témoins anti-éclipse
//!   (≥ 3 membres, règle de `miyuregistre::federal`) ; accusation au quorum de
//!   6 ⇒ suspension immédiate, révocation d'identité après 72 h sauf recours
//!   accepté par 6 membres (A-007) ; fraîcheur de la vue (EX-31..33, EX-35)
//! - [`note`] : note locale des pairs, jamais partagée (EX-37)
//! - [`eclipse`] : carnet de pairs diversifiés, anciens conservés (EX-37)
//! - [`persistance`] : état fédéral local `MFED` (statuts, confirmations,
//!   suspensions, levées, révocations, note locale), titres signés
//!   revérifiés au chargement, reste scellé par HMAC du secret local ;
//!   reprise fail-closed (à consigner par amendement)
//! - [`support`] : trait [`SupportEtat`] (octets scellés, écriture atomique) —
//!   fichier ([`StockageFederal`]), mémoire de test ([`SupportMemoire`]),
//!   KindMother (fonctionnalité `kindmother`, base dédiée `miyufederation`)
//! - [`persistante`] : [`FederationPersistante`], chaque transition écrite
//!   atomiquement avant d'être visible, rechargement revérifié, hors service
//!   après un échec d'écriture
//!
//! **Sans Origin** (EX-30) : Origin n'est qu'un témoin facultatif.
//! **Aucune destruction à distance** (EX-33) : seule sanction, la révocation
//! d'identité ([`Sanction`]).
//!
//! Aucune dépendance réseau par défaut : la crate décide ; le transport
//! (commérage `inv` / `getdata`, WireGuard) reste hors de son périmètre. La
//! fonctionnalité `kindmother` ajoute seulement le client IPC local de
//! KindMother (seul écrivain du COG) pour la persistance.

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

pub mod calcul;
pub mod eclipse;
pub mod erreurs;
pub mod federation;
pub mod note;
pub mod persistance;
pub mod persistante;
pub mod preuve;
pub mod statut;
pub mod support;
pub mod titres;

#[cfg(feature = "kindmother")]
pub mod kindmother;

pub use eclipse::{CarnetPairs, MAX_PAIRS_CARNET};
pub use erreurs::ErreurFederation;
pub use federation::{
    Federation, Parametres, RevocationFederale, RevocationsFederation, SuspensionFederale, MOTIF_REVOCATION_FEDERALE,
    PERIODE_QUOTA_SECS, QUOTA_CONFIRMATIONS, TAILLE_MAX_ETAT, TEMOINS_MIN,
};
pub use note::{NoteLocale, Observation, SEUIL_BANNISSEMENT};
pub use persistance::{
    AncreRegistre, CleEtatFederal, ErreurEtat, RegistreLocal, Reprise, StockageFederal, FICHIER_ETAT, FICHIER_VERROU,
};
pub use persistante::{BilanReprise, ErreurTransition, FederationPersistante};
pub use preuve::Preuve;
pub use support::{SupportEtat, SupportMemoire};

#[cfg(feature = "kindmother")]
pub use kindmother::SupportKindMother;
pub use statut::{
    Sanction, Statut, CONFIRMATIONS_INVITE, CONFIRMATIONS_MEMBRE, DELAI_CONTESTATION_SECS, DUREE_CONFIRMATION_MAX_SECS,
    DUREE_MEMBRE_SECS, QUORUM_BLACKLIST, QUORUM_RECOURS,
};
pub use titres::{Accusation, Candidature, Confirmation, Recours, NATURE_ACCUSATION, NATURE_CANDIDATURE, NATURE_RECOURS};
