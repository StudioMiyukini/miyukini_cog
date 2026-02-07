//! UNC-E11 — CTA contextuels (message non connecté).
//!
//! Conforme [Specification UI § 4.1]. Modal / bandeau : message + boutons Se connecter, S'inscrire, Retour.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e11
/// @do: define_cta_contextuels_screen_component
/// @layer: app
/// Écran CTA contextuel : modal/bandeau avec message « réservé aux connectés » et boutons Connexion / Inscription / Retour.

/// @id: unc_e11_show
/// @do: render_cta_contextuels_modal_with_message_and_buttons
/// @layer: app
/// Affiche le message contextuel (Réserver / Candidater) avec boutons Se connecter, S'inscrire, Retour.
pub fn unc_e11_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    message: &str,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Se connecter", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.first().and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            } else if responses.get(3).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }
        },
        |_ui| {},
        |ui| {
            ui.add_space(theme.item_spacing() * 4.0);
            card_show(theme, ui, Some("Action réservée aux utilisateurs connectés"), |ui| {
                label(ui, theme, message, LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                ui.horizontal(|ui| {
                    if button(ui, theme, "Se connecter", ButtonVariant::Primary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncConnexion));
                    }
                    if button(ui, theme, "S'inscrire", ButtonVariant::Secondary, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncInscription));
                    }
                    if button(ui, theme, "Retour", ButtonVariant::Outline, Default::default()).clicked() {
                        let _ = nav_request.replace(Some(ScreenId::UncLanding));
                    }
                });
            }, None::<fn(&mut egui::Ui)>);
        },
    );
}
