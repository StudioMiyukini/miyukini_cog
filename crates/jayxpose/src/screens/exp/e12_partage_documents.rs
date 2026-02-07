//! XP-E12 — Partage de documents.
//!
//! Gestion des demandes de partage reçues (accepter/refuser), partages actifs (révoquer), historique.

use crate::data::JayXposeDb;
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e12
/// @do: render_document_sharing_management
/// @layer: app
/// Écran partage documents : demandes reçues (accepter/refuser), partages actifs (révoquer), historique.
/// Affiche l'écran XP-E12 — Partage documents.
pub fn exp_e12_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Rechargement si nécessaire.
    if state.partages.is_empty() {
        if let Some(ref eid) = state.current_exposant_id {
            if let Ok(partages) = db.partages_by_exposant(eid) {
                state.partages = partages;
            }
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Documents").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDocuments));
                }
                ui.heading("Partage de documents");
            });
            ui.add_space(theme.section_spacing());

            // --- Demandes de partage reçues (status = "demande") ---
            ui.label(
                egui::RichText::new("Demandes de partage reçues")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            let demandes_indices: Vec<usize> = state
                .partages
                .iter()
                .enumerate()
                .filter(|(_, p)| p.status.as_deref() == Some("demande"))
                .map(|(i, _)| i)
                .collect();

            if demandes_indices.is_empty() {
                ui.label(
                    egui::RichText::new("Aucune demande en attente.")
                        .color(theme.text_secondary()),
                );
            }

            let mut accept_idx: Option<usize> = None;
            let mut refuse_idx: Option<usize> = None;

            for &idx in &demandes_indices {
                let partage = &state.partages[idx];
                let target = partage.target_user_id.as_deref().unwrap_or("—");
                let context = partage.target_context_type.as_deref().unwrap_or("—");
                let message = partage.message.as_deref().unwrap_or("");
                let requested = partage.requested_at.as_deref().unwrap_or("—");

                egui::Frame::new()
                    .fill(theme.card_background())
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!("De : {}", target))
                                .color(theme.text_primary()),
                        );
                        ui.label(
                            egui::RichText::new(format!("Contexte : {} — Demandé le {}", context, requested))
                                .color(theme.text_secondary()),
                        );
                        if !message.is_empty() {
                            ui.label(
                                egui::RichText::new(format!("Message : {}", message))
                                    .color(theme.text_secondary()),
                            );
                        }
                        ui.horizontal(|ui| {
                            if ui.button("Accepter").clicked() {
                                accept_idx = Some(idx);
                            }
                            if ui.button("Refuser").clicked() {
                                refuse_idx = Some(idx);
                            }
                        });
                    });
                ui.add_space(theme.item_spacing());
            }

            // Gestion accept.
            if let Some(idx) = accept_idx {
                if let Some(ref pid) = state.partages[idx].id {
                    match db.partage_update_status(pid, "accepte") {
                        Ok(()) => {
                            state.partages[idx].status = Some("accepte".to_string());
                            state.status_message = Some("Partage accepté.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur acceptation : {}", e));
                        }
                    }
                }
            }

            // Gestion refuse.
            if let Some(idx) = refuse_idx {
                if let Some(ref pid) = state.partages[idx].id {
                    match db.partage_update_status(pid, "refuse") {
                        Ok(()) => {
                            state.partages[idx].status = Some("refuse".to_string());
                            state.status_message = Some("Partage refusé.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur refus : {}", e));
                        }
                    }
                }
            }

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Partages actifs (status = "accepte") ---
            ui.label(
                egui::RichText::new("Partages actifs")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            let actifs_indices: Vec<usize> = state
                .partages
                .iter()
                .enumerate()
                .filter(|(_, p)| p.status.as_deref() == Some("accepte"))
                .map(|(i, _)| i)
                .collect();

            if actifs_indices.is_empty() {
                ui.label(
                    egui::RichText::new("Aucun partage actif.")
                        .color(theme.text_secondary()),
                );
            }

            let mut revoke_idx: Option<usize> = None;

            for &idx in &actifs_indices {
                let partage = &state.partages[idx];
                let target = partage.target_user_id.as_deref().unwrap_or("—");
                let context = partage.target_context_type.as_deref().unwrap_or("—");

                egui::Frame::new()
                    .fill(theme.card_background())
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(format!("Partagé avec : {}", target))
                                    .color(theme.text_primary()),
                            );
                            ui.label(
                                egui::RichText::new(format!("Contexte : {}", context))
                                    .color(theme.text_secondary()),
                            );
                            if ui.button("Révoquer").clicked() {
                                revoke_idx = Some(idx);
                            }
                        });
                    });
                ui.add_space(theme.item_spacing());
            }

            // Gestion revoke.
            if let Some(idx) = revoke_idx {
                if let Some(ref pid) = state.partages[idx].id {
                    match db.partage_update_status(pid, "revoque") {
                        Ok(()) => {
                            state.partages[idx].status = Some("revoque".to_string());
                            state.status_message = Some("Partage révoqué.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur révocation : {}", e));
                        }
                    }
                }
            }

            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Historique partages passés ---
            ui.label(
                egui::RichText::new("Historique")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            let past_statuses = ["refuse", "revoque", "expire"];
            let historique: Vec<&crate::data::DocumentPartage> = state
                .partages
                .iter()
                .filter(|p| {
                    p.status
                        .as_deref()
                        .map(|s| past_statuses.contains(&s))
                        .unwrap_or(false)
                })
                .collect();

            if historique.is_empty() {
                ui.label(
                    egui::RichText::new("Aucun historique.")
                        .color(theme.text_secondary()),
                );
            } else {
                for partage in &historique {
                    let target = partage.target_user_id.as_deref().unwrap_or("—");
                    let status = partage.status.as_deref().unwrap_or("—");
                    let responded = partage.responded_at.as_deref().unwrap_or("—");

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(format!("{} — {}", target, status))
                                .color(theme.text_secondary()),
                        );
                        ui.label(
                            egui::RichText::new(format!("le {}", responded))
                                .color(theme.text_secondary()),
                        );
                    });
                }
            }

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(egui::RichText::new(msg).color(theme.accent()));
            }
        });
    });
}
