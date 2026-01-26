//! Tests console pour la Permission Boundary
//!
//! Valide l'implémentation de la Permission Boundary conceptuelle.

use kindmother::api::CoreDataAPI;
use kindmother::core::{
    AuthorityContext, DomainContext, InstanceContext, InstanceType, WriteIntent,
};
use kindmother::state::KMState;

fn main() {
    println!("==========================================");
    println!("TESTS PERMISSION BOUNDARY");
    println!("==========================================");
    println!();

    // Contexte de base valide
    let instance = InstanceContext::new("test-instance".to_string(), InstanceType::Mother);
    let domain = DomainContext::new("test-domain".to_string(), "TestDomain".to_string());

    // Test 1: Lecture autorisée (can_read = true)
    println!("TEST 1: Lecture autorisée (can_read = true)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read
            false, // can_write
            false, // can_sync
        );

        match api.read_entity("entity-1", authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✓ Test 1 RÉUSSI: Lecture autorisée avec can_read"),
            Err(e) => println!("✗ Test 1 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    // Test 2: Lecture refusée (can_read = false)
    println!("TEST 2: Lecture refusée (can_read = false)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            false, // can_read
            false, // can_write
            false, // can_sync
        );

        match api.read_entity("entity-1", authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 2 ÉCHOUÉ: Lecture acceptée sans can_read (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("can_read") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 2 RÉUSSI: Lecture rejetée sans can_read (comme attendu)");
                } else {
                    println!("✗ Test 2 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 3: Écriture autorisée (can_write = true)
    println!("TEST 3: Écriture autorisée (can_write = true)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            false, // can_read (pas nécessaire pour écriture)
            true,  // can_write
            false, // can_sync
        );
        let intent = WriteIntent::new("intent-1".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✓ Test 3 RÉUSSI: Écriture autorisée avec can_write"),
            Err(e) => println!("✗ Test 3 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    // Test 4: Écriture refusée (can_write = false)
    println!("TEST 4: Écriture refusée (can_write = false)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read (ne suffit pas pour écriture)
            false, // can_write
            false, // can_sync
        );
        let intent = WriteIntent::new("intent-2".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 4 ÉCHOUÉ: Écriture acceptée sans can_write (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("can_write") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 4 RÉUSSI: Écriture rejetée sans can_write (comme attendu)");
                } else {
                    println!("✗ Test 4 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 5: Refus d'escalade (can_read ≠ can_write)
    println!("TEST 5: Refus d'escalade (can_read ≠ can_write)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read
            false, // can_write (pas d'escalade depuis can_read)
            false, // can_sync
        );
        let intent = WriteIntent::new("intent-3".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 5 ÉCHOUÉ: Escalade acceptée (can_read → write, devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("can_write") || e.to_string().contains("escalade") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 5 RÉUSSI: Escalade rejetée (can_read ≠ can_write, comme attendu)");
                } else {
                    println!("✗ Test 5 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 6: Refus d'escalade (can_write ≠ can_sync)
    println!("TEST 6: Refus d'escalade (can_write ≠ can_sync)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            false, // can_read
            true,  // can_write
            false, // can_sync (pas d'escalade depuis can_write)
        );
        let intent = WriteIntent::new("intent-4".to_string(), "sync_operation".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 6 ÉCHOUÉ: Escalade acceptée (can_write → sync, devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("synchronisation") || e.to_string().contains("toujours refusée") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 6 RÉUSSI: Escalade rejetée (can_write ≠ can_sync, sync toujours refusée, comme attendu)");
                } else {
                    println!("✗ Test 6 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 7: Sync toujours refusée (même avec can_sync = true)
    println!("TEST 7: Sync toujours refusée (même avec can_sync = true)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read
            true,  // can_write
            true,  // can_sync (mais sync toujours refusée)
        );
        let intent = WriteIntent::new("intent-5".to_string(), "sync_operation".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 7 ÉCHOUÉ: Sync acceptée (devrait être toujours refusée)"),
            Err(e) => {
                if e.to_string().contains("synchronisation") || e.to_string().contains("toujours refusée") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 7 RÉUSSI: Sync rejetée même avec can_sync = true (comme attendu)");
                } else {
                    println!("✗ Test 7 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 8: Sync toujours refusée (opération avec "sync" dans le nom)
    println!("TEST 8: Sync toujours refusée (opération avec 'sync' dans le nom)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read
            true,  // can_write
            true,  // can_sync (mais sync toujours refusée)
        );
        let intent = WriteIntent::new("intent-6".to_string(), "synchronize_data".to_string());

        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 8 ÉCHOUÉ: Sync acceptée (devrait être toujours refusée)"),
            Err(e) => {
                if e.to_string().contains("synchronisation") || e.to_string().contains("toujours refusée") || e.to_string().contains("Permissions insuffisantes") {
                    println!("✓ Test 8 RÉUSSI: Sync rejetée (opération 'synchronize', comme attendu)");
                } else {
                    println!("✗ Test 8 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 9: Permissions complètes (read + write, pas sync)
    println!("TEST 9: Permissions complètes (read + write, pas sync)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let authority = AuthorityContext::with_permissions(
            "test-caller".to_string(),
            vec![],
            true,  // can_read
            true,  // can_write
            false, // can_sync
        );

        // Test lecture
        match api.read_entity("entity-1", authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("  Lecture: OK"),
            Err(e) => println!("  Lecture échouée: {}", e),
        }

        // Test écriture
        let intent = WriteIntent::new("intent-7".to_string(), "test_operation".to_string());
        match api.submit_write_intent(intent, authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✓ Test 9 RÉUSSI: Permissions complètes (read + write) fonctionnent"),
            Err(e) => println!("✗ Test 9 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    println!("==========================================");
    println!("FIN DES TESTS PERMISSION BOUNDARY");
    println!("==========================================");
}
