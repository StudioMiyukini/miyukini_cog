//! Adaptateur JayFestival → JayKoa : stub MVP lecture réfléchie.
//!
//! @id: jaykoa_jayfestival_adapter
//! @do: read_jayfestival_editions_and_create_reflections
//! @role: sync
//! @layer: infra
//! Production : lirait via BondingBrother + Mandat de Permission.

use crate::data::types::{EntryType, EventSource, TemporalEntry, TemporalStatus};
use crate::data::JayKoaDb;
use chrono::Local;
use std::sync::Arc;

/// Erreur de synchronisation JayFestival.
#[derive(Debug)]
pub struct JayFestivalSyncError(pub String);
impl std::fmt::Display for JayFestivalSyncError { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { write!(f, "JayFestival sync: {}", self.0) } }
impl std::error::Error for JayFestivalSyncError {}

/// Adaptateur de synchronisation JayFestival → JayKoa (stub MVP).
pub struct JayFestivalAdapter;

impl JayFestivalAdapter {
    /// Synchronise les éditions JayFestival en reflets dans un agenda JayKoa.
    pub fn sync_editions(koa_db: &Arc<JayKoaDb>, target_agenda_id: &str) -> Result<usize, JayFestivalSyncError> {
        let now = Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
        let mock = vec![
            ("fest_ed_2026_printemps", "Festival Printemps 2026", "2026-03-15T09:00:00", "2026-03-17T18:00:00", "Parc des Expositions"),
            ("fest_ed_2026_ete", "Festival Été 2026", "2026-07-01T10:00:00", "2026-07-03T22:00:00", "Esplanade du Lac"),
        ];
        let mut count = 0;
        for (sid, title, start, end, loc) in &mock {
            let e = TemporalEntry {
                id: Some(uuid::Uuid::new_v4().to_string()), agenda_id: Some(target_agenda_id.to_string()),
                title: Some(title.to_string()), description: Some(format!("Édition JayFestival : {}", title)),
                start_datetime: Some(start.to_string()), end_datetime: Some(end.to_string()), all_day: false,
                location: Some(loc.to_string()), status: Some(TemporalStatus::Confirmed.as_str().to_string()),
                entry_type: Some(EntryType::ReflectJayFestival.as_str().to_string()),
                source_service: Some(EventSource::JayFestival.as_str().to_string()),
                source_event_id: Some(sid.to_string()),
                color: Some(EventSource::JayFestival.default_color().to_string()),
                created_at: Some(now.clone()), updated_at: Some(now.clone()), last_synced_at: Some(now.clone()),
                ..Default::default()
            };
            koa_db.reflect_upsert(&e).map_err(|e| JayFestivalSyncError(e.to_string()))?;
            count += 1;
        }
        Ok(count)
    }
}
