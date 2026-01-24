//! Relations entre contenus.

use miyukini_kernel::Id;

/// Relation entre deux contenus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentRelation {
    /// Identifiant du contenu source.
    pub source_id: Id,
    /// Type de relation (défini par le produit, ex. "parent", "enfant", "lié").
    pub relation_type: String,
    /// Identifiant du contenu cible.
    pub target_id: Id,
}

impl ContentRelation {
    /// Crée une nouvelle relation.
    pub fn new(source_id: Id, relation_type: String, target_id: Id) -> Self {
        Self {
            source_id,
            relation_type,
            target_id,
        }
    }
}
