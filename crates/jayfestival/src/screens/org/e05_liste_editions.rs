//! ORG-E05 — Liste des éditions.
//!
//! Conforme [Specification UI § 4.2] et [Organisateurs - Ecrans et cycle].
//! GestionLayout, Card, Select, Button. Sortie vers Dashboard édition, Création édition.

use crate::screens::ScreenId;
use crate::supabase::Edition;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::gestion_layout_show;
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_org_e05
/// @do: define_org_liste_editions_screen_component
/// @layer: app
/// Écran liste des éditions : grille/liste d'éditions, accès Dashboard édition, Création édition, Duplication.

/// @id: org_e05_show
/// @do: render_org_liste_editions_with_gestion_layout_and_cards
/// @layer: app
/// Affiche la liste des éditions avec cartes cliquables et bouton Créer une édition.
pub fn org_e05_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition_options: &[String],
    selected_edition: &mut usize,
    editions: &[Edition],
    pending_edition_idx: &mut Option<usize>,
) {
    let nav_labels = ["Mon compte", "Déconnexion"];
    gestion_layout_show(
        ctx,
        theme,
        "Mes éditions",
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
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(20)));
            }
            if button(ui, theme, "Équipe", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgReserved(21)));
            }
        },
        |ui| {
            label(ui, theme, "Liste des éditions", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if button(ui, theme, "Créer une édition", ButtonVariant::Primary, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::OrgCreationEdition));
            }
            ui.add_space(theme.item_spacing() * 2.0);

            for (idx, edition) in editions.iter().enumerate().take(50) {
                let name = edition.name.as_deref().unwrap_or("Sans nom");
                let dates = format!(
                    "{} — {}",
                    edition.start_date.as_deref().unwrap_or("?"),
                    edition.end_date.as_deref().unwrap_or("?")
                );
                let location = edition.location.as_deref().unwrap_or("—");
                let idx = idx.min(edition_options.len().saturating_sub(1));
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        label(ui, theme, &dates, LabelLevel::Body);
                        label(ui, theme, location, LabelLevel::Small);
                        if button(ui, theme, "Ouvrir", ButtonVariant::Secondary, Default::default()).clicked() {
                            *pending_edition_idx = Some(idx);
                            let _ = nav_request.replace(Some(ScreenId::OrgDashboardEdition));
                        }
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                ui.add_space(theme.item_spacing());
            }
        },
    );
}
