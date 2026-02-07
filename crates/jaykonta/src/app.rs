//! Application JayKonta.

use crate::backend::service::JayKontaBackend;
use crate::theme::JayKontaTheme;
use crate::ui::organisms;
use eframe::egui;
use eframe::App;

/// Application principale JayKonta (embarquable dans Miyukini Central).
pub struct JayKontaApp {
    backend: JayKontaBackend,
    theme: JayKontaTheme,
}

impl Default for JayKontaApp {
    fn default() -> Self {
        Self {
            backend: JayKontaBackend::default(),
            theme: JayKontaTheme::default(),
        }
    }
}

impl JayKontaApp {
    /// Cree l'application en mode standalone.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    /// Cree l'application en mode embarque (Central).
    pub fn new_embedded() -> Self {
        Self::default()
    }

    /// Affiche l'UI JayKonta dans un contexte egui.
    pub fn show_in_ui(&mut self, ctx: &egui::Context) {
        self.theme.apply(ctx);
        let view_model = self.backend.view_model();

        egui::SidePanel::left("jaykonta_sidebar")
            .resizable(true)
            .default_width(240.0)
            .show(ctx, |ui| {
                organisms::sidebar(ui, &mut self.backend, &self.theme);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                organisms::hero(ui, &view_model, &self.theme);
                ui.add_space(10.0);
                organisms::kpis(ui, &view_model, &self.theme);
                ui.add_space(10.0);
                organisms::body(ui, self.backend.section(), &view_model, &self.theme);
            });
        });
    }
}

impl App for JayKontaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.show_in_ui(ctx);
    }
}
