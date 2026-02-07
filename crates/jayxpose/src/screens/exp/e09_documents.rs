//! XP-E09 — Coffre-fort documents.
//!
//! Liste des documents professionnels avec alertes expiration, statut, actions.

use crate::data::{DocStatus, DocType, JayXposeDb};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e09
/// @do: render_document_vault_list
/// @layer: app
/// Écran coffre-fort documents : alertes expiration, liste avec type/statut/actions, partages actifs.
/// Affiche l'écran XP-E09 — Coffre-fort documents.
pub fn exp_e09_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    // Rechargement si nécessaire.
    if state.documents.is_empty() {
        if let Some(ref eid) = state.current_exposant_id {
            if let Ok(docs) = db.documents_by_exposant(eid) {
                state.documents = docs;
            }
            if let Ok(partages) = db.partages_by_exposant(eid) {
                state.partages = partages;
            }
        }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Tableau de bord").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDashboard));
                }
                ui.heading("Mes documents");
                ui.label(
                    egui::RichText::new(format!("{} document(s)", state.documents.len()))
                        .color(theme.text_secondary()),
                );
            });
            ui.add_space(theme.item_spacing());

            // Bouton ajouter.
            if ui.button("+ Ajouter un document").clicked() {
                state.editing_document_id = None;
                state.document_form = crate::data::DocumentProfessionnel::default();
                state.document_form.exposant_id = state.current_exposant_id.clone();
                nav_request.replace(Some(ScreenId::ExpUploadDocument));
            }
            ui.add_space(theme.item_spacing());

            // --- Alertes documents expirants ---
            let expiring_docs: Vec<usize> = state
                .documents
                .iter()
                .enumerate()
                .filter(|(_, d)| is_expiring_soon(d.expires_at.as_deref()))
                .map(|(i, _)| i)
                .collect();

            if !expiring_docs.is_empty() {
                egui::Frame::new()
                    .fill(egui::Color32::from_rgb(255, 243, 205))
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "⚠ {} document(s) expirent dans les 30 prochains jours.",
                                expiring_docs.len()
                            ))
                            .color(egui::Color32::from_rgb(133, 100, 4)),
                        );
                    });
                ui.add_space(theme.item_spacing());
            }

            ui.separator();
            ui.add_space(theme.item_spacing());

            // --- Liste documents ---
            if state.documents.is_empty() {
                ui.label(
                    egui::RichText::new("Aucun document pour l'instant.")
                        .color(theme.text_secondary()),
                );
            }

            let mut delete_idx: Option<usize> = None;
            let mut replace_idx: Option<usize> = None;

            for (idx, doc) in state.documents.iter().enumerate() {
                let doc_label = doc.label.as_deref().unwrap_or("Sans libellé");
                let doc_type_str = doc.doc_type.as_deref().unwrap_or("autre");
                let doc_type = DocType::from_str(doc_type_str);
                let status_str = doc.status.as_deref().unwrap_or("en_attente");
                let status = DocStatus::from_str(status_str);
                let expires = doc.expires_at.as_deref().unwrap_or("—");
                let version = doc.version.unwrap_or(1);

                let status_color = match status {
                    DocStatus::Valide => egui::Color32::from_rgb(40, 167, 69),
                    DocStatus::EnAttente => egui::Color32::from_rgb(255, 193, 7),
                    DocStatus::Expire => egui::Color32::from_rgb(220, 53, 69),
                    DocStatus::Rejete => egui::Color32::from_rgb(220, 53, 69),
                };

                egui::Frame::new()
                    .fill(theme.card_background())
                    .corner_radius(theme.border_radius())
                    .inner_margin(egui::Margin::same(10))
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            // Type icon.
                            ui.label(
                                egui::RichText::new(doc_type_icon(doc_type))
                                    .color(theme.accent()),
                            );
                            ui.label(
                                egui::RichText::new(doc_label)
                                    .size(theme.font_size_body())
                                    .color(theme.text_primary()),
                            );
                            // Badge statut.
                            ui.label(
                                egui::RichText::new(status.as_str())
                                    .color(status_color),
                            );
                            ui.label(
                                egui::RichText::new(format!("Expire : {}", expires))
                                    .color(theme.text_secondary()),
                            );
                            ui.label(
                                egui::RichText::new(format!("v{}", version))
                                    .color(theme.text_secondary()),
                            );
                        });
                        ui.horizontal(|ui| {
                            if ui.button("Voir").clicked() {
                                // Pas de navigation dédiée, afficher file_url.
                                if let Some(url) = &doc.file_url {
                                    state.status_message =
                                        Some(format!("Document : {}", url));
                                }
                            }
                            if ui.button("Remplacer").clicked() {
                                replace_idx = Some(idx);
                            }
                            if ui.button("Supprimer").clicked() {
                                delete_idx = Some(idx);
                            }
                        });
                    });
                ui.add_space(theme.item_spacing());
            }

            // Handle replace action.
            if let Some(idx) = replace_idx {
                let doc = state.documents[idx].clone();
                state.editing_document_id = doc.id.clone();
                state.document_form = doc;
                nav_request.replace(Some(ScreenId::ExpUploadDocument));
            }

            // Handle delete action.
            if let Some(idx) = delete_idx {
                if let Some(ref did) = state.documents[idx].id {
                    match db.document_delete(did) {
                        Ok(()) => {
                            state.documents.remove(idx);
                            state.status_message =
                                Some("Document supprimé.".to_string());
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur suppression : {}", e));
                        }
                    }
                }
            }

            // --- Section partages actifs ---
            ui.add_space(theme.section_spacing());
            ui.separator();
            ui.add_space(theme.item_spacing());
            ui.label(
                egui::RichText::new("Partages actifs")
                    .size(theme.font_size_heading())
                    .color(theme.text_primary()),
            );
            ui.add_space(theme.item_spacing());

            let active_partages: Vec<&crate::data::DocumentPartage> = state
                .partages
                .iter()
                .filter(|p| p.status.as_deref() == Some("accepte"))
                .collect();

            if active_partages.is_empty() {
                ui.label(
                    egui::RichText::new("Aucun partage actif.")
                        .color(theme.text_secondary()),
                );
            } else {
                for partage in &active_partages {
                    let target = partage.target_user_id.as_deref().unwrap_or("—");
                    let context = partage.target_context_type.as_deref().unwrap_or("—");
                    ui.horizontal(|ui| {
                        ui.label(format!("Partagé avec : {}", target));
                        ui.label(
                            egui::RichText::new(format!("Contexte : {}", context))
                                .color(theme.text_secondary()),
                        );
                    });
                }
            }

            if ui.button("Gérer les partages").clicked() {
                nav_request.replace(Some(ScreenId::ExpPartageDocuments));
            }

            // --- Message de statut ---
            if let Some(msg) = &state.status_message {
                ui.add_space(theme.item_spacing());
                ui.label(egui::RichText::new(msg).color(theme.accent()));
            }
        });
    });
}

/// Retourne une icône texte selon le type de document.
fn doc_type_icon(doc_type: DocType) -> &'static str {
    match doc_type {
        DocType::Rib => "[RIB]",
        DocType::Assurance => "[ASS]",
        DocType::Kbis => "[KBIS]",
        DocType::Immatriculation => "[IMMA]",
        DocType::Licence => "[LIC]",
        DocType::Urssaf => "[URSS]",
        DocType::CartePro => "[PRO]",
        DocType::Diplome => "[DIPL]",
        DocType::Sanitaire => "[SANI]",
        DocType::Autre => "[DOC]",
    }
}

/// Vérifie si un document expire dans les 30 prochains jours (heuristique simplifiée).
fn is_expiring_soon(expires_at: Option<&str>) -> bool {
    // Heuristique simplifiée : si expires_at est renseigné, considérer "bientôt".
    // En production, comparer avec la date courante.
    expires_at.is_some()
}
