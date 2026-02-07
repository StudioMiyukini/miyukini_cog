//! UNC-E02 — Liste des événements.
//!
//! Conforme [Specification UI § 4.1]. Layout, Card, Input, Select, Button.

use crate::screens::ScreenId;
use crate::data::Edition;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, input, select, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e02
/// @do: define_liste_evenements_screen_component
/// @layer: app
/// Écran liste des événements : filtres, vue liste/cartes, pagination.

/// @id: unc_e02_show
/// @do: render_liste_evenements_with_filters_and_cards
/// @layer: app
/// Affiche la liste des événements avec filtres (date, lieu, organisateur, thème) et cartes.
pub fn unc_e02_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    editions: &[Edition],
    filter_date: &mut String,
    filter_lieu: &mut String,
    filter_theme_idx: &mut usize,
    theme_options: &[String],
    selected_edition_id: &mut Option<String>,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Organisateurs", "Exposants", "Se connecter", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.first().and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            } else if responses.get(3).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            } else if responses.get(4).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }
        },
        |ui| {
            if button(ui, theme, "Accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
            if button(ui, theme, "Événements", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            }
            if button(ui, theme, "Recherche", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncRecherche));
            }
        },
        |ui| {
            label(ui, theme, "Événements", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Date", LabelLevel::Body);
                input(ui, theme, filter_date, Some("à venir"), false, true);
                label(ui, theme, "Lieu", LabelLevel::Body);
                input(ui, theme, filter_lieu, Some("Région ou ville"), false, true);
                if !theme_options.is_empty() {
                    select(ui, theme, filter_theme_idx, theme_options, Some("Thème"), 0u64);
                }
                if button(ui, theme, "Réinitialiser", ButtonVariant::Outline, Default::default()).clicked() {
                    *filter_date = String::new();
                    *filter_lieu = String::new();
                    *filter_theme_idx = 0;
                }
            });

            ui.add_space(theme.item_spacing() * 2.0);

            for edition in editions.iter().take(50) {
                let name = edition.name.as_deref().unwrap_or("Sans nom");
                let dates = format!(
                    "{} — {}",
                    edition.start_date.as_deref().unwrap_or("?"),
                    edition.end_date.as_deref().unwrap_or("?")
                );
                let location = edition.location.as_deref().unwrap_or("—");
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        label(ui, theme, &dates, LabelLevel::Body);
                        label(ui, theme, location, LabelLevel::Small);
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                if button(ui, theme, "Voir la fiche", ButtonVariant::Primary, Default::default()).clicked() {
                    *selected_edition_id = edition.id.clone();
                    let _ = nav_request.replace(Some(ScreenId::UncFicheEvenement));
                }
                ui.add_space(theme.item_spacing());
            }

            if editions.is_empty() {
                label(ui, theme, "Aucun événement pour le moment.", LabelLevel::Muted);
            }

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
