//! Module Taxonomies — SPM CMS (Phase 0)
//!
//! Système de classification générique d'entités. Le module ne connaît pas le CMS,
//! le rendu, ni les utilisateurs. Il fournit uniquement un système de taxonomies
//! et de termes pour classifier des entités référencées par des IDs opaques.
//!
//! Contrat fonctionnel : gestion de taxonomies avec :
//! - Création de taxonomies
//! - Ajout de termes à une taxonomie
//! - Assignation/désassignation de termes à des entités
//! - Recherche d'entités par terme et de termes par entité
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_taxonomies::{TaxonomyManager, EntityId};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente TaxonomyManager
//! // struct MyAdapter { ... }
//! // impl TaxonomyManager for MyAdapter { ... }
//!
//! // Créer une taxonomie
//! // let taxonomy_id = adapter.create_taxonomy("Tags".to_string());
//! // let term_id = adapter.add_term(taxonomy_id, "News".to_string())?;
//! // adapter.assign_term(term_id, entity_id)?;
//! ```

pub mod taxonomy;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use taxonomy::{EntityId, TaxonomyError, TaxonomyId, TaxonomyManager, TermId};

#[cfg(feature = "memory")]
pub use memory::MemoryTaxonomyManager;
