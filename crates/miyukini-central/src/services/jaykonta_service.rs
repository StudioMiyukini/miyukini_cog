//! Service JayKonta (version prod) - integration du vrai JayKonta dans le Central.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use jaykonta::app::JayKontaApp;

/// Service UI JayKonta (version production) : rendu embarque de l'app JayKonta.
pub struct JayKontaService {
    app: JayKontaApp,
}

impl Default for JayKontaService {
    fn default() -> Self {
        Self {
            app: JayKontaApp::new_embedded(),
        }
    }
}

impl ServiceUi for JayKontaService {
    fn id(&self) -> ServiceId {
        ServiceId::JayKonta
    }
    fn title(&self) -> &'static str {
        "JayKonta"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_in_ui(ui.ctx());
    }
}
