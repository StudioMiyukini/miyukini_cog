//! Implémentation en mémoire du HierarchyManager (pour tests et démo uniquement).
//!
//! Cette implémentation n'est pas destinée à la production. Elle sert uniquement
//! à valider le contrat et à fournir une base pour les tests et démos.
//!
//! Le produit fournira sa propre implémentation adaptée à sa stack technique.

use crate::hierarchy::{EntityId, HierarchyError, HierarchyManager, NodeId};
use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use std::collections::HashMap;

/// Structure interne d'un nœud dans la hiérarchie.
#[derive(Debug, Clone)]
struct Node {
    /// Identifiant de l'entité externe référencée.
    #[allow(dead_code)] // Stocké pour référence future (Phase 1+)
    entity_id: EntityId,
    /// Identifiant du parent (None pour une racine).
    parent: Option<NodeId>,
    /// Liste des identifiants des enfants directs.
    children: Vec<NodeId>,
}

impl Node {
    fn new(entity_id: EntityId) -> Self {
        Self {
            entity_id,
            parent: None,
            children: Vec::new(),
        }
    }
}

/// Implémentation en mémoire du HierarchyManager.
///
/// Stocke les nœuds dans une HashMap. Utilise UuidIdGenerator du kernel.
/// Pas de persistance : données perdues à l'arrêt.
pub struct MemoryHierarchyManager {
    nodes: HashMap<NodeId, Node>,
    id_generator: UuidIdGenerator,
}

impl MemoryHierarchyManager {
    /// Crée un nouveau gestionnaire en mémoire.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            id_generator: UuidIdGenerator::new(),
        }
    }
}

impl Default for MemoryHierarchyManager {
    fn default() -> Self {
        Self::new()
    }
}

impl HierarchyManager for MemoryHierarchyManager {
    fn create_root(&mut self, entity_id: EntityId) -> NodeId {
        let node_id = self.id_generator.generate();
        let node = Node::new(entity_id);
        self.nodes.insert(node_id, node);
        node_id
    }

    fn create_child(&mut self, parent: NodeId, entity_id: EntityId) -> Result<NodeId, HierarchyError> {
        // Vérifier que le parent existe
        if !self.nodes.contains_key(&parent) {
            return Err(HierarchyError::NodeNotFound);
        }

        // Créer le nouveau nœud
        let node_id = self.id_generator.generate();
        let mut node = Node::new(entity_id);
        node.parent = Some(parent);

        // Vérifier qu'on ne crée pas de cycle (le parent ne doit pas être un descendant du nouveau nœud)
        // Cette vérification est triviale ici car le nœud n'a pas encore d'enfants,
        // mais on la fait pour la cohérence avec move_node
        if self.would_create_cycle(node_id, Some(parent)) {
            return Err(HierarchyError::CycleDetected);
        }

        // Ajouter le nœud
        self.nodes.insert(node_id, node);

        // Ajouter le nœud comme enfant du parent
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.children.push(node_id);
        }

        Ok(node_id)
    }

    fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.nodes.get(&node).and_then(|n| n.parent)
    }

    fn children(&self, node: NodeId) -> Vec<NodeId> {
        self.nodes
            .get(&node)
            .map(|n| n.children.clone())
            .unwrap_or_default()
    }

    fn ancestors(&self, node: NodeId) -> Vec<NodeId> {
        let mut ancestors = Vec::new();
        let mut current = self.nodes.get(&node).and_then(|n| n.parent);

        while let Some(parent_id) = current {
            ancestors.push(parent_id);
            current = self.nodes.get(&parent_id).and_then(|n| n.parent);
        }

        ancestors
    }

    fn path_to_root(&self, node: NodeId) -> Vec<NodeId> {
        let mut path = vec![node];
        let mut current = self.nodes.get(&node).and_then(|n| n.parent);

        while let Some(parent_id) = current {
            path.push(parent_id);
            current = self.nodes.get(&parent_id).and_then(|n| n.parent);
        }

        path
    }

    fn move_node(&mut self, node: NodeId, new_parent: NodeId) -> Result<(), HierarchyError> {
        // Vérifier que le nœud et le nouveau parent existent
        if !self.nodes.contains_key(&node) {
            return Err(HierarchyError::NodeNotFound);
        }
        if !self.nodes.contains_key(&new_parent) {
            return Err(HierarchyError::NodeNotFound);
        }

        // Vérifier qu'on ne déplace pas un nœud vers lui-même
        if node == new_parent {
            return Err(HierarchyError::InvalidOperation);
        }

        // Vérifier qu'on ne crée pas de cycle (le nouveau parent ne doit pas être un descendant du nœud)
        if self.would_create_cycle(node, Some(new_parent)) {
            return Err(HierarchyError::CycleDetected);
        }

        // Récupérer l'ancien parent
        let old_parent = self.nodes.get(&node).and_then(|n| n.parent);

        // Retirer le nœud de la liste des enfants de l'ancien parent
        if let Some(old_parent_id) = old_parent {
            if let Some(old_parent_node) = self.nodes.get_mut(&old_parent_id) {
                old_parent_node.children.retain(|&child_id| child_id != node);
            }
        }

        // Ajouter le nœud comme enfant du nouveau parent
        if let Some(new_parent_node) = self.nodes.get_mut(&new_parent) {
            new_parent_node.children.push(node);
        }

        // Mettre à jour le parent du nœud
        if let Some(node_ref) = self.nodes.get_mut(&node) {
            node_ref.parent = Some(new_parent);
        }

        Ok(())
    }

    fn remove_node(&mut self, node: NodeId) -> Result<(), HierarchyError> {
        // Vérifier que le nœud existe et récupérer ses données
        let node_data = self.nodes.get(&node).ok_or(HierarchyError::NodeNotFound)?;
        
        // Récupérer les enfants et le parent directement depuis le nœud
        let children = node_data.children.clone();
        let parent_id = node_data.parent;

        // Retirer le nœud de la liste des enfants de son parent
        if let Some(parent_id) = parent_id {
            if let Some(parent_node) = self.nodes.get_mut(&parent_id) {
                parent_node.children.retain(|&child_id| child_id != node);
            }
        }

        // Supprimer le nœud
        self.nodes.remove(&node);

        // Les enfants deviennent des racines (comportement Phase 0)
        for child_id in children {
            if let Some(child_node) = self.nodes.get_mut(&child_id) {
                child_node.parent = None;
            }
        }

        Ok(())
    }
}

impl MemoryHierarchyManager {
    /// Vérifie si l'ajout d'un parent créerait un cycle.
    ///
    /// Un cycle serait créé si le parent proposé est un descendant du nœud.
    fn would_create_cycle(&self, node: NodeId, proposed_parent: Option<NodeId>) -> bool {
        let Some(parent_id) = proposed_parent else {
            return false; // Pas de parent = pas de cycle possible
        };

        // Parcourir les ancêtres du parent proposé
        // Si on trouve le nœud dans les ancêtres, alors il y aurait un cycle
        let mut current = Some(parent_id);
        while let Some(ancestor_id) = current {
            if ancestor_id == node {
                return true; // Cycle détecté
            }
            current = self.nodes.get(&ancestor_id).and_then(|n| n.parent);
        }

        false
    }
}
