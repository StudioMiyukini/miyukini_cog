//! UNC-E09 — Fiche exposant (détail public).
//!
//! Conforme [Specification UI § 4.1]. Layout, Card (données JayXpose/Supabase).

use crate::screens::ScreenId;
use crate::data::Exposant;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e09
/// @do: define_fiche_exposant_screen_component
/// @layer: app
/// Écran fiche exposant : entreprise, secteur, éditions participées, contact.

/// @id: unc_e09_show
/// @do: render_fiche_exposant_with_blocks
/// @layer: app
/// Affiche la fiche exposant avec blocs (présentation, éditions, contact).
pub fn unc_e09_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    exposant: Option<&Exposant>,
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
            if button(ui, theme, "Retour liste", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            }
        },
        |ui| {
            let name = exposant
                .and_then(|e| e.company_name.as_deref().or(e.stand_name.as_deref()))
                .unwrap_or("Exposant");
            let contact = exposant
                .as_ref()
                .and_then(|e| e.contact_email.as_deref())
                .unwrap_or("—");
            let phone = exposant
                .as_ref()
                .and_then(|e| e.contact_phone.as_deref())
                .unwrap_or("—");
            let adresse = exposant
                .as_ref()
                .and_then(|e| e.adresse.as_deref())
                .unwrap_or("—");

            card_show(theme, ui, Some(name), |ui| {
                label(ui, theme, &format!("Contact : {contact}"), LabelLevel::Body);
                label(ui, theme, &format!("Tél : {phone}"), LabelLevel::Body);
                label(ui, theme, adresse, LabelLevel::Small);
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Éditions participées"), |ui| {
                label(ui, theme, "Liste des événements (liens vers fiches).", LabelLevel::Muted);
                if button(ui, theme, "Voir les événements", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour liste exposants", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            }
        },
    );
}
