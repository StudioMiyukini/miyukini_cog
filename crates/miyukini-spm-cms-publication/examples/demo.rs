//! Démonstration console du Module Publication.
//!
//! Exemple d'utilisation du MemoryPublicationManager pour créer une publication,
//! la programmer, la publier, et l'archiver.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_publication::{
    PublicationError, PublicationManager, PublicationStatus, MemoryPublicationManager,
};
use std::time::{Duration, SystemTime};

fn main() -> Result<(), PublicationError> {
    let manager = MemoryPublicationManager::new();
    let id_gen = UuidIdGenerator::new();

    println!("=== Démonstration du Module Publication ===\n");

    // 1. Création d'une publication
    println!("1. Création d'une publication...");
    let content_id = id_gen.generate();
    println!("   ✓ Contenu créé: {}", content_id);

    let publication_id = manager.create_publication(content_id)?;
    println!("   ✓ Publication créée: {}", publication_id);

    let status = manager.status(publication_id)?;
    println!("   ✓ Statut initial: {:?}", status);
    assert_eq!(status, PublicationStatus::Draft);

    // 2. Programmer une publication future
    println!("\n2. Programmation d'une publication future...");
    let future = SystemTime::now() + Duration::from_secs(3600);
    manager.schedule(publication_id, future)?;
    println!("   ✓ Publication programmée pour: {:?}", future);

    let status = manager.status(publication_id)?;
    println!("   ✓ Statut: {:?}", status);
    assert_eq!(status, PublicationStatus::Scheduled);

    // 3. Statut avant publication
    println!("\n3. Statut effectif avant la date de publication...");
    let effective = manager.effective_status(publication_id)?;
    println!("   ✓ Statut effectif: {:?}", effective);
    assert_eq!(effective, PublicationStatus::Scheduled);

    // 4. Publication immédiate (depuis Draft)
    println!("\n4. Création d'une deuxième publication et publication immédiate...");
    let content2_id = id_gen.generate();
    let publication2_id = manager.create_publication(content2_id)?;
    println!("   ✓ Publication 2 créée: {}", publication2_id);

    manager.publish_now(publication2_id)?;
    println!("   ✓ Publication 2 publiée immédiatement");

    let status2 = manager.status(publication2_id)?;
    println!("   ✓ Statut: {:?}", status2);
    assert_eq!(status2, PublicationStatus::Published);

    // Simuler le passage du temps pour la première publication
    println!("\n5. Simulation du passage du temps...");
    println!("   (Dans un vrai système, un job vérifierait les publications Scheduled)");
    
    // Pour la démo, on ne peut pas vraiment simuler le temps, mais on peut montrer
    // que effective_status fonctionne correctement
    let effective = manager.effective_status(publication_id)?;
    println!("   ✓ Statut effectif de la publication 1: {:?}", effective);

    // Publier la première publication maintenant (simule le job qui détecte que c'est l'heure)
    println!("\n6. Publication de la première publication (simulation job)...");
    // Note: dans la vraie vie, on vérifierait d'abord que now >= publish_at
    // Ici, on simule en programmant pour le passé immédiat
    // On ne peut pas modifier directement, donc on crée une nouvelle publication pour la démo
    let content3_id = id_gen.generate();
    let publication3_id = manager.create_publication(content3_id)?;
    let near_future = SystemTime::now() + Duration::from_millis(10);
    manager.schedule(publication3_id, near_future)?;
    
    // Attendre un peu
    std::thread::sleep(Duration::from_millis(50));
    
    // Vérifier le statut effectif
    let effective = manager.effective_status(publication3_id)?;
    println!("   ✓ Statut effectif après passage du temps: {:?}", effective);
    if effective == PublicationStatus::Published {
        manager.publish_now(publication3_id)?;
        println!("   ✓ Publication 3 publiée (statut effectif détecté)");
    }

    // 5. Archiver une publication
    println!("\n7. Archivage d'une publication...");
    manager.archive(publication2_id)?;
    println!("   ✓ Publication 2 archivée");

    let status2 = manager.status(publication2_id)?;
    println!("   ✓ Statut final: {:?}", status2);
    assert_eq!(status2, PublicationStatus::Archived);

    // Tentative d'opération sur publication archivée (doit échouer)
    println!("\n8. Tentative d'opération sur publication archivée...");
    match manager.publish_now(publication2_id) {
        Err(PublicationError::InvalidTransition) => {
            println!("   ✓ Transition invalide refusée (comportement attendu)");
        }
        Ok(_) => {
            println!("   ✗ ERREUR: La transition devrait être refusée !");
            return Err(PublicationError::Other("invalid transition allowed".to_string()));
        }
        Err(e) => {
            println!("   ✗ ERREUR inattendue: {:?}", e);
            return Err(e);
        }
    }

    println!("\n=== Démonstration terminée avec succès ===");
    Ok(())
}
