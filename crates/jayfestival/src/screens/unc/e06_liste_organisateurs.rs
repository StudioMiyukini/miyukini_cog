//! UNC-E06 — Liste des organisateurs.
//!
//! Conforme [Specification UI § 4.1]. Layout, Card, Input, Select.

use crate::screens::ScreenId;
use crate::supabase::Organisateur;
use crate::theme::JayFestivalTheme;
use crate::ui::molecules::card_show;
use crate::ui::organisms::{header_render, layout_show};
use crate::ui::{button, label, input, select, ButtonVariant, LabelLevel};
use eframe::egui;
use std::cell::RefCell;

/// @id: screen_unc_e06
/// @do: define_liste_organisateurs_screen_component
/// @layer: app
/// Écran liste des organisateurs : filtres (nom, région, type, année), cartes.

/// @id: unc_e06_show
/// @do: render_liste_organisateurs_with_filters_and_cards
/// @layer: app
/// Affiche la liste des organisateurs avec filtres et cartes (nom, région, lien fiche).
pub fn unc_e06_show(
    ctx: &egui::Context,
    theme: &JayFestivalTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    organisateurs: &[Organisateur],
    filter_nom: &mut String,
    filter_region_idx: &mut usize,
    region_options: &[String],
    selected_organisateur_id: &mut Option<String>,
) {
    layout_show(
        ctx,
        theme,
        |ui| {
            let nav = ["Accueil", "Événements", "Exposants", "Se connecter", "S'inscrire"];
            let responses = header_render(ui, theme, "JayFestival", &nav);
            if responses.get(0).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            } else if responses.get(1).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeEvenements));
            } else if responses.get(2).and_then(|r| r.clicked().then_some(())).is_some() {
                let _ = nav_request.replace(Some(ScreenId::UncListeExposants));
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
            if button(ui, theme, "Organisateurs", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncListeOrganisateurs));
            }
            if button(ui, theme, "Recherche", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncRecherche));
            }
        },
        |ui| {
            label(ui, theme, "Organisateurs", LabelLevel::Heading);
            ui.add_space(theme.item_spacing());

            ui.horizontal(|ui| {
                label(ui, theme, "Nom", LabelLevel::Body);
                input(ui, theme, filter_nom, Some("Rechercher"), false, true);
                if !region_options.is_empty() {
                    select(ui, theme, filter_region_idx, region_options, Some("Région"), 1u64);
                }
            });

            ui.add_space(theme.item_spacing() * 2.0);

            for org in organisateurs.iter().take(50) {
                let name = org.name.as_deref().unwrap_or("Sans nom");
                let region = org.region.as_deref().unwrap_or("—");
                card_show(
                    theme,
                    ui,
                    Some(name),
                    |ui| {
                        label(ui, theme, region, LabelLevel::Body);
                    },
                    None::<fn(&mut egui::Ui)>,
                );
                if button(ui, theme, "Voir la fiche", ButtonVariant::Primary, Default::default()).clicked() {
                    *selected_organisateur_id = org.id.clone();
                    let _ = nav_request.replace(Some(ScreenId::UncFicheOrganisateur));
                }
                ui.add_space(theme.item_spacing());
            }

            if organisateurs.is_empty() {
                label(ui, theme, "Aucun organisateur pour le moment.", LabelLevel::Muted);
            }

            ui.add_space(theme.item_spacing());
            if button(ui, theme, "Retour accueil", ButtonVariant::Ghost, Default::default()).clicked() {
                let _ = nav_request.replace(Some(ScreenId::UncLanding));
            }
        },
    );
}
