//! Router EXP — écrans Exposant (EXP-E04 à EXP-E19).
//!
//! Conforme [Specification UI § 4.2] et [Exposants - Ecrans et cycle].
//!
//! @id: router_exp
//! @do: dispatch_exp_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran EXP correspondant à l'identifiant ; délègue à chaque module e04..e06 et e07_e19_rest.
//!
//! @id: exp_navigate
//! @do: change_current_screen_from_exp
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell).

mod e04_dashboard;
mod e05_candidatures;
mod e06_participations;
mod e07_e19_rest;

use std::cell::RefCell;

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use eframe::egui;

/// État mutable partagé pour les écrans EXP (exposant : pas de sélecteur d'édition dans l'en-tête ; données candidatures/participations à venir).
#[derive(Default)]
pub struct ExpState {
    /// Index d'édition affiché dans le header (pour cohérence avec GestionLayout ; exposant n'a pas d'édition courante, gardé à 0).
    pub selected_edition_idx: usize,
}


impl ExpState {
    /// Options pour le sélecteur d'édition (vide pour exposant ; GestionLayout accepte un slice vide).
    #[must_use] 
    pub fn edition_options(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Affiche l'écran EXP correspondant à `screen`. Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn exp_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
) {
    let edition_options = state.edition_options();
    let nav = nav_request;

    match screen {
        ScreenId::ExpDashboard => e04_dashboard::exp_e04_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
        ),
        ScreenId::ExpCandidatures => e05_candidatures::exp_e05_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
        ),
        ScreenId::ExpParticipations => e06_participations::exp_e06_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
        ),
        ScreenId::ExpReserved(n) => e07_e19_rest::exp_e07_e19_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
            n,
        ),
        _ => {}
    }
}
