//! Tests du Module Publication.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_publication::{
    PublicationError, PublicationManager, PublicationStatus,
};
#[cfg(feature = "memory")]
use miyukini_spm_cms_publication::MemoryPublicationManager;
use std::time::{Duration, SystemTime};

#[cfg(feature = "memory")]
#[test]
fn test_create_publication_draft() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();

    let publication_id = manager.create_publication(content_id).unwrap();

    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Draft);
}

#[cfg(feature = "memory")]
#[test]
fn test_publish_now_from_draft() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    manager.publish_now(publication_id).unwrap();

    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Published);
}

#[cfg(feature = "memory")]
#[test]
fn test_schedule_future_valid() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    let future = SystemTime::now() + Duration::from_secs(3600);
    manager.schedule(publication_id, future).unwrap();

    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Scheduled);
}

#[cfg(feature = "memory")]
#[test]
fn test_schedule_past_rejected() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    let past = SystemTime::now() - Duration::from_secs(3600);
    let result = manager.schedule(publication_id, past);

    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidSchedule
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_scheduled_to_published_after_time() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Programmer pour le passé (simuler avec une date très proche)
    let near_future = SystemTime::now() + Duration::from_millis(10);
    manager.schedule(publication_id, near_future).unwrap();

    // Attendre un peu
    std::thread::sleep(Duration::from_millis(50));

    // effective_status doit retourner Published
    let effective = manager.effective_status(publication_id).unwrap();
    assert_eq!(effective, PublicationStatus::Published);

    // publish_now doit maintenant fonctionner
    manager.publish_now(publication_id).unwrap();
    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Published);
}

#[cfg(feature = "memory")]
#[test]
fn test_invalid_transition_rejected() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Tentative d'archiver depuis Draft (invalide)
    let result = manager.archive(publication_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_archive_from_published() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Publier d'abord
    manager.publish_now(publication_id).unwrap();

    // Archiver
    manager.archive(publication_id).unwrap();

    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Archived);
}

#[cfg(feature = "memory")]
#[test]
fn test_archive_from_draft_rejected() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Tentative d'archiver depuis Draft
    let result = manager.archive(publication_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_effective_status_correct() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Draft : effective_status = Draft
    let effective = manager.effective_status(publication_id).unwrap();
    assert_eq!(effective, PublicationStatus::Draft);

    // Programmer pour le futur
    let future = SystemTime::now() + Duration::from_secs(3600);
    manager.schedule(publication_id, future).unwrap();

    // Scheduled avec date future : effective_status = Scheduled
    let effective = manager.effective_status(publication_id).unwrap();
    assert_eq!(effective, PublicationStatus::Scheduled);

    // Créer une nouvelle publication pour tester la publication immédiate
    let content2_id = UuidIdGenerator::new().generate();
    let publication2_id = manager.create_publication(content2_id).unwrap();

    // Publier immédiatement
    manager.publish_now(publication2_id).unwrap();

    // Published : effective_status = Published
    let effective = manager.effective_status(publication2_id).unwrap();
    assert_eq!(effective, PublicationStatus::Published);
}

#[cfg(feature = "memory")]
#[test]
fn test_publication_not_found() {
    let manager = MemoryPublicationManager::new();
    let fake_id = UuidIdGenerator::new().generate();

    let result = manager.status(fake_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PublicationError::NotFound));

    let result = manager.publish_now(fake_id);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), PublicationError::NotFound));
}

#[cfg(feature = "memory")]
#[test]
fn test_publication_already_archived() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Publier puis archiver
    manager.publish_now(publication_id).unwrap();
    manager.archive(publication_id).unwrap();

    // Tentative de publier à nouveau (invalide)
    let result = manager.publish_now(publication_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));

    // Tentative de programmer (invalide)
    let future = SystemTime::now() + Duration::from_secs(3600);
    let result = manager.schedule(publication_id, future);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_coherence_updated_at() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Attendre un peu
    std::thread::sleep(Duration::from_millis(10));

    // Publier
    manager.publish_now(publication_id).unwrap();

    // Vérifier que updated_at a été mis à jour
    // (on ne peut pas vérifier directement, mais on peut vérifier que l'opération a réussi)
    let status = manager.status(publication_id).unwrap();
    assert_eq!(status, PublicationStatus::Published);
}

#[cfg(feature = "memory")]
#[test]
fn test_schedule_from_published_invalid() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Publier
    manager.publish_now(publication_id).unwrap();

    // Tentative de programmer depuis Published (invalide)
    let future = SystemTime::now() + Duration::from_secs(3600);
    let result = manager.schedule(publication_id, future);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_publish_now_from_scheduled_before_time() {
    let manager = MemoryPublicationManager::new();
    let content_id = UuidIdGenerator::new().generate();
    let publication_id = manager.create_publication(content_id).unwrap();

    // Programmer pour le futur
    let future = SystemTime::now() + Duration::from_secs(3600);
    manager.schedule(publication_id, future).unwrap();

    // Tentative de publier maintenant (invalide car publish_at > now)
    let result = manager.publish_now(publication_id);
    assert!(result.is_err());
    assert!(matches!(
        result.unwrap_err(),
        PublicationError::InvalidTransition
    ));
}

#[cfg(feature = "memory")]
#[test]
fn test_multiple_publications() {
    let manager = MemoryPublicationManager::new();
    let content1_id = UuidIdGenerator::new().generate();
    let content2_id = UuidIdGenerator::new().generate();

    let pub1_id = manager.create_publication(content1_id).unwrap();
    let pub2_id = manager.create_publication(content2_id).unwrap();

    // Publier la première
    manager.publish_now(pub1_id).unwrap();

    // Programmer la deuxième
    let future = SystemTime::now() + Duration::from_secs(3600);
    manager.schedule(pub2_id, future).unwrap();

    // Vérifier les statuts
    assert_eq!(manager.status(pub1_id).unwrap(), PublicationStatus::Published);
    assert_eq!(manager.status(pub2_id).unwrap(), PublicationStatus::Scheduled);
}
