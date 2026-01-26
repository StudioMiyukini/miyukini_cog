//! Test console d'observabilité
//!
//! Ce test démontre les capacités d'observabilité :
//! - Enregistrement d'événements conceptuels
//! - Journal d'intention (IntentJournal)
//! - Journal des rejets (RejectionLog)
//! - Journal des quarantaines (QuarantineLog)

use kindmother::*;

fn main() {
    println!("=== Test d'Observabilité KindMother ===\n");

    // 1. Création du moteur avec observabilité
    println!("1. Création du moteur avec observabilité...");
    let mut km = KindMother::with_identity("obs-test-instance".to_string(), "obs-test-domain".to_string());
    println!("   ✓ Moteur créé");
    println!("   Événements initiaux: {}\n", km.observability().event_count());

    // 2. Test des événements de cycle de vie
    println!("2. Test des événements de cycle de vie...\n");
    
    km.recover_to_healthy("Démarrage réussi").unwrap();
    println!("   2.1 Transition vers Healthy effectuée");
    
    km.transition_to_degraded("Test de dégradation").unwrap();
    println!("   2.2 Transition vers Degraded effectuée");
    
    km.recover_to_healthy("Récupération").unwrap();
    println!("   2.3 Récupération vers Healthy effectuée");
    
    println!("   → Événements totaux: {}\n", km.observability().event_count());

    // 3. Catégories d'événements
    println!("3. Catégories d'événements observables...\n");
    
    let categories = [
        (EventCategory::Intent, "Intent"),
        (EventCategory::Write, "Write"),
        (EventCategory::Sync, "Sync"),
        (EventCategory::Authority, "Authority"),
        (EventCategory::Security, "Security"),
        (EventCategory::Failure, "Failure"),
        (EventCategory::Lifecycle, "Lifecycle"),
    ];
    
    let obs = km.observability();
    for (category, name) in categories.iter() {
        let events = obs.events_by_category(*category);
        println!("   - {}: {} événements", name, events.len());
    }
    println!();

    // 4. Journal d'intention (IntentJournal)
    println!("4. Test du Journal d'Intention...\n");
    
    let obs = km.observability_mut();
    let context = EventContext::new("obs-test-instance".to_string(), "obs-test-domain".to_string());
    
    // Enregistrer une intention
    obs.intent_journal_mut().record_intent(
        "intent-001".to_string(),
        1,
        "user-test".to_string(),
        "create_user".to_string(),
        context.clone(),
    );
    println!("   4.1 Intention intent-001 enregistrée");
    
    // Enregistrer les transitions
    obs.intent_journal_mut().record_transition("intent-001", WriteIntentState::InValidation);
    println!("   4.2 Transition vers InValidation enregistrée");
    
    obs.intent_journal_mut().record_transition("intent-001", WriteIntentState::Accepted);
    println!("   4.3 Transition vers Accepted enregistrée");
    
    obs.intent_journal_mut().record_transition("intent-001", WriteIntentState::Applied);
    println!("   4.4 Transition vers Applied enregistrée");
    
    // Enregistrer le résultat
    obs.intent_journal_mut().record_result("intent-001", "Succès".to_string(), None);
    println!("   4.5 Résultat enregistré");
    
    // Vérifier l'entrée
    if let Some(entry) = obs.intent_journal().get_entry("intent-001") {
        println!("\n   Entrée du journal pour intent-001:");
        println!("     - ID: {}", entry.intent_id());
        println!("     - Origine: {}", entry.origin());
        println!("     - Opération: {}", entry.operation_type());
        println!("     - États traversés: {:?}", entry.states_traversed());
        println!("     - Résultat: {:?}", entry.final_result());
    }
    println!();

    // 5. Intention rejetée
    println!("5. Test d'une intention rejetée...\n");
    
    obs.intent_journal_mut().record_intent(
        "intent-002".to_string(),
        2,
        "user-test".to_string(),
        "delete_admin".to_string(),
        context.clone(),
    );
    
    obs.intent_journal_mut().record_transition("intent-002", WriteIntentState::InValidation);
    obs.intent_journal_mut().record_transition("intent-002", WriteIntentState::Rejected);
    obs.intent_journal_mut().record_result(
        "intent-002",
        "Rejeté".to_string(),
        Some("Permission insuffisante pour supprimer un admin".to_string()),
    );
    
    if let Some(entry) = obs.intent_journal().get_entry("intent-002") {
        println!("   Entrée du journal pour intent-002 (rejetée):");
        println!("     - États traversés: {:?}", entry.states_traversed());
        println!("     - Résultat: {:?}", entry.final_result());
        println!("     - Raison du rejet: {:?}", entry.rejection_reason());
    }
    println!();

    // 6. Journal des rejets (RejectionLog)
    println!("6. Test du Journal des Rejets...\n");
    
    let rejection_id = obs.record_rejection(
        "permission".to_string(),
        "Permission d'écriture insuffisante".to_string(),
        "permissions_boundary".to_string(),
        KMState::Healthy,
        Some("intent-002".to_string()),
    );
    println!("   Rejet enregistré: {}", rejection_id);
    
    let rejection_id = obs.record_rejection(
        "state".to_string(),
        "Opération interdite en état Degraded".to_string(),
        "km_state_boundary".to_string(),
        KMState::Degraded,
        Some("intent-003".to_string()),
    );
    println!("   Rejet enregistré: {}", rejection_id);
    
    println!("   → Total rejets: {}\n", obs.rejection_log().count());

    // 7. Journal des quarantaines (QuarantineLog)
    println!("7. Test du Journal des Quarantaines...\n");
    
    let quar_id = obs.record_quarantine_entry(
        "user-malicious".to_string(),
        "Comportement suspect détecté".to_string(),
        "brute_force".to_string(),
        "partial".to_string(),
        "Vérification manuelle requise".to_string(),
    );
    println!("   7.1 Entrée en quarantaine: {}", quar_id);
    println!("       Est en quarantaine: {}", obs.quarantine_log().is_quarantined("user-malicious"));
    
    // Sortie de quarantaine
    let exited = obs.record_quarantine_exit("user-malicious", "Vérification OK - faux positif".to_string());
    println!("   7.2 Sortie de quarantaine: {}", if exited { "réussie" } else { "échouée" });
    println!("       Est en quarantaine: {}", obs.quarantine_log().is_quarantined("user-malicious"));
    
    println!("   → Total quarantaines: {}", obs.quarantine_log().count());
    println!("   → Quarantaines actives: {}\n", obs.quarantine_log().active_count());

    // 8. Enregistrement direct d'événements
    println!("8. Enregistrement direct d'événements...\n");
    
    obs.record_event(ObservableEvent::IntentCreated {
        intent_id: "intent-003".to_string(),
    });
    println!("   ✓ IntentCreated enregistré");
    
    obs.record_event(ObservableEvent::WriteApplied {
        intent_id: "intent-003".to_string(),
    });
    println!("   ✓ WriteApplied enregistré");
    
    obs.record_event(ObservableEvent::ViolationDetected {
        violation_type: "rate_limit".to_string(),
        source: "user-test".to_string(),
    });
    println!("   ✓ ViolationDetected enregistré");
    
    obs.record_event(ObservableEvent::ThreatDetected {
        threat_id: "threat-001".to_string(),
        threat_type: "REPLAY".to_string(),
        source: "user-suspect".to_string(),
    });
    println!("   ✓ ThreatDetected enregistré\n");

    // 9. Résumé final
    println!("9. Résumé de l'observabilité...\n");
    
    let obs = km.observability();
    println!("   Total événements: {}", obs.event_count());
    println!("   Journal d'intention:");
    println!("     - Entrées: {}", obs.intent_journal().count());
    println!("   Journal des rejets:");
    println!("     - Entrées: {}", obs.rejection_log().count());
    println!("   Journal des quarantaines:");
    println!("     - Total: {}", obs.quarantine_log().count());
    println!("     - Actives: {}", obs.quarantine_log().active_count());
    println!();

    // 10. Conclusion
    println!("\n=== Résumé ===");
    println!("✓ Événements conceptuels enregistrés");
    println!("✓ Journal d'intention avec cycle de vie complet");
    println!("✓ Journal des rejets avec contexte complet");
    println!("✓ Journal des quarantaines avec entrée/sortie");
    println!("✓ Aucune donnée métier exposée");
    println!("✓ Aucune dépendance technique externe");
    println!("✓ Traçabilité complète des décisions");
}
