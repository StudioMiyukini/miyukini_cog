//! Organism GestionLayout — layout avec HeaderWithEdition (espaces ORG/EXP/VIS).
//!
//! Dépend de [51] Header, [52] HeaderWithEdition, [64] Layout. Utilisé pour les tableaux de bord avec édition courante.

use crate::theme::JayFestivalTheme;
use crate::ui::organisms::header_with_edition_render;
use crate::ui::organisms::layout_show;
use eframe::egui::Context;

/// @id: organism_gestion_layout
/// @do: define_gestion_layout_with_edition_header
/// @layer: ui
/// @human: Layout de gestion : header avec sélecteur d'édition + sidebar + contenu central.

/// @id: gestion_layout_render
/// @do: render_gestion_layout_with_header_edition_sidebar_content
/// @layer: ui
/// Affiche le layout de gestion : header (titre + select édition + nav), sidebar, CentralPanel.
/// Les closures sidebar_ui et content_ui sont FnMut ; les paramètres d'édition sont passés pour le header.
pub fn gestion_layout_show<S, C>(
    ctx: &Context,
    theme: &JayFestivalTheme,
    title: &str,
    edition_options: &[String],
    selected_edition: &mut usize,
    nav_labels: &[&str],
    sidebar_ui: S,
    content_ui: C,
) where
    S: FnMut(&mut eframe::egui::Ui),
    C: FnMut(&mut eframe::egui::Ui),
{
    layout_show(ctx, theme, |ui| {
        header_with_edition_render(
            ui,
            theme,
            title,
            edition_options,
            selected_edition,
            nav_labels,
        );
    }, sidebar_ui, content_ui);
}
