//! Adaptateur JayRDV → JayKoa : stub MVP lecture réfléchie.
//!
//! @id: jaykoa_jayrdv_adapter
//! @do: read_jayrdv_appointments_and_create_reflections
//! @role: sync
//! @layer: infra
//! Production : lirait via BondingBrother + Mandat de Permission.

use crate::data::types::{EntryType, EventSource, TemporalEntry, TemporalStatus};
use crate::data::JayKoaDb;
use chrono::Local;
use std::sync::Arc;

/// Erreur de synchronisation JayRDV.
#[derive(Debug)]
pub struct JayRDVSyncError(pub String);
impl std::fmt::Display for JayRDVSyncError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "JayRDV sync: {}", self.0) } }
impl std::error::Error for JayRDVSyncError {}

/// Adaptateur de synchronisation JayRDV → JayKoa (stub MVP).
pub struct JayRDVAdapter;

impl JayRDVAdapter {
    /// Synchronise les rendez-vous JayRDV en reflets dans un agenda JayKoa.
    pub fn sync_appointments(koa_db: &Arc<JayKoaDb>, target_agenda_id: &str) -> Result<usize, JayRDVSyncError> {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let mock = vec![
            ("rdv_001", "Consultation Dr. Martin", "2026-02-10T10:00:00", "2026-02-10T10:30:00", "Cabinet médical"),
            ("rdv_002", "RDV Client — Projet Alpha", "2026-02-12T14:00:00", "2026-02-12T15:30:00", "Salle B"),
            ("rdv_003", "Entretien annuel", "2026-02-20T09:00:00", "2026-02-20T10:00:00", "Bureau RH"),
        ];
        let mut count = 0;
        for (sid, title, start, end, loc) in &mock {
            let e = TemporalEntry {
                id: Some(uuid::Uuid::new_v4().to_string()), agenda_id: Some(target_agenda_id.to_string()),
                title: Some(title.to_string()), description: Some(format!("RDV JayRDV : {}", title)),
                start_datetime: Some(start.to_string()), end_datetime: Some(end.to_string()), all_day: false,
                location: Some(loc.to_string()), status: Some(TemporalStatus::Confirmed.as_str().to_string()),
                entry_type: Some(EntryType::ReflectJayRDV.as_str().to_string()),
                source_service: Some(EventSource::JayRDV.as_str().to_string()),
                source_event_id: Some(sid.to_string()),
                color: Some(EventSource::JayRDV.default_color().to_string()),
                created_at: Some(now.clone()), updated_at: Some(now.clone()), last_synced_at: Some(now.clone()),
                ..Default::default()
            };
            koa_db.reflect_upsert(&e).map_err(|e| JayRDVSyncError(e.to_string()))?;
            count += 1;
        }
        Ok(count)
    }
}
