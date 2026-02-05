//! UNC-E12 — Connexion.
//!
//! Conforme [Specification UI § 4.1]. Input, Button, Label, Auth service [83].

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use std::cell::RefCell;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, input, ButtonVariant, LabelLevel};
use eframe::egui;

/// @id: screen_unc_e12
/// @do: define_connexion_screen_component
/// @layer: app
/// Écran connexion : champs email, mot de passe, bouton Se connecter, liens Mot de passe oublié / S'inscrire.

/// @id: unc_e12_show
/// @do: render_connexion_form_and_call_auth_sign_in
/// @layer: app
/// Affiche le formulaire de connexion ; appelle auth_sign_in côté app après clic Se connecter.
pub fn unc_e12_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    email: &mut String,
    password: &mut String,
    error_message: Option<&str>,
    submit_requested: &mut bool,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Organisateurs", "Exposants", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.get(0).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            } else if responses.get(3).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            } else if responses.get(4).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }
        },
        |ui| {
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
        |ui| {
            label(ui, theme, "Se connecter", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            if let Some(err) = error_message {
                label(ui, theme, err, LabelLevel::Body);
                ui.add_space(theme.item_spacing());
            }

            card_show(theme, ui, None, |ui| {
                label(ui, theme, "Email", LabelLevel::Body);
                input(ui, theme, email, Some("email@exemple.fr"), false, true);
                ui.add_space(theme.item_spacing());
                label(ui, theme, "Mot de passe", LabelLevel::Body);
                input(ui, theme, password, Some("••••••••"), true, true);
                ui.add_space(theme.item_spacing());
                if button(ui, theme, "Se connecter", ButtonVariant::Primary, Default::default()).clicked() {
                    *submit_requested = true;
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Mot de passe oublié ?", ButtonVariant::Ghost, Default::default()).clicked() {
                // TODO: écran récupération mot de passe (hors alpha)
            }
            if button(ui, theme, "S'inscrire", ButtonVariant::Outline, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
