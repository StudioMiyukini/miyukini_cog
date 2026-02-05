//! ORG-E14 — Devis / Factures (suivi et conversion).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card (Miyuinvoice/JayKonta). Données Supabase invoices (alpha).

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e14
/// @do: define_org_devis_factures_screen_component
/// @layer: app
/// Écran devis et factures : liste devis, conversion en facture, suivi (Miyuinvoice/JayKonta alpha).

/// @id: org_e14_show
/// @do: render_org_devis_factures_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des devis et factures (alpha : placeholder).
pub fn org_e14_show(
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
        "Devis / Factures",
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
            if button(ui, theme, "Fiche exposant", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgFicheExposant));
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
            label(ui, theme, "Devis et factures", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Générer un devis", ButtonVariant::Primary, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgFicheExposant));
            }
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Liste des devis"), |ui| {
                label(ui, theme, "Aucun devis pour l'instant. (Alpha : intégration Miyuinvoice/JayKonta à venir.)", LabelLevel::Small);
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("Factures"), |ui| {
                label(ui, theme, "Aucune facture pour l'instant.", LabelLevel::Small);
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
