//! @id     jaykoa_tests_import_heritage
//! @do     verifier_import_base_heritee_jaykoa
//! @role   test
//! @layer  infra
//! @human  jaykoa — import de la base SQLite héritée dans un KindMother éphémère :
//!         lignes identiques, orphelines écartées, marqueur, no-op à la relance,
//!         fichier intact, `open` accepté après import.

#![cfg(feature = "import-heritage")]

use std::path::PathBuf;

use jaykoa::data::import_heritage::{importer_base_heritee, plan};
use jaykoa::data::JayKoaDb;
use kindmother_client::heritage::{chemin_marqueur_import, refuser_si_base_heritee};
use kindmother_client::{KindMotherClient, OperatorCredential, SqlValue};
use kindmother_heritage::fabrique::{creer_base_heritee, verifier_dans_kindmother};

/// DDL du chemin hérité (`kindmother_db.rs` retiré au lot C, `init_schema`), copié à l'identique.
const DDL_HERITE: &str = r"
CREATE TABLE IF NOT EXISTS agendas (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    color TEXT DEFAULT '#4285F4',
    calendar_type TEXT DEFAULT 'personal',
    visible INTEGER DEFAULT 1,
    is_default INTEGER DEFAULT 0,
    source_service TEXT,
    created_at TEXT,
    updated_at TEXT
);
CREATE TABLE IF NOT EXISTS temporal_entries (
    id TEXT PRIMARY KEY,
    agenda_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    start_datetime TEXT NOT NULL,
    end_datetime TEXT NOT NULL,
    all_day INTEGER DEFAULT 0,
    location TEXT,
    status TEXT DEFAULT 'confirmed',
    entry_type TEXT DEFAULT 'internal',
    source_service TEXT,
    source_event_id TEXT,
    color TEXT,
    recurrence_rule TEXT,
    reminders_json TEXT,
    created_at TEXT,
    updated_at TEXT,
    last_synced_at TEXT,
    FOREIGN KEY (agenda_id) REFERENCES agendas(id)
);
CREATE TABLE IF NOT EXISTS user_settings (
    profile_id TEXT PRIMARY KEY,
    default_view TEXT DEFAULT 'week',
    week_start_day INTEGER DEFAULT 0,
    day_start_hour INTEGER DEFAULT 8,
    day_end_hour INTEGER DEFAULT 20,
    timezone TEXT DEFAULT 'Europe/Paris',
    default_reminder_minutes INTEGER DEFAULT 30,
    use_24h_format INTEGER DEFAULT 1,
    show_week_numbers INTEGER DEFAULT 0,
    show_cancelled_events INTEGER DEFAULT 0
);
CREATE INDEX IF NOT EXISTS idx_entries_agenda ON temporal_entries(agenda_id);
CREATE INDEX IF NOT EXISTS idx_entries_dates ON temporal_entries(start_datetime, end_datetime);
CREATE INDEX IF NOT EXISTS idx_entries_type ON temporal_entries(entry_type);
CREATE INDEX IF NOT EXISTS idx_agendas_profile ON agendas(profile_id);
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

/// Lignes héritées, dont un engagement orphelin (agenda supprimé).
fn lignes() -> Vec<(&'static str, Vec<(&'static str, SqlValue)>)> {
    vec![
        (
            "agendas",
            vec![
                ("id", t("ag1")),
                ("profile_id", t("p1")),
                ("name", t("Perso")),
                ("description", N),
                ("color", t("#4285F4")),
                ("calendar_type", t("personal")),
                ("visible", i(1)),
                ("is_default", i(1)),
                ("source_service", N),
                ("created_at", t("2025-01-01T00:00:00Z")),
                ("updated_at", N),
            ],
        ),
        (
            "temporal_entries",
            vec![
                ("id", t("e1")),
                ("agenda_id", t("ag1")),
                ("title", t("Réunion")),
                ("description", t("")),
                ("start_datetime", t("2025-06-01T09:00:00")),
                ("end_datetime", t("2025-06-01T10:00:00")),
                ("all_day", i(0)),
                ("location", N),
                ("status", t("confirmed")),
                ("entry_type", t("internal")),
                ("source_service", N),
                ("source_event_id", N),
                ("color", N),
                ("recurrence_rule", N),
                ("reminders_json", t("[15]")),
                ("created_at", t("2025-05-01T00:00:00Z")),
                ("updated_at", t("2025-05-01T00:00:00Z")),
                ("last_synced_at", N),
            ],
        ),
        (
            "temporal_entries",
            vec![
                ("id", t("e-orpheline")),
                ("agenda_id", t("ag-supprime")),
                ("title", t("Perdu")),
                ("start_datetime", t("2025-06-02T09:00:00")),
                ("end_datetime", t("2025-06-02T10:00:00")),
            ],
        ),
        (
            "user_settings",
            vec![
                ("profile_id", t("p1")),
                ("default_view", t("month")),
                ("week_start_day", i(1)),
                ("day_start_hour", i(7)),
                ("day_end_hour", i(19)),
                ("timezone", N),
                ("default_reminder_minutes", i(10)),
                ("use_24h_format", i(1)),
                ("show_week_numbers", i(1)),
                ("show_cancelled_events", i(0)),
            ],
        ),
    ]
}

#[test]
fn import_base_heritee_jaykoa() {
    let lignes = lignes();
    let (_tmp, addr, creds, cred, base) = preparer("jaykoa", "jaykoa.db", &lignes);
    let octets = std::fs::read(&base).expect("lecture");

    // Avant import : `open` refuse.
    assert!(JayKoaDb::open(&base).is_err(), "garde de bascule");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        let client = KindMotherClient::connect(&addr, &cred, "jaykoa")
            .await
            .expect("client");
        let rapport = importer_base_heritee(&base, &client).await.expect("import");
        println!("{rapport}");
        assert_eq!(rapport.total_orphelines(), 1);
        let attendues: Vec<_> = lignes
            .iter()
            .filter(|(_, l)| l[0].1 != t("e-orpheline"))
            .cloned()
            .collect();
        let n = verifier_dans_kindmother(&client, &plan(), &attendues, &[])
            .await
            .expect("lignes identiques");
        assert_eq!(n, 3);

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
    let db = JayKoaDb::open(&base).expect("open après import");
    let agendas = db.agendas_by_profile("p1").expect("agendas");
    assert_eq!(agendas.len(), 1);
    assert_eq!(db.entries_by_agenda("ag1").expect("engagements").len(), 1);
}
