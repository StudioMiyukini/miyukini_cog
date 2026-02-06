//! ORG-E10 — Fiche exposant (organisateur).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card. Données JayXpose/Supabase. Blocs : Identité, Statut, Emplacement, Documents. Actions : Devis, Facture.

use crate::screens::ScreenId;
use crate::data::{EditionExposant, Exposant};
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{badge, button, label, BadgeVariant, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e10
/// @do: define_org_fiche_exposant_screen_component
/// @layer: app
/// Écran fiche exposant (org) : identité, statut, emplacement, documents, actions Modifier, Devis, Facture.

/// @id: org_e10_show
/// @do: render_org_fiche_exposant_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la fiche exposant avec blocs Identité, Statut, Emplacement et boutons Devis / Factures.
pub fn org_e10_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    exposant: Option<&Exposant>,
    edition_exposant: Option<&EditionExposant>,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Fiche exposant",
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
            let name = exposant
                .and_then(|e| e.company_name.as_deref())
                .or(exposant.and_then(|e| e.stand_name.as_deref()))
                .unwrap_or("Fiche exposant");
            label(ui, theme, name, LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if let Some(ee) = edition_exposant {
                let statut = match (ee.is_accepted, ee.is_validated) {
                    (Some(true), Some(true)) => badge(ui, theme, "Validé", BadgeVariant::Success),
                    (Some(true), Some(false)) => badge(ui, theme, "En attente", BadgeVariant::Warning),
                    (Some(false), _) => badge(ui, theme, "Refusé", BadgeVariant::Error),
                    _ => badge(ui, theme, "Candidat", BadgeVariant::Default),
                };
                let _ = statut;
            }
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Identité"), |ui| {
                if let Some(e) = exposant {
                    if let Some(s) = e.company_name.as_deref() {
                        label(ui, theme, &format!("Raison sociale : {}", s), LabelLevel::Body);
                    }
                    if let Some(s) = e.stand_name.as_deref() {
                        label(ui, theme, &format!("Stand : {}", s), LabelLevel::Body);
                    }
                    if let Some(s) = e.contact_email.as_deref() {
                        label(ui, theme, &format!("Contact : {}", s), LabelLevel::Body);
                    }
                    if let Some(s) = e.category.as_deref() {
                        label(ui, theme, &format!("Catégorie : {}", s), LabelLevel::Small);
                    }
                } else {
                    label(ui, theme, "Aucune donnée exposant.", LabelLevel::Body);
                }
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("Emplacement"), |ui| {
                if let Some(ee) = edition_exposant {
                    if let Some(s) = ee.assigned_stand.as_deref() {
                        label(ui, theme, &format!("Stand attribué : {}", s), LabelLevel::Body);
                    }
                    if let Some(m) = ee.size_meters {
                        label(ui, theme, &format!("Surface : {} m²", m), LabelLevel::Body);
                    }
                    if ee.assigned_stand.is_none() {
                        label(ui, theme, "Aucun stand attribué.", LabelLevel::Small);
                    }
                } else {
                    label(ui, theme, "—", LabelLevel::Body);
                }
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                if button(ui, theme, "Générer devis", ButtonVariant::Primary, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::OrgDevisFactures));
                }
                if button(ui, theme, "Factures", ButtonVariant::Secondary, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::OrgDevisFactures));
                }
                if button(ui, theme, "Retour liste", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::OrgListeExposants));
                }
            });
        },
    );
}
