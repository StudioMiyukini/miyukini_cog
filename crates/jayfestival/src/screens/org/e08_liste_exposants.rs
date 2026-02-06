//! ORG-E08 — Liste des exposants (annuaire local édition).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Badge. Données JayXpose/Supabase. Sortie vers Fiche exposant, Candidatures.

use crate::screens::ScreenId;
use crate::data::{EditionExposant, Exposant};
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{badge, button, label, BadgeVariant, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e08
/// @do: define_org_liste_exposants_screen_component
/// @layer: app
/// Écran liste des exposants de l'édition : annuaire local, statuts (candidat/validé/refusé), accès Fiche exposant, Candidatures.

/// @id: org_e08_show
/// @do: render_org_liste_exposants_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des exposants de l'édition avec badges de statut et lien vers fiche exposant.
pub fn org_e08_show(
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
    gestion_layout_show(
        ctx,
        theme,
        "Liste des exposants",
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
            label(ui, theme, "Liste des exposants", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Candidatures", ButtonVariant::Secondary, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgCandidatures));
            }
            ui.add_space(theme.item_spacing() * 2.0);

            for ee in edition_exposants.iter().take(50) {
                let exp = ee
                    .exposant_id
                    .as_ref()
                    .and_then(|id| exposants.iter().find(|e| e.id.as_deref() == Some(id.as_str())));
                let name = exp
                    .and_then(|e| e.company_name.as_deref())
                    .or(exp.and_then(|e| e.stand_name.as_deref()))
                    .unwrap_or("Exposant");
                let statut = match (ee.is_accepted, ee.is_validated) {
                    (Some(true), Some(true)) => ("Validé", BadgeVariant::Success),
                    (Some(true), Some(false)) => ("En attente", BadgeVariant::Warning),
                    (Some(false), _) => ("Refusé", BadgeVariant::Error),
                    _ => ("Candidat", BadgeVariant::Default),
                };
                let exp_id = exp.and_then(|e| e.id.clone()).or_else(|| ee.exposant_id.clone());
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        badge(ui, theme, statut.0, statut.1);
                        if let Some(e) = exp {
                            if let Some(cat) = e.category.as_deref() {
                                label(ui, theme, cat, LabelLevel::Small);
                            }
                        }
                        if button(ui, theme, "Voir la fiche", ButtonVariant::Secondary, Default::default()).clicked() {
                            *selected_exposant_id = exp_id.clone();
                            let _ = nav_request.replace(Some(ScreenId::OrgFicheExposant));
                        }
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                ui.add_space(theme.item_spacing());
            }
        },
    );
}
