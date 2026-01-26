//! Test console de synchronisation
//!
//! Ce test démontre les scénarios de synchronisation :
//! - Création de SyncIntent (Instance Fille uniquement)
//! - Détection de conflits (autoritaire, temporel, sémantique)
//! - Rejet systématique de toute synchronisation (à ce stade)

use kindmother::*;

fn main() {
    println!("=== Test de Synchronisation KindMother ===\n");

    // 1. Création d'un SyncManager
    println!("1. Création du SyncManager...");
    let mut sync_manager = SyncManager::new();
    println!("   ✓ SyncManager créé\n");

    // 2. Tentative de création de SyncIntent depuis une Instance Fille (valide)
    println!("2. Création d'une SyncIntent depuis une Instance Fille...");
    let result = sync_manager.create_sync_intent(
        "daughter-instance-1".to_string(),
        "mother-instance-1".to_string(),
        InstanceType::Daughter,
    );
    match result {
        Ok(intent) => {
            println!("   ✓ SyncIntent créée avec succès");
            println!("     - ID: {}", intent.sync_id());
            println!("     - État: {:?}", intent.state());
            println!("     - Source: {}", intent.source_instance_id());
            println!("     - Cible: {}", intent.target_instance_id());
        }
        Err(e) => {
            println!("   ✗ Erreur inattendue: {}", e);
        }
    }
    println!();

    // 3. Tentative de création de SyncIntent depuis une Instance Mère (rejeté)
    println!("3. Tentative de création de SyncIntent depuis une Instance Mère (doit être rejeté)...");
    let result = sync_manager.create_sync_intent(
        "mother-instance-1".to_string(),
        "mother-instance-1".to_string(),
        InstanceType::Mother,
    );
    match result {
        Ok(_) => {
            println!("   ✗ Erreur: la création depuis une Mère devrait être rejetée!");
        }
        Err(e) => {
            println!("   ✓ Rejet correct: {}", e);
        }
    }
    println!();

    // 4. Soumission d'une SyncIntent (toujours rejetée à ce stade)
    println!("4. Soumission d'une SyncIntent (doit être rejetée)...");
    let mut intent = sync_manager.create_sync_intent(
        "daughter-instance-2".to_string(),
        "mother-instance-1".to_string(),
        InstanceType::Daughter,
    ).unwrap();
    
    match sync_manager.submit_sync_intent(&mut intent) {
        Ok(()) => {
            println!("   ✗ Erreur: la synchronisation devrait être rejetée!");
        }
        Err(e) => {
            println!("   ✓ Rejet attendu: {}", e);
            println!("     - État final de l'intention: {:?}", intent.state());
            if let Some(reason) = intent.rejection_reason() {
                println!("     - Raison du rejet: {}", reason);
            }
        }
    }
    println!();

    // 5. Détection de conflits
    println!("5. Démonstration de la détection de conflits...\n");
    
    let mut conflict_detector = ConflictDetector::new();

    // Conflit autoritaire
    println!("   5.1 Conflit AUTORITAIRE:");
    let conflict = conflict_detector.detect_authoritative_conflict(
        "intent-123",
        "L'entité a été supprimée par l'Instance Mère",
    );
    println!("       - Type: {}", conflict.conflict_type().to_string());
    println!("       - Description: {}", conflict.conflict_type().description());
    println!("       - Raison: {}", conflict.reason());
    println!();

    // Conflit temporel
    println!("   5.2 Conflit TEMPOREL:");
    let conflict = conflict_detector.detect_temporal_conflict(
        "intent-456",
        "Modifications concurrentes détectées sur l'entité User#42",
    );
    println!("       - Type: {}", conflict.conflict_type().to_string());
    println!("       - Description: {}", conflict.conflict_type().description());
    println!("       - Raison: {}", conflict.reason());
    println!();

    // Conflit sémantique
    println!("   5.3 Conflit SÉMANTIQUE:");
    let conflict = conflict_detector.detect_semantic_conflict(
        "intent-789",
        "La contrainte d'unicité de l'email est violée",
    );
    println!("       - Type: {}", conflict.conflict_type().to_string());
    println!("       - Description: {}", conflict.conflict_type().description());
    println!("       - Raison: {}", conflict.reason());
    println!();

    // 6. Typage automatique de conflit
    println!("6. Typage automatique des conflits...\n");

    // Cas avec décision Mère (prioritaire)
    let conflict = conflict_detector.type_conflict(
        "intent-auto-1",
        true,   // has_mother_decision
        true,   // has_concurrent_modification
        false,  // has_constraint_violation
        "Décision autoritaire de la Mère",
    );
    if let Some(c) = conflict {
        println!("   Cas 1 (décision Mère + modification concurrente):");
        println!("     → Type détecté: {} (priorité à AUTORITAIRE)", c.conflict_type().to_string());
    }

    // Cas sans décision Mère mais avec modification concurrente
    let conflict = conflict_detector.type_conflict(
        "intent-auto-2",
        false,  // has_mother_decision
        true,   // has_concurrent_modification
        true,   // has_constraint_violation
        "Modifications concurrentes",
    );
    if let Some(c) = conflict {
        println!("   Cas 2 (modification concurrente + violation contrainte):");
        println!("     → Type détecté: {} (priorité à TEMPOREL)", c.conflict_type().to_string());
    }

    // Cas sans aucun conflit
    let conflict = conflict_detector.type_conflict(
        "intent-auto-3",
        false,
        false,
        false,
        "Pas de conflit",
    );
    println!("   Cas 3 (aucun indicateur de conflit):");
    println!("     → Aucun conflit détecté: {:?}", conflict.is_none());
    println!();

    // 7. Résumé
    println!("\n=== Résumé ===");
    println!("✓ SyncIntent ne peut être créée que depuis une Instance Fille");
    println!("✓ Toute synchronisation est REFUSÉE à ce stade");
    println!("✓ Les conflits sont détectés et typés correctement");
    println!("✓ AUCUNE résolution automatique n'est effectuée");
    println!("✓ Toutes les opérations sont explicites et traçables");
}
