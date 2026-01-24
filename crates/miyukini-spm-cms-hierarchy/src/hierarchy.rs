//! Types et trait HierarchyManager.

use miyukini_kernel::Id;

/// Identifiant de nœud dans la hiérarchie (opaque, basé sur kernel::Id).
pub type NodeId = Id;

/// Identifiant d'entité externe référencée par un nœud (alias vers Id du kernel).
pub type EntityId = Id;

/// Erreurs possibles lors des opérations sur la hiérarchie.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HierarchyError {
    /// Nœud introuvable.
    NodeNotFound,
    /// Cycle détecté (tentative de créer une boucle dans l'arbre).
    CycleDetected,
    /// Opération invalide (ex. déplacer un nœud vers lui-même).
    InvalidOperation,
}

impl std::fmt::Display for HierarchyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HierarchyError::NodeNotFound => write!(f, "node not found"),
            HierarchyError::CycleDetected => write!(f, "cycle detected"),
            HierarchyError::InvalidOperation => write!(f, "invalid operation"),
        }
    }
}

impl std::error::Error for HierarchyError {}

/// Trait principal : gestion de la hiérarchie.
///
/// Le produit implémente ce trait pour adapter le contrat fonctionnel
/// vers sa stack technique (DB, sérialisation, etc.).
///
/// # Invariants
///
/// - Chaque nœud a au plus un parent (structure arborescente)
/// - Pas de cycles (l'arbre reste acyclique)
/// - Les références sont cohérentes (parent/children bidirectionnel)
pub trait HierarchyManager {
    /// Crée un nœud racine (sans parent).
    ///
    /// L'identifiant du nœud est généré par le kernel (IdGenerator).
    /// Retourne le `NodeId` du nœud créé.
    fn create_root(&mut self, entity_id: EntityId) -> NodeId;

    /// Crée un nœud enfant sous un parent donné.
    ///
    /// L'identifiant du nœud est généré par le kernel (IdGenerator).
    /// Retourne le `NodeId` du nœud créé.
    ///
    /// # Erreurs
    ///
    /// - `HierarchyError::NodeNotFound` si le parent n'existe pas
    /// - `HierarchyError::CycleDetected` si la création créerait un cycle
    fn create_child(&mut self, parent: NodeId, entity_id: EntityId) -> Result<NodeId, HierarchyError>;

    /// Retourne le parent d'un nœud, s'il existe.
    ///
    /// Retourne `None` pour un nœud racine.
    fn parent(&self, node: NodeId) -> Option<NodeId>;

    /// Retourne la liste des enfants directs d'un nœud.
    ///
    /// Retourne un vecteur vide si le nœud n'a pas d'enfants.
    fn children(&self, node: NodeId) -> Vec<NodeId>;

    /// Retourne la liste de tous les ancêtres d'un nœud (du parent direct jusqu'à la racine).
    ///
    /// L'ordre est du parent direct vers la racine (du plus proche au plus lointain).
    /// Retourne un vecteur vide si le nœud est une racine.
    fn ancestors(&self, node: NodeId) -> Vec<NodeId>;

    /// Retourne le chemin complet d'un nœud jusqu'à la racine.
    ///
    /// L'ordre est du nœud lui-même jusqu'à la racine (du plus proche au plus lointain).
    /// Le premier élément est le nœud lui-même, le dernier est la racine.
    fn path_to_root(&self, node: NodeId) -> Vec<NodeId>;

    /// Déplace un nœud sous un nouveau parent.
    ///
    /// # Erreurs
    ///
    /// - `HierarchyError::NodeNotFound` si le nœud ou le nouveau parent n'existe pas
    /// - `HierarchyError::CycleDetected` si le déplacement créerait un cycle
    /// - `HierarchyError::InvalidOperation` si le nœud est déjà sous ce parent
    fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), HierarchyError>;

    /// Supprime un nœud de la hiérarchie.
    ///
    /// # Comportement
    ///
    /// Le comportement vis-à-vis des enfants est défini par le produit :
    /// - Suppression en cascade (les enfants sont également supprimés)
    /// - Orphelinage (les enfants deviennent des racines)
    /// - Refus si le nœud a des enfants
    ///
    /// Pour Phase 0, on adopte un comportement simple : les enfants deviennent des racines.
    ///
    /// # Erreurs
    ///
    /// - `HierarchyError::NodeNotFound` si le nœud n'existe pas
    fn remove_node(&mut self, node: NodeId) -> Result<(), HierarchyError>;
}
