//! UNC-E07 — Fiche organisateur (détail public).
//!
//! Conforme [Specification UI § 4.1]. Layout, Card, Label, Button.

use crate::screens::ScreenId;
use crate::data::Organisateur;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e07
/// @do: define_fiche_organisateur_screen_component
/// @layer: app
/// Écran fiche organisateur : nom, description, événements publiés, contact, charte.

/// @id: unc_e07_show
/// @do: render_fiche_organisateur_with_blocks
/// @layer: app
/// Affiche la fiche organisateur avec blocs (présentation, événements, contact).
pub fn unc_e07_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    organisateur: Option<&Organisateur>,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Organisateurs", "Exposants", "Se connecter"];
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
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            }
        },
        |ui| {
            if button(ui, theme, "Retour liste", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            }
        },
        |ui| {
            let (name, desc, contact, website) = match organisateur {
                Some(o) => (
                    o.name.as_deref().unwrap_or("Sans nom"),
                    o.description.as_deref().unwrap_or(""),
                    o.contact_email.as_deref().unwrap_or("—"),
                    o.website.as_deref().unwrap_or("—"),
                ),
                None => ("Organisateur", "", "—", "—"),
            };

            card_show(theme, ui, Some(name), |ui| {
                if !desc.is_empty() {
                    label(ui, theme, desc, LabelLevel::Body);
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Événements publiés"), |ui| {
                label(ui, theme, "Liste des événements (liens vers fiches).", LabelLevel::Muted);
                if button(ui, theme, "Voir les événements", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Contact"), |ui| {
                label(ui, theme, &format!("Email : {}", contact), LabelLevel::Body);
                label(ui, theme, &format!("Site : {}", website), LabelLevel::Body);
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour liste organisateurs", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            }
        },
    );
}
