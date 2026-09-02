//! @id     miyukini_central_db_mod
//! @do     declarer_module_kindmother_adapter
//! @role   donnee
//! @layer  core
//! @human  miyukini-central — module « db mod ».

//! Module de base de données central - Adaptateurs KindMother.
//!
//! Fournit les abstractions et helpers communs pour les bases
//! de données KindMother Daughter (jaykoa, jaykonta).

pub mod kindmother_adapter;

pub use kindmother_adapter::*;
