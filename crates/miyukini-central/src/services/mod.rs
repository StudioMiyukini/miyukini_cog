//! Services factices (démonstration) — UIs minimales pour le MVP du Hub.
//!
//! En production, chaque Service serait porté par un Opérateur distinct ; ici on simule
//! avec des vues intégrées au Hub.

mod calculator;
mod game;
mod text_editor;
mod notes;

pub use calculator::CalculatorService;
pub use game::GameService;
pub use text_editor::TextEditorService;
pub use notes::NotesService;

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
}
