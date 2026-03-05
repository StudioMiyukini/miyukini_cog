//! # TAMR
//!
//! Gestionnaire de taxonomies et métadonnées du Miyukini Core System.
//!
//! TAMR gère les taxonomies, les métadonnées, et la classification des entités.

pub mod classification;
pub mod metadata;
pub mod taxonomy;

pub use classification::{Classification, Classifier};
pub use metadata::{DefaultMetadataManager, Metadata, MetadataManager};
pub use taxonomy::{DefaultTaxonomyManager, Taxonomy, TaxonomyManager};
