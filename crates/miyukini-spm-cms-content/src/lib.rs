//! Module Contenu — SPM CMS
//!
//! Gestion des entités de contenu (pages, articles, blocs) : CRUD, statuts (brouillon/publié/archivé),
//! relations, versioning, métadonnées.
//!
//! Contrat fonctionnel : voir [docs/spm-cms/modules/content/contrat.md](../../docs/spm-cms/modules/content/contrat.md)
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_content::{ContentManager, ContentInput, ContentStatus};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente ContentManager
//! // struct MyAdapter { ... }
//! // impl ContentManager for MyAdapter { ... }
//!
//! // Créer un contenu
//! // let input = ContentInput::new("article".to_string(), b"metadata".to_vec());
//! // let id = adapter.create_content(input)?;
//! ```

pub mod content;
pub mod error;
pub mod relation;
pub mod status;
pub mod version;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use content::{
    Content, ContentFilters, ContentInput, ContentListResult, ContentManager, ContentUpdates,
};
pub use error::ContentError;
pub use relation::ContentRelation;
pub use status::ContentStatus;
pub use version::ContentVersion;

#[cfg(feature = "memory")]
pub use memory::MemoryContentManager;
