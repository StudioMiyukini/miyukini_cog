//! XP-E10 — Upload / remplacement de document.
//!
//! Formulaire d'upload d'un document professionnel : type, libellé, URL fichier, expiration, notes.

use crate::data::{DocType, JayXposeDb};
use crate::screens::exp::ExpState;
use crate::screens::ScreenId;
use crate::theme::JayXposeTheme;
use eframe::egui;
use std::cell::RefCell;
use std::sync::Arc;

/// @id: screen_exp_e10
/// @do: render_document_upload_form
/// @layer: app
/// Écran upload document : type, libellé, URL fichier, date expiration, notes, bouton enregistrer.
/// Affiche l'écran XP-E10 — Upload document.
pub fn exp_e10_show(
    ctx: &egui::Context,
    theme: &JayXposeTheme,
    nav_request: &RefCell<Option<ScreenId>>,
    state: &mut ExpState,
    db: &Arc<JayXposeDb>,
) {
    let is_replace = state.editing_document_id.is_some();
    let title = if is_replace {
        "Remplacer un document"
    } else {
        "Ajouter un document"
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // --- En-tête ---
            ui.horizontal(|ui| {
                if ui.button("← Documents").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDocuments));
                }
                ui.heading(title);
            });
            ui.add_space(theme.section_spacing());

            let form = &mut state.document_form;

            // Type de document (ComboBox).
            ui.label("Type de document");
            let current_type = DocType::from_str(
                form.doc_type.as_deref().unwrap_or("autre"),
            );
            egui::ComboBox::from_label("Type")
                .selected_text(current_type.as_str())
                .show_ui(ui, |ui| {
                    let all_types = [
                        DocType::Rib,
                        DocType::Assurance,
                        DocType::Kbis,
                        DocType::Immatriculation,
                        DocType::Licence,
                        DocType::Urssaf,
                        DocType::CartePro,
                        DocType::Diplome,
                        DocType::Sanitaire,
                        DocType::Autre,
                    ];
                    for dt in &all_types {
                        let selected = current_type == *dt;
                        if ui
                            .selectable_label(selected, dt.as_str())
                            .clicked()
                        {
                            form.doc_type = Some(dt.as_str().to_string());
                        }
                    }
                });
            ui.add_space(theme.item_spacing());

            // Libellé.
            ui.label("Libellé");
            let mut label_val = form.label.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut label_val).changed() {
                form.label = Some(label_val);
            }
            ui.add_space(theme.item_spacing());

            // URL fichier (texte pour l'instant).
            ui.label("URL du fichier");
            let mut file_url = form.file_url.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut file_url).changed() {
                form.file_url = Some(file_url);
            }
            ui.add_space(theme.item_spacing());

            // Date d'expiration.
            ui.label("Date d'expiration (YYYY-MM-DD, optionnel)");
            let mut expires = form.expires_at.clone().unwrap_or_default();
            if ui.text_edit_singleline(&mut expires).changed() {
                if expires.trim().is_empty() {
                    form.expires_at = None;
                } else {
                    form.expires_at = Some(expires);
                }
            }
            ui.add_space(theme.item_spacing());

            // Notes.
            ui.label("Notes");
            let mut notes = form.notes.clone().unwrap_or_default();
            if ui.text_edit_multiline(&mut notes).changed() {
                form.notes = Some(notes);
            }
            ui.add_space(theme.section_spacing());

            // --- Boutons ---
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    let result = if is_replace {
                        // Incrémenter la version.
                        form.version = Some(form.version.unwrap_or(1) + 1);
                        db.document_update(form)
                    } else {
                        form.version = Some(1);
                        form.status = Some("en_attente".to_string());
                        db.document_insert(form)
                    };
                    match result {
                        Ok(()) => {
                            state.status_message =
                                Some("Document enregistré avec succès.".to_string());
                            // Recharger la liste.
                            if let Some(ref eid) = state.current_exposant_id {
                                if let Ok(docs) = db.documents_by_exposant(eid) {
                                    state.documents = docs;
                                }
                            }
                        }
                        Err(e) => {
                            state.status_message =
                                Some(format!("Erreur enregistrement : {}", e));
                        }
                    }
                }
                if ui.button("Annuler").clicked() {
                    nav_request.replace(Some(ScreenId::ExpDocuments));
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
