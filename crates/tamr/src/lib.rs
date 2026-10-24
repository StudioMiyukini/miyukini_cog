//! @id     tamr_lib
//! @do     declarer_module_classification
//! @role   orchestration
//! @layer  core
//! @human  tamr — module « lib ».

//! # TAMR
//!
//! Gestionnaire de taxonomies et métadonnées du Miyukini Core System.
//!
//! TAMR gère les taxonomies, les métadonnées, et la classification des entités.
//!
//! ## État réel (K-17)
//!
//! Ce crate est un **modèle** : types et registres en mémoire (taxonomies,
//! métadonnées, contrat de classification). Ce n'est **pas** une gate active :
//! rien ici n'autorise, ne bloque ni ne supervise une action.
//!
//! Les mécanismes APPROVAL, OVERRIDE, ESCALATION et SUPERVISION (dont
//! l'APPROVAL tranchée par StrongFather, D2) sont **prévus mais non implémentés** ;
//! leur API reste à mettre au plan (constat K-17, suite du lot C « Cores réels »).
//! Aucun appelant ne doit supposer qu'une approbation TAMR a eu lieu.

pub mod classification;
pub mod metadata;
pub mod taxonomy;

pub use classification::{Classification, Classifier};
pub use metadata::{DefaultMetadataManager, Metadata, MetadataManager};
pub use taxonomy::{DefaultTaxonomyManager, Taxonomy, TaxonomyManager};
