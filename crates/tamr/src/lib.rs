//! # TAMR
//!
//! Gestionnaire de taxonomies et métadonnées du Miyukini Core System.
//!
//! TAMR gère les taxonomies, les métadonnées, et la classification des entités.

pub mod taxonomy;
pub mod metadata;
pub mod classification;

pub use classification::{Classification, Classifier};
pub use metadata::{Metadata, MetadataManager};
pub use taxonomy::{Taxonomy, TaxonomyManager};
