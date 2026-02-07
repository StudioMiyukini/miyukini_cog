//! UNC-E03 — Fiche événement (détail public).
//!
//! Conforme [Specification UI § 4.1]. Layout, Card, Label, Button.

use crate::screens::ScreenId;
use crate::data::Edition;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e03
/// @do: define_fiche_evenement_screen_component
/// @layer: app
/// Écran fiche événement : titre, dates, lieu, organisateur, description, programme, exposants, CTAs.

/// @id: unc_e03_show
/// @do: render_fiche_evenement_with_blocks_and_ctas
/// @layer: app
/// Affiche la fiche événement avec blocs (présentation, programme, exposants) et CTAs (Réserver, Candidater).
pub fn unc_e03_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    edition: Option<&Edition>,
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
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            }
        },
        |ui| {
            let (name, dates, location, desc) = match edition {
                Some(e) => (
                    e.name.as_deref().unwrap_or("Sans nom"),
                    format!(
                        "{} — {}",
                        e.start_date.as_deref().unwrap_or("?"),
                        e.end_date.as_deref().unwrap_or("?")
                    ),
                    e.location.as_deref().unwrap_or("—"),
                    e.theme.as_deref().unwrap_or(""),
                ),
                None => ("Événement", "—".into(), "—", ""),
            };

            card_show(theme, ui, Some(name), |ui| {
                label(ui, theme, &dates, LabelLevel::Body);
                label(ui, theme, location, LabelLevel::Body);
                if !desc.is_empty() {
                    ui.add_space(theme.item_spacing());
                    label(ui, theme, desc, LabelLevel::Muted);
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Organisateur"), |ui| {
                if button(ui, theme, "Voir la fiche organisateur", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncFicheOrganisateur));
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Exposants de l'événement"), |ui| {
                label(ui, theme, "Liste des exposants (lien vers fiches).", LabelLevel::Muted);
                if button(ui, theme, "Voir liste exposants", ButtonVariant::Outline, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            card_show(theme, ui, Some("Services"), |ui| {
                if button(ui, theme, "Réserver un atelier / Acheter un pass", ButtonVariant::Primary, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncCtaContextuels));
                }
                if button(ui, theme, "Déposer une candidature exposant", ButtonVariant::Secondary, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncCtaContextuels));
                }
            }, None::<fn(&mut egui::Ui)>);

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour liste événements", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            }
        },
    );
}
