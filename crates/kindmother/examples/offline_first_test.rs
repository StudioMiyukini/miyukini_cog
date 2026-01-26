//! Test console du scénario offline-first
//!
//! Ce test démontre le comportement offline-first :
//! - Toutes les opérations fonctionnent localement
//! - Pas de dépendance réseau
//! - Synchronisation explicite (et refusée à ce stade)
//! - Persistance locale autoritaire

use kindmother::*;

fn main() {
    println!("=== Test Offline-First KindMother ===\n");
    println!("Ce test démontre que KindMother fonctionne en mode offline-first:");
    println!("- Toutes les opérations sont locales");
    println!("- Aucune dépendance réseau");
    println!("- La synchronisation est explicite (et refusée à ce stade)\n");

    // 1. Création d'une Instance Fille (offline)
    println!("1. Création d'une Instance Fille (offline)...");
    let mut km = KindMother::with_identity(
        "daughter-offline".to_string(),
        "local-domain".to_string(),
    );
    km.recover_to_healthy("Démarrage local").unwrap();
    println!("   ✓ Instance Fille créée et démarrée localement");
    println!("   Instance: {}", km.instance_id());
    println!("   Domaine: {}", km.domain_id());
    println!("   État: {:?}\n", km.state());

    // 2. Création et soumission d'intentions locales
    println!("2. Création d'intentions locales (sans réseau)...\n");
    
    let mut api = CoreDataAPI::new(km.state());
    
    let authority = AuthorityContext::with_permissions(
        "local-user".to_string(),
        vec![],
        true,   // can_read
        true,   // can_write
        false,  // can_sync (toujours false)
    );
    let instance = InstanceContext::new(
        "daughter-offline".to_string(),
        InstanceType::Daughter,
    );
    let domain = DomainContext::new(
        "local-domain".to_string(),
        "TestDomain".to_string(),
    );

    // Intention 1 - Création
    let intent = WriteIntent::new("local-intent-001".to_string(), "create_entity".to_string());
    match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   2.1 Intent local-intent-001: ✓ Acceptée (traitement local)"),
        Err(e) => println!("   2.1 Intent local-intent-001: ✗ Rejetée - {}", e),
    }

    // Intention 2 - Mise à jour
    let intent = WriteIntent::new("local-intent-002".to_string(), "update_entity".to_string());
    match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   2.2 Intent local-intent-002: ✓ Acceptée (traitement local)"),
        Err(e) => println!("   2.2 Intent local-intent-002: ✗ Rejetée - {}", e),
    }

    // Lecture locale
    match api.read_entity("local-entity-001", authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   2.3 Lecture locale: ✓ Autorisée"),
        Err(e) => println!("   2.3 Lecture locale: ✗ Rejetée - {}", e),
    }
    println!();

    // 3. Persistance locale
    println!("3. Persistance locale (sans réseau)...\n");
    
    let mut intent = WriteIntent::new("persist-intent-001".to_string(), "create_user".to_string());
    intent.start_validation().unwrap();
    intent.accept().unwrap();
    intent.apply().unwrap();
    
    match km.persist_intent(intent, "local-domain") {
        Ok(()) => {
            println!("   ✓ Intention persistée localement");
            println!("   Intentions dans le storage local: {}", km.count_persisted_intents("local-domain"));
        }
        Err(e) => println!("   ✗ Erreur de persistance: {}", e),
    }
    println!();

    // 4. Tentative de synchronisation (refusée)
    println!("4. Tentative de synchronisation (doit être refusée)...\n");
    println!("   Note: La synchronisation est explicite et TOUJOURS refusée à ce stade.\n");
    
    let mut sync_manager = SyncManager::new();
    let mut sync_intent = sync_manager.create_sync_intent(
        "daughter-offline".to_string(),
        "mother-instance".to_string(),
        InstanceType::Daughter,
    ).unwrap();
    
    match sync_manager.submit_sync_intent(&mut sync_intent) {
        Ok(()) => println!("   ✗ Synchronisation acceptée (ne devrait pas arriver!)"),
        Err(e) => {
            println!("   ✓ Synchronisation explicitement refusée");
            println!("   Raison: {}", e);
            println!("   État de la SyncIntent: {:?}", sync_intent.state());
        }
    }
    println!();

    // 5. Vérification que les permissions sync sont toujours false
    println!("5. Vérification des permissions (can_sync = false)...\n");
    
    let auth_with_sync = AuthorityContext::with_permissions(
        "user-with-sync".to_string(),
        vec![],
        true,   // can_read
        true,   // can_write
        true,   // can_sync (même si true dans l'AuthorityContext)
    );
    
    println!("   AuthorityContext créé avec can_sync=true");
    println!("   Mais les opérations de sync restent refusées (règle contractuelle):");
    
    // La vérification via RuntimeBoundary rejette toujours sync
    let sync_result = kindmother::runtime::RuntimeBoundary::check_permissions_boundary(
        &auth_with_sync,
        "sync_data",
        true,
    );
    match sync_result {
        Ok(()) => println!("   ✗ Sync autorisée (erreur!)"),
        Err(e) => println!("   ✓ Sync refusée: {}", e),
    }
    println!();

    // 6. Fonctionnement dégradé (offline continu)
    println!("6. Fonctionnement en mode dégradé (offline continu)...\n");
    
    // Simuler un problème de connectivité (représenté par Degraded)
    km.transition_to_degraded("Connectivité perdue (simulé)").unwrap();
    println!("   6.1 Passage en mode Degraded (connectivité perdue)");
    println!("       État: {:?}", km.state());
    
    // Les lectures restent autorisées
    let mut api_degraded = CoreDataAPI::new(km.state());
    match api_degraded.read_entity("entity", authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   6.2 Lecture en mode Degraded: ✓ Autorisée"),
        Err(e) => println!("   6.2 Lecture en mode Degraded: ✗ Rejetée - {}", e),
    }
    
    // Les écritures sont refusées en mode Degraded
    let intent = WriteIntent::new("degraded-intent".to_string(), "create".to_string());
    match api_degraded.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   6.3 Écriture en mode Degraded: ✓ Acceptée (ne devrait pas arriver!)"),
        Err(e) => println!("   6.3 Écriture en mode Degraded: ✗ Refusée (correct) - {}", e),
    }
    println!();

    // 7. Récupération après reconnexion
    println!("7. Récupération après reconnexion...\n");
    
    km.recover_to_healthy("Connectivité rétablie").unwrap();
    println!("   ✓ Passage en mode Healthy");
    println!("   État: {:?}", km.state());
    
    // Les écritures sont à nouveau autorisées
    let mut api_healthy = CoreDataAPI::new(km.state());
    let intent = WriteIntent::new("post-recovery-intent".to_string(), "create".to_string());
    match api_healthy.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   ✓ Écriture après récupération: acceptée"),
        Err(e) => println!("   ✗ Écriture après récupération: refusée - {}", e),
    }
    println!();

    // 8. Observabilité des événements offline
    println!("8. Observabilité (événements enregistrés offline)...");
    println!("   Total événements: {}", km.observability().event_count());
    println!("   Événements Lifecycle: {}", km.observability().events_by_category(EventCategory::Lifecycle).len());
    println!("   Événements Failure: {}", km.observability().events_by_category(EventCategory::Failure).len());
    println!();

    // 9. Résumé
    println!("\n=== Résumé Offline-First ===");
    println!("✓ Instance Fille fonctionne entièrement en local");
    println!("✓ Aucune opération ne dépend du réseau");
    println!("✓ La synchronisation est EXPLICITE (non automatique)");
    println!("✓ La synchronisation est REFUSÉE à ce stade");
    println!("✓ Persistance locale autoritaire");
    println!("✓ Mode Degraded pour perte de connectivité (lectures seules)");
    println!("✓ Récupération possible après reconnexion");
    println!("✓ Toutes les décisions sont traçables localement");
    println!("\nKindMother est conçu pour fonctionner SANS connectivité.");
    println!("La synchronisation viendra dans une version future, de manière EXPLICITE.");
}
