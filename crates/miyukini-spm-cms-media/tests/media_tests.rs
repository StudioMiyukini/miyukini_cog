//! Tests du Module Médias.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_media::{MediaError, MediaInput, MediaManager};
#[cfg(feature = "memory")]
use miyukini_spm_cms_media::MemoryMediaManager;

#[cfg(feature = "memory")]
#[test]
fn test_create_media() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());

    let id = manager.create_media(input).unwrap();

    let media = manager.get_media(id).unwrap();
    assert_eq!(media.mime_type, "image/jpeg");
    assert_eq!(media.metadata, b"metadata");
}

#[cfg(feature = "memory")]
#[test]
fn test_get_media_not_found() {
    let manager = MemoryMediaManager::new();
    let id = UuidIdGenerator::new().generate();

    let result = manager.get_media(id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MediaError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_delete_media() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/png".to_string(), b"metadata".to_vec());
    let id = manager.create_media(input).unwrap();

    manager.delete_media(id).unwrap();

    let result = manager.get_media(id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MediaError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_delete_media_not_found() {
    let manager = MemoryMediaManager::new();
    let id = UuidIdGenerator::new().generate();

    let result = manager.delete_media(id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MediaError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_attach_media_to_entity() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
    let media_id = manager.create_media(input).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    manager.attach_media_to_entity(media_id, entity_id).unwrap();

    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 1);
    assert_eq!(media_list[0].id, media_id);
}

#[cfg(feature = "memory")]
#[test]
fn test_attach_media_to_entity_media_not_found() {
    let manager = MemoryMediaManager::new();
    let media_id = UuidIdGenerator::new().generate();
    let entity_id = UuidIdGenerator::new().generate();

    let result = manager.attach_media_to_entity(media_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MediaError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_detach_media_from_entity() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("video/mp4".to_string(), b"metadata".to_vec());
    let media_id = manager.create_media(input).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    manager.attach_media_to_entity(media_id, entity_id).unwrap();
    manager.detach_media_from_entity(media_id, entity_id).unwrap();

    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 0);
}

#[cfg(feature = "memory")]
#[test]
fn test_detach_media_from_entity_not_attached() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("application/pdf".to_string(), b"metadata".to_vec());
    let media_id = manager.create_media(input).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    let result = manager.detach_media_from_entity(media_id, entity_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), MediaError::EntityNotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_list_media_for_entity() {
    let manager = MemoryMediaManager::new();
    let entity_id = UuidIdGenerator::new().generate();

    // Créer plusieurs médias
    let media1_id = manager
        .create_media(MediaInput::new("image/jpeg".to_string(), b"meta1".to_vec()))
        .unwrap();
    let media2_id = manager
        .create_media(MediaInput::new("image/png".to_string(), b"meta2".to_vec()))
        .unwrap();
    let media3_id = manager
        .create_media(MediaInput::new("video/mp4".to_string(), b"meta3".to_vec()))
        .unwrap();

    // Attacher media1 et media2 à l'entité
    manager.attach_media_to_entity(media1_id, entity_id).unwrap();
    manager.attach_media_to_entity(media2_id, entity_id).unwrap();

    // media3 n'est pas attaché
    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 2);
    let media_ids: Vec<_> = media_list.iter().map(|m| m.id).collect();
    assert!(media_ids.contains(&media1_id));
    assert!(media_ids.contains(&media2_id));
    assert!(!media_ids.contains(&media3_id));
}

#[cfg(feature = "memory")]
#[test]
fn test_list_media_for_entity_empty() {
    let manager = MemoryMediaManager::new();
    let entity_id = UuidIdGenerator::new().generate();

    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 0);
}

#[cfg(feature = "memory")]
#[test]
fn test_attach_multiple_media_to_entity() {
    let manager = MemoryMediaManager::new();
    let entity_id = UuidIdGenerator::new().generate();

    // Créer et attacher plusieurs médias à la même entité
    let media1_id = manager
        .create_media(MediaInput::new("image/jpeg".to_string(), b"meta1".to_vec()))
        .unwrap();
    let media2_id = manager
        .create_media(MediaInput::new("image/png".to_string(), b"meta2".to_vec()))
        .unwrap();
    let media3_id = manager
        .create_media(MediaInput::new("image/gif".to_string(), b"meta3".to_vec()))
        .unwrap();

    manager.attach_media_to_entity(media1_id, entity_id).unwrap();
    manager.attach_media_to_entity(media2_id, entity_id).unwrap();
    manager.attach_media_to_entity(media3_id, entity_id).unwrap();

    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 3);
}

#[cfg(feature = "memory")]
#[test]
fn test_attach_same_media_to_multiple_entities() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
    let media_id = manager.create_media(input).unwrap();
    let entity1_id = UuidIdGenerator::new().generate();
    let entity2_id = UuidIdGenerator::new().generate();

    manager.attach_media_to_entity(media_id, entity1_id).unwrap();
    manager.attach_media_to_entity(media_id, entity2_id).unwrap();

    let media_list1 = manager.list_media_for_entity(entity1_id).unwrap();
    let media_list2 = manager.list_media_for_entity(entity2_id).unwrap();
    assert_eq!(media_list1.len(), 1);
    assert_eq!(media_list2.len(), 1);
    assert_eq!(media_list1[0].id, media_id);
    assert_eq!(media_list2[0].id, media_id);
}

#[cfg(feature = "memory")]
#[test]
fn test_delete_media_removes_attachments() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
    let media_id = manager.create_media(input).unwrap();
    let entity_id = UuidIdGenerator::new().generate();

    manager.attach_media_to_entity(media_id, entity_id).unwrap();

    // Vérifier que le média est attaché
    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 1);

    // Supprimer le média
    manager.delete_media(media_id).unwrap();

    // Vérifier que le média n'est plus attaché
    let media_list = manager.list_media_for_entity(entity_id).unwrap();
    assert_eq!(media_list.len(), 0);
}

#[cfg(feature = "memory")]
#[test]
fn test_invariants_dates() {
    let manager = MemoryMediaManager::new();
    let input = MediaInput::new("image/jpeg".to_string(), b"metadata".to_vec());
    let id = manager.create_media(input).unwrap();

    let media = manager.get_media(id).unwrap();
    // Invariant : updated_at >= created_at
    assert!(media.updated_at >= media.created_at);
}

#[cfg(feature = "memory")]
#[test]
fn test_multiple_entities_multiple_media() {
    let manager = MemoryMediaManager::new();
    let entity1_id = UuidIdGenerator::new().generate();
    let entity2_id = UuidIdGenerator::new().generate();

    // Créer des médias pour entity1
    let media1_id = manager
        .create_media(MediaInput::new("image/jpeg".to_string(), b"meta1".to_vec()))
        .unwrap();
    let media2_id = manager
        .create_media(MediaInput::new("image/png".to_string(), b"meta2".to_vec()))
        .unwrap();

    // Créer des médias pour entity2
    let media3_id = manager
        .create_media(MediaInput::new("video/mp4".to_string(), b"meta3".to_vec()))
        .unwrap();

    // Attacher les médias
    manager.attach_media_to_entity(media1_id, entity1_id).unwrap();
    manager.attach_media_to_entity(media2_id, entity1_id).unwrap();
    manager.attach_media_to_entity(media3_id, entity2_id).unwrap();

    // Vérifier les listes
    let media_list1 = manager.list_media_for_entity(entity1_id).unwrap();
    let media_list2 = manager.list_media_for_entity(entity2_id).unwrap();
    assert_eq!(media_list1.len(), 2);
    assert_eq!(media_list2.len(), 1);
    assert_eq!(media_list1[0].id, media1_id);
    assert_eq!(media_list1[1].id, media2_id);
    assert_eq!(media_list2[0].id, media3_id);
}
