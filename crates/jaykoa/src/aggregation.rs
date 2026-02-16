//! Agrégation temporelle et détection de conflits.
//!
//! JayKoa reflète et agrège ; ne décide pas du temps. Les conflits sont
//! exposés à l'utilisateur (visualisation), jamais résolus automatiquement.

use crate::data::types::{TemporalConflict, TemporalEntry};

/// Détecte les conflits (chevauchements) entre entrées temporelles.
///
/// Deux entrées sont en conflit si leurs plages [start, end] se chevauchent
/// (start_a < end_b && end_a > start_b). Les entrées annulées sont ignorées.
pub fn compute_conflicts(entries: &[TemporalEntry]) -> Vec<TemporalConflict> {
    let mut conflicts = Vec::new();
    let active: Vec<&TemporalEntry> = entries
        .iter()
        .filter(|e| {
            e.status.as_deref() != Some("cancelled")
                && e.start_datetime.is_some()
                && e.end_datetime.is_some()
        })
        .collect();
    for (i, a) in active.iter().enumerate() {
        let start_a = a.start_datetime.as_deref().unwrap_or("");
        let end_a = a.end_datetime.as_deref().unwrap_or("");
        for b in active.iter().skip(i + 1) {
            let start_b = b.start_datetime.as_deref().unwrap_or("");
            let end_b = b.end_datetime.as_deref().unwrap_or("");
            if overlaps(start_a, end_a, start_b, end_b) {
                let (overlap_start, overlap_end) = overlap_range(start_a, end_a, start_b, end_b);
                conflicts.push(TemporalConflict {
                    entry_a_id: a.id.clone(),
                    entry_b_id: b.id.clone(),
                    overlap_start: Some(overlap_start.clone()),
                    overlap_end: Some(overlap_end.clone()),
                    description: Some(format!("Chevauchement {} - {}", overlap_start, overlap_end)),
                });
            }
        }
    }
    conflicts
}

fn overlaps(start_a: &str, end_a: &str, start_b: &str, end_b: &str) -> bool {
    start_a < end_b && end_a > start_b
}

fn overlap_range(start_a: &str, end_a: &str, start_b: &str, end_b: &str) -> (String, String) {
    let s = if start_a >= start_b { start_a } else { start_b };
    let e = if end_a <= end_b { end_a } else { end_b };
    (s.to_string(), e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::TemporalEntry;

    fn entry(id: &str, start: &str, end: &str) -> TemporalEntry {
        TemporalEntry {
            id: Some(id.into()),
            start_datetime: Some(start.into()),
            end_datetime: Some(end.into()),
            ..Default::default()
        }
    }

    #[test]
    fn no_overlap() {
        let entries = vec![
            entry("1", "2025-02-16T09:00:00Z", "2025-02-16T10:00:00Z"),
            entry("2", "2025-02-16T10:00:00Z", "2025-02-16T11:00:00Z"),
        ];
        let c = compute_conflicts(&entries);
        assert!(c.is_empty());
    }

    #[test]
    fn overlap() {
        let entries = vec![
            entry("1", "2025-02-16T09:00:00Z", "2025-02-16T10:30:00Z"),
            entry("2", "2025-02-16T10:00:00Z", "2025-02-16T11:00:00Z"),
        ];
        let c = compute_conflicts(&entries);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].entry_a_id.as_deref(), Some("1"));
        assert_eq!(c[0].entry_b_id.as_deref(), Some("2"));
    }
}
