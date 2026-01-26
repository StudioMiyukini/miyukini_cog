//! Module Publication — SPM CMS
//!
//! Gestion du cycle éditorial générique des entités de contenu :
//! statuts (draft, scheduled, published, archived), dates de publication,
//! publication différée (scheduling).
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_publication::{PublicationManager, PublicationStatus};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente PublicationManager
//! // struct MyAdapter { ... }
//! // impl PublicationManager for MyAdapter { ... }
//!
//! // Créer une publication
//! // let content_id = Id::parse("...").unwrap();
//! // let publication_id = adapter.create_publication(content_id)?;
//! ```

pub mod error;
pub mod publication;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use error::PublicationError;
pub use publication::{
    ContentId, Publication, PublicationId, PublicationManager, PublicationStatus,
};

#[cfg(feature = "memory")]
pub use memory::MemoryPublicationManager;
