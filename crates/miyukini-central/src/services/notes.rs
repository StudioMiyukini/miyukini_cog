//! Service factice : Notes rapides (MVP).

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;

/// Liste de notes (factice).
#[derive(Default)]
pub struct NotesService {
    notes: Vec<String>,
    new_note: String,
}

impl ServiceUi for NotesService {
    fn id(&self) -> ServiceId {
        ServiceId::Notes
    }
    fn title(&self) -> &'static str {
        "Notes"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading("Notes rapides");
            ui.label("Ajoutez des notes ou des tâches.");
            ui.add_space(8.0);

            ui.horizontal(|ui| {
                ui.label("Nouvelle note :");
                ui.text_edit_singleline(&mut self.new_note);
                if ui.button("Ajouter").clicked() {
                    let s = self.new_note.trim().to_string();
                    if !s.is_empty() {
                        self.notes.push(s);
                        self.new_note.clear();
                    }
                }
            });
            ui.add_space(8.0);

            ui.separator();
            let mut to_remove = Vec::new();
            for (i, note) in self.notes.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label("•");
                    ui.label(note);
                    if ui.small_button("Suppr").clicked() {
                        to_remove.push(i);
                    }
                });
            }
            for i in to_remove.into_iter().rev() {
                self.notes.remove(i);
            }
        });
    }
}
