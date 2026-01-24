//! Produit pilote mini-cms
//! 
//! Valide la Phase 0 du SPM CMS en utilisant les Modules Contenu, Hiérarchie et Taxonomies.
//! Ce produit sert UNIQUEMENT à valider le SPM, pas à être un produit final.

// Note: Clock et Config sont importés pour démontrer l'initialisation des composants du kernel
// dans la phase de boot, même si les MemoryManagers gèrent leurs propres instances en interne.
// C'est une démonstration pédagogique du processus de boot du produit.
use miyukini_kernel::{DefaultClock, EnvConfig, Level, Logger};
use miyukini_spm_cms_content::{
    ContentFilters, ContentInput, ContentManager, ContentUpdates, MemoryContentManager,
};
use miyukini_spm_cms_hierarchy::{HierarchyManager, MemoryHierarchyManager};
use miyukini_spm_cms_taxonomies::{TaxonomyManager, MemoryTaxonomyManager};

/// Logger simple en stdout pour le produit pilote.
struct StdoutLogger;

impl StdoutLogger {
    fn new() -> Self {
        Self
    }
}

impl Logger for StdoutLogger {
    fn log(&self, level: Level, message: &str) {
        println!("[{}] {}", format!("{:?}", level), message);
    }
}

fn main() {
    println!("=== mini-cms - Produit pilote Phase 0 ===\n");

    // 1. Boot
    println!("1. Boot");
    // Initialisation des composants du kernel pour démontrer le processus de boot.
    // Note: Les MemoryManagers gèrent leurs propres instances de Clock et IdGenerator en interne,
    // mais on initialise ici les composants du kernel pour montrer l'architecture du produit.
    let _config = EnvConfig::from_env();
    let logger = StdoutLogger::new();
    let _clock = DefaultClock::new();
    let content_manager = MemoryContentManager::new();
    let mut hierarchy_manager = MemoryHierarchyManager::new();
    let mut taxonomy_manager = MemoryTaxonomyManager::new();
    
    logger.log(Level::Info, "Composants du kernel initialisés");
    logger.log(Level::Info, "MemoryContentManager initialisé");
    logger.log(Level::Info, "MemoryHierarchyManager initialisé");
    logger.log(Level::Info, "MemoryTaxonomyManager initialisé");
    println!();

    // 2. Création de contenu
    println!("2. Création de contenu");
    
    let accueil_metadata = "Accueil".as_bytes().to_vec();
    let accueil_input = ContentInput::new("page".to_string(), accueil_metadata.clone());
    let accueil_id = content_manager.create_content(accueil_input).expect("Création Accueil");
    logger.log(Level::Info, &format!("Contenu créé - ID: {:?}, Titre: Accueil", accueil_id));
    
    let apropos_metadata = "À propos".as_bytes().to_vec();
    let apropos_input = ContentInput::new("page".to_string(), apropos_metadata.clone());
    let apropos_id = content_manager.create_content(apropos_input).expect("Création À propos");
    logger.log(Level::Info, &format!("Contenu créé - ID: {:?}, Titre: À propos", apropos_id));
    println!();

    // 3. Hiérarchie
    println!("3. Hiérarchie");
    
    // Créer une racine hiérarchique pour "Accueil"
    let accueil_node_id = hierarchy_manager.create_root(accueil_id);
    logger.log(Level::Info, &format!("Racine hiérarchique créée pour 'Accueil' - Node ID: {:?}", accueil_node_id));
    
    // Ajouter "À propos" comme enfant de "Accueil"
    let apropos_node_id = hierarchy_manager.create_child(accueil_node_id, apropos_id)
        .expect("Création nœud enfant pour À propos");
    logger.log(Level::Info, &format!("Nœud enfant créé pour 'À propos' - Node ID: {:?}", apropos_node_id));
    
    // Afficher le parent de "À propos"
    let parent = hierarchy_manager.parent(apropos_node_id);
    if let Some(parent_id) = parent {
        println!("Parent de 'À propos': Node ID {:?}", parent_id);
        logger.log(Level::Info, &format!("Parent de 'À propos' vérifié: {:?}", parent_id));
    }
    
    // Afficher le chemin racine → "À propos"
    let path = hierarchy_manager.path_to_root(apropos_node_id);
    println!("Chemin racine → 'À propos':");
    for (i, node_id) in path.iter().enumerate() {
        let indent = "  ".repeat(i);
        println!("  {}{:?}", indent, node_id);
    }
    logger.log(Level::Info, &format!("Chemin vers racine affiché ({} nœuds)", path.len()));
    println!();

    // 4. Taxonomies
    println!("4. Taxonomies");
    
    // Créer une taxonomie "Tags"
    let tags_taxonomy_id = taxonomy_manager.create_taxonomy("Tags".to_string());
    logger.log(Level::Info, &format!("Taxonomie 'Tags' créée - ID: {:?}", tags_taxonomy_id));
    
    // Ajouter les termes "News" et "Tech"
    let news_term_id = taxonomy_manager.add_term(tags_taxonomy_id, "News".to_string())
        .expect("Ajout terme News");
    logger.log(Level::Info, &format!("Terme 'News' créé - ID: {:?}", news_term_id));
    
    let tech_term_id = taxonomy_manager.add_term(tags_taxonomy_id, "Tech".to_string())
        .expect("Ajout terme Tech");
    logger.log(Level::Info, &format!("Terme 'Tech' créé - ID: {:?}", tech_term_id));
    
    // Assigner "News" → "Accueil"
    taxonomy_manager.assign_term(news_term_id, accueil_id)
        .expect("Assignation News → Accueil");
    logger.log(Level::Info, "Terme 'News' assigné à 'Accueil'");
    
    // Assigner "Tech" → "À propos"
    taxonomy_manager.assign_term(tech_term_id, apropos_id)
        .expect("Assignation Tech → À propos");
    logger.log(Level::Info, "Terme 'Tech' assigné à 'À propos'");
    
    // Afficher les termes de chaque entité
    let accueil_terms = taxonomy_manager.terms_for_entity(accueil_id);
    println!("Termes de 'Accueil': {} terme(s)", accueil_terms.len());
    for term_id in &accueil_terms {
        println!("  - Terme ID: {:?}", term_id);
    }
    
    let apropos_terms = taxonomy_manager.terms_for_entity(apropos_id);
    println!("Termes de 'À propos': {} terme(s)", apropos_terms.len());
    for term_id in &apropos_terms {
        println!("  - Terme ID: {:?}", term_id);
    }
    println!();

    // 5. Listing
    println!("5. Listing");
    let filters = ContentFilters::new();
    let list_result = content_manager
        .list_contents(filters, 0, 10)
        .expect("Listing des contenus");
    
    println!("Contenus listés (offset=0, limit=10):");
    for content in &list_result.items {
        let titre = String::from_utf8_lossy(&content.metadata);
        println!("  - {} (ID: {:?}, Type: {}, Statut: {:?})", 
                 titre, content.id, content.content_type, content.status);
    }
    println!("Total: {}", list_result.total);
    println!();

    // 6. Mise à jour
    println!("6. Mise à jour");
    
    // Mise à jour sans création de version
    let updates_sans_version = ContentUpdates::new()
        .with_metadata("Accueil - Mis à jour".as_bytes().to_vec());
    content_manager
        .update_content(accueil_id, updates_sans_version, false)
        .expect("Mise à jour Accueil sans version");
    logger.log(Level::Info, "Accueil mis à jour (sans version)");
    
    // Mise à jour avec création de version
    let updates_avec_version = ContentUpdates::new()
        .with_metadata("Accueil - Version finale".as_bytes().to_vec());
    content_manager
        .update_content(accueil_id, updates_avec_version, true)
        .expect("Mise à jour Accueil avec version");
    logger.log(Level::Info, "Accueil mis à jour (avec version créée)");
    println!();

    // 7. Relations
    println!("7. Relations");
    content_manager
        .add_relation(apropos_id, "related".to_string(), accueil_id)
        .expect("Ajout relation");
    logger.log(Level::Info, &format!("Relation créée : À propos -> Accueil (type: related)"));
    
    let relations = content_manager
        .list_relations(apropos_id)
        .expect("Liste des relations");
    println!("Relations de 'À propos': {}", relations.len());
    for relation in &relations {
        println!("  - Type: {}, Target ID: {:?}", relation.relation_type, relation.target_id);
    }
    println!();

    // 8. Versioning
    println!("8. Versioning");
    let versions = content_manager
        .list_versions(accueil_id)
        .expect("Liste des versions");
    logger.log(Level::Info, &format!("Versions de 'Accueil': {}", versions.len()));
    for version in &versions {
        println!("  - Version ID: {:?}, Créée le: {:?}", version.version_id, version.created_at);
    }
    println!();

    // 9. Compte-rendu final (CR)
    println!("9. Compte-rendu final");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    // Contenus
    let final_list = content_manager
        .list_contents(ContentFilters::new(), 0, 10)
        .expect("Liste finale");
    println!("CONTENUS:");
    println!("  - Nombre de contenus: {}", final_list.total);
    
    let final_versions = content_manager
        .list_versions(accueil_id)
        .expect("Versions finales");
    println!("  - Versions pour 'Accueil': {}", final_versions.len());
    
    // Hiérarchie
    println!("HIÉRARCHIE:");
    let accueil_children = hierarchy_manager.children(accueil_node_id);
    println!("  - Enfants de 'Accueil': {} nœud(s)", accueil_children.len());
    let apropos_ancestors = hierarchy_manager.ancestors(apropos_node_id);
    println!("  - Ancêtres de 'À propos': {} nœud(s)", apropos_ancestors.len());
    
    // Taxonomies
    println!("TAXONOMIES:");
    let accueil_final_terms = taxonomy_manager.terms_for_entity(accueil_id);
    println!("  - Termes de 'Accueil': {} terme(s)", accueil_final_terms.len());
    let apropos_final_terms = taxonomy_manager.terms_for_entity(apropos_id);
    println!("  - Termes de 'À propos': {} terme(s)", apropos_final_terms.len());
    let news_entities = taxonomy_manager.entities_for_term(news_term_id);
    println!("  - Entités avec 'News': {} entité(s)", news_entities.len());
    let tech_entities = taxonomy_manager.entities_for_term(tech_term_id);
    println!("  - Entités avec 'Tech': {} entité(s)", tech_entities.len());
    
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Statut global: SUCCESS");
    println!("Phase 0 validée: Contenu ✓ | Hiérarchie ✓ | Taxonomies ✓");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
