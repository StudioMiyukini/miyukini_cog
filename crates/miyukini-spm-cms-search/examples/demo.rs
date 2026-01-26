//! Démo du module Recherche & Indexation.
//!
//! Cette démo illustre les capacités du module :
//! - Indexation d'entités (contenus, médias)
//! - Recherche par critères simples
//! - Recherche combinée (ET, OU)
//! - Pagination
//! - Nouveaux opérateurs (StartsWith, EndsWith, Between, In)
//! - Champs multi-valeurs
//! - Statistiques d'index

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_search::{
    IndexedField, LogicalOperator, MemorySearchManager, SearchCriterion, SearchManager,
    SearchOperator, SearchQuery, SearchType,
};

fn main() {
    println!("=== Démo Module Recherche & Indexation ===\n");

    let manager = MemorySearchManager::new();
    let id_gen = UuidIdGenerator::new();

    // === Indexation de contenus ===
    println!("1. Indexation de contenus\n");

    let article1_id = id_gen.generate();
    let article2_id = id_gen.generate();
    let page1_id = id_gen.generate();

    manager
        .index_entity(
            article1_id,
            "content".to_string(),
            vec![
                IndexedField::new("title".to_string(), "Introduction à Rust".to_string(), SearchType::Contains),
                IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
                IndexedField::new("content_type".to_string(), "article".to_string(), SearchType::Exact),
            ],
        )
        .unwrap();

    manager
        .index_entity(
            article2_id,
            "content".to_string(),
            vec![
                IndexedField::new("title".to_string(), "Guide Python avancé".to_string(), SearchType::Contains),
                IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
                IndexedField::new("content_type".to_string(), "article".to_string(), SearchType::Exact),
            ],
        )
        .unwrap();

    manager
        .index_entity(
            page1_id,
            "content".to_string(),
            vec![
                IndexedField::new("title".to_string(), "Page d'accueil".to_string(), SearchType::Contains),
                IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
                IndexedField::new("content_type".to_string(), "page".to_string(), SearchType::Exact),
            ],
        )
        .unwrap();

    println!("✓ 3 contenus indexés\n");

    // === Indexation de médias ===
    println!("2. Indexation de médias\n");

    let image1_id = id_gen.generate();
    let video1_id = id_gen.generate();

    manager
        .index_entity(
            image1_id,
            "media".to_string(),
            vec![
                IndexedField::new("mime_type".to_string(), "image/jpeg".to_string(), SearchType::Exact),
                IndexedField::new("filename".to_string(), "photo.jpg".to_string(), SearchType::Contains),
            ],
        )
        .unwrap();

    manager
        .index_entity(
            video1_id,
            "media".to_string(),
            vec![
                IndexedField::new("mime_type".to_string(), "video/mp4".to_string(), SearchType::Exact),
                IndexedField::new("filename".to_string(), "demo.mp4".to_string(), SearchType::Contains),
            ],
        )
        .unwrap();

    println!("✓ 2 médias indexés\n");

    // === Recherche simple : articles publiés ===
    println!("3. Recherche : articles publiés\n");

    let query = SearchQuery::with_entity_type("content".to_string())
        .with_logical_operator(LogicalOperator::And)
        .add_criterion(SearchCriterion::new(
            "content_type".to_string(),
            SearchOperator::Equals,
            "article".to_string(),
        ))
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Résultats : {} article(s) trouvé(s)", result.total);
    for (i, id) in result.entity_ids.iter().enumerate() {
        println!("    {}. {}", i + 1, id);
    }
    println!();

    // === Recherche par titre (contient) ===
    println!("4. Recherche : titre contient 'Rust'\n");

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::Contains,
            "Rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Résultats : {} contenu(s) trouvé(s)", result.total);
    for (i, id) in result.entity_ids.iter().enumerate() {
        println!("    {}. {}", i + 1, id);
    }
    println!();

    // === Recherche OU : articles OU pages ===
    println!("5. Recherche : articles OU pages (statut publié)\n");

    let query = SearchQuery::with_entity_type("content".to_string())
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

    let result1 = manager.search(query.clone(), 0, 10).unwrap();

    let query2 = SearchQuery::with_entity_type("content".to_string())
        .with_logical_operator(LogicalOperator::And)
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ))
        .add_criterion(SearchCriterion::new(
            "content_type".to_string(),
            SearchOperator::Equals,
            "page".to_string(),
        ));

    let result2 = manager.search(query2, 0, 10).unwrap();

    println!("  Articles publiés : {}", result1.total);
    println!("  Pages publiées : {}", result2.total);
    println!();

    // === Recherche de médias par type MIME ===
    println!("6. Recherche : médias image/jpeg\n");

    let query = SearchQuery::with_entity_type("media".to_string())
        .add_criterion(SearchCriterion::new(
            "mime_type".to_string(),
            SearchOperator::Equals,
            "image/jpeg".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Résultats : {} média(x) trouvé(s)", result.total);
    for (i, id) in result.entity_ids.iter().enumerate() {
        println!("    {}. {}", i + 1, id);
    }
    println!();

    // === Liste des entités indexées ===
    println!("7. Liste des entités indexées (contenus)\n");

    let result = manager.list_indexed_entities(Some("content".to_string()), 0, 10).unwrap();
    println!("  Total : {} entité(s) indexée(s)", result.total);
    println!();

    // === Pagination ===
    println!("8. Pagination (offset=0, limit=2)\n");

    let query = SearchQuery::with_entity_type("content".to_string())
        .add_criterion(SearchCriterion::new(
            "status".to_string(),
            SearchOperator::Equals,
            "published".to_string(),
        ));

    let result = manager.search(query, 0, 2).unwrap();
    println!("  Total : {} résultat(s)", result.total);
    println!("  Page 1 : {} résultat(s) (offset={}, limit={})", result.entity_ids.len(), result.offset, result.limit);
    println!();

    // === Indexation multi-valeurs ===
    println!("9. Indexation multi-valeurs (tags)\n");

    let article3_id = id_gen.generate();
    manager
        .index_entity(
            article3_id,
            "content".to_string(),
            vec![
                IndexedField::new("title".to_string(), "Article sur Rust et Python".to_string(), SearchType::Contains),
                IndexedField::new("status".to_string(), "published".to_string(), SearchType::Exact),
                IndexedField::with_values(
                    "tags".to_string(),
                    vec!["rust".to_string(), "python".to_string(), "programming".to_string()],
                    SearchType::Exact,
                ),
            ],
        )
        .unwrap();

    println!("✓ Article indexé avec tags multi-valeurs\n");

    // Recherche avec un tag
    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "tags".to_string(),
            SearchOperator::Equals,
            "rust".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Articles avec tag 'rust' : {}", result.total);
    println!();

    // === Recherche avec Between ===
    println!("10. Recherche avec Between (plage de prix)\n");

    let product1_id = id_gen.generate();
    let product2_id = id_gen.generate();
    let product3_id = id_gen.generate();

    manager
        .index_entity(
            product1_id,
            "product".to_string(),
            vec![IndexedField::new("price".to_string(), "050".to_string(), SearchType::Exact)],
        )
        .unwrap();

    manager
        .index_entity(
            product2_id,
            "product".to_string(),
            vec![IndexedField::new("price".to_string(), "100".to_string(), SearchType::Exact)],
        )
        .unwrap();

    manager
        .index_entity(
            product3_id,
            "product".to_string(),
            vec![IndexedField::new("price".to_string(), "200".to_string(), SearchType::Exact)],
        )
        .unwrap();

    let query = SearchQuery::with_entity_type("product".to_string())
        .add_criterion(SearchCriterion::with_values(
            "price".to_string(),
            SearchOperator::Between,
            vec!["075".to_string(), "150".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Produits entre 75 et 150 : {} trouvé(s)", result.total);
    println!();

    // === Recherche avec In ===
    println!("11. Recherche avec In (liste de statuts)\n");

    let query = SearchQuery::with_entity_type("content".to_string())
        .add_criterion(SearchCriterion::with_values(
            "status".to_string(),
            SearchOperator::In,
            vec!["published".to_string(), "draft".to_string()],
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Contenus avec statut 'published' ou 'draft' : {} trouvé(s)", result.total);
    println!();

    // === Recherche avec StartsWith et EndsWith ===
    println!("12. Recherche avec StartsWith et EndsWith\n");

    let query = SearchQuery::new()
        .add_criterion(SearchCriterion::new(
            "title".to_string(),
            SearchOperator::StartsWith,
            "Introduction".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Titres commençant par 'Introduction' : {} trouvé(s)", result.total);

    let query = SearchQuery::with_entity_type("media".to_string())
        .add_criterion(SearchCriterion::new(
            "filename".to_string(),
            SearchOperator::EndsWith,
            ".jpg".to_string(),
        ));

    let result = manager.search(query, 0, 10).unwrap();
    println!("  Fichiers se terminant par '.jpg' : {} trouvé(s)", result.total);
    println!();

    // === Statistiques d'index ===
    println!("13. Statistiques d'index\n");

    let stats = manager.get_index_stats();
    println!("  Total d'entités indexées : {}", stats.total_entities);
    println!("  Total de champs indexés : {}", stats.total_fields);
    println!("  Entités par type :");
    for (entity_type, count) in &stats.entities_by_type {
        println!("    - {} : {}", entity_type, count);
    }
    println!();

    println!("=== Fin de la démo ===");
}
