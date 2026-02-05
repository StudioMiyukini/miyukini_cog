//! ORG-E16 à E25 — Écrans restants (Annonces, Mon compte, Équipe, Paramétrage, etc.).
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, atoms/molecules. Regroupés pour la tâche [161].

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e16_e25
/// @do: define_org_rest_screens_component
/// @layer: app
/// Écrans ORG-E16 à E25 : Annonces (E23), Mon compte (E20), Équipe (E21), Services visiteur (E24), Publication (E25), etc.

/// @id: org_e16_e25_show
/// @do: render_org_rest_screen_by_reserved_id
/// @layer: app
/// Affiche l'écran ORG correspondant à OrgReserved(n) : titre et contenu placeholder selon n.
pub fn org_e16_e25_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    reserved_id: u8,
) {
    let (title, description) = match reserved_id {
        16 => ("Visualisation plan", "Visualisation du plan de salle."),
        17 => ("Programme (détail)", "Création / édition d'une animation."),
        18 => ("Import exposants", "Import CSV / tableur."),
        19 => ("Budget (détail)", "Saisie et ventilation budget."),
        20 => ("Mon compte", "Paramètres du compte organisateur."),
        21 => ("Équipe et invitations", "Membres et rôles (Admin, Manager, Bénévole)."),
        22 => ("Documents et légal", "Contrats types, CGV, règlements."),
        23 => ("Annonces et notifications", "Configuration et envoi d'annonces."),
        24 => ("Services visiteur", "Activation jeux, concours, ateliers, pass."),
        25 => ("Publication et clôture", "Publication et clôture de l'édition."),
        _ => ("Espace organisateur", "Écran réservé."),
    };

    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        title,
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
            if button(ui, theme, "Mon compte", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(20)));
            }
            if button(ui, theme, "Équipe", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(21)));
            }
        },
        |ui| {
            label(ui, theme, title, LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some(description), |ui| {
                label(ui, theme, "Contenu alpha : à compléter selon les spécifications.", LabelLevel::Small);
                if reserved_id == 25 {
                    ui.horizontal(|ui| {
                        if button(ui, theme, "Publier l'édition", ButtonVariant::Primary, Default::default()).clicked() {}
                        if button(ui, theme, "Clôturer", ButtonVariant::Outline, Default::default()).clicked() {
                            let _ = nav_request.replace(Some(ScreenId::OrgListeEditions));
                        }
                    });
                } else if reserved_id == 20 {
                    if button(ui, theme, "Retour tableau de bord", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::OrgDashboard));
                    }
                } else if reserved_id == 21 {
                    let _ = button(ui, theme, "Inviter un membre", ButtonVariant::Primary, Default::default());
                }
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
