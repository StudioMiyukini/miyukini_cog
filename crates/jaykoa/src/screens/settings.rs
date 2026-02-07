//! Écran de paramètres utilisateur JayKoa.
//!
//! @id: jaykoa_screen_settings
//! @do: render_user_settings_screen
//! @role: ui
//! @layer: app

use crate::data::types::UserSettings;
use crate::data::JayKoaDb;
use crate::theme::JayKoaTheme;
use egui::Ui;
use std::sync::Arc;

/// Affiche l'écran de paramètres.
pub fn show_settings(ui: &mut Ui, settings: &mut UserSettings, db: &Arc<JayKoaDb>, _theme: &JayKoaTheme) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.add_space(16.0);
        ui.heading(egui::RichText::new("Paramètres JayKoa").size(20.0).strong());
        ui.add_space(16.0);
        let mut changed = false;

        ui.label(egui::RichText::new("Vue par défaut").size(13.0).strong());
        let views = [("week","Semaine"),("day","Jour"),("month","Mois"),("year","Année"),("schedule","Planning")];
        egui::ComboBox::from_id_salt("dv").selected_text(views.iter().find(|(k,_)| *k==settings.default_view).map(|(_,v)| *v).unwrap_or("Semaine"))
            .show_ui(ui, |ui| { for (k, l) in &views { if ui.selectable_value(&mut settings.default_view, k.to_string(), *l).changed() { changed = true; } } });
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Premier jour de la semaine").size(13.0).strong());
        let days = [(0u8,"Lundi"),(6,"Dimanche"),(5,"Samedi")];
        egui::ComboBox::from_id_salt("ws").selected_text(days.iter().find(|(k,_)| *k==settings.week_start_day).map(|(_,v)| *v).unwrap_or("Lundi"))
            .show_ui(ui, |ui| { for (k, l) in &days { if ui.selectable_value(&mut settings.week_start_day, *k, *l).changed() { changed = true; } } });
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Plage horaire").size(13.0).strong());
        ui.horizontal(|ui| { ui.label("De :"); let mut s = settings.day_start_hour as i32;
            if ui.add(egui::Slider::new(&mut s, 0..=23).suffix("h")).changed() { settings.day_start_hour = s as u8; changed = true; } });
        ui.horizontal(|ui| { ui.label("À :"); let mut e = settings.day_end_hour as i32;
            if ui.add(egui::Slider::new(&mut e, 0..=23).suffix("h")).changed() { settings.day_end_hour = e as u8; changed = true; } });
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Fuseau horaire").size(13.0).strong());
        ui.text_edit_singleline(&mut settings.timezone); ui.add_space(12.0);

        if ui.checkbox(&mut settings.use_24h_format, "Format 24h").changed() { changed = true; }
        if ui.checkbox(&mut settings.show_week_numbers, "Numéros de semaine").changed() { changed = true; }
        if ui.checkbox(&mut settings.show_cancelled_events, "Événements annulés").changed() { changed = true; }
        ui.add_space(12.0);

        ui.label(egui::RichText::new("Rappel par défaut").size(13.0).strong());
        let reminders = [(0,"Aucun"),(5,"5 min"),(10,"10 min"),(15,"15 min"),(30,"30 min"),(60,"1h"),(1440,"1 jour")];
        egui::ComboBox::from_id_salt("dr").selected_text(reminders.iter().find(|(k,_)| *k==settings.default_reminder_minutes).map(|(_,v)| *v).unwrap_or("30 min"))
            .show_ui(ui, |ui| { for (k, l) in &reminders { if ui.selectable_value(&mut settings.default_reminder_minutes, *k, *l).changed() { changed = true; } } });
        ui.add_space(20.0);

        if changed { let _ = db.settings_upsert(settings); }
        if ui.button("Enregistrer").clicked() { let _ = db.settings_upsert(settings); }
    });
}
