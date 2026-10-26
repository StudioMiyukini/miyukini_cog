//! @id     miyuportail_lib
//! @do     declarer_modules_portail
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — Portail interne : voies à sens unique par service, rupture protocolaire MSCE, CDR, sas de décodage Landlock + seccomp.

//! # MiyuPortail — Portail interne (lot 3 « Frontières »)
//!
//! Spécification : `docs/architecture/SERVICES-HYBRIDES-ET-PORTAIL.md` §3,
//! `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §5–§6, §10, §13.
//!
//! - **EX-44** deux voies à sens unique, une voie par service ([`portail`]) ;
//! - **EX-45** CDR obligatoire sur tout document ([`cdr::Desarmement`],
//!   refus par défaut) ;
//! - **EX-46** sas de décodage en processus séparé, confiné par Landlock
//!   (aucun accès au système de fichiers ; noyau sans Landlock = refus),
//!   rlimits et seccomp ([`sas`], [`bac_a_sable`]) + deuxième validation
//!   indépendante ([`validation2`]) ;
//! - **EX-49** coupure d'une voie sans impact sur les autres
//!   ([`portail::CoupureVoie`], [`portail::PoigneeCoupure`]) ;
//! - **A-022 §3** Portail scindé : moitié **entrée** ([`entree`], relève,
//!   aucune capacité de publier) et moitié **sortie** ([`sortie`], réponses
//!   à jeton à usage unique ([`jeton`]) et publications H1, aucune capacité
//!   de relever), sans état commun ; en production, deux processus
//!   (`portail-entree`, `portail-sortie`, [`demon`]) reliés par des files à
//!   sens unique ([`files`]) ;
//! - **rupture protocolaire** : seule la trame MSCE « déclaration » est admise
//!   ([`msce`]) ; elle est décodée en structure typée puis réencodée, aucun
//!   octet reçu ne traverse.
//!
//! Fail-closed partout (C-12) : non déclaré, non activé, coupé, sans session,
//! hors traité, signature absente, sas défaillant, validations divergentes,
//! CDR absent → refus.

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

pub mod bac_a_sable;
pub mod cdr;
#[cfg(target_os = "linux")]
pub mod demon;
pub mod entree;
pub mod erreurs;
#[cfg(target_os = "linux")]
pub mod files;
pub mod jeton;
pub mod msce;
pub mod portail;
pub mod registre;
pub mod sas;
pub mod sortie;
pub mod traite;
pub mod validation2;

pub use cdr::{Desarmement, ObjetADesarmer, ObjetDesarme, RefusDesarmement, RefusParDefaut};
pub use entree::{PortailEntree, SessionEntree};
pub use erreurs::{ErreurPortail, ErreurSas, RefusDecodage};
pub use jeton::{CleJetons, IdSession, JetonReponse};
pub use msce::{Ancres, Champ, Declaration, Valeur};
pub use portail::{
    CompteursVoie, ContratVoies, CoupureVoie, DeclarationAdmise, Declencheur, EtatVoie, PoigneeCoupure, PortailInterne,
    Publication, Sensibilite,
};
#[cfg(unix)]
pub use registre::RegistreJournal;
pub use registre::{Evenement, Inscription, MotifCoupure, RegistreMemoire};
pub use sas::{DecodeurIsole, RequeteSas, SasProcessus};
pub use sortie::{PortailSortie, SessionSortie};
pub use traite::{
    ChampTraite, ClasseExposition, IdService, NatureChamp, NatureDocument, Sens, Traite, TypeDeclaration, TypeMime,
};

/// Hexadécimal minuscule (journaux, clé publique de publication).
pub(crate) fn hex(octets: &[u8]) -> String {
    use std::fmt::Write;
    octets.iter().fold(String::with_capacity(2 * octets.len()), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}
