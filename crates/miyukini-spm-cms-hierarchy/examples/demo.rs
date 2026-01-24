//! Démonstration console du Module Hiérarchie.
//!
//! Exemple d'utilisation du MemoryHierarchyManager pour créer un arbre simple,
//! afficher le chemin d'un nœud, et tenter de créer un cycle (refusé).

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_hierarchy::{HierarchyError, HierarchyManager, MemoryHierarchyManager};

fn main() -> Result<(), HierarchyError> {
    let mut manager = MemoryHierarchyManager::new();
    let id_gen = UuidIdGenerator::new();

    println!("=== Démonstration du Module Hiérarchie ===\n");

    // Créer un arbre simple :
    // root
    //   ├─ child1
    //   │   └─ grandchild1
    //   └─ child2

    println!("1. Création d'un arbre simple...");
    let root_id = manager.create_root(id_gen.generate());
    println!("   ✓ Racine créée: {}", root_id);

    let child1_id = manager.create_child(root_id, id_gen.generate())?;
    println!("   ✓ Enfant 1 créé: {}", child1_id);

    let child2_id = manager.create_child(root_id, id_gen.generate())?;
    println!("   ✓ Enfant 2 créé: {}", child2_id);

    let grandchild1_id = manager.create_child(child1_id, id_gen.generate())?;
    println!("   ✓ Petit-enfant 1 créé: {}", grandchild1_id);

    // Afficher le chemin vers la racine
    println!("\n2. Chemin du petit-enfant vers la racine:");
    let path = manager.path_to_root(grandchild1_id);
    for (i, node_id) in path.iter().enumerate() {
        let indent = "  ".repeat(i);
        println!("   {}{}", indent, node_id);
    }

    // Afficher les ancêtres
    println!("\n3. Ancêtres du petit-enfant:");
    let ancestors = manager.ancestors(grandchild1_id);
    for ancestor_id in &ancestors {
        println!("   - {}", ancestor_id);
    }

    // Tenter de créer un cycle (doit être refusé)
    println!("\n4. Tentative de créer un cycle (déplacer root sous grandchild1)...");
    match manager.move_node(root_id, grandchild1_id) {
        Err(HierarchyError::CycleDetected) => {
            println!("   ✓ Cycle détecté et refusé (comportement attendu)");
        }
        Ok(_) => {
            println!("   ✗ ERREUR: Le cycle n'a pas été détecté !");
            return Err(HierarchyError::InvalidOperation);
        }
        Err(e) => {
            println!("   ✗ ERREUR inattendue: {:?}", e);
            return Err(e);
        }
    }

    // Déplacer un nœud valide
    println!("\n5. Déplacement valide (grandchild1 sous child2)...");
    manager.move_node(grandchild1_id, child2_id)?;
    println!("   ✓ Déplacement réussi");

    // Afficher le nouveau chemin
    println!("\n6. Nouveau chemin du petit-enfant vers la racine:");
    let new_path = manager.path_to_root(grandchild1_id);
    for (i, node_id) in new_path.iter().enumerate() {
        let indent = "  ".repeat(i);
        println!("   {}{}", indent, node_id);
    }

    println!("\n=== Démonstration terminée avec succès ===");
    Ok(())
}
