//! EXP-E05 — Liste des candidatures (exposant).
//!
//! Conforme [Specification UI § 4.2] et [Exposants - Ecrans et cycle].
//! GestionLayout, Card, Badge. Filtres statut, tableau/cartes, actions Voir/Modifier/Annuler, bouton Déposer une candidature.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_exp_e05
/// @do: define_exp_candidatures_screen_component
/// @layer: app
/// Écran liste des candidatures exposant : toutes les candidatures (en attente, validées, refusées) par édition ; Voir, Modifier, Annuler ; Déposer une candidature.

/// @id: exp_e05_show
/// @do: render_exp_candidatures_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des candidatures de l'exposant avec filtres, badges de statut et actions (Voir, Modifier, Annuler, Déposer).
pub fn exp_e05_show(
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
        "Mes candidatures",
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
            label(ui, theme, "Mes candidatures", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Déposer une candidature", ButtonVariant::Primary, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::ExpReserved(8)));
            }
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some("Filtres"), |ui| {
                ui.horizontal(|ui| {
                    label(ui, theme, "Statut :", LabelLevel::Small);
                    button(ui, theme, "Toutes", ButtonVariant::Ghost, Default::default()).clicked();
                    button(ui, theme, "En attente", ButtonVariant::Ghost, Default::default()).clicked();
                    button(ui, theme, "Validées", ButtonVariant::Ghost, Default::default()).clicked();
                    if button(ui, theme, "Refusées", ButtonVariant::Ghost, Default::default()).clicked() {}
                });
            }, None::<fn(&mut egui::Ui)>);
            ui.add_space(theme.item_spacing());

            card_show(theme, ui, Some("Liste des candidatures"), |ui| {
                label(ui, theme, "Aucune candidature pour le moment. Utilisez « Déposer une candidature » pour candidater à un événement.", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                if button(ui, theme, "Voir l'annuaire des événements", ButtonVariant::Secondary, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::ExpReserved(8)));
                }
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
