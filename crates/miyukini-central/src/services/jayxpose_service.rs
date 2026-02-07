//! Service JayXpose (version prod) — intégration du vrai JayXpose dans le Central.
//!
//! Remplace le mock : affiche l'application JayXpose complète (écrans EXP, PUB)
//! dans l'onglet du Central.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use jayxpose::app::JayXposeApp;

/// Service UI JayXpose (version production) : rendu embarqué de l'app JayXpose.
pub struct JayXposeService {
    app: JayXposeApp,
}

impl Default for JayXposeService {
    fn default() -> Self {
        Self {
            app: JayXposeApp::new_embedded(),
        }
    }
}

impl ServiceUi for JayXposeService {
    fn id(&self) -> ServiceId {
        ServiceId::JayXpose
    }
    fn title(&self) -> &'static str {
        "JayXpose"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_in_ui(ui.ctx());
    }
}
