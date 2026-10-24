//! @id     miyukiniwatch_tests_import_heritage
//! @do     verifier_import_base_heritee_miyukiniwatch
//! @role   test
//! @layer  infra
//! @human  miyukiniwatch — import de la base SQLite héritée dans un KindMother éphémère :
//!         lignes identiques, orphelines écartées, marqueur, no-op à la relance,
//!         fichier intact, `open` accepté après import.

#![cfg(all(
    feature = "import-heritage",
    feature = "kindmother-only",
    not(feature = "legacy-sqlite")
))]

use std::path::PathBuf;

use kindmother_client::heritage::{chemin_marqueur_import, refuser_si_base_heritee};
use kindmother_client::{KindMotherClient, OperatorCredential, SqlValue};
use kindmother_heritage::fabrique::{creer_base_heritee, verifier_dans_kindmother};
use miyukiniwatch::data::import_heritage::{importer_base_heritee, plan};
use miyukiniwatch::data::MiyukiniWatchDb;

/// DDL du chemin hérité (`db.rs`, `init_schema`), copié à l'identique.
const DDL_HERITE: &str = r"
CREATE TABLE IF NOT EXISTS miyukiniwatch_metrics (
    record_id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    metric_id TEXT NOT NULL,
    session_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    value_int INTEGER,
    value_str TEXT,
    UNIQUE(session_id, metric_id, timestamp)
);
CREATE INDEX IF NOT EXISTS idx_mw_metrics_profile_ts
    ON miyukiniwatch_metrics(profile_id, timestamp);
CREATE INDEX IF NOT EXISTS idx_mw_metrics_metric
    ON miyukiniwatch_metrics(metric_id);

CREATE TABLE IF NOT EXISTS miyukiniwatch_daily (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    date TEXT NOT NULL,
    metric_id TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    count INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    min_value INTEGER,
    max_value INTEGER,
    UNIQUE(profile_id, date, metric_id, service_id, friend_id)
);

CREATE TABLE IF NOT EXISTS miyukiniwatch_weekly (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    year INTEGER NOT NULL,
    week INTEGER NOT NULL,
    metric_id TEXT NOT NULL,
    service_id TEXT,
    friend_id TEXT,
    count INTEGER NOT NULL,
    total_value INTEGER NOT NULL,
    avg_value REAL,
    UNIQUE(profile_id, year, week, metric_id, service_id, friend_id)
);

CREATE TABLE IF NOT EXISTS miyukiniwatch_globals (
    profile_id TEXT NOT NULL,
    key TEXT NOT NULL,
    value INTEGER NOT NULL,
    updated_at TEXT NOT NULL,
    PRIMARY KEY (profile_id, key)
);

CREATE TABLE IF NOT EXISTS miyukiniwatch_audit (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    profile_id TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    event_type TEXT NOT NULL,
    details TEXT,
    records_affected INTEGER
);

CREATE TABLE IF NOT EXISTS miyukiniwatch_prefs (
    profile_id TEXT PRIMARY KEY,
    collect_enabled INTEGER NOT NULL DEFAULT 1,
    collect_sessions INTEGER NOT NULL DEFAULT 1,
    collect_services INTEGER NOT NULL DEFAULT 1,
    collect_friends INTEGER NOT NULL DEFAULT 1,
    collect_activity INTEGER NOT NULL DEFAULT 1,
    retention_raw_days INTEGER NOT NULL DEFAULT 30,
    retention_daily_days INTEGER NOT NULL DEFAULT 90,
    retention_weekly_days INTEGER NOT NULL DEFAULT 365,
    updated_at TEXT NOT NULL
);
";

struct TempDir(PathBuf);

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn t(s: &str) -> SqlValue {
    SqlValue::Text(s.to_string())
}

#[allow(dead_code)]
fn i(v: i64) -> SqlValue {
    SqlValue::Integer(v)
}

const N: SqlValue = SqlValue::Null;

/// Serveur KindMother éphémère, justificatif de l'opérateur, base héritée créée.
fn preparer(
    operateur: &str,
    fichier: &str,
    lignes: &[(&str, Vec<(&str, SqlValue)>)],
) -> (TempDir, String, PathBuf, OperatorCredential, PathBuf) {
    let tmp = TempDir(
        std::env::temp_dir().join(format!("{operateur}-heritage-{}", uuid::Uuid::new_v4())),
    );
    std::fs::create_dir_all(&tmp.0).expect("tempdir");
    let (addr, creds) =
        kindmother_service::spawn_ephemeral_blocking(tmp.0.join("km")).expect("serveur éphémère");
    let cred = OperatorCredential::load_from_dir(operateur, &creds).expect("justificatif");
    let base = tmp.0.join(fichier);
    creer_base_heritee(&base, DDL_HERITE, lignes).expect("base héritée");
    (tmp, addr.to_string(), creds, cred, base)
}

/// Lignes héritées (identifiants AUTOINCREMENT non contigus conservés).
fn lignes() -> Vec<(&'static str, Vec<(&'static str, SqlValue)>)> {
    let ts = || t("2026-03-01T12:00:00+01:00");
    vec![
        (
            "miyukiniwatch_metrics",
            vec![
                ("record_id", t("rec1")),
                ("profile_id", t("p1")),
                ("metric_id", t("session.duration")),
                ("session_id", t("s1")),
                ("timestamp", ts()),
                ("service_id", t("jaykoa")),
                ("friend_id", N),
                ("value_int", i(5)),
                ("value_str", N),
            ],
        ),
        (
            "miyukiniwatch_daily",
            vec![
                ("id", i(42)),
                ("profile_id", t("p1")),
                ("date", t("2026-03-01")),
                ("metric_id", t("session.duration")),
                ("service_id", N),
                ("friend_id", N),
                ("count", i(3)),
                ("total_value", i(15)),
                ("min_value", i(1)),
                ("max_value", i(9)),
            ],
        ),
        (
            "miyukiniwatch_weekly",
            vec![
                ("id", i(7)),
                ("profile_id", t("p1")),
                ("year", i(2026)),
                ("week", i(9)),
                ("metric_id", t("session.duration")),
                ("service_id", N),
                ("friend_id", N),
                ("count", i(3)),
                ("total_value", i(15)),
                ("avg_value", SqlValue::Real(2.5)),
            ],
        ),
        (
            "miyukiniwatch_globals",
            vec![
                ("profile_id", t("p1")),
                ("key", t("sessions")),
                ("value", i(3)),
                ("updated_at", ts()),
            ],
        ),
        (
            "miyukiniwatch_audit",
            vec![
                ("id", i(100)),
                ("profile_id", t("p1")),
                ("timestamp", ts()),
                ("event_type", t("purge")),
                ("details", t("")),
                ("records_affected", N),
            ],
        ),
        (
            "miyukiniwatch_prefs",
            vec![
                ("profile_id", t("p1")),
                ("collect_enabled", i(1)),
                ("collect_sessions", i(0)),
                ("collect_services", i(1)),
                ("collect_friends", i(0)),
                ("collect_activity", i(1)),
                ("retention_raw_days", i(7)),
                ("retention_daily_days", i(60)),
                ("retention_weekly_days", i(200)),
                ("updated_at", ts()),
            ],
        ),
    ]
}

#[test]
fn import_base_heritee_miyukiniwatch() {
    let lignes = lignes();
    let (_tmp, addr, creds, cred, base) = preparer("miyukiniwatch", "miyukiniwatch.db", &lignes);
    let octets = std::fs::read(&base).expect("lecture");

    // Avant import : `open` refuse.
    assert!(MiyukiniWatchDb::open(&base).is_err(), "garde de bascule");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        let client = KindMotherClient::connect(&addr, &cred, "miyukiniwatch")
            .await
            .expect("client");
        let rapport = importer_base_heritee(&base, &client).await.expect("import");
        println!("{rapport}");
        assert_eq!(rapport.total_attendues(), lignes.len());
        let n = verifier_dans_kindmother(&client, &plan(), &lignes, &[])
            .await
            .expect("lignes identiques");
        assert_eq!(n, lignes.len());

        // Relance : no-op.
        let encore = importer_base_heritee(&base, &client)
            .await
            .expect("relance");
        assert!(encore.deja_importe);
    });

    assert!(chemin_marqueur_import(&base).is_file());
    assert_eq!(
        std::fs::read(&base).expect("lecture"),
        octets,
        "fichier hérité intact"
    );
    assert!(refuser_si_base_heritee(&base).is_ok());

    // Après import : `open` (client global, environnement) accepte et lit les données.
    std::env::set_var("KINDMOTHER_ADDR", &addr);
    std::env::set_var(kindmother_client::ENV_CREDENTIALS_DIR, &creds);
    let db = MiyukiniWatchDb::open(&base).expect("open après import");
    let prefs = db.get_prefs("p1").expect("préférences");
    assert_eq!(prefs.retention_raw_days, 7);
    assert!(!prefs.collect_sessions);
}
