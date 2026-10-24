//! @id     kindmother_heritage_tests_moteur
//! @do     verifier_moteur_import_heritage
//! @role   test
//! @layer  toolkit
//! @human  Moteur d'import sur serveur KindMother éphémère : orphelines, normalisation,
//!         déclencheurs, reprise, idempotence, fichier hérité intact, refus.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use kindmother_client::heritage::{chemin_marqueur_import, refuser_si_base_heritee};
use kindmother_client::{
    KindMotherClient, OperatorCredential, SchemaMigration, SqlValue, TypedSelect, TypedWriteIntent,
};
use kindmother_heritage::{importer, ClefEtrangere, ErreurImport, PlanImport, PlanTable};

/// DDL « hérité » : pas de clé étrangère appliquée (comme les anciens chemins).
const DDL_HERITE: &str = "
CREATE TABLE parents (id TEXT PRIMARY KEY, nom TEXT, total REAL NOT NULL DEFAULT 0, maj TEXT);
CREATE TABLE enfants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id TEXT REFERENCES parents(id),
    data BLOB,
    note TEXT
);
";

/// Schéma KindMother : clés étrangères appliquées, déclencheurs qui modifient le parent.
const DDL_KM: &str = "
CREATE TABLE IF NOT EXISTS parents (id TEXT PRIMARY KEY, nom TEXT, total REAL NOT NULL DEFAULT 0, maj TEXT);
CREATE TABLE IF NOT EXISTS enfants (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    parent_id TEXT REFERENCES parents(id),
    data BLOB,
    note TEXT
);
CREATE TRIGGER IF NOT EXISTS trg_total AFTER INSERT ON enfants WHEN NEW.parent_id IS NOT NULL
BEGIN
    UPDATE parents SET total = total + 1000 WHERE id = NEW.parent_id;
END;
CREATE TRIGGER IF NOT EXISTS trg_maj AFTER UPDATE ON parents
BEGIN
    UPDATE parents SET maj = 'déclencheur' WHERE id = NEW.id;
END;
";

const FK_ENFANTS: &[ClefEtrangere] = &[ClefEtrangere::nullable("parent_id", "parents")];

fn plan() -> PlanImport {
    PlanImport {
        service: "essai",
        operateur: "jaykoa",
        base: "jaykoa",
        migrations: vec![SchemaMigration::new(1, "essai", &[DDL_KM])],
        tables: vec![
            PlanTable {
                table: "parents",
                cle: &["id"],
                cles_etrangeres: &[],
                colonnes_tolerees: &["maj"],
            },
            PlanTable {
                table: "enfants",
                cle: &["id"],
                cles_etrangeres: FK_ENFANTS,
                colonnes_tolerees: &[],
            },
        ],
        tables_ignorees: &[],
    }
}

struct Env {
    _tmp: tempfile::TempDir,
    dir: PathBuf,
    addr: String,
    cred: OperatorCredential,
}

async fn env() -> Env {
    let tmp = tempfile::tempdir().expect("tempdir");
    let (addr, creds) = kindmother_service::spawn_ephemeral(tmp.path().join("km"))
        .await
        .expect("serveur éphémère");
    let cred = OperatorCredential::load_from_dir("jaykoa", &creds).expect("justificatif");
    let dir = tmp.path().join("heritage");
    std::fs::create_dir_all(&dir).expect("dir");
    Env {
        dir,
        addr: addr.to_string(),
        cred,
        _tmp: tmp,
    }
}

fn t(s: &str) -> SqlValue {
    SqlValue::Text(s.to_string())
}

fn base_heritee(chemin: &Path) {
    kindmother_heritage::fabrique::creer_base_heritee(
        chemin,
        DDL_HERITE,
        &[
            (
                "parents",
                vec![
                    ("id", t("p1")),
                    ("nom", t("Un")),
                    ("total", SqlValue::Real(12.5)),
                    ("maj", t("2026-01-01")),
                ],
            ),
            (
                "parents",
                vec![
                    ("id", t("p2")),
                    ("nom", t("")),
                    ("total", SqlValue::Real(0.0)),
                    ("maj", SqlValue::Null),
                ],
            ),
            (
                "enfants",
                vec![
                    ("id", SqlValue::Integer(7)),
                    ("parent_id", t("p1")),
                    ("data", SqlValue::Blob(vec![0, 1, 255])),
                    ("note", SqlValue::Null),
                ],
            ),
            (
                "enfants",
                vec![
                    ("id", SqlValue::Integer(9)),
                    ("parent_id", t("")),
                    ("data", SqlValue::Null),
                    ("note", t("")),
                ],
            ),
            (
                "enfants",
                vec![
                    ("id", SqlValue::Integer(12)),
                    ("parent_id", t("disparu")),
                    ("data", SqlValue::Null),
                    ("note", t("orpheline")),
                ],
            ),
        ],
    )
    .expect("base héritée");
}

async fn lignes(client: &KindMotherClient, table: &str) -> Vec<HashMap<String, serde_json::Value>> {
    client
        .select(TypedSelect::from(table).order_by("id", false))
        .await
        .expect("lecture")
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn import_complet_reprise_et_idempotence() {
    let e = env().await;
    let chemin = e.dir.join("essai.db");
    base_heritee(&chemin);
    let octets = std::fs::read(&chemin).expect("lecture");
    assert!(
        refuser_si_base_heritee(&chemin).is_err(),
        "garde active avant import"
    );

    let client = KindMotherClient::connect(&e.addr, &e.cred, "jaykoa")
        .await
        .expect("client");

    // Reprise : une ligne déjà présente (lot précédent appliqué) n'est pas réécrite.
    client
        .apply_migrations(plan().migrations)
        .await
        .expect("migrations");
    client
        .submit(
            TypedWriteIntent::upsert("parents")
                .key("id", "p2")
                .value("nom", "")
                .value("total", 0.0)
                .value("maj", Option::<String>::None),
        )
        .await
        .expect("lot précédent");

    let rapport = importer(&chemin, &client, &plan()).await.expect("import");
    assert!(!rapport.deja_importe);
    assert!(!rapport.chiffree);
    let p = &rapport.tables[0];
    assert_eq!((p.lues, p.ecrites, p.deja_presentes), (2, 1, 1));
    assert_eq!(p.corrigees, 1, "p1 modifié par le déclencheur puis rétabli");
    assert_eq!(
        p.ecarts_toleres, 1,
        "maj réécrite par le déclencheur de mise à jour"
    );
    let en = &rapport.tables[1];
    assert_eq!(
        (en.lues, en.orphelines, en.normalisees, en.dans_kindmother),
        (3, 1, 1, 2)
    );
    println!("{rapport}");

    // Valeurs identiques (hors colonne tolérée), identifiants conservés.
    let ps = lignes(&client, "parents").await;
    assert_eq!(ps[0]["id"], "p1");
    assert_eq!(ps[0]["total"], 12.5);
    assert_eq!(ps[1]["nom"], "", "chaîne vide conservée");
    assert!(ps[1]["maj"].is_null(), "NULL conservé");
    let es = lignes(&client, "enfants").await;
    assert_eq!(es.len(), 2);
    assert_eq!(es[0]["id"], 7);
    assert_eq!(es[0]["data"], "AAH/");
    assert!(es[0]["note"].is_null());
    assert_eq!(es[1]["id"], 9);
    assert!(es[1]["parent_id"].is_null(), "'' normalisée en NULL");
    assert_eq!(es[1]["note"], "");

    // Marqueur écrit, fichier hérité intact, garde levée.
    let marqueur = std::fs::read_to_string(chemin_marqueur_import(&chemin)).expect("marqueur");
    assert!(marqueur.contains(&format!("sha256: {}", rapport.sha256)));
    assert!(marqueur.contains("table enfants: lues=3 importees=2 orphelines=1"));
    assert_eq!(std::fs::read(&chemin).expect("lecture"), octets);
    assert!(refuser_si_base_heritee(&chemin).is_ok());

    // Relance : no-op.
    let encore = importer(&chemin, &client, &plan()).await.expect("relance");
    assert!(encore.deja_importe);
    assert!(encore.tables.is_empty());
    assert_eq!(lignes(&client, "enfants").await.len(), 2);
    assert_eq!(std::fs::read(&chemin).expect("lecture"), octets);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn refus_sans_marqueur() {
    let e = env().await;
    let client = KindMotherClient::connect(&e.addr, &e.cred, "jaykoa")
        .await
        .expect("client");

    // Table héritée hors plan : refus avant toute écriture.
    let chemin = e.dir.join("inconnue.db");
    kindmother_heritage::fabrique::creer_base_heritee(
        &chemin,
        &format!("{DDL_HERITE} CREATE TABLE mystere (id TEXT PRIMARY KEY);"),
        &[("mystere", vec![("id", t("x"))])],
    )
    .expect("base");
    let err = importer(&chemin, &client, &plan())
        .await
        .expect_err("table inconnue");
    assert!(
        matches!(err, ErreurImport::Heritee(ref m) if m.contains("mystere")),
        "{err}"
    );
    assert!(!chemin_marqueur_import(&chemin).exists());

    // Colonne héritée inconnue de KindMother : refus du service, pas de marqueur.
    let chemin = e.dir.join("colonne.db");
    kindmother_heritage::fabrique::creer_base_heritee(
        &chemin,
        "CREATE TABLE parents (id TEXT PRIMARY KEY, nom TEXT, total REAL, maj TEXT, en_trop TEXT);",
        &[("parents", vec![("id", t("p")), ("en_trop", t("perdu ?"))])],
    )
    .expect("base");
    let err = importer(&chemin, &client, &plan())
        .await
        .expect_err("colonne inconnue");
    assert!(matches!(err, ErreurImport::KindMother(_)), "{err}");
    assert!(!chemin_marqueur_import(&chemin).exists());

    // Fichier non SQLite : refus.
    let chemin = e.dir.join("pas_sqlite.db");
    std::fs::write(
        &chemin,
        b"ceci n'est pas une base SQLite, vraiment pas du tout....",
    )
    .expect("fichier");
    let err = importer(&chemin, &client, &plan())
        .await
        .expect_err("non SQLite");
    assert!(matches!(err, ErreurImport::Heritee(_)), "{err}");

    // Journal non vide : refus.
    let chemin = e.dir.join("journal.db");
    base_heritee(&chemin);
    std::fs::write(e.dir.join("journal.db-journal"), b"x").expect("journal");
    let err = importer(&chemin, &client, &plan())
        .await
        .expect_err("journal");
    assert!(
        matches!(err, ErreurImport::Heritee(ref m) if m.contains("-journal")),
        "{err}"
    );

    // Client sur une autre base : refus.
    let mut autre = plan();
    autre.base = "jaykonta";
    let err = importer(&e.dir.join("essai.db"), &client, &autre)
        .await
        .expect_err("base");
    assert!(matches!(err, ErreurImport::MauvaiseBase { .. }), "{err}");
}

#[cfg(feature = "sqlcipher")]
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn base_heritee_chiffree() {
    let e = env().await;
    let chemin = e.dir.join("chiffree.db");
    {
        // Comme l'ancien chemin `legacy-sqlite` + `db-encryption`.
        let kd = kindmother_db_key::KeyDerivation::new(&e.dir).expect("secret");
        let cle = kd.pragma_key_hex("chiffree").expect("clé");
        let conn = rusqlite::Connection::open(&chemin).expect("open");
        conn.pragma_update(None, "key", &cle).expect("key");
        conn.execute_batch(DDL_HERITE).expect("ddl");
        conn.execute("INSERT INTO parents (id, nom) VALUES ('c', 'chiffré')", [])
            .expect("ligne");
    }
    // Illisible en clair.
    let clair =
        rusqlite::Connection::open_with_flags(&chemin, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .expect("open");
    assert!(clair
        .query_row("SELECT count(*) FROM sqlite_master", [], |r| r
            .get::<_, i64>(0))
        .is_err());
    drop(clair);
    let octets = std::fs::read(&chemin).expect("lecture");

    let client = KindMotherClient::connect(&e.addr, &e.cred, "jaykoa")
        .await
        .expect("client");
    let rapport = importer(&chemin, &client, &plan()).await.expect("import");
    assert!(rapport.chiffree);
    assert_eq!(rapport.tables[0].dans_kindmother, 1);
    assert_eq!(lignes(&client, "parents").await[0]["nom"], "chiffré");
    assert_eq!(std::fs::read(&chemin).expect("lecture"), octets);
}
