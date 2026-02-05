//! Router VIS — écrans Visiteur (VIS-E04 à VIS-E15).
//!
//! Conforme [Visiteurs - Ecrans et cycle] et [Specification UI § 4].
//!
//! @id: router_vis
//! @do: dispatch_vis_screen_rendering_by_screen_id
//! @layer: app
//! Affiche l'écran VIS correspondant à l'identifiant ; délègue à chaque module e04, e05 et e06_e15_billets_reservations_pass.
//!
//! @id: vis_navigate
//! @do: change_current_screen_from_vis
//! @layer: app
//! Navigation : appelé par les écrans pour changer d'écran (via RefCell).

mod e04_dashboard;
mod e05_agenda;
mod e06_e15_billets_reservations_pass;

use std::cell::RefCell;

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use eframe::egui;

/// État mutable partagé pour les écrans VIS (visiteur : agenda, billets, réservations, pass).
pub struct VisState {
    /// Index d'édition affiché dans le header (pour cohérence avec GestionLayout ; visiteur peut avoir une édition courante).
    pub selected_edition_idx: usize,
}

impl Default for VisState {
    fn default() -> Self {
        Self {
            selected_edition_idx: 0,
        }
    }
}

impl VisState {
    /// Options pour le sélecteur d'édition (visiteur : à remplir selon événements accessibles).
    pub fn edition_options(&self) -> Vec<String> {
        Vec::new()
    }
}

/// Affiche l'écran VIS correspondant à `screen`. Les écrans écrivent dans `nav_request` pour demander une navigation.
pub fn vis_show(
    screen: ScreenId,
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut VisState,
) {
    let edition_options = state.edition_options();
    let nav = nav_request;

    match screen {
        ScreenId::VisDashboard => e04_dashboard::vis_e04_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
        ),
        ScreenId::VisAgenda => e05_agenda::vis_e05_show(
            ctx,
            theme,
            nav,
            &edition_options,
            &mut state.selected_edition_idx,
        ),
        ScreenId::VisReserved(n) if (6..=15).contains(&n) => e06_e15_billets_reservations_pass::vis_e06_e15_show(
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
