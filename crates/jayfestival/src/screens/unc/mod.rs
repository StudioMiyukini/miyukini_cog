//! Router UNC — écrans Utilisateur non connecté (UNC-E01 à UNC-E14).
//!
//! Conforme [Specification UI § 4.1] et [UtilisateurNonConnecte - Ecrans et cycle].
//!
//! @id: router_unc
//! @do: dispatch_unc_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran UNC correspondant à l'identifiant ; délègue à chaque module e01..e14.
//!
//! @id: unc_navigate
//! @do: change_current_screen_from_unc
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell pour éviter double emprunt).

use std::cell::RefCell;

mod e01_landing;
mod e02_liste_evenements;
mod e03_fiche_evenement;
mod e06_liste_organisateurs;
mod e07_fiche_organisateur;
mod e08_liste_exposants;
mod e09_fiche_exposant;
mod e10_recherche;
mod e11_cta_contextuels;
mod e12_connexion;
mod e13_inscription;
mod e14_mentions;

use crate::screens::ScreenId;
use crate::data::{Edition, Exposant, Organisateur};
use crate::theme::JayFestivalTheme;
use eframe::egui;

/// État mutable partagé pour les écrans UNC (listes, filtres, formulaire connexion, etc.).
pub struct UncState {
    /// Liste des éditions chargées (événements).
    pub editions: Vec<Edition>,
    /// Liste des organisateurs chargés.
    pub organisateurs: Vec<Organisateur>,
    /// Liste des exposants chargés.
    pub exposants: Vec<Exposant>,
    /// ID de l'édition sélectionnée pour la fiche événement (UNC-E03).
    pub selected_edition_id: Option<String>,
    /// ID de l'organisateur sélectionné pour la fiche organisateur (UNC-E07).
    pub selected_organisateur_id: Option<String>,
    /// ID de l'exposant sélectionné pour la fiche exposant (UNC-E09).
    pub selected_exposant_id: Option<String>,
    /// Filtre date (liste événements).
    pub filter_date: String,
    /// Filtre lieu (liste événements).
    pub filter_lieu: String,
    /// Index du filtre thème (liste événements).
    pub filter_theme_idx: usize,
    /// Options du sélecteur thème.
    pub theme_options: Vec<String>,
    /// Filtre nom organisateur (liste organisateurs).
    pub filter_nom_org: String,
    /// Index du filtre région (liste organisateurs).
    pub filter_region_idx: usize,
    /// Options du sélecteur région.
    pub region_options: Vec<String>,
    /// Filtre recherche exposant (liste exposants).
    pub filter_search_exp: String,
    /// Index du filtre catégorie (liste exposants).
    pub filter_categorie_idx: usize,
    /// Options du sélecteur catégorie.
    pub categorie_options: Vec<String>,
    /// Requête de recherche (écran recherche).
    pub search_query: String,
    /// Indique si une recherche a été effectuée.
    pub has_searched: bool,
    /// Message CTA affiché (ex. réservation réservée aux connectés).
    pub cta_message: String,
    /// Email saisi (formulaire connexion).
    pub connexion_email: String,
    /// Mot de passe saisi (formulaire connexion).
    pub connexion_password: String,
    /// Erreur affichée après tentative de connexion.
    pub connexion_error: Option<String>,
    /// Demande de soumission du formulaire connexion.
    pub connexion_submit_requested: bool,
}

impl Default for UncState {
    fn default() -> Self {
        Self {
            editions: Vec::new(),
            organisateurs: Vec::new(),
            exposants: Vec::new(),
            selected_edition_id: None,
            selected_organisateur_id: None,
            selected_exposant_id: None,
            filter_date: String::new(),
            filter_lieu: String::new(),
            filter_theme_idx: 0,
            theme_options: vec!["Tous".into()],
            filter_nom_org: String::new(),
            filter_region_idx: 0,
            region_options: vec!["Toutes".into()],
            filter_search_exp: String::new(),
            filter_categorie_idx: 0,
            categorie_options: vec!["Toutes".into()],
            search_query: String::new(),
            has_searched: false,
            cta_message: "La réservation est réservée aux utilisateurs connectés. Connectez-vous ou créez un compte visiteur pour réserver.".into(),
            connexion_email: String::new(),
            connexion_password: String::new(),
            connexion_error: None,
            connexion_submit_requested: false,
        }
    }
}

/// Affiche l'écran UNC correspondant à `screen`. Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn unc_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut UncState,
) {
    match screen {
        ScreenId::UncLanding => e01_landing::unc_e01_show(ctx, theme, nav_request),
        ScreenId::UncListeEvenements => e02_liste_evenements::unc_e02_show(
            ctx,
            theme,
            nav_request,
            &state.editions,
            &mut state.filter_date,
            &mut state.filter_lieu,
            &mut state.filter_theme_idx,
            &state.theme_options,
            &mut state.selected_edition_id,
        ),
        ScreenId::UncFicheEvenement => {
            let edition = state
                .selected_edition_id
                .as_ref()
                .and_then(|id| state.editions.iter().find(|e| e.id.as_deref() == Some(id.as_str())))
                .or_else(|| state.editions.first());
            e03_fiche_evenement::unc_e03_show(ctx, theme, nav_request, edition);
        }
        ScreenId::UncListeOrganisateurs => e06_liste_organisateurs::unc_e06_show(
            ctx,
            theme,
            nav_request,
            &state.organisateurs,
            &mut state.filter_nom_org,
            &mut state.filter_region_idx,
            &state.region_options,
            &mut state.selected_organisateur_id,
        ),
        ScreenId::UncFicheOrganisateur => {
            let org = state
                .selected_organisateur_id
                .as_ref()
                .and_then(|id| state.organisateurs.iter().find(|o| o.id.as_deref() == Some(id.as_str())))
                .or_else(|| state.organisateurs.first());
            e07_fiche_organisateur::unc_e07_show(ctx, theme, nav_request, org);
        }
        ScreenId::UncListeExposants => e08_liste_exposants::unc_e08_show(
            ctx,
            theme,
            nav_request,
            &state.exposants,
            &mut state.filter_search_exp,
            &mut state.filter_categorie_idx,
            &state.categorie_options,
            &mut state.selected_exposant_id,
        ),
        ScreenId::UncFicheExposant => {
            let exp = state
                .selected_exposant_id
                .as_ref()
                .and_then(|id| state.exposants.iter().find(|e| e.id.as_deref() == Some(id.as_str())))
                .or_else(|| state.exposants.first());
            e09_fiche_exposant::unc_e09_show(ctx, theme, nav_request, exp);
        }
        ScreenId::UncRecherche => e10_recherche::unc_e10_show(
            ctx,
            theme,
            nav_request,
            &mut state.search_query,
            &mut state.has_searched,
        ),
        ScreenId::UncCtaContextuels => e11_cta_contextuels::unc_e11_show(
            ctx,
            theme,
            nav_request,
            &state.cta_message,
        ),
        ScreenId::UncConnexion => e12_connexion::unc_e12_show(
            ctx,
            theme,
            nav_request,
            &mut state.connexion_email,
            &mut state.connexion_password,
            state.connexion_error.as_deref(),
            &mut state.connexion_submit_requested,
        ),
        ScreenId::UncInscription => e13_inscription::unc_e13_show(ctx, theme, nav_request),
        ScreenId::UncMentions => e14_mentions::unc_e14_show(ctx, theme, nav_request),
        _ => {
            // Écran non-UNC : ne rien afficher (l'app ne doit appeler unc_show que pour des ScreenId UNC)
        }
    }
}
