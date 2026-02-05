//! EXP-E04 — Tableau de bord exposant (accueil).
//!
//! Conforme [Specification UI § 4.2] et [Exposants - Ecrans et cycle].
//! GestionLayout, Card. Vue d'ensemble : candidatures, participations, agenda, documents, factures.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_exp_e04
/// @do: define_exp_dashboard_screen_component
/// @layer: app
/// Écran tableau de bord exposant : synthèse candidatures en attente, prochain événement, alertes ; accès Candidatures, Participations, Agenda, Documents, Factures, Mon compte.

/// @id: exp_e04_show
/// @do: render_exp_dashboard_with_gestion_layout_and_cards
/// @layer: app
/// Affiche le tableau de bord exposant : GestionLayout, blocs synthèse et cartes d'accès (Candidatures, Participations, Agenda, Documents, Factures).
pub fn exp_e04_show(
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
        "Tableau de bord exposant",
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
            if button(ui, theme, "Notifications", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(19)));
            }
        },
        |ui| {
            label(ui, theme, "Tableau de bord exposant", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Synthèse"), |ui| {
                label(ui, theme, "Candidatures en attente, prochain événement, alertes (conflit dates, document à signer, facture à payer).", LabelLevel::Body);
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            egui::Grid::new("exp_e04_grid").num_columns(2).show(ui, |ui| {
                card_show(theme, ui, Some("Mes candidatures"), |ui| {
                    label(ui, theme, "Consulter et déposer des candidatures.", LabelLevel::Body);
                    if button(ui, theme, "Voir les candidatures", ButtonVariant::Primary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpCandidatures));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Mes participations"), |ui| {
                    label(ui, theme, "Éditions auxquelles vous participez (validé).", LabelLevel::Body);
                    if button(ui, theme, "Voir les participations", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpParticipations));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Agenda"), |ui| {
                    label(ui, theme, "Calendrier cross-événements, export.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir l'agenda", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(9)));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Documents et factures"), |ui| {
                    label(ui, theme, "Documents reçus, devis et factures.", LabelLevel::Body);
                    ui.horizontal(|ui| {
                        if button(ui, theme, "Documents", ButtonVariant::Ghost, Default::default()).clicked() {
                            let _ = nav_request.replace(Some(ScreenId::ExpReserved(12)));
                        }
                        if button(ui, theme, "Factures", ButtonVariant::Ghost, Default::default()).clicked() {
                            let _ = nav_request.replace(Some(ScreenId::ExpReserved(13)));
                        }
                    });
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();
            });
        },
    );
}
