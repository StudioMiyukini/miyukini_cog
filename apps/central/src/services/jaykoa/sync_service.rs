//! Service de synchronisation JayKoa ↔ JayFestival.
//!
//! Lit les éditions JayFestival et crée des reflets en lecture seule dans JayKoa.

use jaykoa::data::{Agenda, TemporalEntry, EntryType, EventSource, TemporalStatus, JayKoaDb};
use jayfestival::data::JayFestivalDb;
use chrono::Local;
use std::sync::Arc;

/// Résultat de synchronisation.
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// Nombre d'éditions synchronisées.
    pub synced_count: usize,
    /// Erreurs rencontrées.
    pub errors: Vec<String>,
}

/// Synchroniseur JayFestival → JayKoa.
pub struct JayFestivalSync;

impl JayFestivalSync {
    /// Synchronise les éditions JayFestival vers un agenda JayKoa.
    ///
    /// Crée un agenda "JayFestival" s'il n'existe pas, puis insère/met à jour
    /// les reflets des éditions.
    pub fn sync_all(
        koa_db: &Arc<JayKoaDb>,
        festival_db: &Arc<JayFestivalDb>,
        profile_id: &str,
    ) -> SyncResult {
        let mut result = SyncResult {
            synced_count: 0,
            errors: Vec::new(),
        };
        
        // 1. S'assurer que l'agenda JayFestival existe
        let agenda_id = match ensure_jayfestival_agenda(koa_db, profile_id) {
            Ok(id) => id,
            Err(e) => {
                result.errors.push(format!("Impossible de créer l'agenda JayFestival: {}", e));
                return result;
            }
        };
        
        // 2. Charger les éditions JayFestival
        let editions = match festival_db.editions_list() {
            Ok(eds) => eds,
            Err(e) => {
                result.errors.push(format!("Erreur de lecture JayFestival: {}", e));
                return result;
            }
        };
        
        // 3. Créer/mettre à jour les reflets
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        
        for edition in editions {
            let Some(edition_id) = &edition.id else { continue };
            let Some(name) = &edition.name else { continue };
            let Some(start_date) = &edition.start_date else { continue };
            let Some(end_date) = &edition.end_date else { continue };
            
            let entry = TemporalEntry {
                id: Some(uuid::Uuid::new_v4().to_string()),
                agenda_id: Some(agenda_id.clone()),
                title: Some(name.clone()),
                description: Some(format!("Édition JayFestival : {}", name)),
                start_datetime: Some(format!("{}T00:00:00", start_date)),
                end_datetime: Some(format!("{}T23:59:59", end_date)),
                all_day: true,
                location: edition.location.clone(),
                status: Some(match edition.status.as_deref() {
                    Some("termine") | Some("annule") => TemporalStatus::Cancelled.as_str(),
                    _ => TemporalStatus::Confirmed.as_str(),
                }.to_string()),
                entry_type: Some(EntryType::ReflectJayFestival.as_str().to_string()),
                source_service: Some(EventSource::JayFestival.as_str().to_string()),
                source_event_id: Some(edition_id.clone()),
                color: Some(EventSource::JayFestival.default_color().to_string()),
                recurrence_rule: None,
                reminders_json: None,
                created_at: Some(now.clone()),
                updated_at: Some(now.clone()),
                last_synced_at: Some(now.clone()),
            };
            
            if let Err(e) = koa_db.reflect_upsert(&entry) {
                result.errors.push(format!("Erreur sync édition {}: {}", name, e));
            } else {
                result.synced_count += 1;
            }
        }
        
        result
    }
    
    /// Synchronise les participations d'un exposant vers JayKoa.
    pub fn sync_exposant_participations(
        koa_db: &Arc<JayKoaDb>,
        festival_db: &Arc<JayFestivalDb>,
        profile_id: &str,
        _exposant_id: &str,
    ) -> SyncResult {
        let mut result = SyncResult {
            synced_count: 0,
            errors: Vec::new(),
        };
        
        // S'assurer que l'agenda JayFestival existe
        let agenda_id = match ensure_jayfestival_agenda(koa_db, profile_id) {
            Ok(id) => id,
            Err(e) => {
                result.errors.push(format!("Impossible de créer l'agenda JayFestival: {}", e));
                return result;
            }
        };
        
        // Note: Dans une implémentation complète, on lirait les participations
        // de l'exposant via `festival_db.participations_by_exposant(exposant_id)`
        // et on créerait des reflets pour chaque participation confirmée.
        
        // Pour le MVP, on synchronise simplement toutes les éditions
        let editions = match festival_db.editions_list() {
            Ok(eds) => eds,
            Err(e) => {
                result.errors.push(format!("Erreur de lecture JayFestival: {}", e));
                return result;
            }
        };
        
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        
        for edition in editions {
            let Some(edition_id) = &edition.id else { continue };
            let Some(name) = &edition.name else { continue };
            let Some(start_date) = &edition.start_date else { continue };
            let Some(end_date) = &edition.end_date else { continue };
            
            let entry = TemporalEntry {
                id: Some(uuid::Uuid::new_v4().to_string()),
                agenda_id: Some(agenda_id.clone()),
                title: Some(format!("🎪 {}", name)),
                description: Some(format!("Participation à l'édition : {}", name)),
                start_datetime: Some(format!("{}T00:00:00", start_date)),
                end_datetime: Some(format!("{}T23:59:59", end_date)),
                all_day: true,
                location: edition.location.clone(),
                status: Some(TemporalStatus::Confirmed.as_str().to_string()),
                entry_type: Some(EntryType::ReflectJayFestival.as_str().to_string()),
                source_service: Some(EventSource::JayFestival.as_str().to_string()),
                source_event_id: Some(format!("participation_{}", edition_id)),
                color: Some(EventSource::JayFestival.default_color().to_string()),
                recurrence_rule: None,
                reminders_json: None,
                created_at: Some(now.clone()),
                updated_at: Some(now.clone()),
                last_synced_at: Some(now.clone()),
            };
            
            if let Err(e) = koa_db.reflect_upsert(&entry) {
                result.errors.push(format!("Erreur sync participation {}: {}", name, e));
            } else {
                result.synced_count += 1;
            }
        }
        
        result
    }
}

/// S'assure que l'agenda JayFestival existe pour un profil donné.
fn ensure_jayfestival_agenda(koa_db: &Arc<JayKoaDb>, profile_id: &str) -> Result<String, String> {
    let agendas = koa_db.agendas_by_profile(profile_id).map_err(|e| e.to_string())?;
    
    // Chercher un agenda JayFestival existant
    if let Some(existing) = agendas.iter().find(|a| a.source_service.as_deref() == Some("jayfestival")) {
        return existing.id.clone().ok_or_else(|| "Agenda sans ID".to_string());
    }
    
    // Créer l'agenda JayFestival
    let id = uuid::Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    
    let agenda = Agenda {
        id: Some(id.clone()),
        profile_id: Some(profile_id.to_string()),
        name: Some("JayFestival".to_string()),
        description: Some("Événements synchronisés depuis JayFestival".to_string()),
        color: Some(EventSource::JayFestival.default_color().to_string()),
        calendar_type: Some("synced_service".to_string()),
        visible: true,
        is_default: false,
        source_service: Some("jayfestival".to_string()),
        created_at: Some(now.clone()),
        updated_at: Some(now),
    };
    
    koa_db.agenda_insert(&agenda).map_err(|e| e.to_string())?;
    Ok(id)
}
