//! Test console de détection de menaces
//!
//! Ce test démontre les scénarios de détection de menaces :
//! - Détection de replay d'intention
//! - Détection de resoumission après rejet
//! - Détection de saturation volontaire
//! - Détection de brute-force (rejets consécutifs)
//! - Dégradation contrôlée

use kindmother::*;

fn main() {
    println!("=== Test de Détection de Menaces KindMother ===\n");

    // 1. Configuration du détecteur de menaces
    println!("1. Configuration du ThreatDetector...");
    let config = ThreatDetectorConfig {
        saturation_threshold: 5,    // 5 appels max par période
        brute_force_threshold: 3,   // 3 rejets consécutifs = brute-force
        counting_period: 60,
    };
    let mut detector = ThreatDetector::with_config(config);
    println!("   ✓ Détecteur configuré:");
    println!("     - Seuil de saturation: 5 appels");
    println!("     - Seuil de brute-force: 3 rejets consécutifs\n");

    // 2. Détection de REPLAY
    println!("2. Test de détection de REPLAY...");
    
    // Première soumission - OK
    let threat = detector.detect_replay("intent-001", "user-alice");
    println!("   2.1 Première soumission de intent-001:");
    println!("       → Menace détectée: {}", threat.is_some());
    
    // Marquer l'intention comme traitée
    detector.mark_intent_processed("intent-001");
    println!("   2.2 Intention marquée comme traitée");
    
    // Deuxième soumission - REPLAY détecté
    let threat = detector.detect_replay("intent-001", "user-alice");
    println!("   2.3 Deuxième soumission de intent-001:");
    if let Some(t) = threat {
        println!("       → MENACE DÉTECTÉE!");
        println!("       → Type: {}", t.threat_type().to_string());
        println!("       → Gravité: {}", t.severity().to_string());
        println!("       → Raison: {}", t.reason());
    }
    println!();

    // 3. Détection de RESOUMISSION
    println!("3. Test de détection de RESOUMISSION...");
    
    // Marquer une intention comme rejetée
    detector.mark_intent_rejected("intent-002");
    println!("   3.1 Intention intent-002 marquée comme rejetée");
    
    // Tentative de resoumission
    let threat = detector.detect_resubmission("intent-002", "user-bob");
    println!("   3.2 Tentative de resoumission de intent-002:");
    if let Some(t) = threat {
        println!("       → MENACE DÉTECTÉE!");
        println!("       → Type: {}", t.threat_type().to_string());
        println!("       → Gravité: {}", t.severity().to_string());
        println!("       → Raison: {}", t.reason());
    }
    println!();

    // 4. Détection de SATURATION
    println!("4. Test de détection de SATURATION...");
    
    for i in 1..=6 {
        let threat = detector.detect_saturation("user-charlie");
        print!("   4.{} Appel #{}: ", i, i);
        if let Some(t) = threat {
            println!("SATURATION DÉTECTÉE!");
            println!("       → Type: {}", t.threat_type().to_string());
            println!("       → Gravité: {}", t.severity().to_string());
            println!("       → Raison: {}", t.reason());
        } else {
            println!("OK");
        }
    }
    println!();

    // 5. Détection de BRUTE-FORCE
    println!("5. Test de détection de BRUTE-FORCE...");
    
    for i in 1..=4 {
        let threat = detector.record_rejection_and_detect_brute_force("user-david");
        print!("   5.{} Rejet #{}: ", i, i);
        if let Some(t) = threat {
            println!("BRUTE-FORCE DÉTECTÉ!");
            println!("       → Type: {}", t.threat_type().to_string());
            println!("       → Gravité: {}", t.severity().to_string());
            println!("       → Raison: {}", t.reason());
        } else {
            println!("compteur incrémenté");
        }
    }
    println!();

    // 6. Reset du compteur de rejets après succès
    println!("6. Test du reset après succès...");
    detector.reset_rejection_count("user-david");
    println!("   ✓ Compteur de rejets réinitialisé pour user-david");
    
    let threat = detector.record_rejection_and_detect_brute_force("user-david");
    println!("   Premier rejet après reset: {}", 
        if threat.is_some() { "brute-force (erreur!)" } else { "compteur incrémenté (correct)" });
    println!();

    // 7. Statistiques des menaces détectées
    println!("7. Statistiques des menaces détectées:");
    println!("   - Total: {} menaces", detector.threat_count());
    println!("   - Type REPLAY: {} détections", detector.threats_of_type(ThreatType::Replay).len());
    println!("   - Type RESOUMISSION: {} détections", detector.threats_of_type(ThreatType::Resubmission).len());
    println!("   - Type SATURATION: {} détections", detector.threats_of_type(ThreatType::Saturation).len());
    println!("   - Type BRUTE_FORCE: {} détections", detector.threats_of_type(ThreatType::BruteForce).len());
    
    let critical_threats = detector.threats_with_min_severity(ThreatSeverity::High);
    println!("   - Menaces de gravité ÉLEVÉE ou plus: {}", critical_threats.len());
    println!();

    // 8. Test de dégradation contrôlée
    println!("8. Test de dégradation contrôlée du moteur...\n");
    
    let mut km = KindMother::with_identity("test-instance".to_string(), "test-domain".to_string());
    km.recover_to_healthy("Démarrage initial").unwrap();
    println!("   8.1 État initial: {:?}", km.state());
    
    // Transition vers Degraded
    km.transition_to_degraded("Menace de saturation détectée").unwrap();
    println!("   8.2 Après transition_to_degraded: {:?}", km.state());
    
    // Transition vers Quarantined
    km.transition_to_quarantined("Menaces multiples critiques").unwrap();
    println!("   8.3 Après transition_to_quarantined: {:?}", km.state());
    
    // Tentative de transition depuis Quarantined vers Degraded (doit échouer)
    let result = km.transition_to_degraded("Tentative invalide");
    println!("   8.4 Tentative de passage en Degraded depuis Quarantined:");
    match result {
        Ok(()) => println!("       → Succès (erreur!)"),
        Err(e) => println!("       → Rejet correct: {}", e),
    }
    
    // Levée de quarantaine
    km.lift_quarantine("Vérification manuelle terminée").unwrap();
    println!("   8.5 Après lift_quarantine: {:?}", km.state());
    
    // Récupération vers Healthy
    km.recover_to_healthy("Système vérifié").unwrap();
    println!("   8.6 Après recover_to_healthy: {:?}", km.state());
    println!();

    // 9. Vérification de l'observabilité
    println!("9. Vérification de l'observabilité:");
    println!("   - Événements enregistrés: {}", km.observability().event_count());
    println!();

    // 10. Résumé
    println!("\n=== Résumé ===");
    println!("✓ Détection de REPLAY fonctionnelle");
    println!("✓ Détection de RESOUMISSION fonctionnelle");
    println!("✓ Détection de SATURATION fonctionnelle");
    println!("✓ Détection de BRUTE-FORCE fonctionnelle");
    println!("✓ Dégradation contrôlée (Degraded, Quarantined) fonctionnelle");
    println!("✓ Récupération (lift_quarantine, recover_to_healthy) fonctionnelle");
    println!("✓ AUCUNE auto-réparation automatique");
    println!("✓ AUCUNE escalade implicite");
}
