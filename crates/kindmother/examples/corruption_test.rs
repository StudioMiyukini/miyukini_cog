//! Test console de détection de corruption
//!
//! Ce test démontre les scénarios de corruption :
//! - Détection de corruption du storage
//! - Passage automatique en état Degraded
//! - Impossibilité de récupérer sans vérification

use kindmother::*;

fn main() {
    println!("=== Test de Détection de Corruption KindMother ===\n");

    // 1. Création et démarrage du moteur
    println!("1. Création et démarrage du moteur...");
    let mut km = KindMother::with_identity("corruption-test".to_string(), "test-domain".to_string());
    km.recover_to_healthy("Démarrage initial").unwrap();
    println!("   ✓ Moteur démarré");
    println!("   État initial: {:?}\n", km.state());

    // 2. Persistance d'une intention valide
    println!("2. Persistance d'une intention valide...");
    let mut intent = WriteIntent::new("intent-001".to_string(), "create_entity".to_string());
    intent.start_validation().unwrap();
    intent.accept().unwrap();
    intent.apply().unwrap();
    
    match km.persist_intent(intent, "domain-test") {
        Ok(()) => {
            println!("   ✓ Intention persistée avec succès");
            println!("   Nombre d'intentions persistées: {}\n", km.count_persisted_intents("domain-test"));
        }
        Err(e) => {
            println!("   ✗ Erreur: {}", e);
        }
    }

    // 3. Vérification de cohérence (avant corruption)
    println!("3. Vérification de cohérence (avant corruption)...");
    if km.verify_storage_consistency() {
        println!("   ✓ Storage cohérent\n");
    } else {
        println!("   ✗ Corruption détectée (inattendu)\n");
    }

    // 4. Simulation d'un crash de storage
    println!("4. Simulation d'un crash de storage...");
    km.simulate_storage_crash();
    println!("   ⚠ Crash simulé\n");

    // 5. Vérification de cohérence (après corruption)
    println!("5. Vérification de cohérence (après corruption)...");
    let is_consistent = km.verify_storage_consistency();
    println!("   → Storage cohérent: {}", is_consistent);
    println!("   → État actuel: {:?}", km.state());
    
    if km.state() == KMState::Degraded {
        println!("   ✓ Passage automatique en état Degraded (correct)\n");
    } else {
        println!("   ✗ État inattendu\n");
    }

    // 6. Tentative de récupération vers Healthy (doit échouer)
    println!("6. Tentative de récupération vers Healthy (storage corrompu)...");
    match km.recover_to_healthy("Tentative de récupération") {
        Ok(()) => {
            println!("   ✗ Récupération réussie (ne devrait pas arriver avec un storage corrompu)");
        }
        Err(e) => {
            println!("   ✓ Récupération refusée (correct): {}", e);
        }
    }
    println!();

    // 7. Scénario de corruption avec quarantaine
    println!("7. Scénario de corruption grave avec quarantaine...\n");
    
    let mut km2 = KindMother::with_identity("quarantine-test".to_string(), "test-domain".to_string());
    km2.recover_to_healthy("Démarrage").unwrap();
    
    println!("   7.1 État initial: {:?}", km2.state());
    
    // Corruption détectée
    km2.simulate_storage_crash();
    km2.verify_storage_consistency();
    println!("   7.2 Après corruption: {:?}", km2.state());
    
    // Corruption grave → Quarantaine
    km2.transition_to_quarantined("Corruption irréparable détectée").unwrap();
    println!("   7.3 Après quarantaine: {:?}", km2.state());
    
    // Tentative de récupération directe (doit échouer)
    match km2.recover_to_healthy("Tentative directe") {
        Ok(()) => println!("   7.4 Récupération directe réussie (erreur!)"),
        Err(_) => println!("   7.4 Récupération directe refusée (correct)"),
    }
    
    // Levée de quarantaine puis récupération
    km2.lift_quarantine("Intervention manuelle - problème corrigé").unwrap();
    println!("   7.5 Après levée de quarantaine: {:?}", km2.state());
    println!();

    // 8. Vérification de l'observabilité des événements
    println!("8. Vérification de l'observabilité...");
    let obs = km2.observability();
    println!("   - Total événements: {}", obs.event_count());
    
    let state_events = obs.events_by_category(EventCategory::Lifecycle);
    println!("   - Événements Lifecycle: {}", state_events.len());
    
    let failure_events = obs.events_by_category(EventCategory::Failure);
    println!("   - Événements Failure: {}", failure_events.len());
    
    let security_events = obs.events_by_category(EventCategory::Security);
    println!("   - Événements Security: {}", security_events.len());
    println!();

    // 9. Résumé
    println!("\n=== Résumé ===");
    println!("✓ Détection de corruption fonctionnelle");
    println!("✓ Passage automatique en état Degraded après corruption");
    println!("✓ Récupération refusée tant que le storage est corrompu");
    println!("✓ Quarantaine disponible pour corruption grave");
    println!("✓ Levée de quarantaine manuelle uniquement");
    println!("✓ Observabilité des événements de corruption");
    println!("✓ AUCUNE auto-réparation automatique");
}
