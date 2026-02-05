//! Service JayFestival (version prod) — intégration du vrai JayFestival dans le Central.
//!
//! Remplace le mock : affiche l'application JayFestival complète (écrans UNC, ORG, EXP, VIS)
//! dans l'onglet du Central.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use jayfestival::app::JayFestivalApp;

/// Service UI JayFestival (version production) : rendu embarqué de l'app JayFestival.
pub struct JayFestivalService {
    app: JayFestivalApp,
}

impl Default for JayFestivalService {
    fn default() -> Self {
        Self {
            app: JayFestivalApp::new_embedded(),
        }
    }
}

impl ServiceUi for JayFestivalService {
    fn id(&self) -> ServiceId {
        ServiceId::JayFestival
    }
    fn title(&self) -> &'static str {
        "JayFestival"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_in_ui(ui.ctx());
    }
}
