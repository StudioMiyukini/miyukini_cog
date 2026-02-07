//! Formulaire de création / édition d'événement JayKoa.
//!
//! @id: jaykoa_screen_event_create
//! @do: render_event_creation_and_edition_form
//! @role: ui
//! @layer: app
//! INVARIANTS : JayKoa ne crée jamais d'événement externe. Toute écriture via KindMother.

use crate::app_state::AppState;
use crate::data::types::{EntryType, TemporalEntry};
use crate::data::JayKoaDb;
use crate::theme::JayKoaTheme;
use chrono::{Local, Timelike};
use egui::Ui;
use std::sync::Arc;

/// État du formulaire de création/édition.
pub struct EventFormState {
    /// Titre.
    pub title: String,
    /// Description.
    pub description: String,
    /// Date de début (YYYY-MM-DD).
    pub start_date: String,
    /// Heure de début (HH:MM).
    pub start_time: String,
    /// Date de fin (YYYY-MM-DD).
    pub end_date: String,
    /// Heure de fin (HH:MM).
    pub end_time: String,
    /// Toute la journée.
    pub all_day: bool,
    /// Lieu.
    pub location: String,
    /// ID de l'agenda cible.
    pub agenda_id: String,
    /// Statut.
    pub status: String,
    /// Couleur personnalisée (hex).
    pub color: String,
    /// Message d'erreur.
    pub error: Option<String>,
    /// Avertissement de conflit temporel (AGD-SEC-6 : notifier sans bloquer).
    pub conflict_warning: Option<String>,
    /// Succès (pour fermeture auto).
    pub saved: bool,
}

impl EventFormState {
    /// Nouveau formulaire vide (création).
    pub fn new_empty(default_agenda_id: &str) -> Self {
        let now = Local::now();
        let eh: u32 = now.hour() + 1;
        Self {
            title: String::new(), description: String::new(),
            start_date: now.format("%Y-%m-%d").to_string(),
            start_time: format!("{:02}:00", now.hour()),
            end_date: now.format("%Y-%m-%d").to_string(),
            end_time: format!("{:02}:00", eh.min(23)),
            all_day: false, location: String::new(),
            agenda_id: default_agenda_id.to_string(),
            status: "confirmed".to_string(), color: String::new(),
            error: None, conflict_warning: None, saved: false,
        }
    }

    /// Pré-rempli depuis une entrée existante (édition).
    pub fn from_entry(entry: &TemporalEntry) -> Self {
        let (sd, st) = split_dt(entry.start_datetime.as_deref().unwrap_or(""));
        let (ed, et) = split_dt(entry.end_datetime.as_deref().unwrap_or(""));
        Self {
            title: entry.title.clone().unwrap_or_default(),
            description: entry.description.clone().unwrap_or_default(),
            start_date: sd, start_time: st, end_date: ed, end_time: et,
            all_day: entry.all_day,
            location: entry.location.clone().unwrap_or_default(),
            agenda_id: entry.agenda_id.clone().unwrap_or_default(),
            status: entry.status.clone().unwrap_or_else(|| "confirmed".to_string()),
            color: entry.color.clone().unwrap_or_default(),
            error: None, conflict_warning: None, saved: false,
        }
    }
}

/// Affiche le popover de création rapide.
pub fn show_quick_add(ui: &mut Ui, state: &mut AppState, form: &mut EventFormState, db: &Arc<JayKoaDb>, _theme: &JayKoaTheme) {
    egui::Window::new("Nouvel événement").collapsible(false).resizable(false).default_width(340.0).show(ui.ctx(), |ui| {
        ui.add_space(4.0);
        ui.horizontal(|ui| { ui.label("Titre :"); ui.text_edit_singleline(&mut form.title); });
        ui.horizontal(|ui| {
            ui.label("Début :"); ui.add(egui::TextEdit::singleline(&mut form.start_date).desired_width(90.0));
            if !form.all_day { ui.add(egui::TextEdit::singleline(&mut form.start_time).desired_width(50.0)); }
        });
        ui.horizontal(|ui| {
            ui.label("Fin :"); ui.add_space(13.0); ui.add(egui::TextEdit::singleline(&mut form.end_date).desired_width(90.0));
            if !form.all_day { ui.add(egui::TextEdit::singleline(&mut form.end_time).desired_width(50.0)); }
        });
        ui.checkbox(&mut form.all_day, "Toute la journée");
        ui.add_space(8.0);
        if let Some(warn) = &form.conflict_warning { ui.colored_label(egui::Color32::from_rgb(234, 67, 53), warn); }
        if let Some(err) = &form.error { ui.colored_label(egui::Color32::RED, err); }
        ui.horizontal(|ui| {
            if ui.button("Plus d'options").clicked() { state.show_quick_add = false; state.show_full_editor = true; }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Enregistrer").clicked() {
                    // Vérification de conflit (AGD-SEC-6 : notifier sans bloquer)
                    check_and_warn_conflict(form, db, &state.profile_id);
                    match save_entry(form, db, None) { Ok(_) => { form.saved = true; state.show_quick_add = false; } Err(e) => { form.error = Some(e); } }
                }
                if ui.button("Annuler").clicked() { state.show_quick_add = false; }
            });
        });
    });
}

/// Affiche le formulaire complet de création/édition.
pub fn show_full_editor(ui: &mut Ui, state: &mut AppState, form: &mut EventFormState, agendas: &[(String, String)], db: &Arc<JayKoaDb>, _theme: &JayKoaTheme) {
    let title = if state.editing_entry_id.is_some() { "Modifier l'événement" } else { "Nouvel événement" };
    egui::Window::new(title).collapsible(false).resizable(true).default_width(450.0).show(ui.ctx(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.label(egui::RichText::new("Titre").size(12.0).strong()); ui.text_edit_singleline(&mut form.title); ui.add_space(8.0);
            ui.label(egui::RichText::new("Début").size(12.0).strong());
            ui.horizontal(|ui| { ui.add(egui::TextEdit::singleline(&mut form.start_date).desired_width(100.0));
                if !form.all_day { ui.add(egui::TextEdit::singleline(&mut form.start_time).desired_width(60.0)); } });
            ui.label(egui::RichText::new("Fin").size(12.0).strong());
            ui.horizontal(|ui| { ui.add(egui::TextEdit::singleline(&mut form.end_date).desired_width(100.0));
                if !form.all_day { ui.add(egui::TextEdit::singleline(&mut form.end_time).desired_width(60.0)); } });
            ui.checkbox(&mut form.all_day, "Toute la journée"); ui.add_space(8.0);
            ui.label(egui::RichText::new("Lieu").size(12.0).strong()); ui.text_edit_singleline(&mut form.location); ui.add_space(8.0);
            ui.label(egui::RichText::new("Description").size(12.0).strong());
            ui.add(egui::TextEdit::multiline(&mut form.description).desired_rows(3).desired_width(f32::INFINITY)); ui.add_space(8.0);
            ui.label(egui::RichText::new("Calendrier").size(12.0).strong());
            egui::ComboBox::from_id_salt("agenda_sel").selected_text(agendas.iter().find(|(id,_)| *id == form.agenda_id).map(|(_,n)| n.as_str()).unwrap_or("Sélectionner"))
                .show_ui(ui, |ui| { for (id, name) in agendas { ui.selectable_value(&mut form.agenda_id, id.clone(), name); } });
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Statut").size(12.0).strong());
            egui::ComboBox::from_id_salt("status_sel").selected_text(status_label(&form.status))
                .show_ui(ui, |ui| { ui.selectable_value(&mut form.status, "confirmed".to_string(), "Confirmé");
                    ui.selectable_value(&mut form.status, "tentative".to_string(), "Provisoire");
                    ui.selectable_value(&mut form.status, "cancelled".to_string(), "Annulé"); });
            ui.add_space(12.0);
            if let Some(warn) = &form.conflict_warning { ui.colored_label(egui::Color32::from_rgb(234, 67, 53), warn); }
            if let Some(err) = &form.error { ui.colored_label(egui::Color32::RED, err); }
            ui.horizontal(|ui| {
                if ui.button("Enregistrer").clicked() {
                    // Vérification de conflit (AGD-SEC-6 : notifier sans bloquer)
                    check_and_warn_conflict(form, db, &state.profile_id);
                    match save_entry(form, db, state.editing_entry_id.clone()) { Ok(_) => { form.saved = true; state.show_full_editor = false; state.editing_entry_id = None; } Err(e) => { form.error = Some(e); } }
                }
                if ui.button("Annuler").clicked() { state.show_full_editor = false; state.editing_entry_id = None; }
                if state.editing_entry_id.is_some() {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button(egui::RichText::new("Supprimer").color(egui::Color32::from_rgb(234,67,53))).clicked() {
                            if let Some(eid) = &state.editing_entry_id { let _ = db.entry_delete(eid); state.show_full_editor = false; state.editing_entry_id = None; form.saved = true; }
                        }
                    });
                }
            });
        });
    });
}

fn save_entry(form: &EventFormState, db: &Arc<JayKoaDb>, existing_id: Option<String>) -> Result<(), String> {
    if form.title.trim().is_empty() { return Err("Le titre est obligatoire.".to_string()); }
    if form.start_date.is_empty() { return Err("La date de début est obligatoire.".to_string()); }
    let sdt = if form.all_day { format!("{}T00:00:00", form.start_date) } else { format!("{}T{}:00", form.start_date, form.start_time) };
    let edt = if form.all_day { format!("{}T23:59:59", form.end_date) } else { format!("{}T{}:00", form.end_date, form.end_time) };
    if edt <= sdt { return Err("La fin doit être après le début.".to_string()); }
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let opt = |s: &str| if s.is_empty() { None } else { Some(s.to_string()) };
    if let Some(eid) = existing_id {
        let e = TemporalEntry { id: Some(eid), title: Some(form.title.clone()), description: opt(&form.description),
            start_datetime: Some(sdt), end_datetime: Some(edt), all_day: form.all_day, location: opt(&form.location),
            status: Some(form.status.clone()), color: opt(&form.color), updated_at: Some(now), ..Default::default() };
        db.entry_update(&e).map_err(|e| format!("Erreur : {}", e))?;
    } else {
        let e = TemporalEntry { id: Some(uuid::Uuid::new_v4().to_string()), agenda_id: Some(form.agenda_id.clone()),
            title: Some(form.title.clone()), description: opt(&form.description),
            start_datetime: Some(sdt), end_datetime: Some(edt), all_day: form.all_day, location: opt(&form.location),
            status: Some(form.status.clone()), entry_type: Some(EntryType::Internal.as_str().to_string()),
            color: opt(&form.color), created_at: Some(now.clone()), updated_at: Some(now), ..Default::default() };
        db.entry_insert(&e).map_err(|e| format!("Erreur : {}", e))?;
    }
    Ok(())
}

/// Vérifie les conflits sur le créneau du formulaire et met à jour le warning.
/// AGD-SEC-6 : notifier sans bloquer — le warning est informatif uniquement.
fn check_and_warn_conflict(form: &mut EventFormState, db: &Arc<JayKoaDb>, profile_id: &str) {
    let sdt = if form.all_day { format!("{}T00:00:00", form.start_date) } else { format!("{}T{}:00", form.start_date, form.start_time) };
    let edt = if form.all_day { format!("{}T23:59:59", form.end_date) } else { format!("{}T{}:00", form.end_date, form.end_time) };
    match db.check_conflict(profile_id, &sdt, &edt) {
        Ok((true, conflicts)) => {
            let descs: Vec<String> = conflicts.iter().filter_map(|c| c.description.clone()).collect();
            let msg = if descs.is_empty() {
                "⚠ Conflit détecté sur ce créneau.".to_string()
            } else {
                format!("⚠ {} conflit(s) : {}", descs.len(), descs.join(" | "))
            };
            form.conflict_warning = Some(msg);
        }
        Ok((false, _)) => { form.conflict_warning = None; }
        Err(_) => { form.conflict_warning = None; }
    }
}

fn split_dt(s: &str) -> (String, String) {
    if let Some(idx) = s.find('T') {
        let d = s[..idx].to_string();
        let t = if s.len() > idx+1 { let tp = &s[idx+1..]; if tp.len()>=5 { tp[..5].to_string() } else { tp.to_string() } } else { "00:00".to_string() };
        (d, t)
    } else { (s.to_string(), "00:00".to_string()) }
}
fn status_label(s: &str) -> &str { match s { "confirmed"=>"Confirmé", "tentative"=>"Provisoire", "cancelled"=>"Annulé", _=>"Confirmé" } }
