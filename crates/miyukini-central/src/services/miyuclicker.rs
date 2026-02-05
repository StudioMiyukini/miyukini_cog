//! Service Lord of the Click — intégré dans le body de Miyukini Central.
//!
//! Point d'accès utilisateur unique : Miyukini Central (CANON-CENTRAL).
//! Le jeu s'exécute dans le body de Central via `show_into`.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use miyuclicker::MiyuClickerApp;

/// Service Lord of the Click : wrapper autour de MiyuClickerApp pour exécution dans le body de Central.
pub struct MiyuClickerService {
    app: MiyuClickerApp,
}

impl MiyuClickerService {
    /// Crée une instance (utilise MiyuClickerApp::new_embedded pour intégration Central).
    pub fn new() -> Self {
        Self {
            app: MiyuClickerApp::new_embedded(),
        }
    }
}

impl Default for MiyuClickerService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUi for MiyuClickerService {
    fn id(&self) -> ServiceId {
        ServiceId::MiyuClicker
    }
    fn title(&self) -> &'static str {
        "Lord of the Click"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_into(ui);
    }
}
