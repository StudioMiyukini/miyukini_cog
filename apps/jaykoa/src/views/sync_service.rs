//! Service de synchronisation JayKoa <-> JayFestival.
//!
//! Lit les editions JayFestival et cree des reflets en lecture seule dans JayKoa.
//!
//! NOTE: En mode standalone, la synchronisation JayFestival necessite l'acces
//! a la DB JayFestival. Pour l'instant, les fonctions sont conservees mais
//! la synchronisation n'est pas declenchee depuis l'UI standalone (un warning
//! est affiche a la place). Quand l'architecture inter-services sera finalisee,
//! ce module sera connecte.

#![allow(dead_code)]

use jaykoa::data::{Agenda, TemporalEntry, EntryType, EventSource, TemporalStatus, JayKoaDb};
use jayfestival::data::JayFestivalDb;
use chrono::Local;
use std::sync::Arc;

/// Resultat de synchronisation.
#[derive(Debug, Clone)]
pub struct SyncResult {
    /// Nombre d'editions synchronisees.
    pub synced_count: usize,
    /// Erreurs rencontrees.
    pub errors: Vec<String>,
}

/// Synchroniseur JayFestival -> JayKoa.
pub struct JayFestivalSync;

impl JayFestivalSync {
    /// Synchronise les editions JayFestival vers un agenda JayKoa.
    ///
    /// Cree un agenda "JayFestival" s'il n'existe pas, puis insere/met a jour
    /// les reflets des editions.
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
                result.errors.push(format!("Impossible de creer l'agenda JayFestival: {e}"));
                return result;
            }
        };

        // 2. Charger les editions JayFestival
        let editions = match festival_db.editions_list() {
            Ok(eds) => eds,
            Err(e) => {
                result.errors.push(format!("Erreur de lecture JayFestival: {e}"));
                return result;
            }
        };

        // 3. Creer/mettre a jour les reflets
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
                description: Some(format!("Edition JayFestival : {name}")),
                start_datetime: Some(format!("{start_date}T00:00:00")),
                end_datetime: Some(format!("{end_date}T23:59:59")),
                all_day: true,
                location: edition.location.clone(),
                status: Some(match edition.status.as_deref() {
                    Some("termine" | "annule") => TemporalStatus::Cancelled.as_str(),
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
                result.errors.push(format!("Erreur sync edition {name}: {e}"));
            } else {
                result.synced_count += 1;
            }
        }

        result
    }

    /// Synchronise une seule edition vers l'agenda JayKoa (pour le flux « Ajouter au calendrier »).
    pub fn sync_single_edition(
        koa_db: &Arc<JayKoaDb>,
        festival_db: &Arc<JayFestivalDb>,
        profile_id: &str,
        edition_id: &str,
    ) -> SyncResult {
        let mut result = SyncResult {
            synced_count: 0,
            errors: Vec::new(),
        };
        let agenda_id = match ensure_jayfestival_agenda(koa_db, profile_id) {
            Ok(id) => id,
            Err(e) => {
                result.errors.push(format!("Impossible de creer l'agenda JayFestival: {e}"));
                return result;
            }
        };
        let edition = match festival_db.edition_by_id(edition_id) {
            Ok(Some(e)) => e,
            Ok(None) => {
                result.errors.push(format!("Edition {edition_id} introuvable"));
                return result;
            }
            Err(e) => {
                result.errors.push(format!("Erreur lecture edition: {e}"));
                return result;
            }
        };
        let Some(name) = &edition.name else {
            result.errors.push("Edition sans nom".to_string());
            return result;
        };
        let Some(start_date) = &edition.start_date else {
            result.errors.push(format!("Edition {name} sans date de debut"));
            return result;
        };
        let Some(end_date) = &edition.end_date else {
            result.errors.push(format!("Edition {name} sans date de fin"));
            return result;
        };
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let entry = TemporalEntry {
            id: Some(uuid::Uuid::new_v4().to_string()),
            agenda_id: Some(agenda_id),
            title: Some(name.clone()),
            description: Some(format!("Edition JayFestival : {name}")),
            start_datetime: Some(format!("{start_date}T00:00:00")),
            end_datetime: Some(format!("{end_date}T23:59:59")),
            all_day: true,
            location: edition.location.clone(),
            status: Some(match edition.status.as_deref() {
                Some("termine" | "annule") => TemporalStatus::Cancelled.as_str(),
                _ => TemporalStatus::Confirmed.as_str(),
            }.to_string()),
            entry_type: Some(EntryType::ReflectJayFestival.as_str().to_string()),
            source_service: Some(EventSource::JayFestival.as_str().to_string()),
            source_event_id: Some(edition_id.to_string()),
            color: Some(EventSource::JayFestival.default_color().to_string()),
            recurrence_rule: None,
            reminders_json: None,
            created_at: Some(now.clone()),
            updated_at: Some(now.clone()),
            last_synced_at: Some(now),
        };
        if let Err(e) = koa_db.reflect_upsert(&entry) {
            result.errors.push(format!("Erreur sync edition {name}: {e}"));
        } else {
            result.synced_count = 1;
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
                result.errors.push(format!("Impossible de creer l'agenda JayFestival: {e}"));
                return result;
            }
        };

        // Note: Dans une implementation complete, on lirait les participations
        // de l'exposant via `festival_db.participations_by_exposant(exposant_id)`
        // et on creerait des reflets pour chaque participation confirmee.

        // Pour le MVP, on synchronise simplement toutes les editions
        let editions = match festival_db.editions_list() {
            Ok(eds) => eds,
            Err(e) => {
                result.errors.push(format!("Erreur de lecture JayFestival: {e}"));
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
                title: Some(format!("[F] {name}")),
                description: Some(format!("Participation a l'edition : {name}")),
                start_datetime: Some(format!("{start_date}T00:00:00")),
                end_datetime: Some(format!("{end_date}T23:59:59")),
                all_day: true,
                location: edition.location.clone(),
                status: Some(TemporalStatus::Confirmed.as_str().to_string()),
                entry_type: Some(EntryType::ReflectJayFestival.as_str().to_string()),
                source_service: Some(EventSource::JayFestival.as_str().to_string()),
                source_event_id: Some(format!("participation_{edition_id}")),
                color: Some(EventSource::JayFestival.default_color().to_string()),
                recurrence_rule: None,
                reminders_json: None,
                created_at: Some(now.clone()),
                updated_at: Some(now.clone()),
                last_synced_at: Some(now.clone()),
            };

            if let Err(e) = koa_db.reflect_upsert(&entry) {
                result.errors.push(format!("Erreur sync participation {name}: {e}"));
            } else {
                result.synced_count += 1;
            }
        }

        result
    }
}

/// S'assure que l'agenda JayFestival existe pour un profil donne.
fn ensure_jayfestival_agenda(koa_db: &Arc<JayKoaDb>, profile_id: &str) -> Result<String, String> {
    let agendas = koa_db.agendas_by_profile(profile_id).map_err(|e| e.to_string())?;

    // Chercher un agenda JayFestival existant
    if let Some(existing) = agendas.iter().find(|a| a.source_service.as_deref() == Some("jayfestival")) {
        return existing.id.clone().ok_or_else(|| "Agenda sans ID".to_string());
    }

    // Creer l'agenda JayFestival
    let id = uuid::Uuid::new_v4().to_string();
    let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();

    let agenda = Agenda {
        id: Some(id.clone()),
        profile_id: Some(profile_id.to_string()),
        name: Some("JayFestival".to_string()),
        description: Some("Evenements synchronises depuis JayFestival".to_string()),
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
