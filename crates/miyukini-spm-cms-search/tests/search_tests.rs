//! Tests unitaires pour le module Recherche & Indexation.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_search::{
    IndexedField, LogicalOperator, MemorySearchManager, SearchCriterion, SearchManager,
    SearchOperator, SearchQuery, SearchType,
};

#[test]
fn test_index_entity() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    let fields = vec![IndexedField::new(
        "title".to_string(),
        "Mon article".to_string(),
        SearchType::Contains,
    )];

    let result = manager.index_entity(entity_id, "content".to_string(), fields);
    assert!(result.is_ok());
    assert!(manager.is_indexed(entity_id, "content".to_string()));
}

#[test]
fn test_index_entity_update() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    // Indexer avec un champ
    let fields1 = vec![IndexedField::new(
        "title".to_string(),
        "Ancien titre".to_string(),
        SearchType::Contains,
    )];
    manager.index_entity(entity_id, "content".to_string(), fields1).unwrap();

    // Mettre à jour avec un nouveau champ
    let fields2 = vec![IndexedField::new(
        "title".to_string(),
        "Nouveau titre".to_string(),
        SearchType::Contains,
    )];
    manager.index_entity(entity_id, "content".to_string(), fields2).unwrap();

    // Vérifier que le champ a été mis à jour
    let fields = manager.get_indexed_fields(entity_id, "content".to_string()).unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].field_value, "Nouveau titre");
}

#[test]
fn test_unindex_entity() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    let fields = vec![IndexedField::new(
        "title".to_string(),
        "Mon article".to_string(),
        SearchType::Contains,
    )];

    manager.index_entity(entity_id, "content".to_string(), fields).unwrap();
    assert!(manager.is_indexed(entity_id, "content".to_string()));

    let result = manager.unindex_entity(entity_id, "content".to_string());
    assert!(result.is_ok());
    assert!(!manager.is_indexed(entity_id, "content".to_string()));
}

#[test]
fn test_unindex_entity_not_found() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    let result = manager.unindex_entity(entity_id, "content".to_string());
    assert!(result.is_err());
}

#[test]
fn test_search_equals() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "published".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "draft".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids.len(), 1);
    assert_eq!(result.entity_ids[0], entity_id1);
}

#[test]
fn test_search_contains() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article sur Rust".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article sur Python".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::Contains,
            "Rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id1);
}

#[test]
fn test_search_and() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![
            IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
            IndexedField::new("content_type".to_string(), "article".to_string(), SearchType::Exact),
        ],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![
            IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
            IndexedField::new("content_type".to_string(), "page".to_string(), SearchType::Exact),
        ],
    ).unwrap();

    let query = SearchQuery::new()
        .with_logical_operator(LogicalOperator::And)
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ))
        .add_criterion(SearchCriterion::new(
            "content_type".to_string(),
            SearchOperator::Equals,
            "article".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id1);
}

#[test]
fn test_search_or() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "published".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "draft".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .with_logical_operator(LogicalOperator::Or)
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ))
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "draft".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 2);
}

#[test]
fn test_search_pagination() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    // Indexer 5 entités
    for _ in 0..5 {
        let entity_id = id_gen.generate();
        manager.index_entity(
            entity_id,
            "content".to_string(),
            vec![IndexedField::new(
                "status".to_string(),
                "published".to_string(),
                SearchType::Exact,
            )],
        ).unwrap();
    }

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ));

    // Première page
    let result1 = manager.search(query.clone(), 0, 2).unwrap();
    assert_eq!(result1.total, 5);
    assert_eq!(result1.entity_ids.len(), 2);
    assert_eq!(result1.offset, 0);
    assert_eq!(result1.limit, 2);

    // Deuxième page
    let result2 = manager.search(query, 2, 2).unwrap();
    assert_eq!(result2.total, 5);
    assert_eq!(result2.entity_ids.len(), 2);
    assert_eq!(result2.offset, 2);
    assert_eq!(result2.limit, 2);
}

#[test]
fn test_search_field_not_found() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Mon article".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "status".to_string(), // Champ absent
            SearchOperator::Equals,
            "published".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
    assert_eq!(result.entity_ids.len(), 0);
}

#[test]
fn test_search_empty_query() {
    let manager = MemorySearchManager::new();

    let query = SearchQuery::new(); // Pas de critères

    let result = manager.search(query, 0, 10);
    assert!(result.is_err());
}

#[test]
fn test_search_empty_index() {
    let manager = MemorySearchManager::new();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
    assert_eq!(result.entity_ids.len(), 0);
}

#[test]
fn test_search_entity_type_filter() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let content_id = id_gen.generate();
    let media_id = id_gen.generate();

    manager.index_entity(
        content_id,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        media_id,
        "media".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Image".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    // Rechercher seulement dans "content"
    let query = SearchQuery::with_entity_type("content".to_string())
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::Contains,
            "Article".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], content_id);
}

#[test]
fn test_list_indexed_entities() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article 1".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article 2".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let result = manager.list_indexed_entities(Some("content".to_string()), 0, 10).unwrap();
    assert_eq!(result.total, 2);
}

#[test]
fn test_list_indexed_entities_all_types() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let content_id = id_gen.generate();
    let media_id = id_gen.generate();

    manager.index_entity(
        content_id,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        media_id,
        "media".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Image".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let result = manager.list_indexed_entities(None, 0, 10).unwrap();
    assert_eq!(result.total, 2);
}

#[test]
fn test_get_indexed_fields() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    let fields = vec![
        IndexedField::new("title".to_string(), "Mon article".to_string(), SearchType::Contains),
        IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
    ];

    manager.index_entity(entity_id, "content".to_string(), fields.clone()).unwrap();

    let retrieved_fields = manager.get_indexed_fields(entity_id, "content".to_string()).unwrap();
    assert_eq!(retrieved_fields.len(), 2);
    assert_eq!(retrieved_fields, fields);
}

#[test]
fn test_get_indexed_fields_not_found() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();
    let entity_id = id_gen.generate();

    let result = manager.get_indexed_fields(entity_id, "content".to_string());
    assert!(result.is_err());
}

#[test]
fn test_clear_index_all() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article 1".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "media".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Image".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.clear_index(None).unwrap();

    assert!(!manager.is_indexed(entity_id1, "content".to_string()));
    assert!(!manager.is_indexed(entity_id2, "media".to_string()));
}

#[test]
fn test_clear_index_by_type() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let content_id = id_gen.generate();
    let media_id = id_gen.generate();

    manager.index_entity(
        content_id,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        media_id,
        "media".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Image".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.clear_index(Some("content".to_string())).unwrap();

    assert!(!manager.is_indexed(content_id, "content".to_string()));
    assert!(manager.is_indexed(media_id, "media".to_string()));
}

#[test]
fn test_search_comparison_operators() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();
    let entity_id3 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "created_at".to_string(),
            "2024-01-01".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "created_at".to_string(),
            "2024-06-01".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id3,
        "content".to_string(),
        vec![IndexedField::new(
            "created_at".to_string(),
            "2024-12-01".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    // Rechercher les dates supérieures à "2024-05-01"
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "created_at".to_string(),
            SearchOperator::GreaterThan,
            "2024-05-01".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 2);
}

// ============================================================================
// Tests Phase 2.4 - Nouveaux opérateurs et fonctionnalités
// ============================================================================

#[test]
fn test_search_starts_with() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Rust Programming".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Python Programming".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::StartsWith,
            "Rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id1);
}

#[test]
fn test_search_ends_with() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "filename".to_string(),
            "document.pdf".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "filename".to_string(),
            "image.jpg".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "filename".to_string(),
            SearchOperator::EndsWith,
            ".pdf".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id1);
}

#[test]
fn test_search_between() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();
    let entity_id3 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "price".to_string(),
            "050".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "price".to_string(),
            "100".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id3,
        "content".to_string(),
        vec![IndexedField::new(
            "price".to_string(),
            "200".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::with_values(
            "price".to_string(),
            SearchOperator::Between,
            vec!["075".to_string(), "150".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id2);
}

#[test]
fn test_search_between_invalid_range() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::new(
            "price".to_string(),
            "100".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    // Plage invalide : min > max
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::with_values(
            "price".to_string(),
            SearchOperator::Between,
            vec!["150".to_string(), "100".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
}

#[test]
fn test_search_between_wrong_count() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::new(
            "price".to_string(),
            "100".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    // Between avec une seule valeur (devrait échouer)
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::with_values(
            "price".to_string(),
            SearchOperator::Between,
            vec!["100".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
}

#[test]
fn test_search_in() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id1 = id_gen.generate();
    let entity_id2 = id_gen.generate();
    let entity_id3 = id_gen.generate();

    manager.index_entity(
        entity_id1,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "published".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id2,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "draft".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    manager.index_entity(
        entity_id3,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "archived".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::with_values(
            "status".to_string(),
            SearchOperator::In,
            vec!["published".to_string(), "draft".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 2);
    assert!(result.entity_ids.contains(&entity_id1));
    assert!(result.entity_ids.contains(&entity_id2));
}

#[test]
fn test_search_in_empty_list() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::new(
            "status".to_string(),
            "published".to_string(),
            SearchType::Exact,
        )],
    ).unwrap();

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::with_values(
            "status".to_string(),
            SearchOperator::In,
            vec![],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
}

#[test]
fn test_multi_value_field_single_match() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::with_values(
            "tags".to_string(),
            vec!["rust".to_string(), "programming".to_string(), "systems".to_string()],
            SearchType::Exact,
        )],
    ).unwrap();

    // Rechercher avec un tag qui existe
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "tags".to_string(),
            SearchOperator::Equals,
            "rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.entity_ids[0], entity_id);
}

#[test]
fn test_multi_value_field_no_match() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::with_values(
            "tags".to_string(),
            vec!["rust".to_string(), "programming".to_string()],
            SearchType::Exact,
        )],
    ).unwrap();

    // Rechercher avec un tag qui n'existe pas
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "tags".to_string(),
            SearchOperator::Equals,
            "python".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 0);
}

#[test]
fn test_multi_value_field_contains() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::with_values(
            "tags".to_string(),
            vec!["rust-programming".to_string(), "systems-design".to_string()],
            SearchType::Contains,
        )],
    ).unwrap();

    // Rechercher avec Contains - doit matcher si au moins une valeur contient
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "tags".to_string(),
            SearchOperator::Contains,
            "rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    assert_eq!(result.total, 1);
}

#[test]
fn test_index_stats_empty() {
    let manager = MemorySearchManager::new();
    let stats = manager.get_index_stats();

    assert_eq!(stats.total_entities, 0);
    assert_eq!(stats.entities_by_type.len(), 0);
    assert_eq!(stats.total_fields, 0);
}

#[test]
fn test_index_stats_single_entity() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![
            IndexedField::new("title".to_string(), "Article".to_string(), SearchType::Contains),
            IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
        ],
    ).unwrap();

    let stats = manager.get_index_stats();
    assert_eq!(stats.total_entities, 1);
    assert_eq!(stats.entities_by_type.get("content"), Some(&1));
    assert_eq!(stats.total_fields, 2);
}

#[test]
fn test_index_stats_multiple_types() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let content_id = id_gen.generate();
    let media_id = id_gen.generate();

    manager.index_entity(
        content_id,
        "content".to_string(),
        vec![IndexedField::new("title".to_string(), "Article".to_string(), SearchType::Contains)],
    ).unwrap();

    manager.index_entity(
        media_id,
        "media".to_string(),
        vec![IndexedField::new("filename".to_string(), "image.jpg".to_string(), SearchType::Contains)],
    ).unwrap();

    let stats = manager.get_index_stats();
    assert_eq!(stats.total_entities, 2);
    assert_eq!(stats.entities_by_type.get("content"), Some(&1));
    assert_eq!(stats.entities_by_type.get("media"), Some(&1));
    assert_eq!(stats.total_fields, 2);
}

#[test]
fn test_index_stats_multi_value_fields() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![
            IndexedField::new("title".to_string(), "Article".to_string(), SearchType::Contains),
            IndexedField::with_values(
                "tags".to_string(),
                vec!["rust".to_string(), "programming".to_string(), "systems".to_string()],
                SearchType::Exact,
            ),
        ],
    ).unwrap();

    let stats = manager.get_index_stats();
    assert_eq!(stats.total_entities, 1);
    // 1 champ simple + 3 valeurs dans le champ multi-valeurs = 4 champs au total
    assert_eq!(stats.total_fields, 4);
}

#[test]
fn test_search_result_metadata() {
    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    let entity_id = id_gen.generate();
    manager.index_entity(
        entity_id,
        "content".to_string(),
        vec![IndexedField::new(
            "title".to_string(),
            "Article".to_string(),
            SearchType::Contains,
        )],
    ).unwrap();

    let query = SearchQuery::with_entity_type("content".to_string())
        .with_logical_operator(LogicalOperator::And)
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::Contains,
            "Article".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    
    assert!(result.metadata.is_some());
    let metadata = result.metadata.unwrap();
    assert_eq!(metadata.criteria_count, 1);
    assert_eq!(metadata.entity_type, Some("content".to_string()));
    assert_eq!(metadata.logical_operator, LogicalOperator::And);
}
