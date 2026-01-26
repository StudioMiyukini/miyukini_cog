//! Démonstration console du Module Médias.
//!
//! Exemple d'utilisation du MemoryMediaManager pour créer des médias,
//! les attacher à des entités, et lister les médias d'une entité.

use miyukini_kernel::{IdGenerator, UuidIdGenerator};
use miyukini_spm_cms_media::{MediaError, MediaInput, MediaManager, MemoryMediaManager};

fn main() -> Result<(), MediaError> {
    let manager = MemoryMediaManager::new();
    let id_gen = UuidIdGenerator::new();

    println!("=== Démonstration du Module Médias ===\n");

    // Créer des médias
    println!("1. Création de médias...");
    let image_id = manager.create_media(MediaInput::new(
        "image/jpeg".to_string(),
        b"{\"width\": 1920, \"height\": 1080}".to_vec(),
    ))?;
    println!("   ✓ Image JPEG créée: {}", image_id);

    let video_id = manager.create_media(MediaInput::new(
        "video/mp4".to_string(),
        b"{\"duration\": 120, \"resolution\": \"1080p\"}".to_vec(),
    ))?;
    println!("   ✓ Vidéo MP4 créée: {}", video_id);

    let pdf_id = manager.create_media(MediaInput::new(
        "application/pdf".to_string(),
        b"{\"pages\": 10}".to_vec(),
    ))?;
    println!("   ✓ PDF créé: {}", pdf_id);

    // Lire un média
    println!("\n2. Lecture d'un média...");
    let media = manager.get_media(image_id)?;
    println!("   ✓ Média récupéré:");
    println!("     - ID: {}", media.id);
    println!("     - Type MIME: {}", media.mime_type);
    println!("     - Métadonnées: {} bytes", media.metadata.len());
    println!("     - Créé le: {:?}", media.created_at);

    // Créer une entité
    println!("\n3. Création d'une entité...");
    let entity_id = id_gen.generate();
    println!("   ✓ Entité créée: {}", entity_id);

    // Attacher des médias à l'entité
    println!("\n4. Attachement de médias à l'entité...");
    manager.attach_media_to_entity(image_id, entity_id)?;
    println!("   ✓ Image attachée à l'entité");

    manager.attach_media_to_entity(video_id, entity_id)?;
    println!("   ✓ Vidéo attachée à l'entité");

    // Lister les médias de l'entité
    println!("\n5. Liste des médias attachés à l'entité:");
    let media_list = manager.list_media_for_entity(entity_id)?;
    println!("   Nombre de médias: {}", media_list.len());
    for media in &media_list {
        println!("   - {} ({})", media.id, media.mime_type);
    }

    // Créer une deuxième entité et attacher un média
    println!("\n6. Création d'une deuxième entité...");
    let entity2_id = id_gen.generate();
    println!("   ✓ Entité 2 créée: {}", entity2_id);

    manager.attach_media_to_entity(pdf_id, entity2_id)?;
    println!("   ✓ PDF attaché à l'entité 2");

    // Attacher le même média à plusieurs entités
    println!("\n7. Attachement du même média à plusieurs entités...");
    manager.attach_media_to_entity(image_id, entity2_id)?;
    println!("   ✓ Image attachée à l'entité 2 (même média que l'entité 1)");

    let media_list2 = manager.list_media_for_entity(entity2_id)?;
    println!("   Médias de l'entité 2: {}", media_list2.len());

    // Détacher un média
    println!("\n8. Détachement d'un média...");
    manager.detach_media_from_entity(video_id, entity_id)?;
    println!("   ✓ Vidéo détachée de l'entité 1");

    let media_list_after = manager.list_media_for_entity(entity_id)?;
    println!("   Médias restants pour l'entité 1: {}", media_list_after.len());

    // Supprimer un média
    println!("\n9. Suppression d'un média...");
    manager.delete_media(pdf_id)?;
    println!("   ✓ PDF supprimé");

    // Vérifier que le média n'est plus attaché
    let media_list2_after = manager.list_media_for_entity(entity2_id)?;
    println!("   Médias restants pour l'entité 2: {}", media_list2_after.len());

    // Tenter de récupérer le média supprimé (doit échouer)
    println!("\n10. Tentative de récupérer le média supprimé...");
    match manager.get_media(pdf_id) {
        Err(MediaError::NotFound) => {
            println!("   ✓ Média introuvable (comportement attendu)");
        }
        Ok(_) => {
            println!("   ✗ ERREUR: Le média existe encore !");
            return Err(MediaError::Other("media should not exist".to_string()));
        }
        Err(e) => {
            println!("   ✗ ERREUR inattendue: {:?}", e);
            return Err(e);
        }
    }

    println!("\n=== Démonstration terminée avec succès ===");
    Ok(())
}
