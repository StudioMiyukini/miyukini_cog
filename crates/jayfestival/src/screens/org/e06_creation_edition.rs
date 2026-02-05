//! ORG-E06 — Création d'une édition.
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Input, Button. Formulaire nom, dates, lieu ; sortie vers Dashboard édition ou Liste éditions.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, input, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e06
/// @do: define_org_creation_edition_screen_component
/// @layer: app
/// Écran création édition : formulaire (nom, dates, lieu, thème), validation, annulation.

/// @id: org_e06_show
/// @do: render_org_creation_edition_form_with_gestion_layout
/// @layer: app
/// Affiche le formulaire de création d'édition avec champs nom, date début, date fin, lieu.
pub fn org_e06_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    form_name: &mut String,
    form_start_date: &mut String,
    form_end_date: &mut String,
    form_location: &mut String,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Créer une édition",
        edition_options,
        selected_edition,
        &nav_labels,
        |ui| {
            if button(ui, theme, "Tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgDashboard));
            }
            if button(ui, theme, "Éditions", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgListeEditions));
            }
            if button(ui, theme, "Nouvelle édition", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgCreationEdition));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(20)));
            }
            if button(ui, theme, "Équipe", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(21)));
            }
        },
        |ui| {
            label(ui, theme, "Nouvelle édition", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            ui.horizontal(|ui| {
                label(ui, theme, "Nom", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                input(ui, theme, form_name, Some("Nom de l'édition"), false, true);
            });
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Date début", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                input(ui, theme, form_start_date, Some("AAAA-MM-JJ"), false, true);
            });
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Date fin", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                input(ui, theme, form_end_date, Some("AAAA-MM-JJ"), false, true);
            });
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Lieu", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                input(ui, theme, form_location, Some("Ville ou lieu"), false, true);
            });
            ui.add_space(theme.item_spacing() * 2.0);

            ui.horizontal(|ui| {
                if button(ui, theme, "Créer", ButtonVariant::Primary, Default::default()).clicked() {
                    // TODO: appel Supabase pour créer l'édition (Phase 4)
                    let _ = nav_request.replace(Some(ScreenId::OrgDashboardEdition));
                }
                if button(ui, theme, "Annuler", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::OrgListeEditions));
                }
            });
        },
    );
}
