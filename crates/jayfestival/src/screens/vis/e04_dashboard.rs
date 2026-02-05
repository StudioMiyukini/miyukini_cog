//! VIS-E04 — Page d'accueil espace visiteur (dashboard).
//!
//! Conforme [Visiteurs - Ecrans et cycle]. GestionLayout, Card.
//! Vue d'ensemble : agenda, billets, réservations, pass VIP, suivi d'activités ; indicateurs (prochain événement, compte à rebours).

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_vis_e04
/// @do: define_vis_dashboard_screen_component
/// @layer: app
/// Écran page d'accueil espace visiteur : synthèse prochain événement, compte à rebours ; accès Agenda, Billets, Réservations, Pass VIP, Activités, Catalogue, Mon compte.

/// @id: vis_e04_show
/// @do: render_vis_dashboard_with_gestion_layout_and_cards
/// @layer: app
/// Affiche le dashboard visiteur : GestionLayout, blocs synthèse et cartes d'accès (Agenda, Billets, Réservations, Pass VIP, Activités).
pub fn vis_e04_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
) {
    let nav_labels = ["Catalogue", "Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Mon espace visiteur",
        edition_options,
        selected_edition,
        &nav_labels,
        |ui| {
            if button(ui, theme, "Tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisDashboard));
            }
            if button(ui, theme, "Agenda", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisAgenda));
            }
            if button(ui, theme, "Billets", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(6)));
            }
            if button(ui, theme, "Réservations", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(7)));
            }
            if button(ui, theme, "Pass VIP", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(8)));
            }
            if button(ui, theme, "Activités", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(11)));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::VisReserved(14)));
            }
        },
        |ui| {
            label(ui, theme, "Page d'accueil espace visiteur", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Synthèse"), |ui| {
                label(
                    ui,
                    theme,
                    "Prochain événement : à venir. Compte à rebours et indicateurs (agenda, billets, réservations, pass).",
                    LabelLevel::Body,
                );
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            egui::Grid::new("vis_e04_grid").num_columns(2).show(ui, |ui| {
                card_show(theme, ui, Some("Mon agenda"), |ui| {
                    label(ui, theme, "Calendrier personnel, ateliers réservés, export.", LabelLevel::Body);
                    if button(ui, theme, "Ouvrir l'agenda", ButtonVariant::Primary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisAgenda));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Mes billets"), |ui| {
                    label(ui, theme, "Billets et tickets acquis par événement.", LabelLevel::Body);
                    if button(ui, theme, "Voir les billets", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisReserved(6)));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Mes réservations"), |ui| {
                    label(ui, theme, "Réservations ateliers, créneaux, places.", LabelLevel::Body);
                    if button(ui, theme, "Voir les réservations", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisReserved(7)));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Mes pass"), |ui| {
                    label(ui, theme, "Pass VIP et avantages par événement.", LabelLevel::Body);
                    if button(ui, theme, "Voir les pass", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisReserved(8)));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Mes activités"), |ui| {
                    label(ui, theme, "Historique participations jeux, concours, ateliers.", LabelLevel::Body);
                    if button(ui, theme, "Voir les activités", ButtonVariant::Ghost, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisReserved(11)));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();
            });
        },
    );
}
