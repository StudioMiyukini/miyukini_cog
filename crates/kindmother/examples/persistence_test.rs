//! Tests console pour la persistance interne de KindMother
//!
//! Valide l'implémentation de la persistance selon le contrat FONDATION.

use kindmother::core::{KindMother, WriteIntent};
use kindmother::state::KMState;

fn main() {
    println!("==========================================");
    println!("TESTS PERSISTANCE INTERNE");
    println!("==========================================");
    println!();

    let domain_id = "test-domain".to_string();

    // Test 1: Application → persistance OK
    println!("TEST 1: Application → persistance OK");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        let mut intent = WriteIntent::new("intent-1".to_string(), "test_operation".to_string());
        
        // Cycle complet : Created → InValidation → Accepted → Applied
        intent.start_validation().unwrap();
        intent.accept().unwrap();
        intent.apply().unwrap();

        // Persistance après application
        match km.persist_intent(intent, &domain_id) {
            Ok(()) => {
                let count = km.count_persisted_intents(&domain_id);
                if count == 1 {
                    println!("✓ Test 1 RÉUSSI: Intention appliquée et persistée ({} intention dans le domaine)", count);
                } else {
                    println!("✗ Test 1 ÉCHOUÉ: Nombre d'intentions persistées incorrect (attendu: 1, obtenu: {})", count);
                }
            }
            Err(e) => println!("✗ Test 1 ÉCHOUÉ: {}", e),
        }
    }
    println!();

    // Test 2: Rejet → non persisté
    println!("TEST 2: Rejet → non persisté");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        let mut intent = WriteIntent::new("intent-2".to_string(), "test_operation".to_string());
        
        // Cycle avec rejet : Created → InValidation → Rejected
        intent.start_validation().unwrap();
        intent.reject("Test de rejet".to_string()).unwrap();

        // Tentative de persistance d'une intention rejetée (devrait échouer)
        match km.persist_intent(intent, &domain_id) {
            Ok(()) => println!("✗ Test 2 ÉCHOUÉ: Intention rejetée persistée (devrait être refusée)"),
            Err(e) => {
                if e.to_string().contains("APPLIQUÉE") || e.to_string().contains("Applied") {
                    println!("✓ Test 2 RÉUSSI: Intention rejetée non persistée (comme attendu)");
                } else {
                    println!("✗ Test 2 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }

        let count = km.count_persisted_intents(&domain_id);
        if count == 0 {
            println!("  Vérification: Aucune intention persistée (comme attendu)");
        } else {
            println!("✗ Test 2 ÉCHOUÉ: {} intention(s) persistée(s) alors qu'aucune ne devrait l'être", count);
        }
    }
    println!();

    // Test 3: Non appliquée → non persistée
    println!("TEST 3: Non appliquée → non persistée");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        let mut intent = WriteIntent::new("intent-3".to_string(), "test_operation".to_string());
        
        // Cycle sans application : Created → InValidation → Accepted (mais pas Applied)
        intent.start_validation().unwrap();
        intent.accept().unwrap();
        // Pas d'appel à apply()

        // Tentative de persistance d'une intention non appliquée (devrait échouer)
        match km.persist_intent(intent, &domain_id) {
            Ok(()) => println!("✗ Test 3 ÉCHOUÉ: Intention non appliquée persistée (devrait être refusée)"),
            Err(e) => {
                if e.to_string().contains("APPLIQUÉE") || e.to_string().contains("Applied") {
                    println!("✓ Test 3 RÉUSSI: Intention non appliquée non persistée (comme attendu)");
                } else {
                    println!("✗ Test 3 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }
    }
    println!();

    // Test 4: Crash simulé → aucun état partiel
    println!("TEST 4: Crash simulé → aucun état partiel");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        // Persistance d'une première intention
        let mut intent1 = WriteIntent::new("intent-4a".to_string(), "test_operation".to_string());
        intent1.start_validation().unwrap();
        intent1.accept().unwrap();
        intent1.apply().unwrap();
        km.persist_intent(intent1, &domain_id).unwrap();

        // Simulation d'un crash
        km.simulate_storage_crash();

        // Tentative de persistance après crash (devrait échouer)
        let mut intent2 = WriteIntent::new("intent-4b".to_string(), "test_operation".to_string());
        intent2.start_validation().unwrap();
        intent2.accept().unwrap();
        intent2.apply().unwrap();

        match km.persist_intent(intent2, &domain_id) {
            Ok(()) => println!("✗ Test 4 ÉCHOUÉ: Persistance acceptée après crash (devrait être refusée)"),
            Err(e) => {
                if e.to_string().contains("corrompu") || e.to_string().contains("corruption") {
                    println!("✓ Test 4 RÉUSSI: Persistance refusée après crash (comme attendu)");
                } else {
                    println!("✗ Test 4 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }

        // Vérification : l'état doit être cohérent (pas d'état partiel)
        let count = km.count_persisted_intents(&domain_id);
        println!("  Vérification: {} intention(s) persistée(s) avant le crash (état cohérent)", count);
    }
    println!();

    // Test 5: Corruption simulée → KM passe en Degraded
    println!("TEST 5: Corruption simulée → KM passe en Degraded");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        // Simulation d'une corruption
        km.simulate_storage_crash();

        // Vérification de cohérence (devrait détecter la corruption)
        let was_healthy = km.state() == KMState::Healthy;
        let is_consistent = km.verify_storage_consistency();
        let is_degraded = km.state() == KMState::Degraded;

        if !is_consistent && is_degraded {
            println!("✓ Test 5 RÉUSSI: Corruption détectée, KM passé en Degraded (comme attendu)");
        } else if was_healthy && !is_consistent {
            println!("✗ Test 5 ÉCHOUÉ: Corruption détectée mais KM n'est pas passé en Degraded (état: {:?})", km.state());
        } else {
            println!("✗ Test 5 ÉCHOUÉ: État inattendu (cohérent: {}, état: {:?})", is_consistent, km.state());
        }
    }
    println!();

    // Test 6: Redémarrage conceptuel → état cohérent
    println!("TEST 6: Redémarrage conceptuel → état cohérent");
    println!("----------------------------------------");
    {
        // Création d'une première instance avec persistance
        let mut km1 = KindMother::new();
        km1.set_state(KMState::Healthy);

        let mut intent = WriteIntent::new("intent-6".to_string(), "test_operation".to_string());
        intent.start_validation().unwrap();
        intent.accept().unwrap();
        intent.apply().unwrap();
        km1.persist_intent(intent, &domain_id).unwrap();

        let count1 = km1.count_persisted_intents(&domain_id);
        println!("  Instance 1: {} intention(s) persistée(s)", count1);

        // Simulation d'un redémarrage (nouvelle instance)
        let mut km2 = KindMother::new();
        km2.set_state(KMState::Healthy);

        // Vérification de cohérence après redémarrage
        let is_consistent = km2.verify_storage_consistency();
        if is_consistent {
            println!("✓ Test 6 RÉUSSI: État cohérent après redémarrage conceptuel (comme attendu)");
        } else {
            println!("✗ Test 6 ÉCHOUÉ: État incohérent après redémarrage");
        }

        // Note: Dans une implémentation réelle, les données seraient restaurées depuis le stockage
        // À ce stade conceptuel, on vérifie juste que l'état est cohérent
        let count2 = km2.count_persisted_intents(&domain_id);
        println!("  Instance 2 (après redémarrage): {} intention(s) persistée(s)", count2);
        println!("  Note: Dans une implémentation réelle, les données seraient restaurées depuis le stockage");
    }
    println!();

    // Test 7: Atomicité - échec de persistance → rollback
    println!("TEST 7: Atomicité - échec de persistance → rollback");
    println!("----------------------------------------");
    {
        let mut km = KindMother::new();
        km.set_state(KMState::Healthy);

        // Persistance d'une première intention
        let mut intent1 = WriteIntent::new("intent-7a".to_string(), "test_operation".to_string());
        intent1.start_validation().unwrap();
        intent1.accept().unwrap();
        intent1.apply().unwrap();
        km.persist_intent(intent1, &domain_id).unwrap();

        let count_before = km.count_persisted_intents(&domain_id);
        println!("  Avant: {} intention(s) persistée(s)", count_before);

        // Tentative de persistance d'une intention avec le même ID (devrait échouer et rollback)
        let mut intent2 = WriteIntent::new("intent-7a".to_string(), "test_operation".to_string());
        intent2.start_validation().unwrap();
        intent2.accept().unwrap();
        intent2.apply().unwrap();

        match km.persist_intent(intent2, &domain_id) {
            Ok(()) => println!("✗ Test 7 ÉCHOUÉ: Duplication acceptée (devrait être refusée)"),
            Err(e) => {
                if e.to_string().contains("déjà présente") || e.to_string().contains("corruption") {
                    println!("✓ Test 7 RÉUSSI: Duplication refusée (comme attendu)");
                } else {
                    println!("✗ Test 7 ÉCHOUÉ: Erreur inattendue: {}", e);
                }
            }
        }

        let count_after = km.count_persisted_intents(&domain_id);
        if count_after == count_before {
            println!("✓ Test 7 RÉUSSI: Atomicité préservée ({} intention(s) persistée(s), pas de duplication)", count_after);
        } else {
            println!("✗ Test 7 ÉCHOUÉ: Atomicité violée (avant: {}, après: {})", count_before, count_after);
        }
    }
    println!();

    println!("==========================================");
    println!("FIN DES TESTS PERSISTANCE INTERNE");
    println!("==========================================");
}
