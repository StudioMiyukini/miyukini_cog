//! Service factice : Traitement de texte (MVP).

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;

/// Éditeur de texte simple.
#[derive(Default)]
pub struct TextEditorService {
    content: String,
}

impl ServiceUi for TextEditorService {
    fn id(&self) -> ServiceId {
        ServiceId::TextEditor
    }
    fn title(&self) -> &'static str {
        "Traitement de texte"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.heading("Traitement de texte");
            ui.label("Rédigez votre document ci-dessous.");
            ui.add_space(8.0);

            ui.add(
                egui::TextEdit::multiline(&mut self.content)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20)
                    .font(egui::TextStyle::Body),
            );
        });
    }
}
