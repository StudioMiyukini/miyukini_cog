//! @id     miyukiniwatch_data_mod
//! @do     declarer_module_db
//! @role   donnee
//! @layer  core
//! @human  miyukiniwatch — module « data mod ».

//! Données MiyukiniWatch : types et persistance KindMother.
//!
//! Persistance :
//! - `kindmother-only` (défaut) : service KindMother, seul écrivain (K-13) ;
//! - `legacy-sqlite` (HÉRITÉ, transitoire) : SQLite local direct. S'il est activé
//!   explicitement, il est prioritaire sur `kindmother-only` (cumul des features Cargo).

#[cfg(not(any(feature = "kindmother-only", feature = "legacy-sqlite")))]
compile_error!("miyukiniwatch : activer `kindmother-only` (défaut) ou `legacy-sqlite`");

#[cfg(feature = "legacy-sqlite")]
mod db;
#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
mod kindmother_client_db;

#[cfg(any(all(feature = "kindmother-only", not(feature = "legacy-sqlite")), feature = "import-heritage"))]
mod schema_kindmother;

#[cfg(feature = "import-heritage")]
pub mod import_heritage;
mod types;

#[cfg(feature = "legacy-sqlite")]
pub use db::MiyukiniWatchDb;
#[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
pub use kindmother_client_db::MiyukiniWatchDb;
pub use types::{AuditEvent, MetricRecord, Prefs, TimeSlot};

/// Tests de contrat communs aux deux persistances : exécutés contre KindMother (serveur
/// éphémère, défaut) ou contre la base héritée (`--no-default-features --features legacy-sqlite`).
#[cfg(test)]
mod contract_tests {
    use super::{MetricRecord, MiyukiniWatchDb, Prefs};
    use chrono::{Local, TimeZone};

    #[cfg(feature = "legacy-sqlite")]
    fn make_db() -> MiyukiniWatchDb {
        let dir = tempfile::tempdir().expect("tempdir").keep();
        MiyukiniWatchDb::open(dir.join("miyukiniwatch.db")).expect("open legacy db")
    }

    #[cfg(all(feature = "kindmother-only", not(feature = "legacy-sqlite")))]
    fn make_db() -> MiyukiniWatchDb {
        super::kindmother_client_db::test_support::ephemeral_db()
    }

    fn metric(id: &str, profile: &str, metric_id: &str, session: &str, day: u32) -> MetricRecord {
        MetricRecord {
            record_id: id.to_string(),
            profile_id: profile.to_string(),
            metric_id: metric_id.to_string(),
            session_id: session.to_string(),
            timestamp: Local
                .with_ymd_and_hms(2026, 3, day, 12, 0, 0)
                .single()
                .expect("date"),
            service_id: Some("jaykoa".to_string()),
            friend_id: None,
            value_int: Some(i64::from(day)),
            value_str: None,
        }
    }

    #[test]
    fn metriques_insertion_dedoublonnage_et_comptage() {
        let db = make_db();
        assert!(!db.has_data("p1").unwrap());
        db.insert_metric(&metric("m1", "p1", "S-01", "s1", 1))
            .unwrap();
        db.insert_metric(&metric("m1", "p1", "S-01", "s1", 1))
            .unwrap(); // même id : ignoré
        let mut same_key = metric("m1-bis", "p1", "S-01", "s1", 1);
        same_key.value_int = None;
        db.insert_metric(&same_key).unwrap(); // même (session, métrique, horodatage) : ignoré
        db.insert_metric(&metric("m2", "p1", "S-02", "s1", 2))
            .unwrap();
        db.insert_metric(&metric("m3", "p1", "S-02", "s2", 3))
            .unwrap();
        assert!(db.has_data("p1").unwrap());
        assert!(!db.has_data("p2").unwrap());
        assert_eq!(db.count_metrics_for_session("p1", "s1").unwrap(), 2);
        assert_eq!(db.count_metrics_for_session("p1", "s2").unwrap(), 1);
        let last = db.get_last_session_end("p1").unwrap().expect("S-02");
        assert_eq!(
            last,
            metric("m3", "p1", "S-02", "s2", 3).timestamp.to_rfc3339()
        );
        assert_eq!(db.get_last_session_end("p2").unwrap(), None);
    }

    #[test]
    fn suppressions_par_plage_categorie_purge_et_totale() {
        let db = make_db();
        for (i, day) in (1..=6).enumerate() {
            let cat = if i % 2 == 0 { "S-0" } else { "F-0" };
            db.insert_metric(&metric(
                &format!("m{day}"),
                "p1",
                &format!("{cat}{day}"),
                "s",
                day,
            ))
            .unwrap();
        }
        db.insert_metric(&metric("other", "p2", "S-01", "s-p2", 1))
            .unwrap();
        let ts = |day| metric("x", "p", "S", "s", day).timestamp.to_rfc3339();

        // Plage [jour 2, jour 3] : m2, m3.
        assert_eq!(
            db.delete_metrics_by_date_range("p1", &ts(2), &ts(3))
                .unwrap(),
            2
        );
        // Catégorie « F- » restante : m4, m6.
        assert_eq!(db.delete_metrics_by_category("p1", "F-").unwrap(), 2);
        // Purge strictement avant le jour 5 : m1.
        assert_eq!(db.purge_raw_before("p1", &ts(5)).unwrap(), 1);
        assert_eq!(db.count_metrics_for_session("p1", "s").unwrap(), 1); // m5

        db.set_global("p1", "total_sessions", 4).unwrap();
        assert_eq!(db.delete_all("p1").unwrap(), 1);
        assert!(!db.has_data("p1").unwrap());
        assert_eq!(db.get_global("p1", "total_sessions").unwrap(), None);
        assert!(
            db.has_data("p2").unwrap(),
            "les autres profils sont intacts"
        );
    }

    #[test]
    fn preferences_defaut_et_mise_a_jour() {
        let db = make_db();
        let p = db.get_prefs("p1").unwrap();
        assert_eq!(
            p,
            Prefs {
                profile_id: "p1".to_string(),
                ..Prefs::default()
            }
        );
        assert_eq!(
            db.get_prefs("p1").unwrap(),
            p,
            "valeurs par défaut persistées"
        );
        let updated = Prefs {
            collect_friends: false,
            retention_raw_days: 7,
            ..p
        };
        db.set_prefs(&updated).unwrap();
        assert_eq!(db.get_prefs("p1").unwrap(), updated);
        let fresh = Prefs {
            profile_id: "p2".to_string(),
            collect_enabled: false,
            ..Prefs::default()
        };
        db.set_prefs(&fresh).unwrap();
        assert_eq!(db.get_prefs("p2").unwrap(), fresh);
    }

    #[test]
    fn compteurs_globaux() {
        let db = make_db();
        assert_eq!(db.get_global("p1", "total_sessions").unwrap(), None);
        db.set_global("p1", "total_sessions", 1).unwrap();
        db.set_global("p1", "total_sessions", 5).unwrap();
        db.set_global("p1", "consecutive_active_days", 2).unwrap();
        assert_eq!(db.get_global("p1", "total_sessions").unwrap(), Some(5));
        assert_eq!(
            db.get_global("p1", "consecutive_active_days").unwrap(),
            Some(2)
        );
        assert_eq!(db.get_global("p2", "total_sessions").unwrap(), None);
    }

    #[test]
    fn audit_ordre_et_limite() {
        let db = make_db();
        db.insert_audit("p1", "collect_off", Some("par l'utilisateur"), None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(5));
        db.insert_audit("p1", "delete_all", None, Some(12)).unwrap();
        db.insert_audit("p2", "purge", None, Some(1)).unwrap();
        let events = db.list_audit_events("p1", 10).unwrap();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, "delete_all");
        assert_eq!(events[0].records_affected, Some(12));
        assert_eq!(events[1].details.as_deref(), Some("par l'utilisateur"));
        assert_eq!(db.list_audit_events("p1", 1).unwrap().len(), 1);
        assert_eq!(db.list_audit_events("p1", -1).unwrap().len(), 2);
    }
}
