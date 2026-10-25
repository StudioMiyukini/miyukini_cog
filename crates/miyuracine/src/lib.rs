//! @id     miyuracine_lib
//! @do     declarer_modules_racine
//! @role   securite
//! @layer  toolkit
//! @human  miyuracine — racine COG hors ligne (SLIP-39 2 sur 3, SLIP-10 par usage, passation).

//! # MiyuRacine — racine COG hors ligne
//!
//! Lot 2, incrément 3 (`docs/implementation/PLAN-IMPLEMENTATION.md` §4) :
//! racine SLIP-39 2 sur 3 sur USB, clés SLIP-10 par usage, cérémonie de
//! rotation (EX-24, EX-25). Spécification : `docs/securite/ARCHITECTURE-lot2-identite-signee.md`
//! §3 et décision 2 (§12).
//!
//! - [`Racine`] : secret maître de 256 bits (`OsRng`), effacé à la destruction.
//! - [`Racine::partager`] / [`Racine::recuperer`] : parts SLIP-39, un groupe,
//!   2 sur 3 ; une part seule, une part corrompue ou des parts mélangées sont refusées.
//! - [`Racine::cle`] : clé Ed25519 par [`Usage`], dérivée en SLIP-10 durcie,
//!   rendue comme [`miyusceau::CleSignature`].
//! - [`fichier_cle`] : fichier de clé racine chiffré (Argon2id +
//!   XChaCha20-Poly1305, format `MYRC` v1) pour le support amovible.
//! - [`durcir_processus`] : core dumps désactivés, mémoire verrouillée
//!   (`mlockall`), NC-ANSSI-25.
//! - [`Passation`] : « l'ancienne racine se porte garante de la nouvelle »,
//!   scellée par les deux clés d'identité ([`verifier_passation`]).
//!
//! SLIP-39 et SLIP-10 sont implémentés ici d'après les spécifications
//! SatoshiLabs et validés par **tous** les vecteurs officiels (45 SLIP-39,
//! 12 nœuds Ed25519 SLIP-10). Primitives : `hmac`, `pbkdf2`, `sha2`
//! (RustCrypto), `rand_core::OsRng`, `miyusceau`/`ed25519-dalek`.
//!
//! **Hors ligne** : aucune dépendance réseau, aucun usage des sockets de `std`
//! (contrôlé par `tests/hors_reseau.rs`). Le binaire `miyuracine` est l'outil
//! de cérémonie, à exécuter sur un poste isolé.

#![forbid(unsafe_code)]

mod chiffrement;
pub mod durcissement;
pub mod erreurs;
pub mod fichier_cle;
mod gf256;
mod mots;
pub mod passation;
pub mod racine;
mod rs1024;
mod shamir;
pub mod slip10;
pub mod slip39;
pub mod usages;

pub use durcissement::{durcir_processus, EtatDurcissement, PolitiqueVerrouillage};
pub use erreurs::ErreurRacine;
pub use fichier_cle::{
    chiffrer_racine, dechiffrer_racine, ecrire_fichier_cle, lire_fichier_cle, ParametresArgon2,
};
pub use passation::{verifier_passation, Passation, NATURE_PASSATION};
pub use racine::{
    cog_id_depuis_cle, PartsRacine, Racine, EXPOSANT_DEFAUT, NOMBRE_PARTS, SEUIL, TAILLE_SECRET,
};
pub use slip10::NoeudSlip10;
pub use usages::{Usage, BUT_MIYUKINI_COG};
