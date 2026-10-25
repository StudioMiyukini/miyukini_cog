//! @id     miyucanal_lib
//! @do     declarer_modules_canal
//! @role   securite
//! @layer  toolkit
//! @human  miyucanal — canal officiel : relevé du dépôt officiel via le Portail, annonces 2 sur 2, quarantaine, anti-retour.

//! # MiyuCanal — canal officiel (lot 4)
//!
//! Spécification : `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §7
//! (canal officiel), §17 (temps de confiance) ; `docs/securite/MODELE-MENACES-COG.md`
//! §3 ; `docs/securite/ARCHITECTURE-lot2-identite-signee.md` §8, §8.3, §9 ;
//! amendement A-006 (`docs/architecture/GEL-DOC-CONCEPTUELLE-v1.md`) ;
//! exigences EX-01 (2 sur 2), EX-02 (quarantaine), EX-20 (relevé via le
//! Portail, aucune route), EX-26 (OS et modèles IA par le canal officiel).
//!
//! - [`transport`] : demandes génériques de relevé, trait [`TransportCanal`],
//!   dépôt en mémoire ;
//! - [`annonce`] : annonces 2 sur 2 {Origin, éditeur}, révocations ;
//! - [`releveur`] : miroir vérifié du registre officiel, quarantaine de 48 h,
//!   anti-retour, remise à l'[`Installateur`] ;
//! - [`quarantaine`] : liste en attente persistée, scellée HMAC-SHA256 (A-006) ;
//! - `banc` (fonctionnalité `banc-essai`, tests seulement) : Origin, éditeur et
//!   dépôt simulés pour éprouver un [`Installateur`].
//!
//! Le canal n'installe rien : l'installation réelle reste le chemin gouverné
//! existant (Central / StrongFather), branché via [`Installateur`].

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

pub mod annonce;
#[cfg(feature = "banc-essai")]
pub mod banc;
pub mod erreurs;
pub mod quarantaine;
pub mod releveur;
pub mod transport;

pub use annonce::{sceller_annonce, sceller_revocation, verifier_annonce, AncresCanal, AnnonceVerifiee};
pub use erreurs::{ErreurCanal, ErreurTransport};
pub use quarantaine::{
    ouvrir_quarantaine, sceller_quarantaine, CleQuarantaine, EtatQuarantaine, VuePersistee,
    MAX_ENREGISTREMENTS_QUARANTAINE,
};
pub use releveur::{
    ConfigCanal, Evenement, Installateur, MiseAJourVerifiee, Motif, Releveur, MAX_ENTREES_PAR_RELEVE, QUARANTAINE_SECS,
};
pub use transport::{decoder_entrees, encoder_entrees, Demande, DepotMemoire, TransportCanal, MAX_ENTREES_PAR_DEMANDE};
