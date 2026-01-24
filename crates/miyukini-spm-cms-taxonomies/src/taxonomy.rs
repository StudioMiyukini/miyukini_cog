//! Types et trait TaxonomyManager.

use miyukini_kernel::Id;

/// Identifiant de taxonomie (alias vers Id du kernel).
pub type TaxonomyId = Id;

/// Identifiant de terme (alias vers Id du kernel).
pub type TermId = Id;

/// Identifiant d'entité externe référencée (alias vers Id du kernel).
pub type EntityId = Id;

/// Erreurs possibles lors des opérations sur les taxonomies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaxonomyError {
    /// Taxonomie introuvable.
    TaxonomyNotFound,
    /// Terme introuvable.
    TermNotFound,
    /// Opération invalide.
    InvalidOperation,
}

impl std::fmt::Display for TaxonomyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaxonomyError::TaxonomyNotFound => write!(f, "taxonomy not found"),
            TaxonomyError::TermNotFound => write!(f, "term not found"),
            TaxonomyError::InvalidOperation => write!(f, "invalid operation"),
        }
    }
}

impl std::error::Error for TaxonomyError {}

/// Trait principal : gestion des taxonomies.
///
/// Le produit implémente ce trait pour adapter le contrat fonctionnel
/// vers sa stack technique (DB, sérialisation, etc.).
///
/// # Invariants
///
/// - Chaque terme appartient à une taxonomie
/// - Les assignations sont bidirectionnelles (terme ↔ entité)
/// - La suppression d'une taxonomie supprime ses termes et assignations
pub trait TaxonomyManager {
    /// Crée une nouvelle taxonomie.
    ///
    /// L'identifiant de la taxonomie est généré par le kernel (IdGenerator).
    /// Retourne le `TaxonomyId` de la taxonomie créée.
    fn create_taxonomy(&mut self, name: String) -> TaxonomyId;

    /// Ajoute un terme à une taxonomie.
    ///
    /// L'identifiant du terme est généré par le kernel (IdGenerator).
    /// Retourne le `TermId` du terme créé.
    ///
    /// # Erreurs
    ///
    /// - `TaxonomyError::TaxonomyNotFound` si la taxonomie n'existe pas
    fn add_term(&mut self, taxonomy: TaxonomyId, label: String) -> Result<TermId, TaxonomyError>;

    /// Assigne un terme à une entité.
    ///
    /// Si le terme est déjà assigné à l'entité, l'opération est idempotente (pas d'erreur).
    ///
    /// # Erreurs
    ///
    /// - `TaxonomyError::TermNotFound` si le terme n'existe pas
    fn assign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError>;

    /// Désassigne un terme d'une entité.
    ///
    /// Si le terme n'est pas assigné à l'entité, l'opération est idempotente (pas d'erreur).
    ///
    /// # Erreurs
    ///
    /// - `TaxonomyError::TermNotFound` si le terme n'existe pas
    fn unassign_term(&mut self, term: TermId, entity: EntityId) -> Result<(), TaxonomyError>;

    /// Retourne la liste des termes assignés à une entité.
    ///
    /// Retourne un vecteur vide si l'entité n'a aucun terme assigné.
    fn terms_for_entity(&self, entity: EntityId) -> Vec<TermId>;

    /// Retourne la liste des entités assignées à un terme.
    ///
    /// Retourne un vecteur vide si le terme n'a aucune entité assignée.
    fn entities_for_term(&self, term: TermId) -> Vec<EntityId>;
}
