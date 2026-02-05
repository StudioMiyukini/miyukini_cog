//! Service Lord of the Castle — intégré dans le body de Miyukini Central.
//!
//! Point d'accès utilisateur unique : Miyukini Central (CANON-CENTRAL).
//! Le jeu s'exécute dans le body de Central via `show_into`.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use lord_of_the_castle::LordOfTheCastleApp;

/// Service Lord of the Castle (Miyukini Survivor) : wrapper autour de LordOfTheCastleApp pour exécution dans le body de Central.
pub struct LordOfTheCastleService {
    app: LordOfTheCastleApp,
}

impl LordOfTheCastleService {
    /// Crée une instance (utilise LordOfTheCastleApp::new_embedded pour intégration Central).
    pub fn new() -> Self {
        Self {
            app: LordOfTheCastleApp::new_embedded(),
        }
    }
}

impl Default for LordOfTheCastleService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceUi for LordOfTheCastleService {
    fn id(&self) -> ServiceId {
        ServiceId::LordOfTheCastle
    }
    fn title(&self) -> &'static str {
        "Lord of the Castle"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        self.app.show_into(ui);
    }
}
