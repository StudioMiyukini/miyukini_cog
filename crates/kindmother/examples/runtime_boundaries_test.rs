//! Tests console pour les Runtime Boundaries
//!
//! Valide l'activation des Runtime Boundaries selon le contrat FONDATION.

use kindmother::api::CoreDataAPI;
use kindmother::core::{
    AuthorityContext, DomainContext, InstanceContext, InstanceType, WriteIntent,
};
use kindmother::state::KMState;

fn main() {
    println!("==========================================");
    println!("TESTS RUNTIME BOUNDARIES");
    println!("==========================================");
    println!();

    // Contexte de test valide
    let authority = AuthorityContext::new(
        "test-caller".to_string(),
        vec!["read".to_string(), "write".to_string()],
    );
    let instance = InstanceContext::new("test-instance".to_string(), InstanceType::Mother);
    let domain = DomainContext::new("test-domain".to_string(), "TestDomain".to_string());

    // Test 1: KM Healthy → pipeline OK
    println!("TEST 1: KM Healthy → pipeline OK");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent = WriteIntent::new("intent-1".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✓ Test 1 RÉUSSI: WriteIntent acceptée en état Healthy"),
            Err(e) => println!("✗ Test 1 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    // Test 2: KM Degraded → rejet écriture
    println!("TEST 2: KM Degraded → rejet écriture");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Degraded);
        let intent = WriteIntent::new("intent-2".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 2 ÉCHOUÉ: WriteIntent acceptée en état Degraded (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("dégradé") || e.to_string().contains("Degraded") {
                    println!("✓ Test 2 RÉUSSI: WriteIntent rejetée en état Degraded (comme attendu)");
                } else {
                    println!("✗ Test 2 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 3: KM Degraded → lecture OK
    println!("TEST 3: KM Degraded → lecture OK");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Degraded);

        match api.read_entity("entity-1", authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✓ Test 3 RÉUSSI: Lecture acceptée en état Degraded"),
            Err(e) => println!("✗ Test 3 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    // Test 4: KM Quarantined → rejet total
    println!("TEST 4: KM Quarantined → rejet total");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Quarantined);
        let intent = WriteIntent::new("intent-3".to_string(), "test_operation".to_string());

        match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 4 ÉCHOUÉ: WriteIntent acceptée en état Quarantined (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("quarantaine") || e.to_string().contains("Quarantined") {
                    println!("✓ Test 4 RÉUSSI: WriteIntent rejetée en état Quarantined (comme attendu)");
                } else {
                    println!("✗ Test 4 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 5: Intention invalide → rejet explicite (état non Created)
    println!("TEST 5: Intention invalide → rejet explicite (état non Created)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let mut intent = WriteIntent::new("intent-4".to_string(), "test_operation".to_string());
        
        // Transitionner l'intention vers un état non-Created
        let _ = intent.start_validation();

        match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 5 ÉCHOUÉ: WriteIntent acceptée avec état non-Created (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("CRÉÉE") || e.to_string().contains("Created") {
                    println!("✓ Test 5 RÉUSSI: WriteIntent rejetée car non en état Created (comme attendu)");
                } else {
                    println!("✗ Test 5 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 6: Intention déjà vue → rejet (resoumission)
    println!("TEST 6: Intention déjà vue → rejet (resoumission)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent1 = WriteIntent::new("intent-5".to_string(), "test_operation".to_string());
        let intent2 = WriteIntent::new("intent-5".to_string(), "test_operation".to_string()); // Même ID

        // Première soumission (devrait réussir)
        match api.submit_write_intent(intent1, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("  Première soumission: OK"),
            Err(e) => println!("  Première soumission échouée: {}", e),
        }

        // Deuxième soumission (devrait échouer)
        match api.submit_write_intent(intent2, authority.clone(), instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 6 ÉCHOUÉ: Resoumission acceptée (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("déjà") || e.to_string().contains("resoumission") || e.to_string().contains("déjà été soumise") {
                    println!("✓ Test 6 RÉUSSI: Resoumission rejetée (comme attendu)");
                } else {
                    println!("✗ Test 6 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 7: Contexte invalide → rejet (InstanceContext vide)
    println!("TEST 7: Contexte invalide → rejet (InstanceContext vide)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent = WriteIntent::new("intent-6".to_string(), "test_operation".to_string());
        let invalid_instance = InstanceContext::new("".to_string(), InstanceType::Mother); // instance_id vide

        match api.submit_write_intent(intent, authority.clone(), invalid_instance, domain.clone()) {
            Ok(()) => println!("✗ Test 7 ÉCHOUÉ: WriteIntent acceptée avec InstanceContext vide (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("instance_id") || e.to_string().contains("InstanceContext") {
                    println!("✓ Test 7 RÉUSSI: WriteIntent rejetée car InstanceContext invalide (comme attendu)");
                } else {
                    println!("✗ Test 7 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 8: Contexte invalide → rejet (DomainContext vide)
    println!("TEST 8: Contexte invalide → rejet (DomainContext vide)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent = WriteIntent::new("intent-7".to_string(), "test_operation".to_string());
        let invalid_domain = DomainContext::new("".to_string(), "TestDomain".to_string()); // domain_id vide

        match api.submit_write_intent(intent, authority.clone(), instance.clone(), invalid_domain) {
            Ok(()) => println!("✗ Test 8 ÉCHOUÉ: WriteIntent acceptée avec DomainContext vide (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("domain_id") || e.to_string().contains("DomainContext") {
                    println!("✓ Test 8 RÉUSSI: WriteIntent rejetée car DomainContext invalide (comme attendu)");
                } else {
                    println!("✗ Test 8 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 9: Contexte invalide → rejet (AuthorityContext vide)
    println!("TEST 9: Contexte invalide → rejet (AuthorityContext vide)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent = WriteIntent::new("intent-8".to_string(), "test_operation".to_string());
        let invalid_authority = AuthorityContext::new("".to_string(), vec![]); // caller_identity vide

        match api.submit_write_intent(intent, invalid_authority, instance.clone(), domain.clone()) {
            Ok(()) => println!("✗ Test 9 ÉCHOUÉ: WriteIntent acceptée avec AuthorityContext vide (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("caller_identity") || e.to_string().contains("AuthorityContext") {
                    println!("✓ Test 9 RÉUSSI: WriteIntent rejetée car AuthorityContext invalide (comme attendu)");
                } else {
                    println!("✗ Test 9 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 10: Contexte invalide → rejet (incohérence structurelle: instance_id == domain_id)
    println!("TEST 10: Contexte invalide → rejet (incohérence structurelle)");
    println!("----------------------------------------");
    {
        let mut api = CoreDataAPI::new(KMState::Healthy);
        let intent = WriteIntent::new("intent-9".to_string(), "test_operation".to_string());
        let invalid_instance = InstanceContext::new("same-id".to_string(), InstanceType::Mother);
        let invalid_domain = DomainContext::new("same-id".to_string(), "TestDomain".to_string()); // Même ID

        match api.submit_write_intent(intent, authority.clone(), invalid_instance, invalid_domain) {
            Ok(()) => println!("✗ Test 10 ÉCHOUÉ: WriteIntent acceptée avec incohérence structurelle (devrait être rejetée)"),
            Err(e) => {
                if e.to_string().contains("cohérence") || e.to_string().contains("identiques") {
                    println!("✓ Test 10 RÉUSSI: WriteIntent rejetée car incohérence structurelle (comme attendu)");
                } else {
                    println!("✗ Test 10 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    println!("==========================================");
    println!("FIN DES TESTS RUNTIME BOUNDARIES");
    println!("==========================================");
}
