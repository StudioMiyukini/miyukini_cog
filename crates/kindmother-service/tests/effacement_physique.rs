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
// Reprise A-020 des bases antérieures : VACUUM unique, marqueur, fail-closed
// ----------------------------------------------------------------------------------

const ANCIENNE: &str = "ancienne";

/// Base « d'avant A-020 » : `secure_delete = OFF`, témoin supprimé mais encore présent
/// dans les pages déchiffrées. Retourne (fichier, clé, témoin supprimé, témoin gardé).
fn base_anterieure(dir: &Path) -> (PathBuf, [u8; 32], String, String) {
    std::fs::create_dir_all(dir).expect("dir");
    let store = DatabaseKeyStore::open(dir).expect("store");
    let (pragma, _) = store.pragma_key(ANCIENNE, false).expect("key");
    let fichier = dir.join(format!("{ANCIENNE}.db"));
    let conn = Connection::open(&fichier).expect("open");
    conn.pragma_update(None, "key", pragma.as_str()).expect("key");
    conn.execute_batch(
        "PRAGMA secure_delete = OFF; PRAGMA journal_mode = WAL;
         CREATE TABLE _kindmother_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL,
             updated_at TEXT NOT NULL DEFAULT (datetime('now')));
         INSERT INTO _kindmother_meta (key, value) VALUES ('schema_version', '1');
         CREATE TABLE souvenirs (id TEXT PRIMARY KEY, texte TEXT NOT NULL);",
    )
    .expect("init");
    let (court, _, garde) = temoins();
    conn.execute("INSERT INTO souvenirs VALUES ('a', ?1), ('c', ?2)", [&court, &garde])
        .expect("insert");
    conn.execute("DELETE FROM souvenirs WHERE id = 'a'", []).expect("delete");
    conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).expect("ckpt");
    drop(conn);
    let cle = cle_depuis_pragma(pragma.as_str());
    assert!(
        temoin_present(&cle, &fichier, court.as_bytes()).0,
        "contrôle : la base antérieure garde la valeur supprimée dans ses pages"
    );
    (fichier, cle, court, garde)
}

fn vacuums_journalises(db: &EncryptedDatabase) -> Vec<String> {
    db.journal()
        .entries()
        .expect("entries")
        .into_iter()
        .filter(|e| {
            e.record.kind == kindmother_service::audit::AuditKind::Effacement
                && e.record.operation == "secure-vacuum"
                && e.record.database == ANCIENNE
        })
        .map(|e| e.record.verdict)
        .collect()
}

async fn marqueur(db: &EncryptedDatabase) -> Option<String> {
    let conn = db.get_connection(ANCIENNE).await.expect("rw");
    let conn = conn.lock().await;
    conn.query_row(
        "SELECT value FROM _kindmother_meta WHERE key = ?1",
        [kindmother_service::SECURE_VACUUM_KEY],
        |r| r.get(0),
    )
    .ok()
}

#[tokio::test]
async fn reprise_base_anterieure_vacuum_unique_efface_les_pages_liberees() {
    let dir = data_dir("reprise");
    let (fichier, cle, court, garde) = base_anterieure(&dir);

    // Premier démarrage de la nouvelle version : VACUUM de reprise à l'ouverture.
    let db = EncryptedDatabase::new(&dir).expect("db");
    assert_eq!(marqueur(&db).await.as_deref(), Some(kindmother_service::SECURE_VACUUM_DONE));
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));
    assert!(temoin_present(&cle, &fichier, garde.as_bytes()).0, "donnée gardée intacte");
    assert_eq!(vacuums_journalises(&db), vec!["appliquee".to_string()]);
    let lignes = db
        .select(ANCIENNE, OP, &kindmother::intent::TypedSelect::from("souvenirs"))
        .await
        .expect("select");
    assert_eq!(lignes.len(), 1);
    drop(db);

    // Second démarrage : marqueur lu, pas de second VACUUM.
    let db = EncryptedDatabase::new(&dir).expect("db");
    assert_eq!(marqueur(&db).await.as_deref(), Some(kindmother_service::SECURE_VACUUM_DONE));
    assert_eq!(vacuums_journalises(&db).len(), 1, "VACUUM refait au second démarrage");
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn base_neuve_marquee_sans_vacuum() {
    let dir = data_dir("neuve");
    let db = EncryptedDatabase::new(&dir).expect("db");
    let conn = db.get_connection("neuve").await.expect("rw");
    let v: String = conn
        .lock()
        .await
        .query_row(
            "SELECT value FROM _kindmother_meta WHERE key = ?1",
            [kindmother_service::SECURE_VACUUM_KEY],
            |r| r.get(0),
        )
        .expect("marqueur");
    assert_eq!(v, kindmother_service::SECURE_VACUUM_DONE);
    assert!(
        db.journal().entries().expect("entries").iter().all(|e| e.record.operation != "secure-vacuum"),
        "aucun VACUUM pour une base neuve"
    );
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

/// Un lecteur extérieur bloque le point de contrôle qui suit le VACUUM : la base est
/// refusée (message explicite), le marqueur n'est pas posé, la reprise est refaite à
/// l'ouverture suivante.
#[tokio::test]
async fn reprise_incomplete_base_refusee_puis_reprise() {
    let dir = data_dir("reprise-ko");
    let (fichier, cle, court, _) = base_anterieure(&dir);
    let lecteur = Connection::open(&fichier).expect("open");
    let store = DatabaseKeyStore::open(&dir).expect("store");
    let (pragma, _) = store.pragma_key(ANCIENNE, true).expect("key");
    lecteur.pragma_update(None, "key", pragma.as_str()).expect("key");
    lecteur.execute_batch("BEGIN; SELECT count(*) FROM souvenirs;").expect("snapshot");

    let db = EncryptedDatabase::new(&dir).expect("db");
    let err = match db.get_connection(ANCIENNE).await {
        Ok(_) => panic!("base ouverte malgré une reprise incomplète"),
        Err(e) => e.to_string(),
    };
    assert!(err.contains("refused") && err.contains("A-020"), "{err}");
    assert_eq!(vacuums_journalises(&db), vec!["echouee".to_string()]);

    lecteur.execute_batch("COMMIT;").expect("fin");
    drop(lecteur);
    assert_eq!(marqueur(&db).await.as_deref(), Some(kindmother_service::SECURE_VACUUM_DONE));
    assert_eq!(vacuums_journalises(&db), vec!["echouee".to_string(), "appliquee".to_string()]);
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

// ----------------------------------------------------------------------------------
// Oubli typé explicite (`TypedWriteIntent::forget` / `.erasure()`)
// ----------------------------------------------------------------------------------

fn taille_wal(fichier: &Path) -> u64 {
    std::fs::metadata(wal_path(fichier)).map(|m| m.len()).unwrap_or(0)
}

#[tokio::test]
async fn oubli_type_point_de_controle_toujours_et_audit_effacement() {
    let dir = data_dir("forget");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, _, garde) = temoins();
    inserer(&db, "a", &court).await;
    inserer(&db, "c", &garde).await;
    let fichier = db.database_path(DB);
    let cle = cle_brute(&dir, DB);
    assert!(taille_wal(&fichier) > 0, "insertions ordinaires : pas de point de contrôle");

    let oubli = intention(TypedWriteIntent::forget("souvenirs").key("id", "a"));
    let id_oubli = oubli.intent_id.clone();
    let issues = db.submit_intents(DB, OP, vec![oubli], vec![]).await;
    assert_eq!(issues[0].state, IntentState::Appliquee, "{:?}", issues[0].reason);
    assert!(issues[0].archived && issues[0].reason.is_none());
    assert_eq!(issues[0].rows_affected, 1);
    assert_eq!(taille_wal(&fichier), 0);
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));

    // Oubli d'une clé absente : point de contrôle quand même (garantie, pas heuristique).
    inserer(&db, "d", "autre").await;
    assert!(taille_wal(&fichier) > 0);
    let issues = db
        .submit_intents(DB, OP, vec![intention(TypedWriteIntent::forget("souvenirs").key("id", "absent"))], vec![])
        .await;
    assert_eq!(issues[0].state, IntentState::Appliquee);
    assert_eq!(issues[0].rows_affected, 0);
    assert_eq!(taille_wal(&fichier), 0, "un oubli typé est toujours suivi du point de contrôle");

    // Journal : entrée `effacement` par oubli, sans la valeur.
    let entrees = db.journal().entries().expect("entries");
    let eff: Vec<_> = entrees
        .iter()
        .filter(|e| e.record.kind == kindmother_service::audit::AuditKind::Effacement)
        .collect();
    assert_eq!(eff.len(), 2);
    let premiere = &eff[0].record;
    assert_eq!(premiere.intent_id.as_deref(), Some(id_oubli.as_str()));
    assert_eq!(premiere.operation, "delete");
    assert_eq!(premiere.table.as_deref(), Some("souvenirs"));
    assert_eq!(premiere.verdict, "effacee");
    assert!(premiere.detail.starts_with("1 rows;"), "{}", premiere.detail);
    assert!(entrees.iter().all(|e| !e.record.detail.contains(&court)));
    let audit = dir.join(kindmother_service::audit::AUDIT_DIR).join(kindmother_service::audit::AUDIT_FILE);
    let (f, w) = temoin_present(&cle_brute(&dir, "_journal"), &audit, court.as_bytes());
    assert!(!f && !w, "le journal d'audit ne doit pas contenir la valeur");
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

#[tokio::test]
async fn ecrasement_type_destine_a_l_oubli_efface_l_ancienne_valeur() {
    let dir = data_dir("erasure-update");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, _, garde) = temoins();
    inserer(&db, "a", &court).await;
    inserer(&db, "c", &garde).await;
    let fichier = db.database_path(DB);
    let cle = cle_brute(&dir, DB);

    // Mise à jour ordinaire : pas un oubli, pas de point de contrôle (comportement documenté).
    let issues = db
        .submit_intents(
            DB,
            OP,
            vec![intention(TypedWriteIntent::update("souvenirs").key("id", "c").value("texte", garde.as_str()))],
            vec![],
        )
        .await;
    assert_eq!(issues[0].state, IntentState::Appliquee);
    assert!(taille_wal(&fichier) > 0);

    // Caviardage : `Update` + `.erasure()` écrase la valeur à oublier.
    let issues = db
        .submit_intents(
            DB,
            OP,
            vec![intention(
                TypedWriteIntent::update("souvenirs").key("id", "a").value("texte", "[oublie]").erasure(),
            )],
            vec![],
        )
        .await;
    assert_eq!(issues[0].state, IntentState::Appliquee, "{:?}", issues[0].reason);
    assert_eq!(taille_wal(&fichier), 0);
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));
    assert!(temoin_present(&cle, &fichier, garde.as_bytes()).0);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

// ----------------------------------------------------------------------------------
// Chemin SQL brut : décision par SQLite (`total_changes`), pas par le texte SQL
// ----------------------------------------------------------------------------------

async fn brut(db: &EncryptedDatabase, policy: SqlPolicy, sql: &str, params: &[&str]) -> Result<u64, String> {
    db.execute(DB, OP, policy, sql, params.iter().map(|p| (*p).to_string()).collect(), "test")
        .await
        .map(|(rows, _)| rows)
        .map_err(|e| e.to_string())
}

#[tokio::test]
async fn sql_brut_insert_or_replace_et_update_ecrasants_effacent() {
    let dir = data_dir("raw-overwrite");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, _, garde) = temoins();
    let (court2, _, _) = temoins();
    inserer(&db, "a", &court).await;
    inserer(&db, "b", &court2).await;
    inserer(&db, "c", &garde).await;
    let fichier = db.database_path(DB);
    let cle = cle_brute(&dir, DB);

    // `INSERT OR REPLACE` sans le mot DELETE : la ligne `a` est écrasée.
    let rows = brut(&db, SqlPolicy::DATA_WRITE, "INSERT OR REPLACE INTO souvenirs (id, texte) VALUES (?1, ?2)", &["a", "remplace"])
        .await
        .expect("replace");
    assert_eq!(rows, 1);
    assert_eq!(taille_wal(&fichier), 0);
    assert_eq!(temoin_present(&cle, &fichier, court.as_bytes()), (false, false));

    // `UPDATE` qui écrase une valeur.
    brut(&db, SqlPolicy::DATA_WRITE, "UPDATE souvenirs SET texte = ?1 WHERE id = ?2", &["ecrase", "b"])
        .await
        .expect("update");
    assert_eq!(taille_wal(&fichier), 0);
    assert_eq!(temoin_present(&cle, &fichier, court2.as_bytes()), (false, false));

    // Transaction brute (upsert écrasant) : même décision par compteur SQLite.
    let (court3, _, _) = temoins();
    inserer(&db, "e", &court3).await;
    db.transaction(
        DB,
        OP,
        SqlPolicy::DATA_WRITE,
        vec![(
            "INSERT INTO souvenirs (id, texte) VALUES (?1, ?2) ON CONFLICT (id) DO UPDATE SET texte = excluded.texte".to_string(),
            vec!["e".to_string(), "upsert".to_string()],
        )],
        "test",
    )
    .await
    .expect("tx");
    assert_eq!(taille_wal(&fichier), 0);
    assert_eq!(temoin_present(&cle, &fichier, court3.as_bytes()), (false, false));
    assert!(temoin_present(&cle, &fichier, garde.as_bytes()).0);
    drop(db);
    let _ = std::fs::remove_dir_all(dir);
}

/// `DROP TABLE` n'efface rien en douce : il est refusé par l'autorisateur sur toutes les
/// politiques (règle non destructive C-41) ; la table et son contenu restent intacts.
#[tokio::test]
async fn sql_brut_drop_table_refuse() {
    let dir = data_dir("raw-drop");
    let db = ouvrir_avec_schema(&dir).await;
    let (court, _, _) = temoins();
    inserer(&db, "a", &court).await;
    for policy in [SqlPolicy::DATA_WRITE, SqlPolicy::ADMIN_RAW] {
        let err = brut(&db, policy, "DROP TABLE souvenirs", &[]).await.expect_err("drop refusé");
        assert!(err.to_lowercase().contains("denied") || err.contains("not authorized"), "{err}");
    }
    let lignes = db
        .select(DB, OP, &kindmother::intent::TypedSelect::from("souvenirs"))
        .await
        .expect("select");
    assert_eq!(lignes.len(), 1);
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

    // Chemin SQL brut : point de contrôle après toute écriture qui modifie des lignes.
    let t = Instant::now();
    for i in 0..N_MESURE {
        brut(&db, SqlPolicy::DATA_WRITE, "INSERT INTO souvenirs (id, texte) VALUES (?1, ?2)", &[&format!("r{i}"), &texte])
            .await
            .expect("raw insert");
    }
    let km_brut_insert = t.elapsed();
    drop(db);

    // VACUUM de reprise A-020 d'une base antérieure d'environ 16 Mio.
    let dir_v = data_dir("mesure-vacuum");
    let (fichier, _, _, _) = base_anterieure(&dir_v);
    {
        let store = DatabaseKeyStore::open(&dir_v).expect("store");
        let (pragma, _) = store.pragma_key(ANCIENNE, true).expect("key");
        let conn = Connection::open(&fichier).expect("open");
        conn.pragma_update(None, "key", pragma.as_str()).expect("key");
        let bloc = "y".repeat(4000);
        let tx = conn.unchecked_transaction().expect("tx");
        for i in 0..4000 {
            tx.execute("INSERT INTO souvenirs VALUES (?1, ?2)", [format!("v{i}"), bloc.clone()]).expect("ins");
        }
        tx.commit().expect("commit");
        conn.execute("DELETE FROM souvenirs WHERE id LIKE 'v%' AND rowid % 2 = 0", []).expect("del");
        conn.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(())).expect("ckpt");
    }
    let taille = std::fs::metadata(&fichier).map(|m| m.len()).unwrap_or(0);
    let t = Instant::now();
    let db_v = EncryptedDatabase::new(&dir_v).expect("db");
    db_v.get_connection(ANCIENNE).await.expect("reprise");
    let km_vacuum = t.elapsed();
    let apres = std::fs::metadata(&fichier).map(|m| m.len()).unwrap_or(0);
    drop(db_v);
    let _ = std::fs::remove_dir_all(dir_v);

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
    println!("KindMother, INSERT brut (+ckpt)    : {:8.1} µs/appel", par(km_brut_insert));
    println!(
        "Reprise A-020 (ouverture + VACUUM) : {:8.1} ms pour {taille} -> {apres} octets",
        km_vacuum.as_secs_f64() * 1e3
    );
    let _ = std::fs::remove_dir_all(dir);
}
