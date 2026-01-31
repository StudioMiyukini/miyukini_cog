//! Service factice : Jeu de démonstration (MVP) — clics rapides.

use crate::catalog::ServiceId;
use crate::services::ServiceUi;
use eframe::egui;
use std::time::Instant;

/// Jeu simple : cliquer le plus vite possible sur la cible.
#[derive(Default)]
pub struct GameService {
    score: u32,
    last_click: Option<Instant>,
    message: String,
}

impl ServiceUi for GameService {
    fn id(&self) -> ServiceId {
        ServiceId::Game
    }
    fn title(&self) -> &'static str {
        "Jeu"
    }
    fn show(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(8.0);
            ui.heading("Jeu de démonstration");
            ui.label("Cliquez sur le bouton le plus vite possible !");
            ui.add_space(16.0);

            ui.horizontal(|ui| {
                ui.label("Score :");
                ui.monospace(format!("{}", self.score));
            });

            if !self.message.is_empty() {
                ui.label(egui::RichText::new(&self.message).color(ui.visuals().warn_fg_color));
            }
            ui.add_space(8.0);

            let btn = egui::Button::new("🎯 Cliquez ici !").min_size(egui::vec2(180.0, 60.0));
            if ui.add(btn).clicked() {
                self.score += 1;
                self.last_click = Some(Instant::now());
                self.message = "Bien vu !".to_string();
            }

            ui.add_space(16.0);
            if ui.button("Réinitialiser le score").clicked() {
                self.score = 0;
                self.message.clear();
            }
        });
    }
}
