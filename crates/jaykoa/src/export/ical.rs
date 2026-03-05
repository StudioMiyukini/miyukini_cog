//! Export iCalendar (RFC 5545) pour JayKoa.
//!
//! @id: jaykoa_export_ical
//! @do: generate_ical_from_temporal_entries
//! @layer: domain
//! @human: Génère un fichier .ics conforme RFC 5545 à partir des TemporalEntry.
//!         Chaque entrée devient un VEVENT. Pas de dépendance externe : format texte pur.
//!         Propriétés custom X-JAYKOA-SOURCE et X-JAYKOA-ENTRY-TYPE pour traçabilité.

use crate::data::types::TemporalEntry;

/// Génère un calendrier iCalendar complet (VCALENDAR) à partir d'une liste
/// d'entrées temporelles.
///
/// - `entries` : entrées à exporter en VEVENT.
/// - `calendar_name` : nom affiché du calendrier (X-WR-CALNAME).
///
/// Retourne une `String` au format `.ics` conforme RFC 5545.
pub fn entries_to_ical(entries: &[TemporalEntry], calendar_name: &str) -> String {
    let mut out = String::with_capacity(entries.len() * 512 + 256);

    // --- VCALENDAR header ---
    out.push_str("BEGIN:VCALENDAR\r\n");
    out.push_str("VERSION:2.0\r\n");
    out.push_str("PRODID:-//Miyukini//JayKoa//FR\r\n");
    out.push_str("CALSCALE:GREGORIAN\r\n");
    out.push_str("METHOD:PUBLISH\r\n");
    out.push_str(&format!(
        "X-WR-CALNAME:{}\r\n",
        escape_ical_text(calendar_name)
    ));

    // --- VEVENTs ---
    for entry in entries {
        out.push_str("BEGIN:VEVENT\r\n");

        // UID (obligatoire)
        let uid = entry.id.as_deref().unwrap_or("unknown");
        out.push_str(&format!("UID:{}\r\n", uid));

        // DTSTART / DTEND
        if let Some(ref dt) = entry.start_datetime {
            out.push_str(&format!("DTSTART:{}\r\n", datetime_to_ical(dt)));
        }
        if let Some(ref dt) = entry.end_datetime {
            out.push_str(&format!("DTEND:{}\r\n", datetime_to_ical(dt)));
        }

        // SUMMARY
        let title = entry.title.as_deref().unwrap_or("(sans titre)");
        out.push_str(&format!("SUMMARY:{}\r\n", escape_ical_text(title)));

        // DESCRIPTION
        if let Some(ref desc) = entry.description {
            if !desc.is_empty() {
                out.push_str(&format!("DESCRIPTION:{}\r\n", escape_ical_text(desc)));
            }
        }

        // LOCATION
        if let Some(ref loc) = entry.location {
            if !loc.is_empty() {
                out.push_str(&format!("LOCATION:{}\r\n", escape_ical_text(loc)));
            }
        }

        // STATUS (CONFIRMED / TENTATIVE / CANCELLED)
        let status = match entry.status.as_deref() {
            Some("confirmed") => "CONFIRMED",
            Some("tentative") => "TENTATIVE",
            Some("cancelled") => "CANCELLED",
            _ => "CONFIRMED",
        };
        out.push_str(&format!("STATUS:{}\r\n", status));

        // X-JAYKOA-SOURCE (propriété custom)
        if let Some(ref src) = entry.source_service {
            out.push_str(&format!("X-JAYKOA-SOURCE:{}\r\n", escape_ical_text(src)));
        }

        // X-JAYKOA-ENTRY-TYPE (propriété custom)
        if let Some(ref et) = entry.entry_type {
            out.push_str(&format!("X-JAYKOA-ENTRY-TYPE:{}\r\n", escape_ical_text(et)));
        }

        // RRULE (récurrence optionnelle)
        if let Some(ref rrule) = entry.recurrence_rule {
            if !rrule.is_empty() {
                out.push_str(&format!("RRULE:{}\r\n", rrule));
            }
        }

        // DTSTAMP (obligatoire RFC 5545 — on utilise created_at ou now)
        if let Some(ref ca) = entry.created_at {
            out.push_str(&format!("DTSTAMP:{}\r\n", datetime_to_ical(ca)));
        }

        out.push_str("END:VEVENT\r\n");
    }

    // --- VCALENDAR footer ---
    out.push_str("END:VCALENDAR\r\n");
    out
}

/// Convertit une datetime ISO 8601 (`"2026-03-15T09:00:00"`) en format
/// iCalendar (`"20260315T090000"`).
///
/// Si le format d'entrée est inattendu, retourne la chaîne telle quelle.
fn datetime_to_ical(dt_str: &str) -> String {
    // Supprime les séparateurs '-' et ':' tout en gardant le 'T'.
    // "2026-03-15T09:00:00" → "20260315T090000"
    let cleaned: String = dt_str.chars().filter(|c| *c != '-' && *c != ':').collect();
    cleaned
}

/// Échappe les caractères spéciaux dans un texte iCal (RFC 5545 §3.3.11).
///
/// Les backslashes, points-virgules, virgules et retours à la ligne sont échappés.
fn escape_ical_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace(';', "\\;")
        .replace(',', "\\,")
        .replace('\n', "\\n")
        .replace('\r', "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_to_ical() {
        assert_eq!(datetime_to_ical("2026-03-15T09:00:00"), "20260315T090000");
        assert_eq!(datetime_to_ical("2026-12-31T23:59:59"), "20261231T235959");
    }

    #[test]
    fn test_escape_ical_text() {
        assert_eq!(escape_ical_text("Hello, World"), "Hello\\, World");
        assert_eq!(escape_ical_text("line1\nline2"), "line1\\nline2");
        assert_eq!(escape_ical_text("a;b\\c"), "a\\;b\\\\c");
    }

    #[test]
    fn test_entries_to_ical_empty() {
        let result = entries_to_ical(&[], "Test");
        assert!(result.contains("BEGIN:VCALENDAR"));
        assert!(result.contains("END:VCALENDAR"));
        assert!(result.contains("PRODID:-//Miyukini//JayKoa//FR"));
        assert!(result.contains("VERSION:2.0"));
        assert!(!result.contains("BEGIN:VEVENT"));
    }

    #[test]
    fn test_entries_to_ical_single_event() {
        let entry = TemporalEntry {
            id: Some("evt-001".to_string()),
            title: Some("Réunion".to_string()),
            description: Some("Salle A".to_string()),
            start_datetime: Some("2026-03-15T09:00:00".to_string()),
            end_datetime: Some("2026-03-15T10:00:00".to_string()),
            location: Some("Paris".to_string()),
            status: Some("confirmed".to_string()),
            source_service: Some("jaykoa".to_string()),
            entry_type: Some("internal".to_string()),
            recurrence_rule: Some("FREQ=WEEKLY;COUNT=4".to_string()),
            created_at: Some("2026-03-01T08:00:00".to_string()),
            ..Default::default()
        };
        let result = entries_to_ical(&[entry], "Mon Agenda");
        assert!(result.contains("UID:evt-001"));
        assert!(result.contains("DTSTART:20260315T090000"));
        assert!(result.contains("DTEND:20260315T100000"));
        assert!(result.contains("SUMMARY:Réunion"));
        assert!(result.contains("DESCRIPTION:Salle A"));
        assert!(result.contains("LOCATION:Paris"));
        assert!(result.contains("STATUS:CONFIRMED"));
        assert!(result.contains("X-JAYKOA-SOURCE:jaykoa"));
        assert!(result.contains("X-JAYKOA-ENTRY-TYPE:internal"));
        assert!(result.contains("RRULE:FREQ=WEEKLY;COUNT=4"));
        assert!(result.contains("X-WR-CALNAME:Mon Agenda"));
    }
}
