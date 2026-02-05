//! Organism DirectoryBanner — bannière répertoire avec DirectoryCard.
//!
//! Conforme [Specification UI] § 2. Dépend de la molecule DirectoryCard.

use crate::theme::JayFestivalTheme;
use crate::ui::molecules::directory_card_render;
use eframe::egui::Response;
use eframe::egui::Ui;

/// @id: organism_directory_banner
/// @do: define_directory_banner_component
/// @layer: ui
/// @human: Bannière mettant en avant un répertoire (événements, organisateurs, exposants) via DirectoryCard.

/// @id: directory_banner_render
/// @do: render_directory_banner_with_card
/// @layer: ui
/// Affiche une bannière avec une DirectoryCard (titre, description optionnelle, libellé CTA).
/// Retourne la réponse du bouton CTA de la carte.
pub fn directory_banner_render(
    theme: &JayFestivalTheme,
    ui: &mut Ui,
    title: &str,
    description: Option<&str>,
    cta_label: &str,
) -> Response {
    directory_card_render(theme, ui, title, description, cta_label)
}
