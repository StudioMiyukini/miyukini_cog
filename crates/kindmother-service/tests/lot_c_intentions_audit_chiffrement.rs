//! Tests du lot C : intentions typées et cycle de vie (K-13, EX-07), journal d'audit hors
//! de portée des opérateurs (K-08), migrations non destructives (C-41), chiffrement par
//! défaut et clé par base (K-06, K-07).
//!
//! @id     kindmother_service_tests_lot_c
//! @do     verifier_intentions_typees_audit_chiffrement
//! @role   test
//! @layer  core
//! @human  kindmother-service — tests de bout en bout sur le serveur TCP réel (C-60).

use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use kindmother::intent::{IntentState, SchemaMigration, TypedSelect, TypedWriteIntent};
use kindmother_client::{ClientError, KindMotherClient, OperatorCredential};
use kindmother_service::audit::{verify_journal_file, AuditJournal, AuditKind, AUDIT_DIR, AUDIT_FILE};
use kindmother_service::KindMotherServer;
use serde_json::json;
use tokio::net::TcpListener;

async fn start_server() -> (SocketAddr, PathBuf) {
    let dir = std::env::temp_dir().join(format!("km-lotc-{}", uuid::Uuid::new_v4()));
    let server = KindMotherServer::new(&dir).await.expect("server");
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("addr");
    tokio::spawn(server.serve_listener(listener));
    (addr, dir)
}

async fn client(addr: SocketAddr, dir: &Path, operator: &str, db: &str) -> KindMotherClient {
    let cred = OperatorCredential::load_from_dir(operator, dir.join("credentials")).expect("cred");
    KindMotherClient::connect(&addr.to_string(), &cred, db)
        .await
        .expect("connect")
}

/// Clé du journal (le test joue le rôle d'un inspecteur disposant du magasin de clés).
fn journal_key(dir: &Path) -> Option<String> {
    #[cfg(feature = "db-encryption")]
    {
        let store = kindmother_service::DatabaseKeyStore::open(dir).expect("store");
        Some(store.pragma_key("_journal", true).expect("key").0.to_string())
    }
    #[cfg(not(feature = "db-encryption"))]
    {
        let _ = dir;
        None
    }
}

fn journal(dir: &Path) -> AuditJournal {
    AuditJournal::open(dir, journal_key(dir).as_deref()).expect("journal")
}

fn migrations_v1() -> Vec<SchemaMigration> {
    vec![SchemaMigration::new(
        1,
        "notes",
        &[
            "CREATE TABLE IF NOT EXISTS notes (id TEXT PRIMARY KEY, body TEXT NOT NULL, n INTEGER NOT NULL DEFAULT 0)",
            "CREATE INDEX IF NOT EXISTS idx_notes_body ON notes (body)",
        ],
    )]
}

fn migrations_v2() -> Vec<SchemaMigration> {
    let mut m = migrations_v1();
    m.push(SchemaMigration::new(
        2,
        "colonne tag",
        &["ALTER TABLE notes ADD COLUMN tag TEXT"],
    ));
    m
}

#[tokio::test]
async fn intentions_typees_cycle_de_vie_et_audit() {
    let (addr, dir) = start_server().await;
    let jk = client(addr, &dir, "jaykonta", "jaykonta").await;

    assert_eq!(jk.apply_migrations(migrations_v1()).await.expect("v1"), (0, 1));

    // Insertion appliquée et archivée, décision StrongFather jointe.
    let outcome = jk
        .submit(
            TypedWriteIntent::insert("notes")
                .value("id", "n1")
                .value("body", "x'); DELETE FROM notes; --")
                .purpose("create_note")
                .decision("sf-decision:abc123"),
        )
        .await
        .expect("insert");
    assert_eq!(outcome.state, IntentState::Appliquee);
    assert!(outcome.archived);
    assert_eq!(outcome.rows_affected, 1);
    let insert_id = outcome.intent_id.clone();

    // Mise à jour avec incrément, upsert, lecture typée.
    jk.submit(TypedWriteIntent::update("notes").key("id", "n1").increment("n", 5_i64))
        .await
        .expect("update");
    jk.submit(TypedWriteIntent::upsert("notes").key("id", "n2").value("body", "b"))
        .await
        .expect("upsert insert");
    jk.submit(TypedWriteIntent::upsert("notes").key("id", "n2").value("body", "c"))
        .await
        .expect("upsert update");
    let rows = jk
        .select(TypedSelect::from("notes").columns(["id", "body", "n"]).order_by("id", false))
        .await
        .expect("select");
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["n"], json!(5));
    assert_eq!(rows[0]["body"], json!("x'); DELETE FROM notes; --"));
    assert_eq!(rows[1]["body"], json!("c"));

    // Suppression bornée par la clé.
    let o = jk
        .submit(TypedWriteIntent::delete("notes").key("id", "n2"))
        .await
        .expect("delete");
    assert_eq!(o.rows_affected, 1);

    // Journal : validée puis appliquée pour l'insertion, décision et empreinte présentes.
    let j = journal(&dir);
    let entries = j.entries().expect("entries");
    let mine: Vec<_> = entries
        .iter()
        .filter(|e| e.record.intent_id.as_deref() == Some(insert_id.as_str()))
        .collect();
    assert_eq!(mine.len(), 2);
    assert_eq!(mine[0].record.verdict, "validee");
    assert_eq!(mine[1].record.verdict, "appliquee");
    assert_eq!(mine[1].record.decision_digest.as_deref(), Some("sf-decision:abc123"));
    assert_eq!(mine[1].record.table.as_deref(), Some("notes"));
    assert_eq!(mine[1].record.params_hash.len(), 64);
    assert!(
        !entries.iter().any(|e| e.record.detail.contains("DELETE FROM notes")),
        "les valeurs ne sont jamais journalisées"
    );
    // Lectures et migrations journalisées.
    assert!(entries.iter().any(|e| e.record.kind == AuditKind::Select));
    assert!(entries.iter().any(|e| e.record.kind == AuditKind::Migration));
    assert!(j.verify().expect("chain") > 0);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn intentions_refusees_journalisees() {
    let (addr, dir) = start_server().await;
    let jk = client(addr, &dir, "jaykonta", "jaykonta").await;
    jk.apply_migrations(migrations_v1()).await.expect("v1");

    let refused = |r: Result<_, ClientError>| matches!(r, Err(ClientError::PermissionDenied(_)));

    // Tables réservées, inconnues ; colonnes inconnues ; suppression sans clé.
    assert!(refused(jk.submit(TypedWriteIntent::insert("_kindmother_meta").value("key", "k").value("value", "v")).await));
    assert!(refused(jk.submit(TypedWriteIntent::insert("sqlite_master").value("name", "x")).await));
    assert!(refused(jk.submit(TypedWriteIntent::insert("absente").value("id", "x")).await));
    assert!(refused(jk.submit(TypedWriteIntent::insert("notes").value("id", "x").value("inconnue", 1_i64)).await));
    assert!(refused(jk.submit(TypedWriteIntent::delete("notes")).await));

    // Base d'un autre opérateur ; lecteur seul (central : Read sur jaykonta).
    assert!(refused(jk.submit(TypedWriteIntent::insert("notes").value("id", "x").value("body", "b").database("jaykoa")).await));
    let central = client(addr, &dir, "miyukini-central", "central").await;
    assert!(refused(
        central
            .submit(TypedWriteIntent::insert("notes").value("id", "x").value("body", "b").database("jaykonta"))
            .await
    ));
    assert!(central.apply_migrations(migrations_v1()).await.is_ok(), "Admin sur sa base");
    let rows = central
        .select_db("jaykonta", TypedSelect::from("notes"))
        .await
        .expect("lecture autorisée");
    assert!(rows.is_empty());

    // Identifiant d'intention réutilisé : refusé, rien n'est écrit.
    let mut intent = TypedWriteIntent::insert("notes").value("id", "r1").value("body", "b");
    intent.intent_id = "intention-unique-1".to_string();
    jk.submit(intent.clone()).await.expect("première");
    let again = jk.submit(intent.value("id", "r2")).await;
    assert!(refused(again), "non-réutilisation");
    let rows = jk.select(TypedSelect::from("notes").filter("id", "r2")).await.expect("select");
    assert!(rows.is_empty());

    // Lot atomique : la seconde intention échoue (clé primaire en double) → rien appliqué.
    let outcomes = jk
        .submit_batch_outcomes(vec![
            TypedWriteIntent::insert("notes").value("id", "b1").value("body", "b"),
            TypedWriteIntent::insert("notes").value("id", "r1").value("body", "doublon"),
        ])
        .await
        .expect("batch");
    assert_eq!(outcomes[0].state, IntentState::Echouee);
    assert_eq!(outcomes[1].state, IntentState::Echouee);
    assert!(outcomes.iter().all(|o| o.archived));
    let rows = jk.select(TypedSelect::from("notes").filter("id", "b1")).await.expect("select");
    assert!(rows.is_empty(), "annulation du lot");

    // Tous les refus sont scellés dans le journal.
    let entries = journal(&dir).entries().expect("entries");
    let refusals = entries.iter().filter(|e| e.record.verdict == "refusee").count();
    assert!(refusals >= 8, "refus journalisés : {refusals}");
    assert!(entries
        .iter()
        .any(|e| e.record.verdict == "refusee" && e.record.operator_id == "miyukini-central"));
    assert!(entries.iter().any(|e| e.record.verdict == "echouee"));
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn migrations_versionnees_non_destructives() {
    let (addr, dir) = start_server().await;
    let jk = client(addr, &dir, "jaykonta", "jaykonta").await;
    assert_eq!(jk.apply_migrations(migrations_v1()).await.expect("v1"), (0, 1));
    jk.submit(TypedWriteIntent::insert("notes").value("id", "a").value("body", "garde"))
        .await
        .expect("data");
    assert_eq!(jk.apply_migrations(migrations_v1()).await.expect("idempotent"), (1, 1));
    assert_eq!(jk.apply_migrations(migrations_v2()).await.expect("v2"), (1, 2));
    jk.submit(TypedWriteIntent::update("notes").key("id", "a").value("tag", "t"))
        .await
        .expect("nouvelle colonne");

    // Sauvegarde avant migration d'une base non vide.
    let backups: Vec<_> = std::fs::read_dir(dir.join("_backups"))
        .expect("backups")
        .filter_map(Result::ok)
        .filter(|e| e.file_name().to_string_lossy().starts_with("jaykonta.v1-"))
        .collect();
    assert!(!backups.is_empty(), "copie de sauvegarde avant v1 → v2");

    // Étapes destructives refusées, version inchangée, données intactes.
    for destructive in [
        "DROP TABLE notes",
        "DROP TABLE IF EXISTS notes",
        "ALTER TABLE notes DROP COLUMN tag",
        "ALTER TABLE notes RENAME TO notes_old",
        "DELETE FROM notes",
        "ATTACH 'x.db' AS y",
        "PRAGMA writable_schema = 1",
    ] {
        let mut m = migrations_v2();
        m.push(SchemaMigration::new(3, "destructive", &[destructive]));
        assert!(
            jk.apply_migrations(m).await.is_err(),
            "migration destructive acceptée : {destructive}"
        );
    }
    // Suite non contiguë et base plus récente que le code : refus.
    assert!(jk.apply_migrations(vec![SchemaMigration::new(2, "trou", &["CREATE TABLE z (a)"])]).await.is_err());
    assert!(jk.apply_migrations(migrations_v1()).await.is_err(), "base en v2, code en v1");

    let rows = jk.select(TypedSelect::from("notes")).await.expect("select");
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["tag"], json!("t"));

    // SQL brut transitoire : création additive permise à l'Admin, jamais de retrait.
    assert!(jk.execute("CREATE TABLE IF NOT EXISTS extra (a TEXT)", Vec::<String>::new(), "s").await.is_ok());
    assert!(jk.execute("DROP TABLE notes", Vec::<String>::new(), "s").await.is_err());
    assert!(jk.execute("ALTER TABLE notes ADD COLUMN z", Vec::<String>::new(), "s").await.is_err());
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn journal_hors_de_portee_des_operateurs() {
    let (addr, dir) = start_server().await;
    let jk = client(addr, &dir, "jaykonta", "jaykonta").await;
    jk.apply_migrations(migrations_v1()).await.expect("v1");
    jk.submit(TypedWriteIntent::insert("notes").value("id", "a").value("body", "b"))
        .await
        .expect("write");
    let before = journal(&dir).verify().expect("chain");

    let journal_path = dir.join(AUDIT_DIR).join(AUDIT_FILE);
    let journal_sql = journal_path.display().to_string();

    // Aucun chemin d'opérateur n'atteint le journal.
    for sql in [
        format!("ATTACH '{journal_sql}' AS j"),
        "SELECT * FROM _kindmother_audit".to_string(),
        "DELETE FROM _kindmother_audit".to_string(),
        "SELECT * FROM journal".to_string(),
    ] {
        assert!(jk.query(&sql, Vec::<String>::new()).await.is_err(), "{sql}");
        assert!(jk.execute(&sql, Vec::<String>::new(), "x").await.is_err(), "{sql}");
    }
    assert!(jk.select(TypedSelect::from("_kindmother_meta")).await.is_err());
    assert!(jk
        .submit(TypedWriteIntent::delete("_kindmother_meta").key("key", "schema_version"))
        .await
        .is_err());
    // Le journal n'est pas adressable comme base (nom réservé).
    assert!(jk.query_db("_audit", "SELECT 1", Vec::<String>::new()).await.is_err());
    assert!(jk.select_db("journal", TypedSelect::from("journal")).await.is_err());

    // Les tentatives ont été journalisées, la chaîne tient.
    let j = journal(&dir);
    let after = j.verify().expect("chain");
    assert!(after > before);
    drop(j);

    // Falsification directe du fichier (attaquant disposant de la clé) : détectée.
    let key = journal_key(&dir);
    {
        let conn = rusqlite::Connection::open(&journal_path).expect("open");
        if let Some(k) = &key {
            conn.pragma_update(None, "key", k.as_str()).expect("key");
        }
        conn.execute_batch(
            "DROP TRIGGER journal_append_only_update;
             UPDATE journal SET verdict = 'appliquee' WHERE verdict = 'refusee';",
        )
        .expect("tamper");
    }
    assert!(verify_journal_file(&journal_path, key.as_deref()).is_err());
    assert!(
        KindMotherServer::new(&dir).await.is_err(),
        "le service refuse de démarrer sur un journal rompu"
    );
    let _ = std::fs::remove_dir_all(dir);
}

#[cfg(feature = "db-encryption")]
#[tokio::test]
async fn bases_chiffrees_par_defaut_et_cle_par_base() {
    use kindmother_service::{is_plaintext_sqlite, SQLITE_PLAINTEXT_HEADER};
    let (addr, dir) = start_server().await;
    let jk = client(addr, &dir, "jaykonta", "jaykonta").await;
    jk.apply_migrations(migrations_v1()).await.expect("v1");
    jk.submit(TypedWriteIntent::insert("notes").value("id", "a").value("body", "secret-visible?"))
        .await
        .expect("write");
    assert!(jk.database_info().await.expect("info").encrypted);

    // En-tête SQLCipher : le fichier ne commence pas par « SQLite format 3 ».
    for path in [
        dir.join("jaykonta.db"),
        dir.join(AUDIT_DIR).join(AUDIT_FILE),
    ] {
        let bytes = std::fs::read(&path).expect("read");
        assert!(bytes.len() >= 16);
        assert_ne!(&bytes[..16], SQLITE_PLAINTEXT_HEADER, "{path:?} en clair");
        assert!(!is_plaintext_sqlite(&path));
        assert!(!bytes.windows(15).any(|w| w == b"secret-visible?"), "contenu en clair");
    }

    // Une clé par base, issue d'un secret propre (K-07).
    assert!(dir.join("keys").join("jaykonta.secret").exists());
    let store = kindmother_service::DatabaseKeyStore::open(&dir).expect("store");
    let (k1, _) = store.pragma_key("jaykonta", true).expect("k1");
    let (k2, _) = store.pragma_key("_journal", true).expect("k2");
    assert_ne!(*k1, *k2);

    // Une base en clair déposée dans le répertoire est refusée (fail-closed).
    {
        let plain = rusqlite::Connection::open(dir.join("jaykoa.db")).expect("plain");
        plain.execute_batch("CREATE TABLE t (a)").expect("t");
    }
    assert!(is_plaintext_sqlite(&dir.join("jaykoa.db")));
    let koa = client(addr, &dir, "jaykoa", "jaykoa").await;
    assert!(koa.select(TypedSelect::from("t")).await.is_err());
    let _ = std::fs::remove_dir_all(dir);
}
