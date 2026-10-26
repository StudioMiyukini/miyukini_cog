//! Sauvegardes de KindMother (A-022 §5) : conservation 30 jours puis effacement sûr,
//! restauration qui réapplique les oublis, sauvegardes chiffrées.
//!
//! @id     kindmother_service_tests_sauvegardes_a022
//! @do     verifier_conservation_restauration_chiffrement_sauvegardes
//! @role   test
//! @layer  core
//! @human  kindmother-service — une sauvegarde de 31 jours est purgée (contenu réécrit,
//!         introuvable), une de 29 jours gardée ; restaurer une sauvegarde qui contient une
//!         donnée oubliée depuis ne la rend jamais visible ; journal altéré ⇒ restauration
//!         refusée ; sauvegarde illisible sans la clé de la base.

#![cfg(feature = "db-encryption")]

mod commun;

use std::path::{Path, PathBuf};

use chrono::{Duration, Utc};
use commun::*;
use kindmother::intent::{IntentState, SchemaMigration, TypedSelect, TypedWriteIntent};
use kindmother_service::audit::{AuditKind, AUDIT_DIR, AUDIT_FILE};
use kindmother_service::sauvegardes::RESTORE_DIR;
use kindmother_service::{is_plaintext_sqlite, DatabaseKeyStore, EncryptedDatabase, ServiceError};
use rusqlite::Connection;

const OP: &str = "op_test";
const DB: &str = "souvenirs";

fn data_dir(tag: &str) -> PathBuf {
    std::env::temp_dir().join(format!("km-sauvegardes-{tag}-{}", uuid::Uuid::new_v4()))
}

fn intention(mut i: TypedWriteIntent) -> TypedWriteIntent {
    i.intent_id = uuid::Uuid::new_v4().to_string();
    i.operator_id = OP.to_string();
    i.database = DB.to_string();
    i
}

fn migrations(jusqu_a: usize) -> Vec<SchemaMigration> {
    vec![
        SchemaMigration::new(1, "souvenirs", &["CREATE TABLE souvenirs (id TEXT PRIMARY KEY, texte TEXT NOT NULL)"]),
        SchemaMigration::new(2, "note", &["ALTER TABLE souvenirs ADD COLUMN note TEXT"]),
    ]
    .into_iter()
    .take(jusqu_a)
    .collect()
}

async fn ecrire(db: &EncryptedDatabase, intent: TypedWriteIntent) {
    let issues = db.submit_intents(DB, OP, vec![intention(intent)], vec![]).await;
    assert_eq!(issues[0].state, IntentState::Appliquee, "{:?}", issues[0].reason);
}

async fn ids(db: &EncryptedDatabase) -> Vec<String> {
    let mut ids: Vec<String> = db
        .select(DB, OP, &TypedSelect::from("souvenirs").columns(["id"]))
        .await
        .expect("select")
        .iter()
        .filter_map(|l| l.get("id").and_then(|v| v.as_str()).map(str::to_string))
        .collect();
    ids.sort();
    ids
}

/// Base v1 avec `a` (témoin à oublier) et `b`, puis migration v2 ⇒ sauvegarde. Retourne
/// le chemin de la sauvegarde.
async fn base_et_sauvegarde(db: &EncryptedDatabase, temoin: &str) -> PathBuf {
    db.apply_migrations(DB, OP, &migrations(1)).await.expect("v1");
    ecrire(db, TypedWriteIntent::insert("souvenirs").value("id", "a").value("texte", temoin)).await;
    ecrire(db, TypedWriteIntent::insert("souvenirs").value("id", "b").value("texte", "garde")).await;
    let rapport = db.apply_migrations(DB, OP, &migrations(2)).await.expect("v2");
    PathBuf::from(rapport.backup.expect("sauvegarde avant migration"))
}

fn nom(path: &Path) -> String {
    path.file_name().and_then(|n| n.to_str()).expect("nom").to_string()
}

fn pragma_cle(dir: &Path, base: &str) -> String {
    let store = DatabaseKeyStore::open(dir).expect("store");
    store.pragma_key(base, true).expect("key").0.to_string()
}

/// Pages d'un fichier déchiffrées sans contrôle d'en-tête (fichier réécrit compris).
fn pages_brutes(cle: &[u8; 32], fichier: &Path) -> Vec<Vec<u8>> {
    std::fs::read(fichier)
        .expect("read")
        .as_chunks::<PAGE>()
        .0
        .iter()
        .enumerate()
        .map(|(i, p)| dechiffrer_page(cle, u32::try_from(i + 1).expect("pgno"), p))
        .collect()
}

#[tokio::test]
async fn sauvegarde_chiffree_illisible_sans_la_cle() {
    let dir = data_dir("chiffrement");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let temoin = format!("TEMOIN-SAUVEGARDE-{}", uuid::Uuid::new_v4());
    let sauvegarde = base_et_sauvegarde(&db, &temoin).await;
    assert!(sauvegarde.starts_with(db.backup_dir()));

    // Pas d'en-tête SQLite en clair, ni le témoin dans les octets bruts.
    assert!(!is_plaintext_sqlite(&sauvegarde), "sauvegarde en clair");
    let brut = std::fs::read(&sauvegarde).expect("read");
    assert!(!brut.windows(temoin.len()).any(|w| w == temoin.as_bytes()));

    // Sans clé, ou avec une autre clé : illisible.
    let sans_cle = Connection::open(&sauvegarde).expect("open");
    assert!(sans_cle.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get::<_, i64>(0)).is_err());
    drop(sans_cle);
    let autre = Connection::open(&sauvegarde).expect("open");
    autre.pragma_update(None, "key", format!("x'{}'", "00".repeat(32))).expect("key");
    assert!(autre.query_row("SELECT count(*) FROM sqlite_master", [], |r| r.get::<_, i64>(0)).is_err());
    drop(autre);

    // Clé de la base (documentée) : lisible ; contrôle positif du scanner.
    let cle = cle_brute(&dir, DB);
    let (f, w) = temoin_present(&cle, &sauvegarde, temoin.as_bytes());
    assert!(f || w, "la clé de la base doit ouvrir la sauvegarde");
    let lecteur = Connection::open(&sauvegarde).expect("open");
    lecteur.pragma_update(None, "key", pragma_cle(&dir, DB)).expect("key");
    let n: i64 = lecteur.query_row("SELECT count(*) FROM souvenirs", [], |r| r.get(0)).expect("count");
    assert_eq!(n, 2);
    drop(lecteur);

    // Inscrite au journal : nom, chiffrement, empreinte (jamais le contenu).
    let entrees = db.journal().entries().expect("journal");
    let inscription = entrees
        .iter()
        .find(|e| e.record.operation == "sauvegarde")
        .expect("sauvegarde journalisée");
    assert_eq!(inscription.record.kind, AuditKind::Migration);
    assert!(inscription.record.detail.contains(&format!("fichier={}", nom(&sauvegarde))));
    assert!(inscription.record.detail.contains("chiffrement=sqlcipher-cle-de-la-base"));
    assert_eq!(db.lister_sauvegardes(DB).expect("liste"), vec![nom(&sauvegarde)]);

    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn sauvegarde_de_31_jours_purgee_de_29_gardee_contenu_introuvable() {
    let dir = data_dir("purge");
    let db = EncryptedDatabase::new(&dir).expect("db");
    assert_eq!(db.retention_sauvegardes().jours(), 30);
    let temoin = format!("TEMOIN-PURGE-{}", uuid::Uuid::new_v4());
    let sauvegarde = base_et_sauvegarde(&db, &temoin).await;
    let cle = cle_brute(&dir, DB);

    // Lien physique : il garde l'accès au contenu du fichier après sa suppression,
    // comme un attaquant qui lirait les blocs.
    let lien = dir.join("lien-vers-la-sauvegarde");
    std::fs::hard_link(&sauvegarde, &lien).expect("hard link");
    let taille = std::fs::metadata(&lien).expect("meta").len();
    assert!(contient(&pages_brutes(&cle, &lien), temoin.as_bytes()), "contrôle positif");

    // 29 jours : gardée.
    let maintenant = Utc::now();
    let r = db.purger_sauvegardes_au(maintenant + Duration::days(29)).expect("purge 29");
    assert!(r.purgees.is_empty(), "{r:?}");
    assert_eq!(r.conservees, 1);
    assert!(sauvegarde.exists());

    // 31 jours : purgée, contenu réécrit puis fichier supprimé (annexes comprises).
    let r = db.purger_sauvegardes_au(maintenant + Duration::days(31)).expect("purge 31");
    assert_eq!(r.purgees, vec![nom(&sauvegarde)], "{r:?}");
    assert!(r.echecs.is_empty());
    for suffixe in ["", "-wal", "-shm"] {
        assert!(!PathBuf::from(format!("{}{suffixe}", sauvegarde.display())).exists());
    }
    let reste = std::fs::read(&lien).expect("read");
    assert_eq!(reste.len() as u64, taille, "réécrit sur place");
    assert!(reste.iter().all(|&b| b == 0), "contenu réécrit de zéros");
    assert!(!contient(&pages_brutes(&cle, &lien), temoin.as_bytes()), "témoin introuvable");
    assert!(!reste.windows(temoin.len()).any(|w| w == temoin.as_bytes()));

    // Trace d'audit de la purge, sans contenu ; chaîne intacte.
    db.journal().verify().expect("chain");
    let entrees = db.journal().entries().expect("journal");
    let purge = entrees
        .iter()
        .find(|e| e.record.operation == "purge-sauvegarde")
        .expect("purge journalisée");
    assert_eq!(purge.record.kind, AuditKind::Effacement);
    assert_eq!(purge.record.verdict, "effacee");
    assert_eq!(purge.record.database, DB);
    assert!(purge.record.detail.contains(&nom(&sauvegarde)));
    assert!(purge.record.detail.contains("retention=30 j"));
    assert!(!purge.record.detail.contains(&temoin));
    let journal = dir.join(AUDIT_DIR).join(AUDIT_FILE);
    let (f, w) = temoin_present(&cle_brute(&dir, "_journal"), &journal, temoin.as_bytes());
    assert!(!f && !w, "le journal ne contient pas le témoin");

    // Purge au démarrage : une sauvegarde déjà ancienne est effacée à l'ouverture.
    let ancienne = db
        .backup_dir()
        .join(format!("{DB}.v1-{}.bak", (maintenant - Duration::days(40)).format("%Y%m%dT%H%M%S%.f")));
    std::fs::write(&ancienne, b"copie ancienne").expect("write");
    drop(db);
    let db = EncryptedDatabase::new(&dir).expect("reopen");
    assert!(!ancienne.exists(), "purgée au démarrage");
    assert!(db
        .journal()
        .entries()
        .expect("journal")
        .iter()
        .any(|e| e.record.operation == "purge-sauvegarde" && e.record.detail.contains(&nom(&ancienne))));

    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

/// Base sauvegardée, puis `a` oublié et `c` ajouté après la sauvegarde.
async fn scenario_oubli_apres_sauvegarde(dir: &Path) -> (EncryptedDatabase, PathBuf, String) {
    let db = EncryptedDatabase::new(dir).expect("db");
    let temoin = format!("TEMOIN-RESTAURATION-{}", uuid::Uuid::new_v4());
    let sauvegarde = base_et_sauvegarde(&db, &temoin).await;
    ecrire(&db, TypedWriteIntent::forget("souvenirs").key("id", "a").purpose("oubli_demande")).await;
    ecrire(&db, TypedWriteIntent::insert("souvenirs").value("id", "c").value("texte", "apres")).await;
    assert_eq!(ids(&db).await, vec!["b", "c"]);
    (db, sauvegarde, temoin)
}

#[tokio::test]
async fn restauration_d_une_sauvegarde_contenant_une_donnee_oubliee_depuis() {
    let dir = data_dir("restauration");
    let (db, sauvegarde, temoin) = scenario_oubli_apres_sauvegarde(&dir).await;
    let cle = cle_brute(&dir, DB);
    assert!(contient(&pages_principales(&cle, &sauvegarde), temoin.as_bytes()), "la sauvegarde contient la donnée oubliée");

    let rapport = db.restaurer_sauvegarde(DB, &nom(&sauvegarde), "admin").await.expect("restauration");
    assert!(rapport.inscrite);
    assert_eq!(rapport.oublis_rejoues, 1);
    assert_eq!(rapport.lignes_effacees, 1);

    // Dès l'ouverture : `a` absent (oubli rejoué), `c` absent (postérieur à la sauvegarde).
    assert_eq!(ids(&db).await, vec!["b"]);
    assert_eq!(db.schema_version(DB).await.expect("version"), 1);
    let fichier = db.database_path(DB);
    let (f, w) = temoin_present(&cle, &fichier, temoin.as_bytes());
    assert!(!f && !w, "la donnée oubliée ne doit pas revenir dans la base restaurée");
    // Préparation effacée, aucune copie intermédiaire.
    let restants: Vec<_> = std::fs::read_dir(dir.join(RESTORE_DIR)).expect("dir").collect();
    assert!(restants.is_empty(), "{restants:?}");
    // État remplacé conservé comme sauvegarde (même règle de 30 jours).
    assert!(db.backup_dir().join(&rapport.sauvegarde_avant_restauration).exists());

    // Journal : rejeu et restauration inscrits.
    let entrees = db.journal().entries().expect("journal");
    assert!(entrees.iter().any(|e| e.record.operation == "rejeu" && e.record.kind == AuditKind::Effacement));
    assert!(entrees
        .iter()
        .any(|e| e.record.operation == "restauration" && e.record.verdict == "appliquee"));

    // La base restaurée est utilisable ; une réouverture ne fait pas revenir la donnée.
    ecrire(&db, TypedWriteIntent::insert("souvenirs").value("id", "d").value("texte", "neuf")).await;
    drop(db);
    let db = EncryptedDatabase::new(&dir).expect("reopen");
    assert_eq!(ids(&db).await, vec!["b", "d"]);

    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

fn ouvrir_journal(dir: &Path) -> Connection {
    let c = Connection::open(dir.join(AUDIT_DIR).join(AUDIT_FILE)).expect("journal");
    c.pragma_update(None, "key", pragma_cle(dir, "_journal")).expect("key");
    c
}

async fn refus_sans_effet(db: &EncryptedDatabase, sauvegarde: &Path) {
    let err = db
        .restaurer_sauvegarde(DB, &nom(sauvegarde), "admin")
        .await
        .expect_err("restauration refusée");
    assert!(matches!(err, ServiceError::PermissionDenied(_)), "{err}");
    // Base courante inchangée et disponible.
    assert_eq!(ids(db).await, vec!["b", "c"]);
}

#[tokio::test]
async fn journal_d_effacement_altere_restauration_refusee() {
    let dir = data_dir("journal-altere");
    let (db, sauvegarde, _) = scenario_oubli_apres_sauvegarde(&dir).await;
    let c = ouvrir_journal(&dir);
    c.execute_batch(
        "DROP TRIGGER journal_append_only_update;
         UPDATE journal SET detail = 'altere' WHERE kind = 'effacement';",
    )
    .expect("altération");
    drop(c);
    refus_sans_effet(&db, &sauvegarde).await;
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn registre_des_oublis_altere_restauration_refusee() {
    let dir = data_dir("registre-altere");
    let (db, sauvegarde, _) = scenario_oubli_apres_sauvegarde(&dir).await;
    let c = ouvrir_journal(&dir);
    c.execute_batch(
        "DROP TRIGGER oublis_append_only_update;
         UPDATE oublis SET cle_empreinte = '00';",
    )
    .expect("altération");
    drop(c);
    refus_sans_effet(&db, &sauvegarde).await;

    // Ligne du registre absente : refus aussi.
    let c = ouvrir_journal(&dir);
    c.execute_batch("DROP TRIGGER oublis_append_only_delete; DELETE FROM oublis;").expect("retrait");
    drop(c);
    refus_sans_effet(&db, &sauvegarde).await;
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn sauvegarde_modifiee_ou_etrangere_refusee() {
    let dir = data_dir("sauvegarde-modifiee");
    let (db, sauvegarde, _) = scenario_oubli_apres_sauvegarde(&dir).await;
    // Nom d'une autre base, ou chemin : refusés.
    assert!(db.restaurer_sauvegarde("autre", &nom(&sauvegarde), "admin").await.is_err());
    assert!(db.restaurer_sauvegarde(DB, "../souvenirs.db", "admin").await.is_err());
    // Contenu différent de l'empreinte inscrite.
    let mut octets = std::fs::read(&sauvegarde).expect("read");
    let dernier = octets.len() - 1;
    octets[dernier] ^= 0xff;
    std::fs::write(&sauvegarde, octets).expect("write");
    refus_sans_effet(&db, &sauvegarde).await;
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}
