//! Module Hiérarchie — SPM CMS (Phase 0)
//!
//! Organisation générique d'entités externes en arbre. Le module ne connaît pas le CMS,
//! le rendu, ni les utilisateurs. Il organise uniquement des entités référencées par des IDs opaques.
//!
//! Contrat fonctionnel : gestion d'une hiérarchie d'entités avec :
//! - Création de racines et d'enfants
//! - Navigation (parent, children, ancestors, path_to_root)
//! - Déplacement de nœuds
//! - Suppression de nœuds
//!
//! Ce module expose uniquement des contrats fonctionnels. Il ne connaît pas la persistance,
//! les permissions, ni la logique métier spécifique. Le produit implémente les adaptateurs.
//!
//! # Exemple d'usage
//!
//! ```rust,no_run
//! use miyukini_spm_cms_hierarchy::{HierarchyManager, EntityId};
//! use miyukini_kernel::Id;
//!
//! // Le produit implémente HierarchyManager
//! // struct MyAdapter { ... }
//! // impl HierarchyManager for MyAdapter { ... }
//!
//! // Créer une racine
//! // let entity_id = Id::parse("...").unwrap();
//! // let root_node = adapter.create_root(entity_id)?;
//! ```

pub mod hierarchy;

#[cfg(feature = "memory")]
pub mod memory;

// Ré-exports publics
pub use hierarchy::{EntityId, HierarchyError, HierarchyManager, NodeId};

#[cfg(feature = "memory")]
pub use memory::MemoryHierarchyManager;
