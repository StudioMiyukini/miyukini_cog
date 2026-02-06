//! UNC-E01 — Landing / Page d'accueil JayFestival.
//!
//! Conforme wireframe Page accueil : Header (Accueil, titre, connexion, conf),
//! Sidebar (News, Mes évènements favoris, J'organise, Informations), zone principale
//! (Mes évènements, Les évènements des 12 prochains mois, News).

use crate::screens::ScreenId;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::{event_card_landing_render, news_card_landing_render};
use crate::ui::organisms::{header_landing_render, layout_show};
use crate::ui::{button, label, ButtonVariant, EventCardLandingItem, LabelLevel, NewsCardLandingItem};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e01
/// @do: define_landing_screen_component
/// @layer: app
/// Écran d'accueil : header, sidebar, blocs Mes évènements, 12 prochains mois, News.

/// Données de démo pour les cartes événement (à remplacer par données Opérateur de Service).
fn placeholder_events() -> Vec<EventCardLandingItem> {
    (1..=6)
        .map(|i| EventCardLandingItem {
            nom: format!("[nom event {}]", i),
            type_event: "[type event]".into(),
            date: "Date".into(),
            lieu: "Lieu".into(),
        })
        .collect()
}

/// Données de démo pour les news (à remplacer par données Opérateur de Service).
fn placeholder_news() -> Vec<NewsCardLandingItem> {
    vec![
        NewsCardLandingItem {
            titre: "Article 1".into(),
            type_article: "Type".into(),
            description: "short description".into(),
        },
        NewsCardLandingItem {
            titre: "Article 2".into(),
            type_article: "Type".into(),
            description: "short description".into(),
        },
        NewsCardLandingItem {
            titre: "Article 3".into(),
            type_article: "Type".into(),
            description: "short description".into(),
        },
    ]
}

/// @id: unc_e01_show
/// @do: render_landing_screen_with_header_sidebar_blocks
/// @layer: app
/// Affiche la page d'accueil : header (Accueil, titre, connexion, conf), sidebar, blocs contenu.
pub fn unc_e01_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
) {
    let events = placeholder_events();
    let events_12 = placeholder_events();
    let news = placeholder_news();

    layout_show(
        ctx,
        theme,
        |ui| {
            let resp = header_landing_render(ui, theme, "JayFestival");
            if resp.connexion_clicked {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            }
            if resp.conf_clicked {
                // TODO: écran paramètres / conf
            }
        },
        |ui| {
            ui.vertical(|ui| {
                ui.add_space(theme.item_spacing());
                label(ui, theme, "Sidebar", LabelLevel::Small);
                ui.add_space(theme.item_spacing() * 2.0);

                label(ui, theme, "News", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                if button(ui, theme, "News", ButtonVariant::Ghost, Default::default()).clicked() {
                    // lien News
                }
                if button(ui, theme, "Les Organisateurs", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
                }
                if button(ui, theme, "Les exposants", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
                }
                if button(ui, theme, "Les évènements", ButtonVariant::Ghost, Default::default()).clicked() {
                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                }
                ui.add_space(theme.item_spacing() * 2.0);

                label(ui, theme, "Mes évènements favoris", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                ui.label("- [nom de l'event]");
                ui.label("- [nom de l'event]");
                ui.add_space(theme.item_spacing() * 2.0);

                label(ui, theme, "J'organise", LabelLevel::Body);
                ui.add_space(theme.item_spacing());
                ui.label("- [nom de l'event]");
                ui.label("- [nom de l'event]");
                ui.add_space(theme.item_spacing() * 2.0);

                ui.add_space(theme.item_spacing() * 4.0);
                label(ui, theme, "Informations", LabelLevel::Small);
                label(ui, theme, "2026 Miyukini", LabelLevel::Small);
            });
        },
        |ui| {
            ui.vertical(|ui| {
                ui.add_space(theme.item_spacing());

                // Container My_event_block — Mes évènements (défilement horizontal)
                label(ui, theme, "Mes évènements", LabelLevel::Heading);
                ui.add_space(theme.item_spacing());
                egui::ScrollArea::horizontal()
                    .id_salt("jayfestival_landing_my_events_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for item in &events {
                                let r = event_card_landing_render(theme, ui, item);
                                if r.clicked() {
                                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                                }
                                ui.add_space(theme.item_spacing());
                            }
                        });
                    });
                ui.add_space(theme.item_spacing() * 2.0);

                // Container Next_12_month_events_block — Les évènements des 12 prochains mois
                ui.horizontal(|ui| {
                    label(ui, theme, "Les évènements des 12 prochains mois", LabelLevel::Heading);
                    ui.add_space(theme.item_spacing());
                    if button(ui, theme, "Mois suivant", ButtonVariant::Ghost, Default::default()).clicked() {
                        // TODO: pagination mois
                    }
                });
                ui.add_space(theme.item_spacing());
                egui::ScrollArea::horizontal()
                    .id_salt("jayfestival_landing_12months_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for item in &events_12 {
                                let r = event_card_landing_render(theme, ui, item);
                                if r.clicked() {
                                    let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
                                }
                                ui.add_space(theme.item_spacing());
                            }
                        });
                    });
                ui.add_space(theme.item_spacing() * 2.0);

                // Container News_block — News
                label(ui, theme, "News", LabelLevel::Heading);
                ui.add_space(theme.item_spacing());
                ui.horizontal(|ui| {
                    for item in &news {
                        let r = news_card_landing_render(theme, ui, item);
                        if r.clicked() {
                            // TODO: fiche article
                        }
                        ui.add_space(theme.item_spacing());
                    }
                });
            });
        },
    );
}
