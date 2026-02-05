//! VIS-E05 — Agenda personnel.
//!
//! Conforme [Visiteurs - Ecrans et cycle]. GestionLayout, vue calendrier (JayKoa) ou liste.
//! Calendrier personnel : ateliers réservés, créneaux, animations, concours ; compte à rebours ; alerte conflits ; export.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_vis_e05
/// @do: define_vis_agenda_screen_component
/// @layer: app
/// Écran agenda personnel : vue calendrier ou liste ; créneaux réservés, compte à rebours, alerte conflits, export iCal/PDF.

/// @id: vis_e05_show
/// @do: render_vis_agenda_with_gestion_layout
/// @layer: app
/// Affiche l'agenda visiteur : GestionLayout, filtres (événement, type, jour), vue calendrier ou liste (placeholder alpha).
pub fn vis_e05_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
) {
    let nav_labels = ["Catalogue", "Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Mon agenda",
        edition_options,
        selected_edition,
        &nav_labels,
        |ui| {
            if button(ui, theme, "Tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisDashboard));
            }
            if button(ui, theme, "Agenda", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisAgenda));
            }
            if button(ui, theme, "Billets", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(6)));
            }
            if button(ui, theme, "Réservations", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(7)));
            }
            if button(ui, theme, "Pass VIP", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(8)));
            }
            if button(ui, theme, "Activités", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(11)));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(14)));
            }
        },
        |ui| {
            label(ui, theme, "Mon agenda", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Vue agenda"), |ui| {
                label(
                    ui,
                    theme,
                    "Filtres : événement, type (atelier, animation, concours), jour. Vue Calendrier (mois, semaine) ou Liste. Compte à rebours pour le prochain ; alerte conflit si chevauchement.",
                    LabelLevel::Body,
                );
                ui.add_space(theme.item_spacing());
                label(ui, theme, "Contenu alpha : vue calendrier ou liste à brancher (JayKoa / données Supabase).", LabelLevel::Small);
                if button(ui, theme, "Export (iCal / PDF)", ButtonVariant::Secondary, Default::default()).clicked() {
                    // TODO: export agenda
                }
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Retour tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisDashboard));
            }
        },
    );
}
