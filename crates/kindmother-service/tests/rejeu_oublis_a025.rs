//! Rejeu des oublis fail-closed (A-025, constat de revue « rejeu ouvert en cas d'échec »).
//!
//! @id     kindmother_service_tests_rejeu_oublis_a025
//! @do     verifier_rejeu_oublis_identite_normalisee
//! @role   test
//! @layer  core
//! @human  kindmother-service — l'oubli enregistre l'identité des lignes telle que stockée
//!         par SQLite ; à la restauration, une ligne oubliée présente dans la sauvegarde est
//!         effacée quelle que soit l'écriture de la clé, une ligne non retrouvée sans preuve
//!         de création postérieure fait refuser la restauration.

#![cfg(feature = "db-encryption")]

use std::path::{Path, PathBuf};

use kindmother::intent::{IntentState, SchemaMigration, SqlValue, TypedSelect, TypedWriteIntent};
use kindmother_service::audit::{AUDIT_DIR, AUDIT_FILE};
use kindmother_service::{DatabaseKeyStore, EncryptedDatabase, ServiceError, SqlPolicy};
use rusqlite::Connection;

const OP: &str = "op_test";
const DB: &str = "fiches";

fn data_dir(tag: &str) -> PathBuf {
    std::env::temp_dir().join(format!("km-rejeu-a025-{tag}-{}", uuid::Uuid::new_v4()))
}

fn intention(mut i: TypedWriteIntent) -> TypedWriteIntent {
    i.intent_id = uuid::Uuid::new_v4().to_string();
    i.operator_id = OP.to_string();
    i.database = DB.to_string();
    i
}

async fn ecrire(db: &EncryptedDatabase, intent: TypedWriteIntent) -> u64 {
    let issues = db.submit_intents(DB, OP, vec![intention(intent)], vec![]).await;
    assert_eq!(issues[0].state, IntentState::Appliquee, "{:?}", issues[0].reason);
    issues[0].rows_affected
}

/// Clés de `table` (colonne `cle`), rendues en texte, triées.
async fn cles(db: &EncryptedDatabase, table: &str) -> Vec<String> {
    let mut v: Vec<String> = db
        .select(DB, OP, &TypedSelect::from(table).columns(["cle"]))
        .await
        .expect("select")
        .iter()
        .filter_map(|l| l.get("cle").map(|v| v.as_str().map_or_else(|| v.to_string(), str::to_string)))
        .collect();
    v.sort();
    v
}

/// v1 : `texte (cle TEXT PRIMARY KEY)`, `nombre (cle INTEGER PRIMARY KEY)`,
/// `libre (cle PRIMARY KEY)` (sans affinité), `sans_cle (cle TEXT)`.
fn migrations(jusqu_a: usize) -> Vec<SchemaMigration> {
    vec![
        SchemaMigration::new(
            1,
            "tables",
            &[
                "CREATE TABLE texte (cle TEXT PRIMARY KEY, v TEXT)",
                "CREATE TABLE nombre (cle INTEGER PRIMARY KEY, v TEXT)",
                "CREATE TABLE libre (cle PRIMARY KEY, v TEXT)",
                "CREATE TABLE sans_cle (cle TEXT, v TEXT)",
            ],
        ),
        SchemaMigration::new(2, "note", &["ALTER TABLE texte ADD COLUMN note TEXT"]),
    ]
    .into_iter()
    .take(jusqu_a)
    .collect()
}

/// Base v1 garnie par `garnir`, puis migration v2 ⇒ sauvegarde (nom de fichier).
async fn sauvegarde_apres<F, Fut>(db: &EncryptedDatabase, garnir: F) -> String
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    db.apply_migrations(DB, OP, &migrations(1)).await.expect("v1");
    garnir().await;
    let rapport = db.apply_migrations(DB, OP, &migrations(2)).await.expect("v2");
    let chemin = PathBuf::from(rapport.backup.expect("sauvegarde avant migration"));
    chemin.file_name().and_then(|n| n.to_str()).expect("nom").to_string()
}

async fn refusee(db: &EncryptedDatabase, sauvegarde: &str) -> String {
    let err = db
        .restaurer_sauvegarde(DB, sauvegarde, "admin")
        .await
        .expect_err("restauration refusée");
    assert!(matches!(err, ServiceError::PermissionDenied(_)), "{err}");
    err.to_string()
}

#[tokio::test]
async fn cle_ecrite_autrement_que_stockee_effacee_quand_meme() {
    let dir = data_dir("normalisation");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let sauvegarde = sauvegarde_apres(&db, || async {
        // Colonne TEXT : 7 (entier) est stocké '7'. Colonne INTEGER : 7.
        ecrire(&db, TypedWriteIntent::insert("texte").value("cle", 7_i64).value("v", "oublier")).await;
        ecrire(&db, TypedWriteIntent::insert("texte").value("cle", "garde").value("v", "x")).await;
        ecrire(&db, TypedWriteIntent::insert("nombre").value("cle", 7_i64).value("v", "oublier")).await;
        ecrire(&db, TypedWriteIntent::insert("nombre").value("cle", 8_i64).value("v", "x")).await;
    })
    .await;
    // Clé entière enregistrée contre une valeur TEXT stockée ; clé texte '07' contre un
    // entier stocké (l'ancienne recherche par empreinte de clé ne la retrouvait pas).
    assert_eq!(ecrire(&db, TypedWriteIntent::forget("texte").key("cle", 7_i64)).await, 1);
    assert_eq!(ecrire(&db, TypedWriteIntent::forget("nombre").key("cle", "07")).await, 1);

    let rapport = db.restaurer_sauvegarde(DB, &sauvegarde, "admin").await.expect("restauration");
    assert_eq!(rapport.oublis_rejoues, 2);
    assert_eq!(rapport.lignes_effacees, 2);
    assert_eq!(rapport.lignes_posterieures, 0);
    assert_eq!(cles(&db, "texte").await, vec!["garde"]);
    assert_eq!(cles(&db, "nombre").await, vec!["8"]);
    // Registre : identité normalisée et nombre de lignes, jamais la valeur.
    let c = journal(&dir);
    let (identites, lignes): (String, i64) = c
        .query_row("SELECT identites, lignes FROM oublis WHERE table_name = 'texte'", [], |r| {
            Ok((r.get(0)?, r.get(1)?))
        })
        .expect("registre");
    assert_eq!(lignes, 1);
    assert!(!identites.contains("oublier") && identites.len() > 64, "{identites}");
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn ligne_presente_mais_identite_non_retrouvee_restauration_refusee() {
    let dir = data_dir("non-retrouvee");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let sauvegarde = sauvegarde_apres(&db, || async {
        // Colonne sans affinité : '7' stocké en texte dans la sauvegarde.
        ecrire(&db, TypedWriteIntent::insert("libre").value("cle", "7").value("v", "oublier")).await;
    })
    .await;
    // Après la sauvegarde, la même ligne change de représentation (chemin brut, non
    // journalisé comme création) : 7 entier. L'oubli enregistre l'identité « entier 7 ».
    db.execute(DB, OP, SqlPolicy::DATA_WRITE, "UPDATE libre SET cle = CAST(cle AS INTEGER)", vec![], "test")
        .await
        .expect("update brut");
    assert_eq!(ecrire(&db, TypedWriteIntent::forget("libre").key("cle", 7_i64)).await, 1);

    // La sauvegarde contient la ligne ('7' texte) mais pas l'identité enregistrée, et rien
    // ne prouve une création postérieure : refus, base courante inchangée.
    let err = refusee(&db, &sauvegarde).await;
    assert!(err.contains("without proof"), "{err}");
    assert!(cles(&db, "libre").await.is_empty());
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn ligne_creee_apres_la_sauvegarde_puis_oubliee_restauration_acceptee() {
    let dir = data_dir("posterieure");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let sauvegarde = sauvegarde_apres(&db, || async {
        ecrire(&db, TypedWriteIntent::insert("texte").value("cle", "a").value("v", "x")).await;
    })
    .await;
    // Créées après la sauvegarde (insert, insert à clé auto, upsert), puis oubliées.
    ecrire(&db, TypedWriteIntent::insert("texte").value("cle", "n").value("v", "neuf")).await;
    ecrire(&db, TypedWriteIntent::insert("nombre").value("v", "auto")).await;
    ecrire(&db, TypedWriteIntent::upsert("texte").key("cle", "u").value("v", "neuf")).await;
    assert_eq!(cles(&db, "nombre").await, vec!["1"]);
    ecrire(&db, TypedWriteIntent::forget("texte").key("cle", "n")).await;
    ecrire(&db, TypedWriteIntent::forget("nombre").key("cle", 1_i64)).await;
    ecrire(&db, TypedWriteIntent::forget("texte").key("cle", "u")).await;

    let rapport = db.restaurer_sauvegarde(DB, &sauvegarde, "admin").await.expect("restauration");
    assert_eq!(rapport.oublis_rejoues, 3);
    assert_eq!(rapport.lignes_effacees, 0);
    assert_eq!(rapport.lignes_posterieures, 3);
    assert_eq!(cles(&db, "texte").await, vec!["a"]);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn registre_des_creations_altere_restauration_refusee() {
    let dir = data_dir("creations-alterees");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let sauvegarde = sauvegarde_apres(&db, || async {}).await;
    ecrire(&db, TypedWriteIntent::insert("texte").value("cle", "n").value("v", "neuf")).await;
    ecrire(&db, TypedWriteIntent::forget("texte").key("cle", "n")).await;
    // Création forgée pour une autre identité (non couverte par la chaîne) : refus.
    let c = journal(&dir);
    c.execute_batch(
        "DROP TRIGGER creations_append_only_update;
         UPDATE creations SET identite = '00' || substr(identite, 3);",
    )
    .expect("altération");
    drop(c);
    refusee(&db, &sauvegarde).await;
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn sauvegarde_non_inscrite_absence_jamais_prouvee() {
    let dir = data_dir("non-inscrite");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let sauvegarde = sauvegarde_apres(&db, || async {}).await;
    // Copie déposée sous un autre nom (non inscrite au journal) : sa date est inconnue.
    let autre = format!("{DB}.v1-20260101T000000.bak");
    std::fs::copy(db.backup_dir().join(&sauvegarde), db.backup_dir().join(&autre)).expect("copie");
    ecrire(&db, TypedWriteIntent::insert("texte").value("cle", "n").value("v", "neuf")).await;
    ecrire(&db, TypedWriteIntent::forget("texte").key("cle", "n")).await;
    let err = refusee(&db, &autre).await;
    assert!(err.contains("without proof"), "{err}");
    // La copie inscrite, elle, est restaurée (création prouvée postérieure).
    db.restaurer_sauvegarde(DB, &sauvegarde, "admin").await.expect("restauration");
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn oubli_sur_table_sans_cle_declaree_refuse() {
    let dir = data_dir("sans-cle");
    let db = EncryptedDatabase::new(&dir).expect("db");
    db.apply_migrations(DB, OP, &migrations(1)).await.expect("v1");
    ecrire(&db, TypedWriteIntent::insert("sans_cle").value("cle", "a").value("v", "x")).await;
    let issues = db
        .submit_intents(DB, OP, vec![intention(TypedWriteIntent::forget("sans_cle").key("cle", "a"))], vec![])
        .await;
    assert_eq!(issues[0].state, IntentState::Refusee);
    assert!(issues[0].reason.as_deref().is_some_and(|r| r.contains("A-025")), "{:?}", issues[0].reason);
    // Un `Delete` ordinaire reste possible (il ne promet pas de rejeu).
    let ordinaire = TypedWriteIntent::delete("sans_cle").key("cle", SqlValue::Text("a".into()));
    assert_eq!(ecrire(&db, ordinaire).await, 1);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

fn journal(dir: &Path) -> Connection {
    let store = DatabaseKeyStore::open(dir).expect("store");
    let c = Connection::open(dir.join(AUDIT_DIR).join(AUDIT_FILE)).expect("journal");
    c.pragma_update(None, "key", store.pragma_key("_journal", true).expect("key").0.to_string())
        .expect("key");
    c
}
