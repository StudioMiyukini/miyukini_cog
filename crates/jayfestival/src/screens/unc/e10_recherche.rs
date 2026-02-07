//! UNC-E10 — Recherche (résultats et affinage).
//!
//! Conforme [Specification UI § 4.1]. Layout, Input, Card, Select.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use std::cell::RefCell;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, input, ButtonVariant, LabelLevel};
use eframe::egui;

/// @id: screen_unc_e10
/// @do: define_recherche_screen_component
/// @layer: app
/// Écran recherche : champ recherche, résultats par type (événements, organisateurs, exposants), affinage.

/// @id: unc_e10_show
/// @do: render_recherche_with_input_and_results_sections
/// @layer: app
/// Affiche la recherche avec champ, bouton Rechercher, résultats regroupés par type.
pub fn unc_e10_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    query: &mut String,
    has_searched: &mut bool,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Organisateurs", "Exposants", "Se connecter"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.first().and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            } else if responses.get(3).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            } else if responses.get(4).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            }
        },
        |ui| {
            if button(ui, theme, "Accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
            if button(ui, theme, "Recherche", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncRecherche));
            }
        },
        |ui| {
            label(ui, theme, "Recherche", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                input(
                    ui,
                    theme,
                    query,
                    Some("Rechercher un événement, un organisateur, un exposant"),
                    false,
                    true,
                );
                if button(ui, theme, "Rechercher", ButtonVariant::Primary, Default::default()).clicked() {
                    *has_searched = true;
                }
            });

            ui.add_space(theme.item_spacing() * 2.0);

            if *has_searched {
                card_show(theme, ui, Some("Événements"), |ui| {
                    label(ui, theme, "Résultats événements (ex. premier résultat).", LabelLevel::Muted);
                    if button(ui, theme, "Voir la fiche événement", ButtonVariant::Outline, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncFicheEvenement));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.add_space(theme.item_spacing());
                card_show(theme, ui, Some("Organisateurs"), |ui| {
                    label(ui, theme, "Résultats organisateurs.", LabelLevel::Muted);
                    if button(ui, theme, "Voir la fiche organisateur", ButtonVariant::Outline, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncFicheOrganisateur));
                    }
                }, None::<fn(&mut egui::Ui)>);
                ui.add_space(theme.item_spacing());
                card_show(theme, ui, Some("Exposants"), |ui| {
                    label(ui, theme, "Résultats exposants.", LabelLevel::Muted);
                    if button(ui, theme, "Voir la fiche exposant", ButtonVariant::Outline, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncFicheExposant));
                    }
                }, None::<fn(&mut egui::Ui)>);
            } else {
                label(ui, theme, "Saisissez un terme et cliquez sur Rechercher.", LabelLevel::Muted);
            }

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
