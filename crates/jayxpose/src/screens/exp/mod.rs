//! Router EXP — écrans Espace Exposant (XP-E01 à XP-E12).
//!
//! @id: router_exp
//! @do: dispatch_exp_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran EXP correspondant à l'identifiant ; délègue à chaque module e01..e12.
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

use std::cell::RefCell;
use std::sync::Arc;

use crate::data::{
    CategorieProduit, ConfidentialiteProfil, DocumentPartage, DocumentProfessionnel,
    ExposantProfile, JayXposeDb, ProduitCatalogue, VitrinePage,
};
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;

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
    /// Identifiant du produit en cours d'édition (None = création).
    pub editing_produit_id: Option<String>,
    /// Formulaire produit (création/modification).
    pub produit_form: ProduitCatalogue,
    /// Nom de la nouvelle catégorie à créer.
    pub new_category_name: String,

    // --- Vitrine ---
    /// Pages de la vitrine.
    pub vitrine_pages: Vec<VitrinePage>,
    /// Contenu éditeur de la page présentation.
    pub presentation_content: String,
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
        _ => {}
    }
}
