//! UNC-E13 — Inscription (choix du type de compte).
//!
//! Conforme [Specification UI § 4.1]. RoleCard/CTACard, Button, Auth [83].

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::organisms::{header_render, layout_show, roles_grid_render, RoleCardItem};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e13
/// @do: define_inscription_screen_component
/// @layer: app
/// Écran inscription : trois options (organisateur, exposant, visiteur), lien Se connecter.

/// @id: unc_e13_show
/// @do: render_inscription_choice_with_role_cards
/// @layer: app
/// Affiche le choix du type d'inscription (RoleCards) ; redirection vers formulaire correspondant (hors UNC).
pub fn unc_e13_show(
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
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
        |ui| {
            label(ui, theme, "S'inscrire", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());
            label(ui, theme, "Choisissez le type de compte :", LabelLevel::Body);
            ui.add_space(theme.item_spacing());

            let roles = [
                RoleCardItem {
                    title: "S'inscrire en tant qu'organisateur".into(),
                    description: Some("Gérer mes éditions et événements.".into()),
                    accent_color: theme.accent_organisateur(),
                },
                RoleCardItem {
                    title: "S'inscrire en tant qu'exposant".into(),
                    description: Some("Candidater et gérer mes participations.".into()),
                    accent_color: theme.accent_exposant(),
                },
                RoleCardItem {
                    title: "S'inscrire en tant que visiteur".into(),
                    description: Some("Réserver, acheter des pass.".into()),
                    accent_color: theme.accent_visiteur(),
                },
            ];
            roles_grid_render(theme, ui, &roles);

            ui.add_space(theme.item_spacing() * 2.0);
            // Clic sur une RoleCard : formulaire inscription (hors périmètre UNC alpha → on reste sur écran ou on navigue vers Connexion après "inscription")
            label(ui, theme, "Après choix : formulaire d'inscription (organisateur / exposant / visiteur).", LabelLevel::Muted);
            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Déjà un compte ? Se connecter", ButtonVariant::Outline, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            }
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
