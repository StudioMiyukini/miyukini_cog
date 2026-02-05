//! UNC-E01 — Landing / Accueil catalogue.
//!
//! Conforme [Specification UI § 4.1] et [UtilisateurNonConnecte - Ecrans et cycle].
//! Layout, HeroSection, FeaturesGrid, DirectoryBanner, RolesGrid, CTASection.

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::FeatureCardVariant;
use crate::ui::organisms::{
    cta_section_render, directory_banner_render, features_grid_render, header_render,
    hero_section_render, layout_show, roles_grid_render, CtaCardItem, FeatureCardItem,
    RoleCardItem,
};
use crate::ui::{button, label, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e01
/// @do: define_landing_screen_component
/// @layer: app
/// Écran d'accueil catalogue : en-tête, recherche, héro, grilles, CTA.

/// @id: unc_e01_show
/// @do: render_landing_screen_with_hero_features_directory_roles_cta
/// @layer: app
/// Affiche la landing : header, sidebar nav, zone centrale (héro, recherche, features, directory, roles, CTA).
pub fn unc_e01_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Événements", "Organisateurs", "Exposants", "Se connecter", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.get(0).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
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
            ui.vertical(|ui| {
                ui.add_space(theme.item_spacing());
                label(ui, theme, "JayFestival", LabelLevel::Heading);
                ui.add_space(theme.item_spacing() * 2.0);
                if button(ui, theme, "Accueil", ButtonVariant::NavSelected, Default::default()).clicked() {
                    // déjà sur landing
                }
                if button(ui, theme, "Événements", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                }
                if button(ui, theme, "Organisateurs", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
                }
                if button(ui, theme, "Exposants", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
                }
                if button(ui, theme, "Recherche", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncRecherche));
                }
                ui.add_space(theme.item_spacing() * 2.0);
                if button(ui, theme, "Mentions", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncMentions));
                }
            });
        },
        |ui| {
            let hero_response = hero_section_render(
                ui,
                theme,
                "Découvrez les événements et festivals",
                "Recherchez un événement, un organisateur ou un exposant.",
                Some("Rechercher"),
            );
            if hero_response.clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncRecherche));
            }

            ui.add_space(theme.item_spacing() * 2.0);

            let features = [
                FeatureCardItem {
                    title: "Événements".into(),
                    description: "Liste des événements à venir.".into(),
                    icon: "📅".into(),
                    variant: FeatureCardVariant::Default,
                },
                FeatureCardItem {
                    title: "Organisateurs".into(),
                    description: "Structures organisatrices.".into(),
                    icon: "🏛️".into(),
                    variant: FeatureCardVariant::Default,
                },
                FeatureCardItem {
                    title: "Exposants".into(),
                    description: "Répertoire des exposants.".into(),
                    icon: "🏪".into(),
                    variant: FeatureCardVariant::Default,
                },
            ];
            features_grid_render(theme, ui, &features);

            ui.add_space(theme.item_spacing() * 2.0);

            let dir_ev = directory_banner_render(theme, ui, "Événements", Some("Prochains événements"), "Voir les événements");
            if dir_ev.clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            }
            let dir_org = directory_banner_render(theme, ui, "Organisateurs", Some("Répertoire des organisateurs"), "Voir les organisateurs");
            if dir_org.clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            }
            let dir_exp = directory_banner_render(theme, ui, "Exposants", Some("Répertoire des exposants"), "Voir les exposants");
            if dir_exp.clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            }

            ui.add_space(theme.item_spacing() * 2.0);

            let roles = [
                RoleCardItem {
                    title: "Organisateur".into(),
                    description: Some("Gérer mes éditions.".into()),
                    accent_color: theme.accent_organisateur(),
                },
                RoleCardItem {
                    title: "Exposant".into(),
                    description: Some("Candidater, gérer mes participations.".into()),
                    accent_color: theme.accent_exposant(),
                },
                RoleCardItem {
                    title: "Visiteur".into(),
                    description: Some("Réserver, acheter des pass.".into()),
                    accent_color: theme.accent_visiteur(),
                },
            ];
            roles_grid_render(theme, ui, &roles);

            ui.add_space(theme.item_spacing() * 2.0);

            let cta_items = [
                CtaCardItem {
                    title: "Se connecter".into(),
                    description: Some("Accéder à mon espace.".into()),
                    button_label: "Se connecter".into(),
                },
                CtaCardItem {
                    title: "S'inscrire".into(),
                    description: Some("Créer un compte.".into()),
                    button_label: "S'inscrire".into(),
                },
            ];
            let cta_responses = cta_section_render(theme, ui, &cta_items);
            if cta_responses.get(0).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            } else if cta_responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }

            ui.add_space(theme.item_spacing() * 2.0);
            label(ui, theme, "Mentions légales · CGU · Confidentialité · Accessibilité", LabelLevel::Small);
            if button(ui, theme, "Mentions légales", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncMentions));
            }
        },
    );
}
