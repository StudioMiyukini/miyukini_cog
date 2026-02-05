//! Organism RolesGrid — grille de RoleCards (organisateur, exposant, visiteur).
//!
//! Conforme [Specification UI] § 2. Dépend de la molecule RoleCard.

use crate::theme::JayFestivalTheme;
use crate::ui::molecules::role_card_render;
use eframe::egui::{Color32, Ui};

/// Élément de grille pour RoleCard (titre, description optionnelle, couleur accent).
#[derive(Clone)]
pub struct RoleCardItem {
    /// Titre du rôle (ex. Organisateur, Exposant).
    pub title: String,
    /// Description optionnelle.
    pub description: Option<String>,
    /// Couleur d'accent de la carte.
    pub accent_color: Color32,
}

/// @id: organism_roles_grid
/// @do: define_roles_grid_component
/// @layer: ui
/// @human: Grille de cartes rôles (organisateur, exposant, visiteur, bénévole).

/// @id: roles_grid_render
/// @do: render_grid_of_role_cards
/// @layer: ui
/// Affiche une grille horizontale (ou wrap) de RoleCards.
pub fn roles_grid_render(theme: &JayFestivalTheme, ui: &mut Ui, items: &[RoleCardItem]) {
    ui.horizontal_wrapped(|ui| {
        for item in items {
            ui.vertical(|ui| {
                role_card_render(
                    theme,
                    ui,
                    &item.title,
                    item.description.as_deref(),
                    item.accent_color,
                );
            });
        }
    });
}
