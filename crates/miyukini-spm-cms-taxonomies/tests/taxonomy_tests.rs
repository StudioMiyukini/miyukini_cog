//! Tests du Module Taxonomies.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_taxonomies::{TaxonomyError, TaxonomyManager};
#[cfg(feature = "memory")]
use miyukini_spm_cms_taxonomies::MemoryTaxonomyManager;

#[cfg(feature = "memory")]
#[test]
fn test_create_taxonomy() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());

    // Vérifier que la taxonomie existe (via add_term qui doit réussir)
    let term_id = manager.add_term(taxonomy_id, "News".to_string());
    assert!(term_id.is_ok());
}

#[cfg(feature = "memory")]
#[test]
fn test_add_term() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());

    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();

    // Vérifier que le terme existe (via assign_term qui doit réussir)
    let entity_id = UuidIdGenerator::new().generate();
    let result = manager.assign_term(term_id, entity_id);
    assert!(result.is_ok());
}

#[cfg(feature = "memory")]
#[test]
fn test_add_term_taxonomy_not_found() {
    let mut manager = MemoryTaxonomyManager::new();
    let fake_taxonomy_id = UuidIdGenerator::new().generate();

    let result = manager.add_term(fake_taxonomy_id, "News".to_string());
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TaxonomyError::TaxonomyNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_assign_term() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Assigner le terme à l'entité
    let result = manager.assign_term(term_id, entity_id);
    assert!(result.is_ok());

    // Vérifier que l'assignation est bidirectionnelle
    let terms = manager.terms_for_entity(entity_id);
    assert!(terms.contains(&term_id));

    let entities = manager.entities_for_term(term_id);
    assert!(entities.contains(&entity_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_assign_term_idempotent() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Assigner deux fois le même terme à la même entité
    assert!(manager.assign_term(term_id, entity_id).is_ok());
    assert!(manager.assign_term(term_id, entity_id).is_ok());

    // Vérifier qu'il n'y a qu'une seule assignation
    let terms = manager.terms_for_entity(entity_id);
    assert_eq!(terms.len(), 1);
    assert!(terms.contains(&term_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_assign_term_not_found() {
    let mut manager = MemoryTaxonomyManager::new();
    let fake_term_id = UuidIdGenerator::new().generate();
    let entity_id = UuidIdGenerator::new().generate();

    let result = manager.assign_term(fake_term_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TaxonomyError::TermNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_unassign_term() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Assigner puis désassigner
    manager.assign_term(term_id, entity_id).unwrap();
    let result = manager.unassign_term(term_id, entity_id);
    assert!(result.is_ok());

    // Vérifier que l'assignation a été retirée
    let terms = manager.terms_for_entity(entity_id);
    assert!(!terms.contains(&term_id));

    let entities = manager.entities_for_term(term_id);
    assert!(!entities.contains(&entity_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_unassign_term_idempotent() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Désassigner un terme non assigné (doit être idempotent)
    let result = manager.unassign_term(term_id, entity_id);
    assert!(result.is_ok());
}

#[cfg(feature = "memory")]
#[test]
fn test_unassign_term_not_found() {
    let mut manager = MemoryTaxonomyManager::new();
    let fake_term_id = UuidIdGenerator::new().generate();
    let entity_id = UuidIdGenerator::new().generate();

    let result = manager.unassign_term(fake_term_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TaxonomyError::TermNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_terms_for_entity() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term1_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let term2_id = manager.add_term(taxonomy_id, "Tech".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Assigner deux termes à l'entité
    manager.assign_term(term1_id, entity_id).unwrap();
    manager.assign_term(term2_id, entity_id).unwrap();

    // Vérifier que les deux termes sont retournés
    let terms = manager.terms_for_entity(entity_id);
    assert_eq!(terms.len(), 2);
    assert!(terms.contains(&term1_id));
    assert!(terms.contains(&term2_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_terms_for_entity_empty() {
    let manager = MemoryTaxonomyManager::new();
    let entity_id = UuidIdGenerator::new().generate();

    // Vérifier qu'une entité sans termes retourne un vecteur vide
    let terms = manager.terms_for_entity(entity_id);
    assert!(terms.is_empty());
}

#[cfg(feature = "memory")]
#[test]
fn test_entities_for_term() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let entity1_id = UuidIdGenerator::new().generate();
    let entity2_id = UuidIdGenerator::new().generate();

    // Assigner le terme à deux entités
    manager.assign_term(term_id, entity1_id).unwrap();
    manager.assign_term(term_id, entity2_id).unwrap();

    // Vérifier que les deux entités sont retournées
    let entities = manager.entities_for_term(term_id);
    assert_eq!(entities.len(), 2);
    assert!(entities.contains(&entity1_id));
    assert!(entities.contains(&entity2_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_entities_for_term_empty() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();

    // Vérifier qu'un terme sans entités retourne un vecteur vide
    let entities = manager.entities_for_term(term_id);
    assert!(entities.is_empty());
}

#[cfg(feature = "memory")]
#[test]
fn test_complex_scenario() {
    let mut manager = MemoryTaxonomyManager::new();

    // Créer deux taxonomies
    let tags_id = manager.create_taxonomy("Tags".to_string());
    let categories_id = manager.create_taxonomy("Categories".to_string());

    // Ajouter des termes
    let news_tag = manager.add_term(tags_id, "News".to_string()).unwrap();
    let tech_tag = manager.add_term(tags_id, "Tech".to_string()).unwrap();
    let article_cat = manager.add_term(categories_id, "Article".to_string()).unwrap();

    // Créer des entités
    let entity1_id = UuidIdGenerator::new().generate();
    let entity2_id = UuidIdGenerator::new().generate();

    // Assigner des termes
    manager.assign_term(news_tag, entity1_id).unwrap();
    manager.assign_term(tech_tag, entity1_id).unwrap();
    manager.assign_term(article_cat, entity1_id).unwrap();

    manager.assign_term(news_tag, entity2_id).unwrap();
    manager.assign_term(article_cat, entity2_id).unwrap();

    // Vérifier les assignations
    let entity1_terms = manager.terms_for_entity(entity1_id);
    assert_eq!(entity1_terms.len(), 3);
    assert!(entity1_terms.contains(&news_tag));
    assert!(entity1_terms.contains(&tech_tag));
    assert!(entity1_terms.contains(&article_cat));

    let entity2_terms = manager.terms_for_entity(entity2_id);
    assert_eq!(entity2_terms.len(), 2);
    assert!(entity2_terms.contains(&news_tag));
    assert!(entity2_terms.contains(&article_cat));

    let news_entities = manager.entities_for_term(news_tag);
    assert_eq!(news_entities.len(), 2);
    assert!(news_entities.contains(&entity1_id));
    assert!(news_entities.contains(&entity2_id));

    // Désassigner un terme
    manager.unassign_term(tech_tag, entity1_id).unwrap();
    let entity1_terms_after = manager.terms_for_entity(entity1_id);
    assert_eq!(entity1_terms_after.len(), 2);
    assert!(!entity1_terms_after.contains(&tech_tag));
}

#[cfg(feature = "memory")]
#[test]
fn test_remove_taxonomy_cascades() {
    let mut manager = MemoryTaxonomyManager::new();
    let taxonomy_id = manager.create_taxonomy("Tags".to_string());
    let term1_id = manager.add_term(taxonomy_id, "News".to_string()).unwrap();
    let term2_id = manager.add_term(taxonomy_id, "Tech".to_string()).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    // Assigner les termes
    manager.assign_term(term1_id, entity_id).unwrap();
    manager.assign_term(term2_id, entity_id).unwrap();

    // Supprimer la taxonomie
    manager.remove_taxonomy(taxonomy_id).unwrap();

    // Vérifier que les termes et assignations ont été supprimés
    let terms = manager.terms_for_entity(entity_id);
    assert!(terms.is_empty());

    // Vérifier que les termes n'existent plus
    let result = manager.assign_term(term1_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), TaxonomyError::TermNotFound));
}
