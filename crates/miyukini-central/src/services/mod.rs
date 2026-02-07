//! Services factices (démonstration) — UIs minimales pour le MVP du Hub.
//!
//! En production, chaque Service serait porté par un Opérateur distinct ; ici on simule
//! avec des vues intégrées au Hub.

mod calculator;
mod dev_center;
mod egui_editor;
mod game;
mod jayfestival_service;
mod jaykoa_service;
mod jayxpose_service;
mod jaykonta_service;
mod lord_of_the_castle;
mod loyalty_service;
mod miyuclicker;
mod mock_jay;
mod notes;
mod text_editor;
mod ui_library;

pub use calculator::CalculatorService;
pub use dev_center::DevCenterService;
pub use egui_editor::{EditorThemeState, EguiEditorService, UI_EDITOR_THEME_STORAGE_KEY};
pub use game::GameService;
pub use jayfestival_service::JayFestivalService;
pub use jaykoa_service::JayKoaService;
pub use jayxpose_service::JayXposeService;
pub use jaykonta_service::JayKontaService;
pub use lord_of_the_castle::LordOfTheCastleService;
pub use loyalty_service::LoyaltyService;
pub use miyuclicker::MiyuClickerService;
pub use mock_jay::MockJayService;
pub use notes::NotesService;
pub use text_editor::TextEditorService;
pub use ui_library::UiLibraryService;

use crate::catalog::ServiceId;
use eframe::egui;

/// Trait commun pour afficher l'UI d'un Service (factice).
pub trait ServiceUi {
    /// Identifiant du Service.
    fn id(&self) -> ServiceId;
    /// Titre affiché dans la barre ou le panneau.
    fn title(&self) -> &'static str;
    /// Dessine l'UI du Service dans le panneau central.
    fn show(&mut self, ui: &mut egui::Ui);
    /// Persiste l'état si nécessaire (ex. thème UI Editor). Appelé par le Hub après show().
    fn persist_if_needed(&mut self, _storage: Option<&mut dyn eframe::Storage>) {}

    /// Thème éditable (UI Editor uniquement). Permet au Hub de synchroniser le thème global.
    fn ui_editor_theme_mut(&mut self) -> Option<&mut EditorThemeState> {
        None
    }

    /// Lie la sauvegarde Lord of the Castle au profil Central (auth_db + profile_id). No-op pour les autres services.
    fn set_lotc_save_load(
        &mut self,
        _auth_db: Option<std::sync::Arc<crate::auth::CentralAuthDb>>,
        _profile_id: Option<&str>,
    ) {
    }
}
