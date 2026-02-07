//! Service JayKoa (version prod) — intégration du vrai JayKoa dans le Central.
//!
//! @id: miyukini_central_jaykoa_service
//! @do: embed_jaykoa_app_in_central_tab
//! @role: integration
//! @layer: app

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use jaykoa::app::JayKoaApp;

/// Service UI JayKoa (version production) : rendu embarqué de l'app JayKoa.
pub struct JayKoaService {
    app: JayKoaApp,
}

impl JayKoaService {
    /// Crée le service avec un profile_id Central.
    pub fn with_profile(profile_id: Option<String>) -> Self {
        Self {
            app: JayKoaApp::new_embedded_with_profile(profile_id),
        }
    }
}

impl Default for JayKoaService {
    fn default() -> Self {
        Self {
            app: JayKoaApp::new_embedded(),
        }
    }
}

impl ServiceUi for JayKoaService {
    fn id(&self) -> ServiceId {
        ServiceId::JayKoa
    }
    fn title(&self) -> &'static str {
        "JayKoa"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_in_ui(ui.ctx());
    }
}
