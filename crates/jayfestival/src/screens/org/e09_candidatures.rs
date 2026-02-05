//! ORG-E09 — Candidatures (réception et traitement).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Badge, Button. Sortie vers Liste exposants, Fiche exposant.

use crate::screens::ScreenId;
use crate::supabase::{EditionExposant, Exposant};
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{badge, button, label, BadgeVariant, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e09
/// @do: define_org_candidatures_screen_component
/// @layer: app
/// Écran candidatures : liste des candidatures en attente, accepter/refuser, accès Fiche exposant.

/// @id: org_e09_show
/// @do: render_org_candidatures_with_gestion_layout_and_cards
/// @layer: app
/// Affiche les candidatures en attente avec actions Accepter / Refuser.
pub fn org_e09_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    edition_exposants: &[EditionExposant],
    exposants: &[Exposant],
    selected_exposant_id: &mut Option<String>,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    let pending: Vec<_> = edition_exposants
        .iter()
        .filter(|ee| ee.is_accepted != Some(true) || ee.is_validated != Some(true))
        .collect();

    gestion_layout_show(
        ctx,
        theme,
        "Candidatures",
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
            label(ui, theme, "Candidatures", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Liste des exposants", ButtonVariant::Secondary, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
            }
            ui.add_space(theme.item_spacing() * 2.0);

            for ee in pending.iter().take(30) {
                let exp = ee
                    .exposant_id
                    .as_ref()
                    .and_then(|id| exposants.iter().find(|e| e.id.as_deref() == Some(id.as_str())));
                let name = exp
                    .and_then(|e| e.company_name.as_deref())
                    .or(exp.and_then(|e| e.stand_name.as_deref()))
                    .unwrap_or("Exposant");
                let exp_id = exp.and_then(|e| e.id.clone()).or_else(|| ee.exposant_id.clone());
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        badge(ui, theme, "En attente", BadgeVariant::Warning);
                        if let Some(e) = exp {
                            if let Some(cat) = e.category.as_deref() {
                                label(ui, theme, cat, LabelLevel::Small);
                            }
                        }
                        ui.horizontal(|ui| {
                            if button(ui, theme, "Accepter", ButtonVariant::Primary, Default::default()).clicked() {
                                let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
                            }
                            if button(ui, theme, "Refuser", ButtonVariant::Outline, Default::default()).clicked() {
                                let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
                            }
                            if button(ui, theme, "Voir la fiche", ButtonVariant::Ghost, Default::default()).clicked() {
                                *selected_exposant_id = exp_id.clone();
                                let _ = nav_request.replace(Some(ScreenId::OrgFicheExposant));
                            }
                        });
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                ui.add_space(theme.item_spacing());
            }
        },
    );
}
