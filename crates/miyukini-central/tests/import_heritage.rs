//! @id     miyukini_central_tests_import_heritage
//! @do     verifier_import_base_heritee_miyukini_central
//! @role   test
//! @layer  infra
//! @human  miyukini-central — import de la base SQLite héritée dans un KindMother éphémère :
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
use miyukini_central::auth::import_heritage::{importer_base_heritee, plan};
use miyukini_central::auth::CentralAuthDb;

/// DDL du chemin hérité (`auth/db.rs`, `init_schema` d'une version antérieure à `migrate_add_profile_columns` : colonnes `is_admin`, `genre`… absentes), copié à l'identique.
const DDL_HERITE: &str = r"
CREATE TABLE IF NOT EXISTS central_profiles (
    id TEXT PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    pseudonyme TEXT,
    nom TEXT,
    prenom TEXT,
    date_naissance TEXT,
    telephone TEXT,
    numero_voie TEXT,
    rue TEXT,
    code_postal TEXT,
    ville TEXT
);
CREATE TABLE IF NOT EXISTS cog_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE IF NOT EXISTS profile_service_refs (
    profile_id TEXT NOT NULL,
    service_key TEXT NOT NULL,
    ref_id TEXT NOT NULL,
    PRIMARY KEY (profile_id, service_key),
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
);
CREATE TABLE IF NOT EXISTS central_profile_saves (
    id TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL,
    service_key TEXT NOT NULL,
    slot INTEGER NOT NULL DEFAULT 0,
    data BLOB NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    UNIQUE(profile_id, service_key, slot),
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
);
CREATE INDEX IF NOT EXISTS idx_central_profile_saves_profile_service
    ON central_profile_saves(profile_id, service_key);
CREATE TABLE IF NOT EXISTS connect_credentials (
    profile_id TEXT PRIMARY KEY,
    password_hash TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
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

/// Lignes héritées : base ancienne (sans `is_admin`…), une sauvegarde orpheline.
fn lignes() -> Vec<(&'static str, Vec<(&'static str, SqlValue)>)> {
    let d = || t("2025-01-01T00:00:00+00:00");
    vec![
        (
            "central_profiles",
            vec![
                ("id", t("pr1")),
                ("email", t("a@exemple.fr")),
                (
                    "password_hash",
                    t("$argon2id$v=19$m=19456,t=2,p=1$c2VsMTIzNDU2Nzg$aGFzaA"),
                ),
                ("created_at", d()),
                ("updated_at", d()),
                ("pseudonyme", t("Alice")),
                ("nom", N),
                ("prenom", t("")),
                ("date_naissance", N),
                ("telephone", N),
                ("numero_voie", N),
                ("rue", N),
                ("code_postal", N),
                ("ville", t("Lyon")),
            ],
        ),
        (
            "central_profiles",
            vec![
                ("id", t("pr2")),
                ("email", t("b@exemple.fr")),
                (
                    "password_hash",
                    t("5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8"),
                ),
                ("created_at", d()),
                ("updated_at", d()),
            ],
        ),
        (
            "cog_meta",
            vec![("key", t("cog_virgin")), ("value", t("0"))],
        ),
        (
            "cog_meta",
            vec![("key", t("current_profile_id")), ("value", t("pr1"))],
        ),
        (
            "profile_service_refs",
            vec![
                ("profile_id", t("pr1")),
                ("service_key", t("jaykoa")),
                ("ref_id", t("ref-1")),
            ],
        ),
        (
            "central_profile_saves",
            vec![
                ("id", t("sv1")),
                ("profile_id", t("pr1")),
                ("service_key", t("jaykoa")),
                ("slot", i(0)),
                ("data", SqlValue::Blob(vec![0, 1, 2, 255, 254])),
                ("created_at", d()),
                ("updated_at", d()),
            ],
        ),
        (
            "central_profile_saves",
            vec![
                ("id", t("sv-orpheline")),
                ("profile_id", t("fantome")),
                ("service_key", t("jaykoa")),
                ("slot", i(1)),
                ("data", SqlValue::Blob(vec![9])),
                ("created_at", d()),
                ("updated_at", d()),
            ],
        ),
        (
            "connect_credentials",
            vec![
                ("profile_id", t("pr1")),
                ("password_hash", t("$argon2id$connect")),
                ("updated_at", d()),
            ],
        ),
    ]
}

#[test]
fn import_base_heritee_central() {
    let lignes = lignes();
    let (_tmp, addr, creds, cred, base) = preparer("miyukini-central", "central.db", &lignes);
    let octets = std::fs::read(&base).expect("lecture");

    // Avant import : `open` refuse.
    assert!(CentralAuthDb::open(&base).is_err(), "garde de bascule");

    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("runtime");
    rt.block_on(async {
        let client = KindMotherClient::connect(&addr, &cred, "central")
            .await
            .expect("client");
        let rapport = importer_base_heritee(&base, &client).await.expect("import");
        println!("{rapport}");
        assert_eq!(rapport.total_orphelines(), 1);
        let attendues: Vec<_> = lignes
            .iter()
            .filter(|(_, l)| l[0].1 != t("sv-orpheline"))
            .cloned()
            .collect();
        let n = verifier_dans_kindmother(&client, &plan(), &attendues, &[])
            .await
            .expect("lignes identiques");
        assert_eq!(n, lignes.len() - 1);
        // Colonnes absentes de la base ancienne : valeur par défaut KindMother.
        let p = client
            .select(kindmother_client::TypedSelect::from("central_profiles").filter("id", "pr2"))
            .await
            .expect("profil");
        assert_eq!(p[0]["is_admin"], 0);
        assert!(p[0]["genre"].is_null());

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
    let db = CentralAuthDb::open(&base).expect("open après import");
    assert_eq!(
        db.get_current_profile_id().expect("session"),
        Some("pr1".to_string())
    );
    assert!(!db.is_cog_virgin().expect("vierge"));
    assert_eq!(db.list_profiles().expect("profils").len(), 2);
    let save = db
        .get_profile_save("sv1")
        .expect("sauvegarde")
        .expect("présente");
    assert_eq!(save.data, vec![0, 1, 2, 255, 254]);
}
