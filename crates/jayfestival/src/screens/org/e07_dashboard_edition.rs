//! ORG-E07 — Dashboard édition.
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Badge. Hub : Exposants, Plan, Programme, Budget, Documents, etc.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::{badge, BadgeVariant};
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e07
/// @do: define_org_dashboard_edition_screen_component
/// @layer: app
/// Écran dashboard édition : synthèse et accès aux modules (exposants, plan, programme, budget, documents).

/// @id: org_e07_show
/// @do: render_org_dashboard_edition_with_gestion_layout_and_cards
/// @layer: app
/// Affiche le dashboard de l'édition courante avec cartes d'accès aux sous-écrans.
pub fn org_e07_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    edition_name: &str,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Dashboard édition",
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
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(20)));
            }
            if button(ui, theme, "Équipe", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(21)));
            }
        },
        |ui| {
            label(ui, theme, edition_name, LabelLevel::Heading);
            badge(ui, theme, "Édition en cours", BadgeVariant::Default);
            ui.add_space(theme.item_spacing() * 2.0);

            egui::Grid::new("org_e07_grid").num_columns(2).show(ui, |ui| {
                card_show(theme, ui, Some("Exposants"), |ui| {
                    label(ui, theme, "Liste des exposants de l'édition.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Candidatures"), |ui| {
                    label(ui, theme, "Réception et traitement des candidatures.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgCandidatures));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Plan de salle"), |ui| {
                    label(ui, theme, "Zones et stands.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgPlanSalle));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Programme"), |ui| {
                    label(ui, theme, "Créneaux et animations.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgProgramme));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Budget"), |ui| {
                    label(ui, theme, "Revenus et dépenses.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgBudget));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Devis / Factures"), |ui| {
                    label(ui, theme, "Suivi devis et factures.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgDevisFactures));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Documents"), |ui| {
                    label(ui, theme, "Contrats types, CGV, règlements.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgDocuments));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();
            });
        },
    );
}
