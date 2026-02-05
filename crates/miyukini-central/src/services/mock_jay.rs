//! Service UI mock pour les services Jay (placeholder au lancement depuis le Central).
//!
//! En production, chaque service Jay serait un Opérateur distinct ; ici on affiche
//! une écran de démo à venir.

use crate::catalog::ServiceId;
use eframe::egui;

/// Service UI mock pour tout service Jay : affiche un placeholder "Démo à venir".
pub struct MockJayService {
    service_id: ServiceId,
    title: &'static str,
}

impl MockJayService {
    /// Crée un service Jay mock avec l’ID et le titre affiché dans l’onglet.
    pub fn new(service_id: ServiceId, title: &'static str) -> Self {
        Self { service_id, title }
    }
}

impl crate::services::ServiceUi for MockJayService {
    fn id(&self) -> ServiceId {
        self.service_id
    }
    fn title(&self) -> &'static str {
        self.title
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(60.0);
            ui.heading(
                egui::RichText::new(self.title).size(22.0).strong(),
            );
            ui.add_space(16.0);
            ui.label(
                egui::RichText::new("Service Jay — démo à venir")
                    .size(14.0)
                    .weak(),
            );
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Ce service sera disponible dans une prochaine version.")
                    .size(12.0)
                    .weak(),
            );
        });
    }
}
