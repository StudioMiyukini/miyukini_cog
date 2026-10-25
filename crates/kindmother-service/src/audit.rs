//! @id     kindmother_service_audit
//! @do     journaliser_hors_portee_des_operateurs
//! @role   securite
//! @layer  core
//! @human  kindmother-service — journal d'audit en ajout seul, chaîné par empreintes, hors des bases opérateurs (K-08).

//! Journal d'audit de KindMother Service (K-08, C-12, EX-47).
//!
//! - **Hors de portée des opérateurs** : fichier `<data_dir>/_audit/journal.db`. Un
//!   opérateur n'adresse une base que par son nom (`<data_dir>/<nom>.db`, nom sans
//!   `/` ni `_` initial) : il ne peut ni l'ouvrir, ni l'attacher (`ATTACH` refusé par
//!   l'autorisateur), ni le lire. Les tables `_kindmother_*` des bases opérateurs ne
//!   portent plus l'audit.
//! - **Ajout seul** : déclencheurs `BEFORE UPDATE` / `BEFORE DELETE` qui annulent
//!   toute modification ; le service n'émet que des `INSERT`.
//! - **Chaîné** : chaque entrée porte `hash = SHA-256(prev_hash ‖ champs encodés
//!   avec longueur)`. Une ligne modifiée, insérée ou retirée au milieu casse la
//!   chaîne ; la chaîne est vérifiée à l'ouverture (refus de démarrer si rompue).
//!   Limite connue : la troncature de la **fin** du journal n'est détectable que par
//!   un ancrage externe de la tête (à faire, voir le rapport du lot C).
//! - **Contenu** : écritures (typées et SQL brut transitoire), lectures, migrations et
//!   refus, avec identifiant d'intention, opérateur, base, opération, table, verdict,
//!   empreinte de décision StrongFather si jointe, empreinte SHA-256 des paramètres.
//!   Les valeurs elles-mêmes ne sont jamais journalisées.
//! - **Étiquettes `publiable`** (EX-47, `kind = etiquette`) : l'état d'une étiquette est
//!   le dernier événement `posee` / `retiree` de la donnée (`params_hash` = empreinte de
//!   la donnée) — dérivé du journal chaîné lui-même, sans table modifiable à part
//!   (voir `crate::etiquettes`).

use std::path::{Path, PathBuf};
use std::sync::Mutex;

use kindmother::intent::push_len_prefixed;
use rusqlite::{params, Connection, OpenFlags, OptionalExtension};
use sha2::{Digest, Sha256};

use crate::errors::ServiceError;

/// Sous-répertoire du journal (inaccessible par nom de base : `_` initial refusé).
pub const AUDIT_DIR: &str = "_audit";

/// Fichier du journal.
pub const AUDIT_FILE: &str = "journal.db";

/// Nom de clé du journal dans le magasin de clés (réservé : `_` initial).
pub const AUDIT_KEY_NAME: &str = "_journal";

/// Empreinte de départ de la chaîne.
pub const GENESIS_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// Nature d'une entrée.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditKind {
    /// Intention d'écriture typée.
    Intent,
    /// Lecture typée.
    Select,
    /// Migration de schéma.
    Migration,
    /// Écriture par SQL brut (chemin transitoire).
    RawWrite,
    /// Lecture par SQL brut (chemin transitoire).
    RawRead,
    /// Étiquette `publiable` : pose, retrait, lecture ou refus (EX-47).
    Etiquette,
    /// Effacement physique (A-020) : intention d'oubli typée appliquée (empreinte de la
    /// clé seulement, jamais la valeur), ou `VACUUM` de reprise d'une base antérieure.
    Effacement,
}

impl AuditKind {
    /// Nom stable.
    pub fn as_str(self) -> &'static str {
        match self {
            AuditKind::Intent => "intent",
            AuditKind::Select => "select",
            AuditKind::Migration => "migration",
            AuditKind::RawWrite => "raw-write",
            AuditKind::RawRead => "raw-read",
            AuditKind::Etiquette => "etiquette",
            AuditKind::Effacement => "effacement",
        }
    }
}

/// Entrée à ajouter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRecord {
    /// Nature.
    pub kind: AuditKind,
    /// Identifiant d'intention (intentions typées).
    pub intent_id: Option<String>,
    /// Opérateur authentifié.
    pub operator_id: String,
    /// Base visée.
    pub database: String,
    /// Opération (`insert`, `select`, `execute`…).
    pub operation: String,
    /// Table visée, si connue.
    pub table: Option<String>,
    /// Verdict (`validee`, `refusee`, `appliquee`, `echouee`, `autorisee`).
    pub verdict: String,
    /// Empreinte de décision StrongFather jointe.
    pub decision_digest: Option<String>,
    /// SHA-256 (hex) des paramètres (et du SQL pour le chemin brut).
    pub params_hash: String,
    /// Détail court (motif de refus, finalité, nombre de lignes).
    pub detail: String,
}

/// Entrée lue du journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditEntry {
    /// Numéro d'ordre (1, 2, …).
    pub seq: i64,
    /// Horodatage RFC 3339.
    pub timestamp: String,
    /// Contenu.
    pub record: AuditRecord,
    /// Empreinte précédente.
    pub prev_hash: String,
    /// Empreinte de l'entrée.
    pub hash: String,
}

/// Empreinte SHA-256 hexadécimale.
pub fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Calcule l'empreinte d'une entrée.
fn entry_hash(seq: i64, timestamp: &str, rec: &AuditRecord, prev_hash: &str) -> String {
    let mut buf = Vec::with_capacity(256);
    push_len_prefixed(&mut buf, prev_hash.as_bytes());
    buf.extend_from_slice(&seq.to_be_bytes());
    for field in [
        timestamp,
        rec.kind.as_str(),
        rec.intent_id.as_deref().unwrap_or("\u{0}none"),
        &rec.operator_id,
        &rec.database,
        &rec.operation,
        rec.table.as_deref().unwrap_or("\u{0}none"),
        &rec.verdict,
        rec.decision_digest.as_deref().unwrap_or("\u{0}none"),
        &rec.params_hash,
        &rec.detail,
    ] {
        push_len_prefixed(&mut buf, field.as_bytes());
    }
    sha256_hex(&buf)
}

fn kind_from_str(s: &str) -> Option<AuditKind> {
    Some(match s {
        "intent" => AuditKind::Intent,
        "select" => AuditKind::Select,
        "migration" => AuditKind::Migration,
        "raw-write" => AuditKind::RawWrite,
        "raw-read" => AuditKind::RawRead,
        "etiquette" => AuditKind::Etiquette,
        "effacement" => AuditKind::Effacement,
        _ => return None,
    })
}

struct Inner {
    conn: Connection,
    head_seq: i64,
    head_hash: String,
}

/// Journal d'audit chaîné, en ajout seul.
pub struct AuditJournal {
    path: PathBuf,
    inner: Mutex<Inner>,
}

impl AuditJournal {
    /// Ouvre (ou crée) le journal dans `data_dir/_audit`, vérifie la chaîne.
    ///
    /// `pragma_key` : clé SQLCipher (`x'…'`) si le chiffrement est actif.
    pub fn open(data_dir: &Path, pragma_key: Option<&str>) -> Result<Self, ServiceError> {
        let dir = data_dir.join(AUDIT_DIR);
        std::fs::create_dir_all(&dir)
            .map_err(|e| ServiceError::Internal(format!("Cannot create audit dir: {e}")))?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&dir, std::fs::Permissions::from_mode(0o700))
                .map_err(|e| ServiceError::Internal(format!("Cannot restrict audit dir: {e}")))?;
        }
        let path = dir.join(AUDIT_FILE);
        let conn = open_journal(&path, pragma_key)?;
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600));
        }
        let (head_seq, head_hash) = verify_chain_on(&conn)?;
        Ok(Self {
            path,
            inner: Mutex::new(Inner {
                conn,
                head_seq,
                head_hash,
            }),
        })
    }

    /// Chemin du fichier du journal.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn lock(&self) -> Result<std::sync::MutexGuard<'_, Inner>, ServiceError> {
        self.inner
            .lock()
            .map_err(|_| ServiceError::Internal("audit journal lock poisoned".to_string()))
    }

    /// Ajoute une entrée ; retourne son empreinte.
    pub fn append(&self, record: &AuditRecord) -> Result<String, ServiceError> {
        let mut inner = self.lock()?;
        append_locked(&mut inner, record)
    }

    /// Ajoute plusieurs entrées dans une même transaction (tout ou rien).
    ///
    /// Si `fresh_intents`, refuse (sans rien écrire) un identifiant d'intention déjà
    /// présent dans le journal ou en double dans le lot (non-réutilisation).
    pub fn append_all(
        &self,
        records: &[AuditRecord],
        fresh_intents: bool,
    ) -> Result<(), ServiceError> {
        let mut inner = self.lock()?;
        if fresh_intents {
            let mut seen = std::collections::HashSet::new();
            for rec in records {
                let Some(id) = rec.intent_id.as_deref() else {
                    continue;
                };
                if !seen.insert(id) || intent_seen(&inner.conn, id)? {
                    return Err(ServiceError::Validation(format!(
                        "intent id already used: {id}"
                    )));
                }
            }
        }
        let (saved_seq, saved_hash) = (inner.head_seq, inner.head_hash.clone());
        inner
            .conn
            .execute_batch("BEGIN IMMEDIATE")
            .map_err(|e| ServiceError::Internal(format!("Audit begin failed: {e}")))?;
        for rec in records {
            if let Err(e) = append_locked(&mut inner, rec) {
                let _ = inner.conn.execute_batch("ROLLBACK");
                inner.head_seq = saved_seq;
                inner.head_hash = saved_hash;
                return Err(e);
            }
        }
        if let Err(e) = inner.conn.execute_batch("COMMIT") {
            let _ = inner.conn.execute_batch("ROLLBACK");
            inner.head_seq = saved_seq;
            inner.head_hash = saved_hash;
            return Err(ServiceError::Internal(format!("Audit commit failed: {e}")));
        }
        Ok(())
    }

    /// Dernier événement **effectif** (pose ou retrait) de l'étiquette de la donnée
    /// d'empreinte `ref_hex` : `(seq, verdict, detail)`. L'état de l'étiquette est
    /// dérivé du journal chaîné lui-même (aucun état modifiable à part, EX-47).
    pub fn dernier_evenement_etiquette(
        &self,
        ref_hex: &str,
    ) -> Result<Option<(i64, String, String)>, ServiceError> {
        let inner = self.lock()?;
        dernier_evenement_etiquette_on(&inner.conn, ref_hex)
    }

    /// Ajoute un événement d'étiquette (pose ou retrait) **atomiquement** avec ses
    /// contrôles : identifiant d'intention jamais vu (anti-rejeu) et horodatage
    /// signé strictement supérieur à celui du dernier événement de la donnée
    /// (`horodatage_de` l'extrait du détail). Retourne le numéro de l'entrée.
    pub fn append_etiquette(
        &self,
        record: &AuditRecord,
        horodatage: u64,
        horodatage_de: fn(&str) -> Option<u64>,
    ) -> Result<i64, ServiceError> {
        let mut inner = self.lock()?;
        if let Some(id) = record.intent_id.as_deref() {
            if intent_seen(&inner.conn, id)? {
                return Err(ServiceError::PermissionDenied(
                    "label intent already used (replay)".to_string(),
                ));
            }
        } else {
            return Err(ServiceError::Validation("label intent id required".to_string()));
        }
        if let Some((_, _, detail)) = dernier_evenement_etiquette_on(&inner.conn, &record.params_hash)? {
            match horodatage_de(&detail) {
                Some(precedent) if horodatage > precedent => {}
                _ => {
                    return Err(ServiceError::PermissionDenied(
                        "label timestamp not after the last label event".to_string(),
                    ))
                }
            }
        }
        append_locked(&mut inner, record)?;
        Ok(inner.head_seq)
    }

    /// Vrai si l'identifiant d'intention figure déjà au journal.
    pub fn intent_used(&self, intent_id: &str) -> Result<bool, ServiceError> {
        let inner = self.lock()?;
        intent_seen(&inner.conn, intent_id)
    }

    /// Vérifie toute la chaîne ; retourne le nombre d'entrées.
    pub fn verify(&self) -> Result<i64, ServiceError> {
        let inner = self.lock()?;
        let (seq, hash) = verify_chain_on(&inner.conn)?;
        if seq != inner.head_seq || hash != inner.head_hash {
            return Err(ServiceError::Internal(
                "audit chain head mismatch (journal altered)".to_string(),
            ));
        }
        Ok(seq)
    }

    /// Entrées du journal, dans l'ordre (lecture réservée au service et aux tests).
    pub fn entries(&self) -> Result<Vec<AuditEntry>, ServiceError> {
        let inner = self.lock()?;
        read_entries(&inner.conn)
    }
}

/// Vérifie la chaîne d'un fichier de journal (outil d'inspection hors service).
pub fn verify_journal_file(path: &Path, pragma_key: Option<&str>) -> Result<i64, ServiceError> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| ServiceError::Internal(format!("Cannot open audit journal: {e}")))?;
    if let Some(key) = pragma_key {
        conn.pragma_update(None, "key", key)
            .map_err(|e| ServiceError::Encryption(format!("Cannot key audit journal: {e}")))?;
    }
    crate::database::enforce_secure_delete(&conn)?;
    verify_chain_on(&conn).map(|(seq, _)| seq)
}

fn open_journal(path: &Path, pragma_key: Option<&str>) -> Result<Connection, ServiceError> {
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE
            | OpenFlags::SQLITE_OPEN_CREATE
            | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(|e| ServiceError::Internal(format!("Cannot open audit journal: {e}")))?;
    if let Some(key) = pragma_key {
        conn.pragma_update(None, "key", key)
            .map_err(|e| ServiceError::Encryption(format!("Cannot key audit journal: {e}")))?;
    }
    // Effacement physique sur toute connexion KindMother (le journal est en ajout seul,
    // le réglage ne coûte rien ; il couvre les pages libérées par SQLite lui-même).
    crate::database::enforce_secure_delete(&conn)?;
    conn.execute_batch(
        r"
        PRAGMA journal_mode = WAL;
        PRAGMA synchronous = FULL;
        PRAGMA busy_timeout = 5000;
        CREATE TABLE IF NOT EXISTS journal (
            seq INTEGER PRIMARY KEY,
            ts TEXT NOT NULL,
            kind TEXT NOT NULL,
            intent_id TEXT,
            operator_id TEXT NOT NULL,
            database_name TEXT NOT NULL,
            operation TEXT NOT NULL,
            table_name TEXT,
            verdict TEXT NOT NULL,
            decision_digest TEXT,
            params_hash TEXT NOT NULL,
            detail TEXT NOT NULL,
            prev_hash TEXT NOT NULL,
            hash TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS journal_intent ON journal (intent_id);
        CREATE INDEX IF NOT EXISTS journal_etiquette ON journal (kind, params_hash, seq);
        CREATE TRIGGER IF NOT EXISTS journal_append_only_update
            BEFORE UPDATE ON journal
            BEGIN SELECT RAISE(ABORT, 'audit journal is append-only'); END;
        CREATE TRIGGER IF NOT EXISTS journal_append_only_delete
            BEFORE DELETE ON journal
            BEGIN SELECT RAISE(ABORT, 'audit journal is append-only'); END;
        ",
    )
    .map_err(|e| ServiceError::Internal(format!("Cannot initialise audit journal: {e}")))?;
    Ok(conn)
}

fn intent_seen(conn: &Connection, intent_id: &str) -> Result<bool, ServiceError> {
    conn.query_row(
        "SELECT 1 FROM journal WHERE intent_id = ?1 LIMIT 1",
        [intent_id],
        |_| Ok(()),
    )
    .optional()
    .map(|r| r.is_some())
    .map_err(|e| ServiceError::Internal(format!("Audit lookup failed: {e}")))
}

/// Verdicts effectifs d'un événement d'étiquette.
pub const VERDICT_ETIQUETTE_POSEE: &str = "posee";
/// Retrait de l'étiquette.
pub const VERDICT_ETIQUETTE_RETIREE: &str = "retiree";

fn dernier_evenement_etiquette_on(
    conn: &Connection,
    ref_hex: &str,
) -> Result<Option<(i64, String, String)>, ServiceError> {
    conn.query_row(
        "SELECT seq, verdict, detail FROM journal
          WHERE kind = 'etiquette' AND params_hash = ?1 AND verdict IN ('posee', 'retiree')
          ORDER BY seq DESC LIMIT 1",
        [ref_hex],
        |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?)),
    )
    .optional()
    .map_err(|e| ServiceError::Internal(format!("Audit lookup failed: {e}")))
}

fn append_locked(inner: &mut Inner, rec: &AuditRecord) -> Result<String, ServiceError> {
    let seq = inner.head_seq + 1;
    let timestamp = chrono::Utc::now().to_rfc3339();
    let hash = entry_hash(seq, &timestamp, rec, &inner.head_hash);
    inner
        .conn
        .execute(
            r"INSERT INTO journal (seq, ts, kind, intent_id, operator_id, database_name,
                operation, table_name, verdict, decision_digest, params_hash, detail,
                prev_hash, hash)
              VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            params![
                seq,
                timestamp,
                rec.kind.as_str(),
                rec.intent_id,
                rec.operator_id,
                rec.database,
                rec.operation,
                rec.table,
                rec.verdict,
                rec.decision_digest,
                rec.params_hash,
                rec.detail,
                inner.head_hash,
                hash,
            ],
        )
        .map_err(|e| ServiceError::Internal(format!("Audit append failed: {e}")))?;
    inner.head_seq = seq;
    inner.head_hash.clone_from(&hash);
    Ok(hash)
}

fn read_entries(conn: &Connection) -> Result<Vec<AuditEntry>, ServiceError> {
    let corrupt = |what: &str| ServiceError::Internal(format!("audit journal corrupt: {what}"));
    let mut stmt = conn
        .prepare(
            r"SELECT seq, ts, kind, intent_id, operator_id, database_name, operation,
                     table_name, verdict, decision_digest, params_hash, detail, prev_hash, hash
              FROM journal ORDER BY seq",
        )
        .map_err(|e| ServiceError::Internal(format!("Audit read failed: {e}")))?;
    let rows = stmt
        .query_map([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
                r.get::<_, Option<String>>(3)?,
                r.get::<_, String>(4)?,
                r.get::<_, String>(5)?,
                r.get::<_, String>(6)?,
                r.get::<_, Option<String>>(7)?,
                r.get::<_, String>(8)?,
                r.get::<_, Option<String>>(9)?,
                r.get::<_, String>(10)?,
                r.get::<_, String>(11)?,
                r.get::<_, String>(12)?,
                r.get::<_, String>(13)?,
            ))
        })
        .map_err(|e| ServiceError::Internal(format!("Audit read failed: {e}")))?;
    let mut out = Vec::new();
    for row in rows {
        let (seq, ts, kind, intent_id, operator_id, database, operation, table, verdict, digest, params_hash, detail, prev_hash, hash) =
            row.map_err(|e| ServiceError::Internal(format!("Audit read failed: {e}")))?;
        out.push(AuditEntry {
            seq,
            timestamp: ts,
            record: AuditRecord {
                kind: kind_from_str(&kind).ok_or_else(|| corrupt("unknown kind"))?,
                intent_id,
                operator_id,
                database,
                operation,
                table,
                verdict,
                decision_digest: digest,
                params_hash,
                detail,
            },
            prev_hash,
            hash,
        });
    }
    Ok(out)
}

/// Vérifie la chaîne : numérotation contiguë depuis 1, liens et empreintes.
fn verify_chain_on(conn: &Connection) -> Result<(i64, String), ServiceError> {
    let mut expected_prev = GENESIS_HASH.to_string();
    let mut expected_seq = 1_i64;
    for entry in read_entries(conn)? {
        if entry.seq != expected_seq {
            return Err(ServiceError::Internal(format!(
                "audit chain broken: expected entry #{expected_seq}, found #{}",
                entry.seq
            )));
        }
        if entry.prev_hash != expected_prev {
            return Err(ServiceError::Internal(format!(
                "audit chain broken at entry #{}: previous hash mismatch",
                entry.seq
            )));
        }
        let recomputed = entry_hash(entry.seq, &entry.timestamp, &entry.record, &entry.prev_hash);
        if recomputed != entry.hash {
            return Err(ServiceError::Internal(format!(
                "audit chain broken at entry #{}: content altered",
                entry.seq
            )));
        }
        expected_prev = entry.hash;
        expected_seq += 1;
    }
    Ok((expected_seq - 1, expected_prev))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("km-audit-{tag}-{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&d).expect("dir");
        d
    }

    fn rec(intent: &str, verdict: &str) -> AuditRecord {
        AuditRecord {
            kind: AuditKind::Intent,
            intent_id: Some(intent.to_string()),
            operator_id: "op".to_string(),
            database: "db".to_string(),
            operation: "insert".to_string(),
            table: Some("t".to_string()),
            verdict: verdict.to_string(),
            decision_digest: None,
            params_hash: sha256_hex(b"p"),
            detail: String::new(),
        }
    }

    #[test]
    fn chaine_valide_et_reouverture() {
        let d = dir("ok");
        let j = AuditJournal::open(&d, None).expect("open");
        j.append(&rec("a", "validee")).expect("a");
        j.append_all(&[rec("b", "validee"), rec("c", "validee")], true)
            .expect("b c");
        assert_eq!(j.verify().expect("verify"), 3);
        drop(j);
        let j = AuditJournal::open(&d, None).expect("reopen");
        assert_eq!(j.verify().expect("verify"), 3);
        assert!(j.intent_used("b").expect("used"));
        assert!(!j.intent_used("z").expect("unused"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn journal_ouvert_avec_secure_delete_actif() {
        let d = dir("secdel");
        let j = AuditJournal::open(&d, None).expect("open");
        let actif: i64 = j
            .lock()
            .expect("lock")
            .conn
            .query_row("PRAGMA secure_delete", [], |r| r.get(0))
            .expect("pragma");
        assert_eq!(actif, 1);
        drop(j);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn identifiant_reutilise_refuse_sans_ecriture() {
        let d = dir("reuse");
        let j = AuditJournal::open(&d, None).expect("open");
        j.append_all(&[rec("a", "validee")], true).expect("a");
        assert!(j.append_all(&[rec("a", "validee")], true).is_err());
        assert!(j
            .append_all(&[rec("x", "validee"), rec("x", "validee")], true)
            .is_err());
        assert_eq!(j.verify().expect("verify"), 1);
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn ajout_seul_par_declencheurs() {
        let d = dir("ro");
        let j = AuditJournal::open(&d, None).expect("open");
        j.append(&rec("a", "validee")).expect("a");
        let path = j.path().to_path_buf();
        drop(j);
        let conn = Connection::open(&path).expect("conn");
        assert!(conn.execute("DELETE FROM journal", []).is_err());
        assert!(conn
            .execute("UPDATE journal SET verdict = 'appliquee'", [])
            .is_err());
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn falsification_detectee() {
        let d = dir("tamper");
        let j = AuditJournal::open(&d, None).expect("open");
        j.append(&rec("a", "refusee")).expect("a");
        j.append(&rec("b", "appliquee")).expect("b");
        j.append(&rec("c", "appliquee")).expect("c");
        let path = j.path().to_path_buf();
        drop(j);

        // Attaquant disposant du fichier : retire les déclencheurs et réécrit un verdict.
        let conn = Connection::open(&path).expect("conn");
        conn.execute_batch(
            "DROP TRIGGER journal_append_only_update;
             DROP TRIGGER journal_append_only_delete;
             UPDATE journal SET verdict = 'appliquee' WHERE seq = 1;",
        )
        .expect("tamper");
        drop(conn);
        assert!(verify_journal_file(&path, None).is_err(), "contenu altéré");
        assert!(AuditJournal::open(&d, None).is_err(), "refus d'ouvrir un journal rompu");

        // Suppression d'une entrée au milieu : également détectée.
        let d2 = dir("gap");
        let j = AuditJournal::open(&d2, None).expect("open");
        for id in ["a", "b", "c"] {
            j.append(&rec(id, "appliquee")).expect("append");
        }
        let path = j.path().to_path_buf();
        drop(j);
        let conn = Connection::open(&path).expect("conn");
        conn.execute_batch(
            "DROP TRIGGER journal_append_only_delete; DELETE FROM journal WHERE seq = 2;",
        )
        .expect("tamper");
        drop(conn);
        assert!(verify_journal_file(&path, None).is_err(), "trou détecté");
        let _ = std::fs::remove_dir_all(d);
        let _ = std::fs::remove_dir_all(d2);
    }
}
