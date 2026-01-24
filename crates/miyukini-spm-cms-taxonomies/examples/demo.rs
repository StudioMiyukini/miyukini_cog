//! Démonstration console du Module Taxonomies.
//!
//! Exemple d'utilisation du MemoryTaxonomyManager pour créer une taxonomie,
//! ajouter des termes, et assigner des termes à des entités.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_taxonomies::{TaxonomyError, TaxonomyManager, MemoryTaxonomyManager};

fn main() -> Result<(), TaxonomyError> {
    let mut manager = MemoryTaxonomyManager::new();
    let id_gen = UuidIdGenerator::new();

    println!("=== Démonstration du Module Taxonomies ===\n");

    // Créer une taxonomie "Tags"
    println!("1. Création d'une taxonomie 'Tags'...");
    let tags_id = manager.create_taxonomy("Tags".to_string());
    println!("   ✓ Taxonomie créée: {}", tags_id);

    // Ajouter des termes
    println!("\n2. Ajout de termes à la taxonomie...");
    let news_term = manager.add_term(tags_id, "News".to_string())?;
    println!("   ✓ Terme 'News' créé: {}", news_term);

    let tech_term = manager.add_term(tags_id, "Tech".to_string())?;
    println!("   ✓ Terme 'Tech' créé: {}", tech_term);

    // Créer une entité
    println!("\n3. Création d'une entité...");
    let entity_id = id_gen.generate();
    println!("   ✓ Entité créée: {}", entity_id);

    // Assigner des termes à l'entité
    println!("\n4. Assignation de termes à l'entité...");
    manager.assign_term(news_term, entity_id)?;
    println!("   ✓ Terme 'News' assigné à l'entité");

    manager.assign_term(tech_term, entity_id)?;
    println!("   ✓ Terme 'Tech' assigné à l'entité");

    // Afficher les termes de l'entité
    println!("\n5. Termes assignés à l'entité:");
    let terms = manager.terms_for_entity(entity_id);
    println!("   Nombre de termes: {}", terms.len());
    for term_id in &terms {
        println!("   - {}", term_id);
    }

    // Afficher les entités pour un terme
    println!("\n6. Entités assignées au terme 'News':");
    let entities = manager.entities_for_term(news_term);
    println!("   Nombre d'entités: {}", entities.len());
    for entity_id in &entities {
        println!("   - {}", entity_id);
    }

    // Désassigner un terme
    println!("\n7. Désassignation du terme 'Tech'...");
    manager.unassign_term(tech_term, entity_id)?;
    println!("   ✓ Terme 'Tech' désassigné");

    // Afficher les termes restants
    println!("\n8. Termes restants pour l'entité:");
    let remaining_terms = manager.terms_for_entity(entity_id);
    println!("   Nombre de termes: {}", remaining_terms.len());
    for term_id in &remaining_terms {
        println!("   - {}", term_id);
    }

    println!("\n=== Démonstration terminée avec succès ===");
    Ok(())
}
