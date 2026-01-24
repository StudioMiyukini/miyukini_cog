//! Tests du Module Hiérarchie.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_hierarchy::{HierarchyError, HierarchyManager};
#[cfg(feature = "memory")]
use miyukini_spm_cms_hierarchy::MemoryHierarchyManager;

#[cfg(feature = "memory")]
#[test]
fn test_create_root() {
    let mut manager = MemoryHierarchyManager::new();
    let entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(entity_id);

    // Vérifier que le nœud existe et est bien une racine
    assert!(manager.parent(root_id).is_none());
    assert!(manager.children(root_id).is_empty());
}

#[cfg(feature = "memory")]
#[test]
fn test_create_child() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();

    // Vérifier la relation parent-enfant
    assert_eq!(manager.parent(child_id), Some(root_id));
    assert!(manager.children(root_id).contains(&child_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_create_child_parent_not_found() {
    let mut manager = MemoryHierarchyManager::new();
    let fake_parent_id = UuidIdGenerator::new().generate();
    let entity_id = UuidIdGenerator::new().generate();

    let result = manager.create_child(fake_parent_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::NodeNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_parent_and_children() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child1_entity_id = UuidIdGenerator::new().generate();
    let child2_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child1_id = manager.create_child(root_id, child1_entity_id).unwrap();
    let child2_id = manager.create_child(root_id, child2_entity_id).unwrap();

    // Vérifier les enfants de la racine
    let children = manager.children(root_id);
    assert_eq!(children.len(), 2);
    assert!(children.contains(&child1_id));
    assert!(children.contains(&child2_id));

    // Vérifier les parents des enfants
    assert_eq!(manager.parent(child1_id), Some(root_id));
    assert_eq!(manager.parent(child2_id), Some(root_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_path_to_root() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();
    let grandchild_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();
    let grandchild_id = manager.create_child(child_id, grandchild_entity_id).unwrap();

    // Vérifier le chemin de la racine (doit contenir uniquement la racine)
    let root_path = manager.path_to_root(root_id);
    assert_eq!(root_path, vec![root_id]);

    // Vérifier le chemin de l'enfant
    let child_path = manager.path_to_root(child_id);
    assert_eq!(child_path, vec![child_id, root_id]);

    // Vérifier le chemin du petit-enfant
    let grandchild_path = manager.path_to_root(grandchild_id);
    assert_eq!(grandchild_path, vec![grandchild_id, child_id, root_id]);
}

#[cfg(feature = "memory")]
#[test]
fn test_ancestors() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();
    let grandchild_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();
    let grandchild_id = manager.create_child(child_id, grandchild_entity_id).unwrap();

    // Vérifier les ancêtres de la racine (aucun)
    let root_ancestors = manager.ancestors(root_id);
    assert!(root_ancestors.is_empty());

    // Vérifier les ancêtres de l'enfant
    let child_ancestors = manager.ancestors(child_id);
    assert_eq!(child_ancestors, vec![root_id]);

    // Vérifier les ancêtres du petit-enfant
    let grandchild_ancestors = manager.ancestors(grandchild_id);
    assert_eq!(grandchild_ancestors, vec![child_id, root_id]);
}

#[cfg(feature = "memory")]
#[test]
fn test_move_node() {
    let mut manager = MemoryHierarchyManager::new();
    let root1_entity_id = UuidIdGenerator::new().generate();
    let root2_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();

    let root1_id = manager.create_root(root1_entity_id);
    let root2_id = manager.create_root(root2_entity_id);
    let child_id = manager.create_child(root1_id, child_entity_id).unwrap();

    // Déplacer l'enfant de root1 vers root2
    manager.move_node(child_id, root2_id).unwrap();

    // Vérifier que l'enfant est maintenant sous root2
    assert_eq!(manager.parent(child_id), Some(root2_id));
    assert!(manager.children(root2_id).contains(&child_id));
    assert!(!manager.children(root1_id).contains(&child_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_move_node_cycle_detection() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();
    let grandchild_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();
    let grandchild_id = manager.create_child(child_id, grandchild_entity_id).unwrap();

    // Tentative de déplacer root sous grandchild (créerait un cycle)
    let result = manager.move_node(root_id, grandchild_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::CycleDetected));
}

#[cfg(feature = "memory")]
#[test]
fn test_move_node_to_self() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let root_id = manager.create_root(root_entity_id);

    // Tentative de déplacer un nœud vers lui-même
    let result = manager.move_node(root_id, root_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::InvalidOperation));
}

#[cfg(feature = "memory")]
#[test]
fn test_move_node_not_found() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let root_id = manager.create_root(root_entity_id);
    let fake_node_id = UuidIdGenerator::new().generate();

    // Tentative de déplacer un nœud inexistant
    let result = manager.move_node(fake_node_id, root_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::NodeNotFound));

    // Tentative de déplacer vers un parent inexistant
    let result = manager.move_node(root_id, fake_node_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::NodeNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_remove_node() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();

    // Supprimer l'enfant
    manager.remove_node(child_id).unwrap();

    // Vérifier que l'enfant n'existe plus
    assert!(!manager.children(root_id).contains(&child_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_remove_node_children_become_roots() {
    let mut manager = MemoryHierarchyManager::new();
    let root_entity_id = UuidIdGenerator::new().generate();
    let child_entity_id = UuidIdGenerator::new().generate();
    let grandchild_entity_id = UuidIdGenerator::new().generate();

    let root_id = manager.create_root(root_entity_id);
    let child_id = manager.create_child(root_id, child_entity_id).unwrap();
    let grandchild_id = manager.create_child(child_id, grandchild_entity_id).unwrap();

    // Supprimer l'enfant (qui a lui-même un enfant)
    manager.remove_node(child_id).unwrap();

    // Vérifier que le petit-enfant est maintenant une racine
    assert!(manager.parent(grandchild_id).is_none());
}

#[cfg(feature = "memory")]
#[test]
fn test_remove_node_not_found() {
    let mut manager = MemoryHierarchyManager::new();
    let fake_node_id = UuidIdGenerator::new().generate();

    // Tentative de supprimer un nœud inexistant
    let result = manager.remove_node(fake_node_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), HierarchyError::NodeNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_complex_hierarchy() {
    let mut manager = MemoryHierarchyManager::new();

    // Créer une hiérarchie complexe :
    // root1
    //   ├─ child1
    //   │   └─ grandchild1
    //   └─ child2
    // root2
    //   └─ child3

    let root1_id = manager.create_root(UuidIdGenerator::new().generate());
    let root2_id = manager.create_root(UuidIdGenerator::new().generate());

    let child1_id = manager.create_child(root1_id, UuidIdGenerator::new().generate()).unwrap();
    let child2_id = manager.create_child(root1_id, UuidIdGenerator::new().generate()).unwrap();
    let _child3_id = manager.create_child(root2_id, UuidIdGenerator::new().generate()).unwrap();

    let grandchild1_id = manager.create_child(child1_id, UuidIdGenerator::new().generate()).unwrap();

    // Vérifier la structure
    assert_eq!(manager.children(root1_id).len(), 2);
    assert_eq!(manager.children(root2_id).len(), 1);
    assert_eq!(manager.children(child1_id).len(), 1);
    assert_eq!(manager.children(child2_id).len(), 0);

    // Vérifier les chemins
    let grandchild1_path = manager.path_to_root(grandchild1_id);
    assert_eq!(grandchild1_path, vec![grandchild1_id, child1_id, root1_id]);

    // Déplacer grandchild1 sous child2
    manager.move_node(grandchild1_id, child2_id).unwrap();
    assert_eq!(manager.parent(grandchild1_id), Some(child2_id));
    assert_eq!(manager.children(child1_id).len(), 0);
    assert_eq!(manager.children(child2_id).len(), 1);

    // Vérifier le nouveau chemin
    let new_path = manager.path_to_root(grandchild1_id);
    assert_eq!(new_path, vec![grandchild1_id, child2_id, root1_id]);
}
