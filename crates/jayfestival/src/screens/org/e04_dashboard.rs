//! ORG-E04 — Tableau de bord organisateur.
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, HeaderWithEdition, Card. Entrée après Connexion.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e04
/// @do: define_org_dashboard_screen_component
/// @layer: app
/// Écran tableau de bord organisateur : synthèse éditions, accès Liste éditions, Création édition, Mon compte, Équipe.

/// @id: org_e04_show
/// @do: render_org_dashboard_with_gestion_layout_and_cards
/// @layer: app
/// Affiche le tableau de bord organisateur : GestionLayout, cartes d'accès (Éditions, Nouvelle édition, Mon compte, Équipe).
pub fn org_e04_show(
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
        "Tableau de bord organisateur",
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
            if button(ui, theme, "Nouvelle édition", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgCreationEdition));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(20))); // E20 Mon compte
            }
            if button(ui, theme, "Équipe", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(21))); // E21 Équipe
            }
        },
        |ui| {
            label(ui, theme, "Tableau de bord", LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            egui::Grid::new("org_e04_grid").num_columns(2).show(ui, |ui| {
                card_show(theme, ui, Some("Mes éditions"), |ui| {
                    label(ui, theme, "Consulter et gérer vos éditions.", LabelLevel::Body);
                    if button(ui, theme, "Voir les éditions", ButtonVariant::Primary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgListeEditions));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();

                card_show(theme, ui, Some("Créer une édition"), |ui| {
                    label(ui, theme, "Lancer une nouvelle édition d'événement.", LabelLevel::Body);
                    if button(ui, theme, "Créer", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgCreationEdition));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.end_row();
            });
        },
    );
}
