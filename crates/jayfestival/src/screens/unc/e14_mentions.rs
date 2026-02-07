//! UNC-E14 — Mentions légales, CGU, Confidentialité, Accessibilité.
//!
//! Conforme [Specification UI § 4.1]. Layout, Label, Button.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e14
/// @do: define_mentions_screen_component
/// @layer: app
/// Écran mentions légales : contenu lecture seule (éditeur, hébergeur, CGU, confidentialité, accessibilité).

/// @id: unc_e14_show
/// @do: render_mentions_legal_content_with_back_button
/// @layer: app
/// Affiche les mentions légales, CGU, politique de confidentialité, accessibilité ; bouton Retour.
pub fn unc_e14_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
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
            if button(ui, theme, "Retour", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
        |ui| {
            label(ui, theme, "Mentions légales", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());
            label(ui, theme, "Éditeur : JayFestival. Hébergeur : à définir. Contact : à définir.", LabelLevel::Body);
            ui.add_space(theme.item_spacing());
            label(ui, theme, "CGU — Conditions générales d'utilisation : contenu à compléter.", LabelLevel::Body);
            ui.add_space(theme.item_spacing());
            label(ui, theme, "Politique de confidentialité : données, cookies, RGPD — contenu à compléter.", LabelLevel::Body);
            ui.add_space(theme.item_spacing());
            label(ui, theme, "Accessibilité : engagement et contact — contenu à compléter.", LabelLevel::Body);
            ui.add_space(theme.item_spacing() * 2.0);
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
