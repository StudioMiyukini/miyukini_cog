//! ORG-E15 — Documents et légal (contrats types, CGV, règlements).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Button. Sortie vers Dashboard édition.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e15
/// @do: define_org_documents_screen_component
/// @layer: app
/// Écran documents : contrats types, CGV, règlements (alpha : liste et téléchargement placeholder).

/// @id: org_e15_show
/// @do: render_org_documents_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des documents (contrats, CGV, règlements) avec boutons (alpha).
pub fn org_e15_show(
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
        "Documents",
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
            label(ui, theme, "Documents et légal", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Contrats types"), |ui| {
                label(ui, theme, "Modèles de contrats pour les exposants.", LabelLevel::Body);
                let _ = button(ui, theme, "Télécharger", ButtonVariant::Secondary, Default::default());
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("CGV"), |ui| {
                label(ui, theme, "Conditions générales de vente.", LabelLevel::Body);
                let _ = button(ui, theme, "Télécharger", ButtonVariant::Secondary, Default::default());
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("Règlement"), |ui| {
                label(ui, theme, "Règlement de l'édition.", LabelLevel::Body);
                let _ = button(ui, theme, "Télécharger", ButtonVariant::Secondary, Default::default());
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
