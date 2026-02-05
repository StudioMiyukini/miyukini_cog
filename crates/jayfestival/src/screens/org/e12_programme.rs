//! ORG-E12 — Programme (vues chronologique / par salle).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, liste créneaux/salles. Sortie vers Création/édition animation.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e12
/// @do: define_org_programme_screen_component
/// @layer: app
/// Écran programme : vues chronologique / par salle, liste créneaux, accès Création/édition animation.

/// @id: org_e12_show
/// @do: render_org_programme_with_gestion_layout_and_cards
/// @layer: app
/// Affiche le programme avec liste de créneaux (alpha : placeholder).
pub fn org_e12_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Programme",
        edition_options,
        selected_edition,
        &nav_labels,
        |ui| {
            if button(ui, theme, "Dashboard édition", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgDashboardEdition));
            }
            if button(ui, theme, "Exposants", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
            }
            if button(ui, theme, "Candidatures", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgCandidatures));
            }
            if button(ui, theme, "Plan de salle", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgPlanSalle));
            }
            if button(ui, theme, "Programme", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgProgramme));
            }
            if button(ui, theme, "Budget", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgBudget));
            }
            if button(ui, theme, "Devis / Factures", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgDevisFactures));
            }
            if button(ui, theme, "Documents", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgDocuments));
            }
        },
        |ui| {
            label(ui, theme, "Programme", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Ajouter une animation", ButtonVariant::Primary, Default::default()).clicked() {
                // TODO: ouvrir modal ou écran ORG-E17b
            }
            ui.add_space(theme.item_spacing() * 2.0);

            label(ui, theme, "Créneaux (alpha : liste vide)", LabelLevel::Body);
            card_show(theme, ui, Some("Vue chronologique"), |ui| {
                label(ui, theme, "Aucun créneau pour l'instant.", LabelLevel::Small);
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
