//! Module Recherche & Indexation — SPM CMS (Phase 2.2)
//!
//! Indexation fonctionnelle et recherche générique d'entités. Le module ne connaît pas le scoring,
//! le ranking, la pertinence, les permissions, ni la logique métier de recherche.
//!
//! Contrat fonctionnel : gestion d'un index avec :
//! - Indexation d'entités avec champs
//! - Recherche par critères (égalité, contient, comparaisons)
//! - Opérateurs logiques (ET, OU)
//! - Pagination
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_search::{SearchManager, IndexedField, SearchQuery, SearchOperator};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente SearchManager
//! // struct MyAdapter { ... }
//! // impl SearchManager for MyAdapter { ... }
//!
//! // Indexer une entité
//! // let fields = vec![IndexedField::new("title".to_string(), "Mon article".to_string(), SearchType::Contains)];
//! // adapter.index_entity(id, "content".to_string(), fields)?;
//! ```

pub mod error;
pub mod search;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use error::SearchError;
pub use search::{
    EntityId, EntityType, FieldName, FieldValue, IndexedEntity, IndexedField, IndexStats,
    LogicalOperator, SearchCriterion, SearchManager, SearchMetadata, SearchOperator, SearchQuery,
    SearchResult, SearchType,
};

#[cfg(feature = "memory")]
pub use memory::MemorySearchManager;
