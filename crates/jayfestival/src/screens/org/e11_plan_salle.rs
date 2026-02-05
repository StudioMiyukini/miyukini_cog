//! ORG-E11 — Plan de salle (définition des zones et stands).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, zones/stands (widget ou canvas). Sortie vers Attribution emplacements, Visualisation plan.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e11
/// @do: define_org_plan_salle_screen_component
/// @layer: app
/// Écran plan de salle : définition des zones et stands (alpha : placeholder ou grille simple).

/// @id: org_e11_show
/// @do: render_org_plan_salle_with_gestion_layout_and_zones
/// @layer: app
/// Affiche le plan de salle : zone de dessin ou grille (alpha), liens Attribution et Visualisation.
pub fn org_e11_show(
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
        "Plan de salle",
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
            label(ui, theme, "Plan de salle", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            label(ui, theme, "Définition des zones et stands. (Alpha : zone placeholder.)", LabelLevel::Body);
            ui.add_space(theme.item_spacing() * 2.0);

            let rect = ui.available_rect_before_wrap();
            let painter = ui.painter();
            let stroke = egui::Stroke::new(1.0, theme.section_border());
            painter.rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Outside);
            ui.allocate_rect(rect, egui::Sense::hover());
            ui.add_space(theme.item_spacing() * 2.0);

            ui.horizontal(|ui| {
                if button(ui, theme, "Attribution emplacements", ButtonVariant::Secondary, Default::default()).clicked() {
                    // E15 : même écran ou futur sous-écran
                    let _ = nav_request.replace(Some(ScreenId::OrgPlanSalle));
                }
                if button(ui, theme, "Visualisation plan", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::OrgPlanSalle));
                }
            });
        },
    );
}
