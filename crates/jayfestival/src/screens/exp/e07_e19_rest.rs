//! EXP-E07 à E19 — Écrans restants (Fiche candidature, Annuaire, Dépôt, Agenda, Fiche participation, Plan de salle, Programme, Documents, Factures, Mon compte, Fiche publique, Notifications).
//!
//! Conforme [Specification UI § 4.2] et [Exposants - Ecrans et cycle].
//! GestionLayout, atoms/molecules. Regroupés pour la tâche [174].

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_exp_e07_e19
/// @do: define_exp_rest_screens_component
/// @layer: app
/// Écrans EXP-E07 à E19 : Fiche candidature (7), Annuaire événements (8), Agenda (9), Dépôt candidature (10), Fiche participation (11), Plan de salle (11b), Programme (11c), Documents (12), Factures (13), Mon compte (17), Fiche publique (18), Notifications (19).

/// @id: exp_e07_e19_show
/// @do: render_exp_rest_screen_by_reserved_id
/// @layer: app
/// Affiche l'écran EXP correspondant à ExpReserved(n) : titre et contenu placeholder selon n.
pub fn exp_e07_e19_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    reserved_id: u8,
) {
    let (title, description) = match reserved_id {
        7 => ("Fiche candidature", "Détail et suivi d'une candidature (données envoyées, pièces, statut). Modifier / Annuler si en attente."),
        8 => ("Annuaire des événements", "Événements ouverts aux candidatures. Candidater par événement."),
        9 => ("Agenda exposant", "Calendrier cross-événements, conflits de dates, export iCal / PDF."),
        10 => ("Dépôt d'une candidature", "Formulaire de candidature pour une édition (champs définis par l'organisateur, pièces jointes)."),
        11 => ("Fiche participation", "Résumé édition validée, documents, emplacement, programme, facturation."),
        12 => ("Documents par édition", "Consulter et télécharger les documents reçus ; envoyer documents signés ou complétés."),
        13 => ("Devis et factures", "Consulter devis et factures par édition ; accepter/refuser devis ; statut de paiement."),
        17 => ("Mon compte", "Profil et fiche entreprise (nom, contact, activité, logo, site web)."),
        18 => ("Ma fiche publique", "Prévisualiser ou gérer les informations affichées dans le répertoire des exposants."),
        19 => ("Notifications et préférences", "Liste des notifications ; préférences (types, canaux) ; historique des communications."),
        _ => ("Espace exposant", "Écran réservé."),
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
            label(ui, theme, title, LabelLevel::Heading);
            ui.add_space(theme.item_spacing() * 2.0);

            card_show(theme, ui, Some(description), |ui| {
                label(ui, theme, "Contenu alpha : à compléter selon les spécifications (Exposants - Ecrans et cycle).", LabelLevel::Small);
                if reserved_id == 17 || reserved_id == 18 {
                    if button(ui, theme, "Retour tableau de bord", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpDashboard));
                    }
                } else if reserved_id == 8 {
                    if button(ui, theme, "Déposer une candidature", ButtonVariant::Primary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::ExpReserved(10)));
                    }
                } else if reserved_id == 10 {
                    ui.horizontal(|ui| {
                        if button(ui, theme, "Prévisualiser", ButtonVariant::Ghost, Default::default()).clicked() {}
                        if button(ui, theme, "Envoyer", ButtonVariant::Primary, Default::default()).clicked() {
                            let _ = nav_request.replace(Some(ScreenId::ExpCandidatures));
                        }
                        if button(ui, theme, "Annuler", ButtonVariant::Outline, Default::default()).clicked() {
                            let _ = nav_request.replace(Some(ScreenId::ExpCandidatures));
                        }
                    });
                }
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
