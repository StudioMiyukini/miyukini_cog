//! UNC-E08 — Liste des exposants.
//!
//! Conforme [Specification UI § 4.1]. Layout, Card, Input, Select (données JayXpose/Supabase).

use crate::screens::ScreenId;
use crate::data::Exposant;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, input, select, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e08
/// @do: define_liste_exposants_screen_component
/// @layer: app
/// Écran liste des exposants : filtres (recherche, catégorie, événement, région), cartes.

/// @id: unc_e08_show
/// @do: render_liste_exposants_with_filters_and_cards
/// @layer: app
/// Affiche la liste des exposants avec filtres et cartes (données JayXpose/Supabase).
pub fn unc_e08_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    exposants: &[Exposant],
    filter_search: &mut String,
    filter_categorie_idx: &mut usize,
    categorie_options: &[String],
    selected_exposant_id: &mut Option<String>,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Organisateurs", "Se connecter", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.get(0).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            } else if responses.get(3).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncConnexion));
            } else if responses.get(4).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncInscription));
            }
        },
        |ui| {
            if button(ui, theme, "Accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
            if button(ui, theme, "Exposants", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
            }
            if button(ui, theme, "Recherche", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncRecherche));
            }
        },
        |ui| {
            label(ui, theme, "Exposants", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Recherche", LabelLevel::Body);
                input(ui, theme, filter_search, Some("Nom, catégorie"), false, true);
                if !categorie_options.is_empty() {
                    select(ui, theme, filter_categorie_idx, categorie_options, Some("Catégorie"), 2u64);
                }
            });

            ui.add_space(theme.item_spacing() * 2.0);

            for exp in exposants.iter().take(50) {
                let name = exp.company_name.as_deref().or(exp.stand_name.as_deref()).unwrap_or("Sans nom");
                let adresse = exp.adresse.as_deref().unwrap_or("—");
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        label(ui, theme, adresse, LabelLevel::Small);
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                if button(ui, theme, "Voir la fiche", ButtonVariant::Primary, Default::default()).clicked() {
                    *selected_exposant_id = exp.id.clone();
                    let _ = nav_request.replace(Some(ScreenId::UncFicheExposant));
                }
                ui.add_space(theme.item_spacing());
            }

            if exposants.is_empty() {
                label(ui, theme, "Aucun exposant pour le moment.", LabelLevel::Muted);
            }

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
