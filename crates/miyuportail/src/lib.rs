//! @id     miyuportail_lib
//! @do     declarer_modules_portail
//! @role   securite
//! @layer  toolkit
//! @human  miyuportail — Portail interne : voies à sens unique par service, rupture protocolaire MSCE, CDR, sas de décodage seccomp.

//! # MiyuPortail — Portail interne (lot 3 « Frontières »)
//!
//! Spécification : `docs/architecture/SERVICES-HYBRIDES-ET-PORTAIL.md` §3,
//! `docs/architecture/MODELE-ZONES-ET-COUCHES-COG.md` §5–§6, §10, §13.
//!
//! - **EX-44** deux voies à sens unique, une voie par service ([`portail`]) ;
//! - **EX-45** CDR obligatoire sur tout document ([`cdr::Desarmement`],
//!   refus par défaut) ;
//! - **EX-46** sas de décodage seccomp en processus séparé ([`sas`],
//!   [`bac_a_sable`]) + deuxième validation indépendante ([`validation2`]) ;
//! - **EX-49** coupure d'une voie sans impact sur les autres
//!   ([`portail::CoupureVoie`], [`portail::PoigneeCoupure`]) ;
//! - **rupture protocolaire** : seule la trame MSCE « déclaration » est admise
//!   ([`msce`]) ; elle est décodée en structure typée puis réencodée, aucun
//!   octet reçu ne traverse.
//!
//! Fail-closed partout (C-12) : non déclaré, non activé, coupé, sans session,
//! hors traité, signature absente, sas défaillant, validations divergentes,
//! CDR absent → refus.

pub mod bac_a_sable;
pub mod cdr;
pub mod erreurs;
pub mod msce;
pub mod portail;
pub mod registre;
pub mod sas;
pub mod traite;
pub mod validation2;

pub use cdr::{Desarmement, ObjetADesarmer, ObjetDesarme, RefusDesarmement, RefusParDefaut};
pub use erreurs::{ErreurPortail, ErreurSas, RefusDecodage};
pub use msce::{Ancres, Champ, Declaration, Valeur};
pub use portail::{
    CompteursVoie, ContratVoies, CoupureVoie, DeclarationAdmise, Declencheur, EtatVoie, PoigneeCoupure, PortailInterne,
    Publication, Sensibilite,
};
pub use registre::{Evenement, Inscription, MotifCoupure, RegistreMemoire};
pub use sas::{DecodeurIsole, RequeteSas, SasProcessus};
pub use traite::{ChampTraite, ClasseExposition, IdService, NatureChamp, NatureDocument, Sens, Traite, TypeDeclaration};
