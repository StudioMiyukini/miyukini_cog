//! Effacement physique des données oubliées (`secure_delete` + point de contrôle WAL).
//!
//! @id     kindmother_service_tests_effacement_physique
//! @do     verifier_effacement_physique_oubli
//! @role   test
//! @layer  core
//! @human  kindmother-service — une valeur témoin oubliée ne doit plus figurer dans les
//!         pages déchiffrées du fichier de base ni du `-wal` ; `secure_delete` est relu
//!         actif sur chaque connexion.
//!
//! Le test joue le rôle d'un attaquant qui détient la clé de la base : il déchiffre
//! lui-même chaque page SQLCipher 4 (AES-256-CBC, IV en fin de page, réserve de 80 octets)
//! du fichier principal et de chaque trame du `-wal`, puis cherche le témoin dans le
//! clair, pages libres et espace libre des pages compris.

#![cfg(feature = "db-encryption")]

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use aes::cipher::{block_padding::NoPadding, BlockDecryptMut, KeyIvInit};
use kindmother::intent::{IntentState, SchemaMigration, TypedWriteIntent};
use kindmother_service::{DatabaseKeyStore, EncryptedDatabase, SqlPolicy};
use rusqlite::Connection;

const OP: &str = "op_test";
const DB: &str = "oubli";

/// Taille de page SQLCipher 4 par défaut.
const PAGE: usize = 4096;
/// Réserve SQLCipher 4 : IV (16) + HMAC-SHA512 (64).
const RESERVE: usize = 80;
/// En-tête du fichier WAL, puis en-tête de chaque trame.
const WAL_HEADER: usize = 32;
const WAL_FRAME_HEADER: usize = 24;

type Dechiffreur = cbc::Decryptor<aes::Aes256>;

fn data_dir(tag: &str) -> PathBuf {
    std::env::temp_dir().join(format!("km-effacement-{tag}-{}", uuid::Uuid::new_v4()))
}

fn wal_path(db: &Path) -> PathBuf {
    PathBuf::from(format!("{}-wal", db.display()))
}

/// Clé AES brute d'une base (`x'<64 hex>'` → 32 octets).
fn cle_brute(dir: &Path, base: &str) -> [u8; 32] {
    let store = DatabaseKeyStore::open(dir).expect("store");
    let (pragma, _) = store.pragma_key(base, true).expect("key");
    cle_depuis_pragma(pragma.as_str())
}

fn cle_depuis_pragma(pragma: &str) -> [u8; 32] {
    let hex = pragma
        .strip_prefix("x'")
        .and_then(|s| s.strip_suffix('\''))
        .expect("raw key");
    assert_eq!(hex.len(), 64, "clé brute SQLCipher attendue");
    let mut out = [0u8; 32];
    for (i, b) in out.iter_mut().enumerate() {
        *b = u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).expect("hex");
    }
    out
}

/// Déchiffre une page SQLCipher (numéro `pgno`, base 1).
fn dechiffrer_page(cle: &[u8; 32], pgno: u32, page: &[u8]) -> Vec<u8> {
    assert_eq!(page.len(), PAGE);
    let debut = if pgno == 1 { 16 } else { 0 };
    let fin = PAGE - RESERVE;
    let iv = &page[fin..fin + 16];
    let mut clair = page[debut..fin].to_vec();
    Dechiffreur::new(cle.into(), iv.into())
        .decrypt_padded_mut::<NoPadding>(&mut clair)
        .expect("decrypt");
    clair
}

/// Toutes les pages déchiffrées du fichier principal (contrôle de la page 1 inclus).
fn pages_principales(cle: &[u8; 32], db: &Path) -> Vec<Vec<u8>> {
    let octets = std::fs::read(db).expect("read db");
    assert_eq!(octets.len() % PAGE, 0, "fichier de base non aligné sur les pages");
    let pages: Vec<Vec<u8>> = octets
        .chunks(PAGE)
        .enumerate()
        .map(|(i, p)| dechiffrer_page(cle, u32::try_from(i + 1).expect("pgno"), p))
        .collect();
    // Contrôle du déchiffrement : en-tête SQLite (octets 16.. du fichier) en clair.
    let p1 = &pages[0];
    assert_eq!(u16::from_be_bytes([p1[0], p1[1]]) as usize, PAGE, "taille de page");
    assert_eq!(p1[4] as usize, RESERVE, "réserve SQLCipher");
    assert_eq!(&p1[5..8], &[64, 32, 32], "fractions de charge utile");
    pages
}

/// Toutes les pages déchiffrées des trames du `-wal` (trames périmées comprises).
fn pages_wal(cle: &[u8; 32], db: &Path) -> Vec<Vec<u8>> {
    let Ok(octets) = std::fs::read(wal_path(db)) else {
        return Vec::new();
    };
    if octets.len() < WAL_HEADER {
        return Vec::new();
    }
    octets[WAL_HEADER..]
        .as_chunks::<{ WAL_FRAME_HEADER + PAGE }>()
        .0
        .iter()
        .map(|trame| {
            let pgno = u32::from_be_bytes([trame[0], trame[1], trame[2], trame[3]]);
            dechiffrer_page(cle, pgno, &trame[WAL_FRAME_HEADER..])
        })
        .collect()
}

fn contient(pages: &[Vec<u8>], temoin: &[u8]) -> bool {
    pages
        .iter()
        .any(|p| p.windows(temoin.len()).any(|w| w == temoin))
}

/// Le témoin figure-t-il dans le clair du fichier principal ou du `-wal` ?
fn temoin_present(cle: &[u8; 32], db: &Path, temoin: &[u8]) -> (bool, bool) {
    (
        contient(&pages_principales(cle, db), temoin),
        contient(&pages_wal(cle, db), temoin),
    )
}

fn intention(mut i: TypedWriteIntent) -> TypedWriteIntent {
    i.intent_id = uuid::Uuid::new_v4().to_string();
    i.operator_id = OP.to_string();
    i.database = DB.to_string();
    i
}

async fn ouvrir_avec_schema(dir: &Path) -> EncryptedDatabase {
    let db = EncryptedDatabase::new(dir).expect("db");
    db.apply_migrations(
        DB,
        OP,
        &[SchemaMigration::new(
            1,
            "souvenirs",
            &["CREATE TABLE souvenirs (id TEXT PRIMARY KEY, texte TEXT NOT NULL)"],
        )],
    )
    .await
    .expect("migration");
    db
}

async fn inserer(db: &EncryptedDatabase, id: &str, texte: &str) {
    let issues = db
        .submit_intents(
            DB,
            OP,
            vec![intention(
                TypedWriteIntent::insert("souvenirs").value("id", id).value("texte", texte),
            )],
            vec![],
        )
        .await;
    assert_eq!(issues[0].state, IntentState::Appliquee, "{:?}", issues[0].reason);
}

/// Témoins : une valeur courte (cellule dans la page) et une longue (pages de débordement).
fn temoins() -> (String, String, String) {
    let court = format!("TEMOIN-OUBLI-{}", uuid::Uuid::new_v4());
    let long = format!("TEMOIN-LONG-{}-", uuid::Uuid::new_v4()).repeat(400); // ~20 Kio
    let garde = format!("TEMOIN-GARDE-{}", uuid::Uuid::new_v4());
    (court, long, garde)
}

#[tokio::test]
async fn oubli_par_intention_efface_le_temoin_du_fichier_et_du_wal() {
    let dir = data_dir("typed");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, long, garde) = temoins();
    inserer(&db, "a", &court).await;
    inserer(&db, "b", &long).await;
    inserer(&db, "c", &garde).await;

    let fichier = db.database_path(DB);
    let cle = cle_brute(&dir, DB);
    // Contrôle positif : avant l'oubli, le témoin est lisible (WAL ou fichier principal).
    let (f, w) = temoin_present(&cle, &fichier, court.as_bytes());
    assert!(f || w, "le scanner doit voir le témoin avant l'oubli");

    // Oubli : suppression + pierre tombale dans le même lot (forme MAIA / Alicia).
    let issues = db
        .submit_intents(
            DB,
            OP,
            vec![
                intention(TypedWriteIntent::delete("souvenirs").key("id", "a").purpose("oubli_demande")),
                intention(TypedWriteIntent::delete("souvenirs").key("id", "b").purpose("oubli_demande")),
            ],
            vec![],
        )
        .await;
    for i in &issues {
        assert_eq!(i.state, IntentState::Appliquee, "{:?}", i.reason);
        assert_eq!(i.rows_affected, 1);
    }

    // Point de contrôle TRUNCATE fait par KindMother : `-wal` vide.
    let wal = std::fs::metadata(wal_path(&fichier)).map(|m| m.len()).unwrap_or(0);
    assert_eq!(wal, 0, "le -wal doit être tronqué après un oubli");
    for temoin in [court.as_bytes(), &long.as_bytes()[..64]] {
        let (f, w) = temoin_present(&cle, &fichier, temoin);
        assert!(!f, "témoin oublié encore présent dans le fichier principal");
        assert!(!w, "témoin oublié encore présent dans le -wal");
    }
    // La donnée conservée reste lisible : le scanner fonctionne toujours.
    let (f, _) = temoin_present(&cle, &fichier, garde.as_bytes());
    assert!(f, "la ligne conservée doit rester dans le fichier principal");

    // Rien dans le journal d'audit non plus (empreintes seulement).
    let audit = dir.join(kindmother_service::audit::AUDIT_DIR).join(kindmother_service::audit::AUDIT_FILE);
    let cle_audit = cle_brute(&dir, "_journal");
    let _ = db.journal().verify().expect("chain");
    let (f, w) = temoin_present(&cle_audit, &audit, court.as_bytes());
    assert!(!f && !w, "le journal d'audit ne doit pas contenir la valeur");

    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn oubli_par_sql_brut_transitoire_efface_aussi() {
    let dir = data_dir("raw");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, _, garde) = temoins();
    inserer(&db, "a", &court).await;
    inserer(&db, "c", &garde).await;

    let (rows, _) = db
        .execute(
            DB,
            OP,
            SqlPolicy::DATA_WRITE,
            "DELETE FROM souvenirs WHERE id = ?1",
            vec!["a".to_string()],
            "oubli",
        )
        .await
        .expect("delete");
    assert_eq!(rows, 1);

    let fichier = db.database_path(DB);
    let cle = cle_brute(&dir, DB);
    let wal = std::fs::metadata(wal_path(&fichier)).map(|m| m.len()).unwrap_or(0);
    assert_eq!(wal, 0);
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));
    assert!(temoin_present(&cle, &fichier, garde.as_bytes()).0);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

/// Contrôle négatif : sans `secure_delete`, le même scanner retrouve le témoin supprimé
/// dans le fichier principal — preuve que le test ci-dessus mesure bien quelque chose.
#[test]
fn controle_sans_secure_delete_le_temoin_survit() {
    let dir = data_dir("controle");
    std::fs::create_dir_all(&dir).expect("dir");
    let store = DatabaseKeyStore::open(&dir).expect("store");
    let (pragma, _) = store.pragma_key("controle", false).expect("key");
    let fichier = dir.join("controle.db");
    let conn = Connection::open(&fichier).expect("open");
    conn.pragma_update(None, "key", pragma.as_str()).expect("key");
    conn.execute_batch(
        "PRAGMA secure_delete = OFF; PRAGMA journal_mode = WAL;
         CREATE TABLE souvenirs (id TEXT PRIMARY KEY, texte TEXT NOT NULL);",
    )
    .expect("init");
    let (court, _, garde) = temoins();
    conn.execute("INSERT INTO souvenirs VALUES ('a', ?1), ('c', ?2)", [&court, &garde])
        .expect("insert");
    conn.execute("DELETE FROM souvenirs WHERE id = 'a'", []).expect("delete");
    conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).expect("ckpt");
    let cle = cle_depuis_pragma(pragma.as_str());
    assert!(
        temoin_present(&cle, &fichier, court.as_bytes()).0,
        "sans secure_delete, la valeur supprimée reste dans la page"
    );
    drop(conn);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn secure_delete_relu_actif_sur_chaque_connexion() {
    let dir = data_dir("pool");
    let db = ouvrir_avec_schema(&dir).await;
    for base in [DB, "seconde"] {
        let rw = db.get_connection(base).await.expect("rw");
        let ro = db.get_ro_connection(base).await.expect("ro");
        for conn in [rw, ro] {
            let conn = conn.lock().await;
            let v: i64 = conn
                .query_row("PRAGMA secure_delete", [], |r| r.get(0))
                .expect("pragma");
            assert_eq!(v, 1, "secure_delete inactif sur une connexion de '{base}'");
        }
    }
    // Réouverture d'une base existante par un nouveau gestionnaire.
    drop(db);
    let db = EncryptedDatabase::new(&dir).expect("reopen");
    let conn = db.get_connection(DB).await.expect("rw");
    let v: i64 = conn
        .lock()
        .await
        .query_row("PRAGMA secure_delete", [], |r| r.get(0))
        .expect("pragma");
    assert_eq!(v, 1);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

// ----------------------------------------------------------------------------------
// Mesure (ordre de grandeur) : `cargo test -p kindmother-service --test
// effacement_physique -- --ignored --nocapture mesure`
// ----------------------------------------------------------------------------------

const N_MESURE: usize = 300;

fn base_brute(dir: &Path, nom: &str, secure_delete: bool) -> Connection {
    let store = DatabaseKeyStore::open(dir).expect("store");
    let (pragma, _) = store.pragma_key(nom, false).expect("key");
    let conn = Connection::open(dir.join(format!("{nom}.db"))).expect("open");
    conn.pragma_update(None, "key", pragma.as_str()).expect("key");
    conn.pragma_update(None, "secure_delete", if secure_delete { "ON" } else { "OFF" })
        .expect("sd");
    conn.execute_batch(
        "PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;
         CREATE TABLE souvenirs (id TEXT PRIMARY KEY, texte TEXT NOT NULL);",
    )
    .expect("init");
    let texte = "x".repeat(300);
    let tx = conn.unchecked_transaction().expect("tx");
    for i in 0..N_MESURE {
        tx.execute("INSERT INTO souvenirs VALUES (?1, ?2)", [i.to_string(), texte.clone()])
            .expect("insert");
    }
    tx.commit().expect("commit");
    conn
}

fn mesurer_brut(dir: &Path, nom: &str, secure_delete: bool, checkpoint: bool) -> Duration {
    let conn = base_brute(dir, nom, secure_delete);
    let t = Instant::now();
    for i in 0..N_MESURE {
        conn.execute("DELETE FROM souvenirs WHERE id = ?1", [i.to_string()])
            .expect("delete");
        if checkpoint {
            conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()))
                .expect("ckpt");
        }
    }
    t.elapsed()
}

#[tokio::test]
#[ignore = "mesure de performance, à lancer à la main"]
async fn mesure_cout_effacement() {
    let dir = data_dir("mesure");
    std::fs::create_dir_all(&dir).expect("dir");
    let par = |d: Duration| d.as_secs_f64() * 1e6 / N_MESURE as f64;

    let off = mesurer_brut(&dir, "m_off", false, false);
    let on = mesurer_brut(&dir, "m_on", true, false);
    let on_ckpt = mesurer_brut(&dir, "m_on_ckpt", true, true);

    // Bout en bout KindMother (journal d'audit synchronous=FULL compris).
    let db = ouvrir_avec_schema(&dir).await;
    let texte = "x".repeat(300);
    for i in 0..N_MESURE {
        inserer(&db, &i.to_string(), &texte).await;
    }
    let t = Instant::now();
    for i in 0..N_MESURE {
        let issues = db
            .submit_intents(
                DB,
                OP,
                vec![intention(TypedWriteIntent::delete("souvenirs").key("id", i.to_string()))],
                vec![],
            )
            .await;
        assert_eq!(issues[0].state, IntentState::Appliquee);
    }
    let km_unitaire = t.elapsed();
    let t = Instant::now();
    for i in 0..N_MESURE {
        inserer(&db, &format!("u{i}"), &texte).await;
    }
    let km_insert = t.elapsed();
    // Un seul lot de suppressions (un seul point de contrôle).
    let lot: Vec<TypedWriteIntent> = (0..N_MESURE.min(kindmother::intent::MAX_INTENTS_PER_BATCH))
        .map(|i| intention(TypedWriteIntent::delete("souvenirs").key("id", format!("u{i}"))))
        .collect();
    let n_lot = lot.len();
    let t = Instant::now();
    let issues = db.submit_intents(DB, OP, lot, vec![]).await;
    let km_lot = t.elapsed();
    assert!(issues.iter().all(|i| i.state == IntentState::Appliquee));

    println!("--- coût d'effacement, {N_MESURE} suppressions d'une ligne (~300 o) ---");
    println!("brut, secure_delete OFF            : {:8.1} µs/suppression", par(off));
    println!("brut, secure_delete ON             : {:8.1} µs/suppression", par(on));
    println!("brut, ON + wal_checkpoint(TRUNCATE): {:8.1} µs/suppression", par(on_ckpt));
    println!("KindMother, 1 suppression par lot  : {:8.1} µs/lot", par(km_unitaire));
    println!("KindMother, 1 insertion par lot    : {:8.1} µs/lot (référence)", par(km_insert));
    println!(
        "KindMother, 1 lot de {n_lot} suppressions : {:8.1} ms au total",
        km_lot.as_secs_f64() * 1e3
    );
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}
