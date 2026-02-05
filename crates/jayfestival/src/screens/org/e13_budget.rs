//! ORG-E13 — Saisie et ventilation budget.
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Input (revenus/dépenses). Données Supabase budget_entries (alpha).

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, input, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e13
/// @do: define_org_budget_screen_component
/// @layer: app
/// Écran budget : saisie et ventilation revenus/dépenses (alpha : formulaire simple).

/// @id: org_e13_show
/// @do: render_org_budget_with_gestion_layout_and_inputs
/// @layer: app
/// Affiche le budget avec cartes Revenus / Dépenses et champs de saisie (alpha).
pub fn org_e13_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    budget_revenus_label: &mut String,
    budget_depenses_label: &mut String,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Budget",
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
            label(ui, theme, "Budget", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Revenus"), |ui| {
                label(ui, theme, "Libellé", LabelLevel::Body);
                input(ui, theme, budget_revenus_label, Some("Ex. Vente stands"), false, true);
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("Dépenses"), |ui| {
                label(ui, theme, "Libellé", LabelLevel::Body);
                input(ui, theme, budget_depenses_label, Some("Ex. Location salle"), false, true);
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing() * 2.0);

            if button(ui, theme, "Enregistrer", ButtonVariant::Primary, Default::default()).clicked() {
                // TODO: Supabase budget_entries (Phase 4)
            }
        },
    );
}
