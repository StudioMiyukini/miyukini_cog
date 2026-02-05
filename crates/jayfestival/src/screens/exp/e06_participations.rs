//! EXP-E06 — Liste des participations (éditions validées).
//!
//! Conforme [Specification UI § 4.2] et [Exposants - Ecrans et cycle].
//! GestionLayout, Card. Liste/cartes des éditions auxquelles l'exposant participe ; liens Documents, Factures, Plan de salle, Programme.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_exp_e06
/// @do: define_exp_participations_screen_component
/// @layer: app
/// Écran liste des participations : éditions validées avec accès Documents, Factures, Plan de salle, Programme ; clic vers Fiche participation (EXP-E11).

/// @id: exp_e06_show
/// @do: render_exp_participations_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des participations (éditions validées) avec cartes et liens vers Documents, Factures, Plan de salle, Programme.
pub fn exp_e06_show(
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
        "Mes participations",
        edition_options,
        selected_edition,
        &nav_labels,
        |ui| {
            if button(ui, theme, "Tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpDashboard));
            }
            if button(ui, theme, "Candidatures", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpCandidatures));
            }
            if button(ui, theme, "Participations", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpParticipations));
            }
            if button(ui, theme, "Agenda", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(9)));
            }
            if button(ui, theme, "Documents", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(12)));
            }
            if button(ui, theme, "Factures", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(13)));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(17)));
            }
        },
        |ui| {
            label(ui, theme, "Mes participations", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Éditions auxquelles vous participez"), |ui| {
                label(ui, theme, "Liste des éditions (nom, dates, lieu), statut, emplacement (stand). Liens : Documents, Factures, Plan de salle, Programme.", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                label(ui, theme, "Aucune participation pour le moment. Les participations apparaîtront ici une fois vos candidatures validées.", LabelLevel::Small);
                ui.add_space(theme.item_spacing());
                ui.horizontal(|ui| {
                    if button(ui, theme, "Documents", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(12)));
                    }
                    if button(ui, theme, "Factures", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(13)));
                    }
                    if button(ui, theme, "Plan de salle", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(11)));
                    }
                    if button(ui, theme, "Programme", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(11)));
                    }
                });
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
