//! Tests console du Write Intent Lifecycle
//!
//! Conforme au contrat FONDATION "KindMother — Write Intent Lifecycle Contract"
//!
//! Tests demandés :
//! 1. Création → validation → acceptation → application → archivage
//! 2. Tentative de double validation (rejet)
//! 3. Tentative de double application (rejet)
//! 4. Tentative de réutilisation après archivage (rejet)

use kindmother::core::WriteIntent;

fn main() {
    println!("=== Tests Console - Write Intent Lifecycle ===\n");
    println!("Conforme au contrat FONDATION :");
    println!("  \"KindMother — Write Intent Lifecycle Contract\"\n");

    // Test 1 : Cycle de vie complet (création → validation → acceptation → application → archivage)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 1 : Cycle de vie complet");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut intent1 = WriteIntent::new(
        "intent-001".to_string(),
        "create_entity".to_string(),
    );
    println!("État initial: {:?}\n", intent1.state());

    // CRÉÉE → EN_VALIDATION
    println!("→ Démarrage de la validation...");
    match intent1.start_validation() {
        Ok(()) => println!("  ✓ Transition CRÉÉE → EN_VALIDATION réussie"),
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }
    println!("  État actuel: {:?}\n", intent1.state());

    // EN_VALIDATION → ACCEPTÉE
    println!("→ Acceptation de l'intention...");
    match intent1.accept() {
        Ok(()) => println!("  ✓ Transition EN_VALIDATION → ACCEPTÉE réussie"),
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }
    println!("  État actuel: {:?}\n", intent1.state());

    // ACCEPTÉE → APPLIQUÉE
    println!("→ Application de l'intention...");
    match intent1.apply() {
        Ok(()) => println!("  ✓ Transition ACCEPTÉE → APPLIQUÉE réussie"),
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }
    println!("  État actuel: {:?}\n", intent1.state());

    // APPLIQUÉE → ARCHIVÉE
    println!("→ Archivage de l'intention...");
    match intent1.archive() {
        Ok(()) => println!("  ✓ Transition APPLIQUÉE → ARCHIVÉE réussie"),
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }
    println!("  État final: {:?}\n", intent1.state());
    println!("  ✓ Cycle de vie complet terminé avec succès\n");

    // Test 2 : Tentative de double validation (rejet)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 2 : Tentative de double validation (doit être rejetée)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut intent2 = WriteIntent::new(
        "intent-002".to_string(),
        "update_entity".to_string(),
    );
    println!("État initial: {:?}\n", intent2.state());

    // Première validation (doit réussir)
    println!("→ Première validation...");
    match intent2.start_validation() {
        Ok(()) => {
            println!("  ✓ Première validation réussie");
            println!("  État actuel: {:?}\n", intent2.state());
        }
        Err(e) => {
            println!("  ✗ Échec inattendu: {}", e);
            return;
        }
    }

    // Tentative de double validation (doit être rejetée)
    println!("→ Tentative de double validation (doit être rejetée)...");
    match intent2.start_validation() {
        Ok(()) => {
            println!("  ✗ ERREUR : La double validation a été acceptée (devrait être rejetée)");
            return;
        }
        Err(e) => {
            println!("  ✓ Double validation correctement rejetée");
            println!("  Erreur: {}\n", e);
        }
    }
    println!("  État inchangé: {:?}\n", intent2.state());

    // Test 3 : Tentative de double application (rejet)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 3 : Tentative de double application (doit être rejetée)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut intent3 = WriteIntent::new(
        "intent-003".to_string(),
        "delete_entity".to_string(),
    );
    println!("État initial: {:?}\n", intent3.state());

    // Transition vers ACCEPTÉE
    intent3.start_validation().unwrap();
    intent3.accept().unwrap();
    println!("État après acceptation: {:?}\n", intent3.state());

    // Première application (doit réussir)
    println!("→ Première application...");
    match intent3.apply() {
        Ok(()) => {
            println!("  ✓ Première application réussie");
            println!("  État actuel: {:?}\n", intent3.state());
        }
        Err(e) => {
            println!("  ✗ Échec inattendu: {}", e);
            return;
        }
    }

    // Tentative de double application (doit être rejetée)
    println!("→ Tentative de double application (doit être rejetée)...");
    match intent3.apply() {
        Ok(()) => {
            println!("  ✗ ERREUR : La double application a été acceptée (devrait être rejetée)");
            return;
        }
        Err(e) => {
            println!("  ✓ Double application correctement rejetée");
            println!("  Erreur: {}\n", e);
        }
    }
    println!("  État inchangé: {:?}\n", intent3.state());

    // Test 4 : Tentative de réutilisation après archivage (rejet)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 4 : Tentative de réutilisation après archivage (doit être rejetée)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut intent4 = WriteIntent::new(
        "intent-004".to_string(),
        "create_relation".to_string(),
    );
    println!("État initial: {:?}\n", intent4.state());

    // Cycle complet jusqu'à archivage
    intent4.start_validation().unwrap();
    intent4.accept().unwrap();
    intent4.apply().unwrap();
    intent4.archive().unwrap();
    println!("État après archivage: {:?}\n", intent4.state());

    // Vérification de la non-réutilisation
    println!("→ Vérification de la non-réutilisation...");
    if intent4.can_reuse() {
        println!("  ✗ ERREUR : L'intention archivée peut être réutilisée (ne devrait pas)");
        return;
    } else {
        println!("  ✓ L'intention archivée ne peut pas être réutilisée");
    }

    // Tentative de validation après archivage (doit être rejetée)
    println!("→ Tentative de validation après archivage (doit être rejetée)...");
    match intent4.start_validation() {
        Ok(()) => {
            println!("  ✗ ERREUR : La validation après archivage a été acceptée (devrait être rejetée)");
            return;
        }
        Err(e) => {
            println!("  ✓ Validation après archivage correctement rejetée");
            println!("  Erreur: {}\n", e);
        }
    }

    // Tentative d'application après archivage (doit être rejetée)
    println!("→ Tentative d'application après archivage (doit être rejetée)...");
    match intent4.apply() {
        Ok(()) => {
            println!("  ✗ ERREUR : L'application après archivage a été acceptée (devrait être rejetée)");
            return;
        }
        Err(e) => {
            println!("  ✓ Application après archivage correctement rejetée");
            println!("  Erreur: {}\n", e);
        }
    }
    println!("  État inchangé: {:?}\n", intent4.state());

    // Test 5 : Cycle avec rejet (CRÉÉE → EN_VALIDATION → REJETÉE → ARCHIVÉE)
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("TEST 5 : Cycle avec rejet");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut intent5 = WriteIntent::new(
        "intent-005".to_string(),
        "invalid_operation".to_string(),
    );
    println!("État initial: {:?}\n", intent5.state());

    // CRÉÉE → EN_VALIDATION
    intent5.start_validation().unwrap();
    println!("État après démarrage validation: {:?}\n", intent5.state());

    // EN_VALIDATION → REJETÉE
    println!("→ Rejet de l'intention...");
    match intent5.reject("Validation échouée: opération invalide".to_string()) {
        Ok(()) => {
            println!("  ✓ Transition EN_VALIDATION → REJETÉE réussie");
            println!("  État actuel: {:?}\n", intent5.state());
        }
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }

    // REJETÉE → ARCHIVÉE
    println!("→ Archivage de l'intention rejetée...");
    match intent5.archive() {
        Ok(()) => {
            println!("  ✓ Transition REJETÉE → ARCHIVÉE réussie");
            println!("  État final: {:?}\n", intent5.state());
        }
        Err(e) => {
            println!("  ✗ Échec: {}", e);
            return;
        }
    }

    // Résumé final
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("RÉSUMÉ DES TESTS");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    println!("✓ Test 1 : Cycle de vie complet (création → validation → acceptation → application → archivage)");
    println!("✓ Test 2 : Tentative de double validation (correctement rejetée)");
    println!("✓ Test 3 : Tentative de double application (correctement rejetée)");
    println!("✓ Test 4 : Tentative de réutilisation après archivage (correctement rejetée)");
    println!("✓ Test 5 : Cycle avec rejet (création → validation → rejet → archivage)");
    println!("\nTous les tests sont conformes au contrat FONDATION.");
    println!("Aucune violation contractuelle détectée.\n");
}
