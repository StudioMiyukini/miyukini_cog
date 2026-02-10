//! Router EXP — écrans Espace Exposant (XP-E01 à XP-E14).
//!
//! @id: router_exp
//! @do: dispatch_exp_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran EXP correspondant à l'identifiant ; délègue à chaque module e01..e14.
//!
//! @id: exp_navigate
//! @do: change_current_screen_from_exp
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell).

mod e01_dashboard;
mod e02_fiche_entreprise;
mod e03_catalogue_liste;
mod e04_fiche_produit;
mod e05_categories;
mod e06_vitrine_params;
mod e07_vitrine_presentation;
mod e08_vitrine_preview;
mod e09_documents;
mod e10_upload_document;
mod e11_fiche_publique;
mod e12_partage_documents;
mod e13_cms_articles;
mod e14_cms_edit_article;

use std::cell::RefCell;
use std::sync::Arc;

use crate::data::{
    CategorieProduit, CmsArticle, CmsCategory, ConfidentialiteProfil, DocumentPartage,
    DocumentProfessionnel, ExposantProfile, JayXposeDb, PageBuilderBlock, PosStockLink,
    ProduitCatalogue, StockConflict, SyncLog, VitrinePage, VitrineTemplate,
};
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::collections::HashSet;

/// État mutable partagé pour les écrans EXP (espace exposant).
#[derive(Default)]
pub struct ExpState {
    /// Index de l'onglet sélectionné dans le sidebar exposant.
    pub selected_tab_idx: usize,
    /// Indique si les données ont déjà été chargées (évite les rechargements inutiles).
    pub data_loaded: bool,
    /// Identifiant de l'exposant courant (session).
    pub current_exposant_id: Option<String>,

    // --- Fiche entreprise ---
    /// Formulaire fiche entreprise (profil exposant en cours d'édition).
    pub fiche_form: ExposantProfile,

    // --- Catalogue ---
    /// Liste des produits du catalogue.
    pub produits: Vec<ProduitCatalogue>,
    /// Liste des catégories de produits.
    pub categories: Vec<CategorieProduit>,
    /// Recherche dans le catalogue.
    pub catalogue_search: String,
    /// Filtre catégorie dans le catalogue (None = toutes).
    pub catalogue_category_filter: Option<String>,
    /// Produits sélectionnés dans la liste catalogue (actions de masse).
    pub catalogue_selected_ids: HashSet<String>,
    /// Identifiant du produit en cours d'édition (None = création).
    pub editing_produit_id: Option<String>,
    /// Formulaire produit (création/modification).
    pub produit_form: ProduitCatalogue,
    /// Nom de la nouvelle catégorie à créer.
    pub new_category_name: String,

    // --- Vitrine ---
    /// Pages de la vitrine.
    pub vitrine_pages: Vec<VitrinePage>,
    /// Blocs structures de la page courante (page builder).
    pub vitrine_pagebuilder_blocks: Vec<PageBuilderBlock>,
    /// Templates disponibles pour accelerer la creation de pages.
    pub vitrine_templates: Vec<VitrineTemplate>,
    /// Contenu éditeur de la page présentation.
    pub presentation_content: String,
    /// Index du bloc sélectionné dans la librairie page builder.
    pub pagebuilder_selected_block_idx: usize,
    /// Index du bloc sélectionné dans le canvas.
    pub pagebuilder_selected_canvas_idx: Option<usize>,
    /// Recherche widgets dans la librairie.
    pub pagebuilder_search: String,
    /// Onglet panneau propriétés (0 content, 1 style, 2 advanced).
    pub pagebuilder_active_tab: usize,
    /// Historique snapshots JSON du builder pour undo.
    pub pagebuilder_history: Vec<String>,
    /// Historique inverse pour redo.
    pub pagebuilder_future: Vec<String>,
    /// Clipboard de style (props partielles).
    pub pagebuilder_style_clipboard: Option<String>,
    /// Clipboard widget (copie de bloc complet pour paste).
    pub pagebuilder_block_clipboard: Option<PageBuilderBlock>,
    /// Widget en cours de déplacement (drag-and-drop natif dans le canvas).
    pub pagebuilder_dnd_widget_id: Option<String>,
    /// Libellé affiché pendant le drag (ghost).
    pub pagebuilder_dnd_widget_label: Option<String>,
    /// Colonne cible courante (pendant le drag).
    pub pagebuilder_dnd_target_col_id: Option<String>,
    /// Position d'insertion (0..=len) dans la colonne cible (pendant le drag).
    pub pagebuilder_dnd_target_pos: Option<usize>,
    /// Barre latérale paramètres page builder ouverte.
    pub pagebuilder_settings_open: bool,
    /// Fenetre navigator ouverte.
    pub pagebuilder_navigator_open: bool,
    /// Fenetre templates ouverte.
    pub pagebuilder_templates_open: bool,
    /// Device preview (0 desktop, 1 tablet, 2 mobile).
    pub pagebuilder_device_preview: usize,
    /// Couleur primaire globale page builder.
    pub pagebuilder_global_primary: String,
    /// Couleur texte globale page builder.
    pub pagebuilder_global_text: String,
    /// Couleur fond globale page builder.
    pub pagebuilder_global_bg: String,
    /// Taille de police globale pour headings.
    pub pagebuilder_global_heading_size: i32,
    /// Taille de police globale pour texte.
    pub pagebuilder_global_body_size: i32,
    /// Onglet sélectionné dans la prévisualisation vitrine.
    pub vitrine_preview_tab: usize,

    // --- Documents ---
    /// Liste des documents professionnels.
    pub documents: Vec<DocumentProfessionnel>,
    /// Identifiant du document en cours d'édition (None = création).
    pub editing_document_id: Option<String>,
    /// Formulaire document (upload/remplacement).
    pub document_form: DocumentProfessionnel,
    /// Partages de documents actifs.
    pub partages: Vec<DocumentPartage>,

    // --- Confidentialité ---
    /// Règles de confidentialité du profil.
    pub confidentialite: Vec<ConfidentialiteProfil>,
    /// Liens de stock PoS associes aux produits.
    pub pos_stock_links: Vec<PosStockLink>,
    /// Historique recent des synchronisations inter-services.
    pub sync_logs: Vec<SyncLog>,
    /// Conflits de stock detectes lors des sync PoS.
    pub stock_conflicts: Vec<StockConflict>,
    /// Mandat de gouvernance actif pour operations non-SQL.
    pub governance_mandate_id: String,
    /// Niveau de sécurité du mandat actif.
    pub governance_security_level: u8,
    /// Mode de synchronisation PoS (manual, batch, realtime).
    pub sync_mode: String,
    /// Politique de resolution des conflits (manual_review, prefer_pos, prefer_local).
    pub sync_conflict_policy: String,

    // --- CMS Articles (M8) ---
    /// Liste des articles CMS.
    pub cms_articles: Vec<CmsArticle>,
    /// Catégories CMS.
    pub cms_categories: Vec<CmsCategory>,
    /// Identifiant de l'article en cours d'édition (None = création).
    pub editing_article_id: Option<String>,
    /// Formulaire article (création/modification).
    pub article_form: CmsArticle,
    /// Recherche dans la liste des articles.
    pub cms_search: String,
    /// Filtre statut CMS (None = tous).
    pub cms_status_filter: Option<String>,
    /// Filtre type article CMS (None = tous).
    pub cms_type_filter: Option<String>,

    // --- UI ---
    /// Message de statut (succès ou erreur).
    pub status_message: Option<String>,
}


/// Affiche l'écran EXP correspondant à `screen`.
/// Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn exp_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    match screen {
        ScreenId::ExpDashboard => {
            e01_dashboard::exp_e01_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpFicheEntreprise => {
            e02_fiche_entreprise::exp_e02_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpCatalogueListe => {
            e03_catalogue_liste::exp_e03_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpFicheProduit => {
            e04_fiche_produit::exp_e04_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpCategories => {
            e05_categories::exp_e05_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpVitrineParams => {
            e06_vitrine_params::exp_e06_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpVitrinePresentation => {
            e07_vitrine_presentation::exp_e07_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpVitrinePreview => {
            e08_vitrine_preview::exp_e08_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpDocuments => {
            e09_documents::exp_e09_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpUploadDocument => {
            e10_upload_document::exp_e10_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpFichePublique => {
            e11_fiche_publique::exp_e11_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpPartageDocuments => {
            e12_partage_documents::exp_e12_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpCmsArticles => {
            e13_cms_articles::exp_e13_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::ExpCmsEditArticle => {
            e14_cms_edit_article::exp_e14_show(ctx, theme, nav_request, state, db);
        }
        _ => {}
    }
}
