//! Router PUB — écrans Accès Public (XP-P01 à XP-P06).
//!
//! @id: router_pub
//! @do: dispatch_pub_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran PUB correspondant à l'identifiant ; délègue à chaque module e01..e06.
//!
//! @id: pub_navigate
//! @do: change_current_screen_from_pub
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell).

mod e01_annuaire;
mod e02_fiche_exposant;
mod e03_catalogue_public;
mod e04_vitrine;
mod e05_recherche;
mod e06_vitrine_contact;

use std::cell::RefCell;
use std::sync::Arc;

use crate::data::{
    CategorieProduit, ConfidentialiteProfil, ExposantProfile, JayXposeDb, ProduitCatalogue,
    VitrinePage,
};
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;

/// État mutable partagé pour les écrans PUB (accès public).
#[derive(Default)]
pub struct PubState {
    /// Terme de recherche courant (annuaire / recherche).
    pub search_query: String,

    // --- Annuaire ---
    /// Indique si l'annuaire a été chargé.
    pub annuaire_loaded: bool,
    /// Liste des exposants pour l'annuaire.
    pub annuaire_exposants: Vec<ExposantProfile>,
    /// Filtre par secteur d'activité (None = tous).
    pub filter_secteur: Option<String>,

    // --- Fiche exposant ---
    /// Identifiant de l'exposant sélectionné.
    pub selected_exposant_id: Option<String>,
    /// Indique si la fiche a été chargée.
    pub fiche_loaded: bool,
    /// Profil exposant chargé pour la fiche publique.
    pub fiche_exposant: Option<ExposantProfile>,
    /// Règles de confidentialité de l'exposant sélectionné.
    pub confidentialite_rules: Vec<ConfidentialiteProfil>,

    // --- Catalogue public ---
    /// Indique si le catalogue a été chargé.
    pub catalogue_loaded: bool,
    /// Produits du catalogue public.
    pub catalogue_produits: Vec<ProduitCatalogue>,
    /// Catégories du catalogue public.
    pub catalogue_categories: Vec<CategorieProduit>,
    /// Recherche dans le catalogue public.
    pub search_produit: String,
    /// Filtre par catégorie dans le catalogue (None = toutes).
    pub filter_categorie: Option<String>,

    // --- Vitrine ---
    /// Indique si la vitrine a été chargée.
    pub vitrine_loaded: bool,
    /// Pages de la vitrine publique.
    pub vitrine_pages: Vec<VitrinePage>,
    /// Onglet actif dans la vitrine (0 = Accueil, 1 = Catalogue, …).
    pub vitrine_tab: usize,

    // --- Contact ---
    /// Nom du visiteur (formulaire contact).
    pub contact_nom: String,
    /// Email du visiteur (formulaire contact).
    pub contact_email: String,
    /// Message du visiteur (formulaire contact).
    pub contact_message: String,
}


/// Affiche l'écran PUB correspondant à `screen`.
/// Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn pub_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut PubState,
    db: &Arc<JayXposeDb>,
) {
    match screen {
        ScreenId::PubAnnuaire => {
            e01_annuaire::pub_e01_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::PubFicheExposant => {
            e02_fiche_exposant::pub_e02_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::PubCataloguePublic => {
            e03_catalogue_public::pub_e03_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::PubVitrine => {
            e04_vitrine::pub_e04_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::PubRecherche => {
            e05_recherche::pub_e05_show(ctx, theme, nav_request, state, db);
        }
        ScreenId::PubVitrineContact => {
            e06_vitrine_contact::pub_e06_show(ctx, theme, nav_request, state, db);
        }
        _ => {}
    }
}
