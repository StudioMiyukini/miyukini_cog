//! Tests du Module Contenu.

use miyukini_kernel::IdGenerator;
use miyukini_spm_cms_content::{
    ContentFilters, ContentInput, ContentManager, ContentStatus, ContentUpdates,
};
#[cfg(feature = "memory")]
use miyukini_spm_cms_content::MemoryContentManager;

#[cfg(feature = "memory")]
#[test]
fn test_create_content() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::new("article".to_string(), b"metadata".to_vec());

    let id = manager.create_content(input).unwrap();

    let content = manager.get_content(id).unwrap();
    assert_eq!(content.content_type, "article");
    assert_eq!(content.status, ContentStatus::Draft);
    assert_eq!(content.metadata, b"metadata");
}

#[cfg(feature = "memory")]
#[test]
fn test_create_content_with_status() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::with_status(
        "page".to_string(),
        ContentStatus::Published,
        b"metadata".to_vec(),
    );

    let id = manager.create_content(input).unwrap();

    let content = manager.get_content(id).unwrap();
    assert_eq!(content.status, ContentStatus::Published);
}

#[cfg(feature = "memory")]
#[test]
fn test_get_content_not_found() {
    let manager = MemoryContentManager::new();
    let id = miyukini_kernel::UuidIdGenerator::new().generate();

    let result = manager.get_content(id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), miyukini_spm_cms_content::ContentError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_update_content() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::new("article".to_string(), b"old".to_vec());
    let id = manager.create_content(input).unwrap();

    let content_before = manager.get_content(id).unwrap();
    let updated_at_before = content_before.updated_at;

    // Attendre un peu pour que la date change
    std::thread::sleep(std::time::Duration::from_millis(10));

    let updates = ContentUpdates::new()
        .with_status(ContentStatus::Published)
        .with_metadata(b"new".to_vec());

    manager.update_content(id, updates, false).unwrap();

    let content_after = manager.get_content(id).unwrap();
    assert_eq!(content_after.status, ContentStatus::Published);
    assert_eq!(content_after.metadata, b"new");
    assert!(content_after.updated_at > updated_at_before);
}

#[cfg(feature = "memory")]
#[test]
fn test_delete_content_soft() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::new("article".to_string(), b"metadata".to_vec());
    let id = manager.create_content(input).unwrap();

    manager.delete_content(id, true).unwrap();

    let content = manager.get_content(id).unwrap();
    assert_eq!(content.status, ContentStatus::Archived);
}

#[cfg(feature = "memory")]
#[test]
fn test_delete_content_hard() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::new("article".to_string(), b"metadata".to_vec());
    let id = manager.create_content(input).unwrap();

    manager.delete_content(id, false).unwrap();

    let result = manager.get_content(id);
    assert!(result.is_err());
}

#[cfg(feature = "memory")]
#[test]
fn test_list_contents() {
    let manager = MemoryContentManager::new();

    // Créer plusieurs contenus
    let _id1 = manager
        .create_content(ContentInput::new("article".to_string(), b"meta1".to_vec()))
        .unwrap();
    let id2 = manager
        .create_content(ContentInput::with_status(
            "page".to_string(),
            ContentStatus::Published,
            b"meta2".to_vec(),
        ))
        .unwrap();
    let _id3 = manager
        .create_content(ContentInput::new("article".to_string(), b"meta3".to_vec()))
        .unwrap();

    // Lister tous les contenus
    let filters = ContentFilters::new();
    let result = manager.list_contents(filters, 0, 10).unwrap();
    assert_eq!(result.total, 3);
    assert_eq!(result.items.len(), 3);

    // Filtrer par type
    let filters = ContentFilters::new().with_content_type("article".to_string());
    let result = manager.list_contents(filters, 0, 10).unwrap();
    assert_eq!(result.total, 2);
    assert!(result.items.iter().all(|c| c.content_type == "article"));

    // Filtrer par statut
    let filters = ContentFilters::new().with_status(ContentStatus::Published);
    let result = manager.list_contents(filters, 0, 10).unwrap();
    assert_eq!(result.total, 1);
    assert_eq!(result.items[0].id, id2);
}

#[cfg(feature = "memory")]
#[test]
fn test_list_contents_pagination() {
    let manager = MemoryContentManager::new();

    // Créer 5 contenus
    for i in 0..5 {
        let _ = manager
            .create_content(ContentInput::new(
                format!("article{}", i),
                b"metadata".to_vec(),
            ))
            .unwrap();
    }

    // Pagination : offset 0, limit 2
    let filters = ContentFilters::new();
    let result = manager.list_contents(filters.clone(), 0, 2).unwrap();
    assert_eq!(result.total, 5);
    assert_eq!(result.items.len(), 2);
    assert_eq!(result.offset, 0);
    assert_eq!(result.limit, 2);

    // Pagination : offset 2, limit 2
    let result = manager.list_contents(filters, 2, 2).unwrap();
    assert_eq!(result.total, 5);
    assert_eq!(result.items.len(), 2);
}

#[cfg(feature = "memory")]
#[test]
fn test_relations() {
    let manager = MemoryContentManager::new();

    let id1 = manager
        .create_content(ContentInput::new("article".to_string(), b"meta1".to_vec()))
        .unwrap();
    let id2 = manager
        .create_content(ContentInput::new("article".to_string(), b"meta2".to_vec()))
        .unwrap();

    // Ajouter une relation
    manager
        .add_relation(id1, "parent".to_string(), id2)
        .unwrap();

    // Lister les relations
    let relations = manager.list_relations(id1).unwrap();
    assert_eq!(relations.len(), 1);
    assert_eq!(relations[0].relation_type, "parent");
    assert_eq!(relations[0].target_id, id2);

    // Supprimer la relation
    manager
        .remove_relation(id1, "parent".to_string(), id2)
        .unwrap();

    let relations = manager.list_relations(id1).unwrap();
    assert_eq!(relations.len(), 0);
}

#[cfg(feature = "memory")]
#[test]
fn test_relations_invalid_content() {
    let manager = MemoryContentManager::new();
    let id1 = manager
        .create_content(ContentInput::new("article".to_string(), b"meta1".to_vec()))
        .unwrap();
    let invalid_id = miyukini_kernel::UuidIdGenerator::new().generate();

    // Essayer d'ajouter une relation vers un contenu inexistant
    let result = manager.add_relation(id1, "parent".to_string(), invalid_id);
    assert!(result.is_err());
}

#[cfg(feature = "memory")]
#[test]
fn test_versioning() {
    let manager = MemoryContentManager::new();
    let id = manager
        .create_content(ContentInput::new("article".to_string(), b"v1".to_vec()))
        .unwrap();

    // Créer une version
    let version_id = manager.create_version(id).unwrap();

    // Lister les versions
    let versions = manager.list_versions(id).unwrap();
    assert_eq!(versions.len(), 1);
    assert_eq!(versions[0].version_id, version_id);

    // Modifier le contenu avec création de version
    let updates = ContentUpdates::new().with_metadata(b"v2".to_vec());
    manager.update_content(id, updates, true).unwrap();

    let versions = manager.list_versions(id).unwrap();
    assert_eq!(versions.len(), 2);
}

#[cfg(feature = "memory")]
#[test]
fn test_invariants_dates() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::new("article".to_string(), b"metadata".to_vec());
    let id = manager.create_content(input).unwrap();

    let content = manager.get_content(id).unwrap();
    // Invariant : updated_at >= created_at
    assert!(content.updated_at >= content.created_at);
}

#[cfg(feature = "memory")]
#[test]
fn test_invariants_status() {
    let manager = MemoryContentManager::new();
    let input = ContentInput::with_status(
        "article".to_string(),
        ContentStatus::Published,
        b"metadata".to_vec(),
    );
    let id = manager.create_content(input).unwrap();

    let content = manager.get_content(id).unwrap();
    // Invariant : statut valide
    assert!(content.status.is_valid());
}
