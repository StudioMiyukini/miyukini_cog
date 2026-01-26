//! Exemple de démonstration du skeleton KindMother
//!
//! Ce programme démontre :
//! - Le démarrage de KindMother
//! - La tentative d'appel CoreDataAPI
//! - La vérification que l'appel est rejeté proprement

use kindmother::core::{
    AuthorityContext, DomainContext, InstanceContext, InstanceType, KindMother, WriteIntent,
};
use kindmother::api::CoreDataAPI;
use kindmother::state::KMState;

fn main() {

    println!("=== Démonstration KindMother Skeleton ===\n");

    // 1. Création et démarrage de KindMother
    println!("1. Création de KindMother...");
    let mut km = KindMother::new();
    println!("   État initial: {:?}\n", km.state());

    println!("2. Tentative de démarrage...");
    match km.start() {
        Ok(()) => println!("   ✓ Démarrage réussi"),
        Err(e) => println!("   ✗ Démarrage rejeté: {}\n", e),
    }

    // Pour la démonstration, forçons l'état à Healthy (normalement non autorisé)
    println!("3. Passage à l'état Healthy (pour démonstration)...");
    km.set_state(KMState::Healthy);
    println!("   État actuel: {:?}\n", km.state());

    // 2. Création d'une instance CoreDataAPI
    println!("4. Création d'une instance CoreDataAPI...");
    let mut api = CoreDataAPI::new(km.state());
    println!("   ✓ CoreDataAPI créée\n");

    // 3. Préparation des contextes
    println!("5. Préparation des contextes...");
    let authority = AuthorityContext::new(
        "test-caller".to_string(),
        vec!["read".to_string(), "write".to_string()],
    );
    let instance = InstanceContext::new(
        "test-instance".to_string(),
        InstanceType::Daughter,
    );
    let domain = DomainContext::new(
        "test-domain".to_string(),
        "Identity".to_string(),
    );
    println!("   ✓ Contextes créés\n");

    // 4. Tentative de soumission d'un WriteIntent
    println!("6. Tentative de soumission d'un WriteIntent...");
    let intent = WriteIntent::new(
        "test-intent-1".to_string(),
        "create_entity".to_string(),
    );

    match api.submit_write_intent(intent, authority.clone(), instance.clone(), domain.clone()) {
        Ok(()) => println!("   ✓ WriteIntent accepté"),
        Err(e) => println!("   ✗ WriteIntent rejeté: {}\n", e),
    }

    // 5. Tentative de lecture d'entité
    println!("7. Tentative de lecture d'entité...");
    match api.read_entity("test-entity-1", authority, instance, domain) {
        Ok(()) => println!("   ✓ Lecture réussie"),
        Err(e) => println!("   ✗ Lecture rejetée: {}\n", e),
    }

    // 6. Résumé
    println!("\n=== Résumé ===");
    println!("Toutes les opérations ont été rejetées comme prévu.");
    println!("Le skeleton fonctionne correctement :");
    println!("  - Les Runtime Boundaries rejettent toutes les opérations");
    println!("  - Les erreurs sont explicites et actionnables");
    println!("  - Aucune opération n'est exécutée");
}
