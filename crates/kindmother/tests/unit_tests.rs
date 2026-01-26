//! Tests unitaires complets pour KindMother
//!
//! Couverture complète de toutes les fonctionnalités :
//! - États KM (Booting, Healthy, Degraded, Quarantined)
//! - Boundaries (état, contexte, permissions, WriteIntent, sync, contournement, charge)
//! - Types d'intentions (WriteIntent, SyncIntent)
//! - Types de conflits (autoritaire, temporel, sémantique)
//! - Rejets contractuels
//! - Détection de corruption
//! - Détection de menaces

use kindmother::*;

// =============================================================================
// Tests des états KM
// =============================================================================

mod km_state_tests {
    use super::*;

    #[test]
    fn test_km_initial_state_is_booting() {
        let km = KindMother::new();
        assert_eq!(km.state(), KMState::Booting);
    }

    #[test]
    fn test_km_with_identity() {
        let km = KindMother::with_identity("test-instance".to_string(), "test-domain".to_string());
        assert_eq!(km.instance_id(), "test-instance");
        assert_eq!(km.domain_id(), "test-domain");
        assert_eq!(km.state(), KMState::Booting);
    }

    #[test]
    fn test_transition_to_healthy_from_booting() {
        let mut km = KindMother::new();
        assert!(km.recover_to_healthy("Démarrage réussi").is_ok());
        assert_eq!(km.state(), KMState::Healthy);
    }

    #[test]
    fn test_transition_to_degraded_from_healthy() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        assert!(km.transition_to_degraded("Corruption détectée").is_ok());
        assert_eq!(km.state(), KMState::Degraded);
    }

    #[test]
    fn test_transition_to_quarantined() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        assert!(km.transition_to_quarantined("Menace grave").is_ok());
        assert_eq!(km.state(), KMState::Quarantined);
    }

    #[test]
    fn test_cannot_transition_from_quarantined_to_degraded() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        km.transition_to_quarantined("Menace").unwrap();
        
        assert!(km.transition_to_degraded("Tentative").is_err());
        assert_eq!(km.state(), KMState::Quarantined);
    }

    #[test]
    fn test_lift_quarantine() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        km.transition_to_quarantined("Menace").unwrap();
        
        assert!(km.lift_quarantine("Problème résolu").is_ok());
        assert_eq!(km.state(), KMState::Degraded);
    }

    #[test]
    fn test_recovery_from_degraded() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        km.transition_to_degraded("Problème").unwrap();
        
        assert!(km.recover_to_healthy("Récupération").is_ok());
        assert_eq!(km.state(), KMState::Healthy);
    }

    #[test]
    fn test_cannot_recover_from_quarantined() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        km.transition_to_quarantined("Menace").unwrap();
        
        assert!(km.recover_to_healthy("Tentative").is_err());
        assert_eq!(km.state(), KMState::Quarantined);
    }
}

// =============================================================================
// Tests du cycle de vie des WriteIntent
// =============================================================================

mod write_intent_lifecycle_tests {
    use super::*;

    #[test]
    fn test_write_intent_initial_state() {
        let intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        assert_eq!(intent.state(), WriteIntentState::Created);
        assert_eq!(intent.intent_id(), "intent-1");
        assert_eq!(intent.operation_type(), "create");
    }

    #[test]
    fn test_write_intent_full_lifecycle() {
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        // CRÉÉE → EN_VALIDATION
        assert!(intent.start_validation().is_ok());
        assert_eq!(intent.state(), WriteIntentState::InValidation);
        
        // EN_VALIDATION → ACCEPTÉE
        assert!(intent.accept().is_ok());
        assert_eq!(intent.state(), WriteIntentState::Accepted);
        
        // ACCEPTÉE → APPLIQUÉE
        assert!(intent.apply().is_ok());
        assert_eq!(intent.state(), WriteIntentState::Applied);
        
        // APPLIQUÉE → ARCHIVÉE
        assert!(intent.archive().is_ok());
        assert_eq!(intent.state(), WriteIntentState::Archived);
    }

    #[test]
    fn test_write_intent_rejection_lifecycle() {
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        intent.start_validation().unwrap();
        
        // EN_VALIDATION → REJETÉE
        assert!(intent.reject("Raison du rejet".to_string()).is_ok());
        assert_eq!(intent.state(), WriteIntentState::Rejected);
        
        // REJETÉE → ARCHIVÉE
        assert!(intent.archive().is_ok());
        assert_eq!(intent.state(), WriteIntentState::Archived);
    }

    #[test]
    fn test_write_intent_invalid_transitions() {
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        // Tentative de transition directe CRÉÉE → ACCEPTÉE (invalide)
        assert!(intent.accept().is_err());
        
        // Tentative de transition directe CRÉÉE → APPLIQUÉE (invalide)
        assert!(intent.apply().is_err());
        
        // Tentative de transition directe CRÉÉE → ARCHIVÉE (invalide)
        assert!(intent.archive().is_err());
    }

    #[test]
    fn test_write_intent_cannot_reuse() {
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        assert!(intent.can_reuse());
        
        intent.start_validation().unwrap();
        assert!(!intent.can_reuse());
    }

    #[test]
    fn test_write_intent_is_terminal() {
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        intent.start_validation().unwrap();
        intent.reject("Rejet".to_string()).unwrap();
        
        assert!(intent.is_terminal());
    }
}

// =============================================================================
// Tests de SyncIntent
// =============================================================================

mod sync_intent_tests {
    use super::*;

    #[test]
    fn test_sync_intent_creation_daughter() {
        let result = SyncIntent::new(
            "sync-1".to_string(),
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        );
        assert!(result.is_ok());
        
        let intent = result.unwrap();
        assert_eq!(intent.sync_id(), "sync-1");
        assert_eq!(intent.state(), SyncIntentState::Pending);
    }

    #[test]
    fn test_sync_intent_creation_mother_rejected() {
        let result = SyncIntent::new(
            "sync-1".to_string(),
            "mother-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Mother,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_sync_intent_submission() {
        let mut intent = SyncIntent::new(
            "sync-1".to_string(),
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        ).unwrap();
        
        assert!(intent.submit().is_ok());
        assert_eq!(intent.state(), SyncIntentState::Submitted);
    }

    #[test]
    fn test_sync_intent_rejection_with_conflict() {
        let mut intent = SyncIntent::new(
            "sync-1".to_string(),
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        ).unwrap();
        
        intent.submit().unwrap();
        
        let conflict = Conflict::new(
            "conflict-1".to_string(),
            ConflictType::Authoritative,
            "intent-1".to_string(),
            "Conflit autoritaire".to_string(),
            1,
        );
        
        assert!(intent.reject_with_conflict(conflict).is_ok());
        assert_eq!(intent.state(), SyncIntentState::Rejected);
        assert!(intent.detected_conflict().is_some());
    }

    #[test]
    fn test_sync_manager_always_rejects() {
        let mut manager = SyncManager::new();
        let mut intent = manager.create_sync_intent(
            "daughter-1".to_string(),
            "mother-1".to_string(),
            InstanceType::Daughter,
        ).unwrap();
        
        assert!(manager.submit_sync_intent(&mut intent).is_err());
        assert_eq!(intent.state(), SyncIntentState::Rejected);
    }
}

// =============================================================================
// Tests de détection de conflits
// =============================================================================

mod conflict_detection_tests {
    use super::*;

    #[test]
    fn test_detect_authoritative_conflict() {
        let mut detector = ConflictDetector::new();
        
        let conflict = detector.detect_authoritative_conflict(
            "intent-1",
            "Entité supprimée par la Mère",
        );
        
        assert_eq!(conflict.conflict_type(), ConflictType::Authoritative);
        assert_eq!(conflict.intent_id(), "intent-1");
    }

    #[test]
    fn test_detect_temporal_conflict() {
        let mut detector = ConflictDetector::new();
        
        let conflict = detector.detect_temporal_conflict(
            "intent-1",
            "Modification concurrente",
        );
        
        assert_eq!(conflict.conflict_type(), ConflictType::Temporal);
    }

    #[test]
    fn test_detect_semantic_conflict() {
        let mut detector = ConflictDetector::new();
        
        let conflict = detector.detect_semantic_conflict(
            "intent-1",
            "Violation de contrainte",
        );
        
        assert_eq!(conflict.conflict_type(), ConflictType::Semantic);
    }

    #[test]
    fn test_type_conflict_priority() {
        let mut detector = ConflictDetector::new();
        
        // Autoritaire a priorité
        let conflict = detector.type_conflict(
            "intent-1",
            true,   // has_mother_decision
            true,   // has_concurrent_modification
            true,   // has_constraint_violation
            "Test",
        );
        assert!(conflict.is_some());
        assert_eq!(conflict.unwrap().conflict_type(), ConflictType::Authoritative);
        
        // Temporel si pas autoritaire
        let conflict = detector.type_conflict(
            "intent-2",
            false,  // has_mother_decision
            true,   // has_concurrent_modification
            true,   // has_constraint_violation
            "Test",
        );
        assert!(conflict.is_some());
        assert_eq!(conflict.unwrap().conflict_type(), ConflictType::Temporal);
        
        // Sémantique si aucun autre
        let conflict = detector.type_conflict(
            "intent-3",
            false,  // has_mother_decision
            false,  // has_concurrent_modification
            true,   // has_constraint_violation
            "Test",
        );
        assert!(conflict.is_some());
        assert_eq!(conflict.unwrap().conflict_type(), ConflictType::Semantic);
        
        // None si aucun conflit
        let conflict = detector.type_conflict(
            "intent-4",
            false,  // has_mother_decision
            false,  // has_concurrent_modification
            false,  // has_constraint_violation
            "Test",
        );
        assert!(conflict.is_none());
    }
}

// =============================================================================
// Tests de détection de menaces
// =============================================================================

mod threat_detection_tests {
    use super::*;

    #[test]
    fn test_detect_replay() {
        let mut detector = ThreatDetector::new();
        
        // Première fois - pas de replay
        assert!(detector.detect_replay("intent-1", "user-1").is_none());
        
        // Marquer comme traité
        detector.mark_intent_processed("intent-1");
        
        // Deuxième fois - replay détecté
        let threat = detector.detect_replay("intent-1", "user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Replay);
    }

    #[test]
    fn test_detect_resubmission() {
        let mut detector = ThreatDetector::new();
        
        // Marquer comme rejeté
        detector.mark_intent_rejected("intent-1");
        
        // Tentative de resoumission
        let threat = detector.detect_resubmission("intent-1", "user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Resubmission);
    }

    #[test]
    fn test_detect_saturation() {
        let config = ThreatDetectorConfig {
            saturation_threshold: 3,
            brute_force_threshold: 10,
            counting_period: 60,
        };
        let mut detector = ThreatDetector::with_config(config);
        
        // 3 appels - pas de saturation
        for _ in 0..3 {
            assert!(detector.detect_saturation("user-1").is_none());
        }
        
        // 4ème appel - saturation détectée
        let threat = detector.detect_saturation("user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::Saturation);
    }

    #[test]
    fn test_detect_brute_force() {
        let config = ThreatDetectorConfig {
            saturation_threshold: 100,
            brute_force_threshold: 2,
            counting_period: 60,
        };
        let mut detector = ThreatDetector::with_config(config);
        
        // 2 rejets - pas de brute-force
        for _ in 0..2 {
            assert!(detector.record_rejection_and_detect_brute_force("user-1").is_none());
        }
        
        // 3ème rejet - brute-force détecté
        let threat = detector.record_rejection_and_detect_brute_force("user-1");
        assert!(threat.is_some());
        assert_eq!(threat.unwrap().threat_type(), ThreatType::BruteForce);
    }

    #[test]
    fn test_reset_rejection_count() {
        let config = ThreatDetectorConfig {
            saturation_threshold: 100,
            brute_force_threshold: 2,
            counting_period: 60,
        };
        let mut detector = ThreatDetector::with_config(config);
        
        // 2 rejets
        detector.record_rejection_and_detect_brute_force("user-1");
        detector.record_rejection_and_detect_brute_force("user-1");
        
        // Reset
        detector.reset_rejection_count("user-1");
        
        // Recommencer - ne devrait pas être brute-force
        assert!(detector.record_rejection_and_detect_brute_force("user-1").is_none());
    }

    #[test]
    fn test_threat_severity_ordering() {
        assert!(ThreatSeverity::Low < ThreatSeverity::Medium);
        assert!(ThreatSeverity::Medium < ThreatSeverity::High);
        assert!(ThreatSeverity::High < ThreatSeverity::Critical);
    }
}

// =============================================================================
// Tests des Runtime Boundaries
// =============================================================================

mod runtime_boundary_tests {
    use super::*;
    use kindmother::runtime::RuntimeBoundary;

    #[test]
    fn test_km_state_boundary_healthy() {
        let result = RuntimeBoundary::check_km_state_boundary(
            KMState::Healthy,
            "test_operation",
            true, // is_write
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_km_state_boundary_degraded_read_ok() {
        let result = RuntimeBoundary::check_km_state_boundary(
            KMState::Degraded,
            "test_operation",
            false, // is_write = false (lecture)
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_km_state_boundary_degraded_write_rejected() {
        let result = RuntimeBoundary::check_km_state_boundary(
            KMState::Degraded,
            "test_operation",
            true, // is_write = true (écriture)
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_km_state_boundary_quarantined_rejected() {
        let result = RuntimeBoundary::check_km_state_boundary(
            KMState::Quarantined,
            "test_operation",
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_km_state_boundary_booting_rejected() {
        let result = RuntimeBoundary::check_km_state_boundary(
            KMState::Booting,
            "test_operation",
            false,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_context_boundary_valid() {
        let authority = AuthorityContext::with_permissions(
            "user-1".to_string(),
            vec![],
            true, true, false,
        );
        let instance = InstanceContext::new("instance-1".to_string(), InstanceType::Daughter);
        let domain = DomainContext::new("domain-1".to_string(), "TestDomain".to_string());
        
        let result = RuntimeBoundary::check_context_boundary(&authority, &instance, &domain);
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_boundary_empty_instance_id() {
        let authority = AuthorityContext::with_permissions(
            "user-1".to_string(),
            vec![],
            true, true, false,
        );
        let instance = InstanceContext::new("".to_string(), InstanceType::Daughter);
        let domain = DomainContext::new("domain-1".to_string(), "TestDomain".to_string());
        
        let result = RuntimeBoundary::check_context_boundary(&authority, &instance, &domain);
        assert!(result.is_err());
    }

    #[test]
    fn test_permissions_boundary_read_with_can_read() {
        let authority = AuthorityContext::with_permissions(
            "user-1".to_string(),
            vec![],
            true,   // can_read
            false,  // can_write
            false,  // can_sync
        );
        
        let result = RuntimeBoundary::check_permissions_boundary(&authority, "read", false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_permissions_boundary_write_without_can_write() {
        let authority = AuthorityContext::with_permissions(
            "user-1".to_string(),
            vec![],
            true,   // can_read
            false,  // can_write
            false,  // can_sync
        );
        
        let result = RuntimeBoundary::check_permissions_boundary(&authority, "write", true);
        assert!(result.is_err());
    }

    #[test]
    fn test_permissions_boundary_sync_always_rejected() {
        let authority = AuthorityContext::with_permissions(
            "user-1".to_string(),
            vec![],
            true,   // can_read
            true,   // can_write
            true,   // can_sync (even with this, sync is rejected)
        );
        
        let result = RuntimeBoundary::check_permissions_boundary(&authority, "sync_data", true);
        assert!(result.is_err());
    }
}

// =============================================================================
// Tests de persistance
// =============================================================================

mod persistence_tests {
    use super::*;

    #[test]
    fn test_persist_applied_intent() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        intent.start_validation().unwrap();
        intent.accept().unwrap();
        intent.apply().unwrap();
        
        assert!(km.persist_intent(intent, "domain-1").is_ok());
        assert_eq!(km.count_persisted_intents("domain-1"), 1);
    }

    #[test]
    fn test_persist_non_applied_intent_rejected() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        // Intention non appliquée
        let intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        assert!(km.persist_intent(intent, "domain-1").is_err());
        assert_eq!(km.count_persisted_intents("domain-1"), 0);
    }

    #[test]
    fn test_persist_rejected_intent_rejected() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        let mut intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        intent.start_validation().unwrap();
        intent.reject("Rejet".to_string()).unwrap();
        
        assert!(km.persist_intent(intent, "domain-1").is_err());
        assert_eq!(km.count_persisted_intents("domain-1"), 0);
    }

    #[test]
    fn test_corruption_detection() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        
        // Simuler un crash
        km.simulate_storage_crash();
        
        // La prochaine vérification devrait détecter la corruption
        assert!(!km.verify_storage_consistency());
        assert_eq!(km.state(), KMState::Degraded);
    }
}

// =============================================================================
// Tests d'observabilité
// =============================================================================

mod observability_tests {
    use super::*;

    #[test]
    fn test_observability_records_initialization() {
        let km = KindMother::with_identity("test-instance".to_string(), "test-domain".to_string());
        
        // L'événement d'initialisation devrait être enregistré
        assert!(km.observability().event_count() >= 1);
    }

    #[test]
    fn test_observability_records_state_changes() {
        let mut km = KindMother::new();
        let initial_count = km.observability().event_count();
        
        km.recover_to_healthy("Démarrage").unwrap();
        
        // Un événement StateChanged devrait être enregistré
        assert!(km.observability().event_count() > initial_count);
    }

    #[test]
    fn test_observability_records_degradation() {
        let mut km = KindMother::new();
        km.recover_to_healthy("Démarrage").unwrap();
        let count_before = km.observability().event_count();
        
        km.transition_to_degraded("Test").unwrap();
        
        // Au moins 2 événements : StateChanged et DegradationTriggered
        assert!(km.observability().event_count() >= count_before + 2);
    }

    #[test]
    fn test_event_categories() {
        assert_eq!(
            ObservableEvent::IntentCreated { intent_id: "x".to_string() }.category(),
            EventCategory::Intent
        );
        assert_eq!(
            ObservableEvent::WriteApplied { intent_id: "x".to_string() }.category(),
            EventCategory::Write
        );
        assert_eq!(
            ObservableEvent::SyncTriggered { sync_id: "x".to_string() }.category(),
            EventCategory::Sync
        );
        assert_eq!(
            ObservableEvent::AuthorityDecision {
                decision_id: "x".to_string(),
                decision: "y".to_string()
            }.category(),
            EventCategory::Authority
        );
        assert_eq!(
            ObservableEvent::BypassAttempt {
                source: "x".to_string(),
                description: "y".to_string()
            }.category(),
            EventCategory::Security
        );
        assert_eq!(
            ObservableEvent::CorruptionDetected {
                description: "x".to_string()
            }.category(),
            EventCategory::Failure
        );
        assert_eq!(
            ObservableEvent::InstanceInitialized {
                instance_id: "x".to_string()
            }.category(),
            EventCategory::Lifecycle
        );
    }

    #[test]
    fn test_intent_journal() {
        let mut journal = IntentJournal::default();
        let context = EventContext::new("instance-1".to_string(), "domain-1".to_string());
        
        journal.record_intent(
            "intent-1".to_string(),
            1,
            "user-1".to_string(),
            "create".to_string(),
            context,
        );
        
        assert_eq!(journal.count(), 1);
        
        let entry = journal.get_entry("intent-1").unwrap();
        assert_eq!(entry.intent_id(), "intent-1");
        assert_eq!(entry.states_traversed().len(), 1);
        assert_eq!(entry.states_traversed()[0], WriteIntentState::Created);
        
        journal.record_transition("intent-1", WriteIntentState::InValidation);
        let entry = journal.get_entry("intent-1").unwrap();
        assert_eq!(entry.states_traversed().len(), 2);
    }

    #[test]
    fn test_rejection_log() {
        let mut log = RejectionLog::default();
        let context = EventContext::new("instance-1".to_string(), "domain-1".to_string());
        
        let rejection_id = log.record_rejection(
            1,
            "permission".to_string(),
            "Permission insuffisante".to_string(),
            context,
            "permissions_boundary".to_string(),
            KMState::Healthy,
            Some("intent-1".to_string()),
        );
        
        assert!(rejection_id.starts_with("rej-"));
        assert_eq!(log.count(), 1);
    }

    #[test]
    fn test_quarantine_log() {
        let mut log = QuarantineLog::default();
        
        let quarantine_id = log.record_quarantine_entry(
            "user-1".to_string(),
            1,
            "Comportement suspect".to_string(),
            "brute_force".to_string(),
            "full".to_string(),
            "Vérification manuelle".to_string(),
        );
        
        assert!(quarantine_id.starts_with("quar-"));
        assert!(log.is_quarantined("user-1"));
        assert_eq!(log.active_count(), 1);
        
        log.record_quarantine_exit("user-1", 2, "Vérification OK".to_string());
        assert!(!log.is_quarantined("user-1"));
        assert_eq!(log.active_count(), 0);
    }
}

// =============================================================================
// Tests CoreDataAPI
// =============================================================================

mod core_data_api_tests {
    use super::*;

    fn create_test_contexts() -> (AuthorityContext, InstanceContext, DomainContext) {
        let authority = AuthorityContext::with_permissions(
            "test-user".to_string(),
            vec![],
            true,   // can_read
            true,   // can_write
            false,  // can_sync
        );
        let instance = InstanceContext::new("test-instance".to_string(), InstanceType::Daughter);
        let domain = DomainContext::new("test-domain".to_string(), "TestDomain".to_string());
        (authority, instance, domain)
    }

    #[test]
    fn test_core_data_api_initial_state() {
        let api = CoreDataAPI::new(KMState::Healthy);
        assert_eq!(api.state(), KMState::Healthy);
    }

    #[test]
    fn test_submit_write_intent_healthy_state() {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let (authority, instance, domain) = create_test_contexts();
        
        let intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        let result = api.submit_write_intent(intent, authority, instance, domain);
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_write_intent_degraded_state() {
        let mut api = CoreDataAPI::new(KMState::Degraded);
        let (authority, instance, domain) = create_test_contexts();
        
        let intent = WriteIntent::new("intent-1".to_string(), "create".to_string());
        
        let result = api.submit_write_intent(intent, authority, instance, domain);
        // Écriture refusée en état Degraded
        assert!(result.is_err());
    }

    #[test]
    fn test_read_entity_degraded_state() {
        let mut api = CoreDataAPI::new(KMState::Degraded);
        let (authority, instance, domain) = create_test_contexts();
        
        // Lecture autorisée en état Degraded
        let result = api.read_entity("entity-1", authority, instance, domain);
        assert!(result.is_ok());
    }

    #[test]
    fn test_submit_sync_intent_always_rejected() {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let (authority, instance, domain) = create_test_contexts();
        
        let sync_intent = SyncIntent::new(
            "sync-1".to_string(),
            "test-instance".to_string(),  // Doit correspondre à instance_id
            "mother-1".to_string(),
            InstanceType::Daughter,
        ).unwrap();
        
        let result = api.submit_sync_intent(&sync_intent, authority, instance, domain);
        // Sync toujours refusée
        assert!(result.is_err());
    }
}
