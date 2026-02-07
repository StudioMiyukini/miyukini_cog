//! VIS-E06 à E15 — Billets, réservations, pass VIP, réservation (flux), fiche événement connecté, activités, jeux, concours, mon compte, préférences notification.
//!
//! Conforme [Visiteurs - Ecrans et cycle]. GestionLayout, Card, Button. Regroupés pour la tâche [183].

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_vis_e06_e15
/// @do: define_vis_rest_screens_component
/// @layer: app
/// Écrans VIS-E06 à E15 : Billets (6), Réservations (7), Pass VIP (8), Réservation flux (9), Fiche événement connecté (10), Activités (11), Jeux (12), Concours (13), Mon compte (14), Préférences notification (15).

/// @id: vis_e06_e15_show
/// @do: render_vis_rest_screen_by_reserved_id
/// @layer: app
/// Affiche l'écran VIS correspondant à VisReserved(n) : titre et contenu placeholder selon n.
pub fn vis_e06_e15_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    reserved_id: u8,
) {
    let (title, description) = match reserved_id {
        6 => ("Mes billets", "Consulter et télécharger les billets et tickets acquis (par événement). Télécharger PDF, afficher QR code."),
        7 => ("Mes réservations", "Consulter les réservations en cours (ateliers, créneaux, places). Annuler ou modifier selon règles de l'édition."),
        8 => ("Mes pass", "Pass VIP et pass journée acquis ; avantages associés ; QR code ou justificatif téléchargeable."),
        9 => ("Réserver", "Réserver un atelier, un créneau ou une place ; ou acheter un pass. Vérification agenda (conflit) avant validation."),
        10 => ("Fiche événement (connecté)", "Fiche événement avec services activés : Réserver, Acheter pass, Participer (jeux, concours)."),
        11 => ("Mes activités", "Historique des participations : jeux joués, concours, ateliers suivis, récompenses."),
        12 => ("Jeux", "Participer aux jeux proposés (quizz, chasses au trésor, défis) ; suivi des scores."),
        13 => ("Concours", "S'inscrire et participer aux concours ; consulter les résultats et les récompenses."),
        14 => ("Mon compte", "Consulter et modifier le profil visiteur (nom, prénom, email, préférences)."),
        15 => ("Préférences de notification", "Configurer les préférences : types (rappels, changements, alertes, récompenses), canaux (email, in-app)."),
        _ => ("Espace visiteur", "Écran réservé."),
    };

    let nav_labels = ["Catalogue", "Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        title,
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
            label(ui, theme, title, LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some(description), |ui| {
                label(ui, theme, "Contenu alpha : à compléter selon les spécifications (Visiteurs - Ecrans et cycle).", LabelLevel::Small);
                if reserved_id == 14
                    && button(ui, theme, "Préférences de notification", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisReserved(15)));
                    }
                if reserved_id == 15 || reserved_id == 14 {
                    if button(ui, theme, "Retour tableau de bord", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::VisDashboard));
                    }
                } else if button(ui, theme, "Retour tableau de bord", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::VisDashboard));
                }
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
