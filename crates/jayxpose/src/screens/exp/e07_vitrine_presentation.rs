//! XP-E07 — Page de présentation vitrine.
//!
//! Éditeur de contenu texte brut pour la page de présentation + aperçu en dessous.

use crate::data::{JayXposeDb, VitrinePage};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e07
/// @do: render_showcase_presentation_editor
/// @layer: app
/// Écran page de présentation vitrine : éditeur texte brut + aperçu + sauvegarde.
/// Affiche l'écran XP-E07 — Page présentation vitrine.
pub fn exp_e07_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Charger le contenu de la page présentation si vide.
    if state.presentation_content.is_empty() {
        if let Some(page) = state
            .vitrine_pages
            .iter()
            .find(|p| p.page_type.as_deref() == Some("presentation"))
        {
            state.presentation_content = page.content.clone().unwrap_or_default();
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Paramètres vitrine").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrineParams));
                }
                ui.heading("Page de présentation");
            });
            ui.add_space(theme.section_spacing());

            // --- Éditeur ---
            ui.label(
                egui::RichText::new("Contenu de la page (texte brut)")
                    .size(theme.font_size_body())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());
            ui.add(
                egui::TextEdit::multiline(&mut state.presentation_content)
                    .desired_width(ui.available_width())
                    .desired_rows(12)
                    .hint_text("Rédigez ici la présentation de votre entreprise…"),
            );

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Aperçu ---
            ui.label(
                egui::RichText::new("Aperçu")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());
            egui::Frame::new()
                .fill(theme.section_background())
                .corner_radius(theme.border_radius())
                .inner_margin(egui::Margin::same(12))
                .show(ui, |ui| {
                    if state.presentation_content.trim().is_empty() {
                        ui.label(
                            egui::RichText::new("Aucun contenu pour l'instant.")
                                .color(theme.text_secondary()),
                        );
                    } else {
                        ui.label(
                            egui::RichText::new(&state.presentation_content)
                                .color(theme.text_primary()),
                        );
                    }
                });

            ui.add_space(theme.section_spacing());

            // --- Boutons ---
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    save_presentation(state, db);
                }
                if ui.button("Prévisualiser la vitrine").clicked() {
                    nav_request.replace(Some(ScreenId::ExpVitrinePreview));
                }
            });

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(egui::RichText::new(msg).color(theme.accent()));
            }
        });
    });
}

/// Enregistre le contenu de la page présentation dans la DB.
fn save_presentation(state: &mut ExpState, db: &Arc<JayXposeDb>) {
    // Trouver ou créer la page "presentation".
    let page = if let Some(existing) = state
        .vitrine_pages
        .iter_mut()
        .find(|p| p.page_type.as_deref() == Some("presentation"))
    {
        existing.content = Some(state.presentation_content.clone());
        existing.clone()
    } else {
        let new_page = VitrinePage {
            id: None,
            exposant_id: state.current_exposant_id.clone(),
            page_type: Some("presentation".to_string()),
            content: Some(state.presentation_content.clone()),
            is_visible: Some(true),
            sort_order: Some(2),
            updated_at: None,
        };
        state.vitrine_pages.push(new_page.clone());
        new_page
    };

    match db.vitrine_page_upsert(&page) {
        Ok(()) => {
            state.status_message = Some("Page de présentation enregistrée.".to_string());
        }
        Err(e) => {
            state.status_message = Some(format!("Erreur enregistrement : {}", e));
        }
    }
}
