//! @id     kindmother_heritage_lecture
//! @do     lire_base_heritee_en_lecture_seule
//! @role   donnee
//! @layer  toolkit
//! @human  Lecture seule d'une base SQLite héritée (clé SQLCipher héritée si besoin) ;
//!         le fichier n'est jamais modifié.

//! Lecture d'une base SQLite héritée.
//!
//! - Ouverture `SQLITE_OPEN_READ_ONLY` : SQLite n'écrit rien dans le fichier.
//! - Un journal `-journal` ou `-wal` non vide signale une transaction interrompue ou
//!   des pages non reportées : la lecture seule ne pourrait pas les rejouer sans
//!   écrire ; refus explicite (rouvrir puis fermer proprement l'ancienne version).
//! - Base chiffrée : clé dérivée comme l'ancien chemin `legacy-sqlite` +
//!   `db-encryption` (`KeyDerivation::new(parent)`, nom = radical du fichier), sans
//!   jamais créer le secret d'installation s'il manque.

use std::io::Read;
use std::path::{Path, PathBuf};

use kindmother::intent::{is_valid_identifier, SqlValue};
use rusqlite::types::ValueRef;
use rusqlite::{Connection, OpenFlags};
use sha2::{Digest, Sha256};

use crate::import::ErreurImport;

/// Base héritée ouverte en lecture seule.
pub struct BaseHeritee {
    conn: Connection,
    chemin: PathBuf,
    chiffree: bool,
}

/// Contenu d'une table héritée : colonnes (ordre du schéma) et lignes.
pub(crate) struct TableLue {
    pub colonnes: Vec<String>,
    pub lignes: Vec<Vec<SqlValue>>,
}

impl BaseHeritee {
    /// Ouvre `chemin` en lecture seule (clé SQLCipher héritée si la base est chiffrée).
    pub fn ouvrir(chemin: &Path) -> Result<Self, ErreurImport> {
        if !chemin.is_file() {
            return Err(ErreurImport::Heritee(format!(
                "{} : fichier introuvable",
                chemin.display()
            )));
        }
        refuser_journal_non_vide(chemin)?;
        let conn = ouvrir_lecture_seule(chemin)?;
        if est_lisible(&conn) {
            return Ok(Self {
                conn,
                chiffree: false,
                chemin: chemin.to_path_buf(),
            });
        }
        drop(conn);
        ouvrir_chiffree(chemin)
    }

    /// Vrai si la base a été ouverte avec la clé SQLCipher héritée.
    pub fn est_chiffree(&self) -> bool {
        self.chiffree
    }

    /// Chemin du fichier hérité.
    pub fn chemin(&self) -> &Path {
        &self.chemin
    }

    /// Tables utilisateur (hors catalogue `sqlite_*`).
    pub fn tables(&self) -> Result<Vec<String>, ErreurImport> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite\\_%' ESCAPE '\\' ORDER BY name",
            )
            .map_err(erreur_sqlite)?;
        let noms = stmt
            .query_map([], |r| r.get::<_, String>(0))
            .map_err(erreur_sqlite)?
            .collect::<Result<Vec<_>, _>>()
            .map_err(erreur_sqlite)?;
        Ok(noms)
    }

    /// Lit toutes les lignes de `table` (valeurs typées, texte UTF-8 exigé).
    pub(crate) fn lire_table(&self, table: &str) -> Result<TableLue, ErreurImport> {
        if !is_valid_identifier(table) {
            return Err(ErreurImport::Plan(format!(
                "nom de table invalide : {table:?}"
            )));
        }
        let mut stmt = self
            .conn
            .prepare(&format!("SELECT * FROM \"{table}\""))
            .map_err(erreur_sqlite)?;
        let colonnes: Vec<String> = stmt
            .column_names()
            .iter()
            .map(|c| (*c).to_string())
            .collect();
        if let Some(c) = colonnes.iter().find(|c| !is_valid_identifier(c)) {
            return Err(ErreurImport::Heritee(format!(
                "colonne {table}.{c:?} : nom non représentable en intention typée"
            )));
        }
        let n = colonnes.len();
        let mut lignes = Vec::new();
        let mut rows = stmt.query([]).map_err(erreur_sqlite)?;
        while let Some(row) = rows.next().map_err(erreur_sqlite)? {
            let mut ligne = Vec::with_capacity(n);
            for (i, colonne) in colonnes.iter().enumerate() {
                let v = match row.get_ref(i).map_err(erreur_sqlite)? {
                    ValueRef::Null => SqlValue::Null,
                    ValueRef::Integer(v) => SqlValue::Integer(v),
                    ValueRef::Real(v) => SqlValue::Real(v),
                    ValueRef::Text(t) => {
                        SqlValue::Text(String::from_utf8(t.to_vec()).map_err(|_| {
                            ErreurImport::Heritee(format!(
                                "{table}.{colonne} : texte non UTF-8, non importable tel quel"
                            ))
                        })?)
                    }
                    ValueRef::Blob(b) => SqlValue::Blob(b.to_vec()),
                };
                ligne.push(v);
            }
            lignes.push(ligne);
        }
        Ok(TableLue { colonnes, lignes })
    }
}

/// Empreinte SHA-256 (hexadécimal) d'un fichier, lu en flux.
pub fn empreinte_sha256(chemin: &Path) -> Result<String, ErreurImport> {
    let mut f = std::fs::File::open(chemin)
        .map_err(|e| ErreurImport::Heritee(format!("{} : {e}", chemin.display())))?;
    let mut h = Sha256::new();
    let mut buf = vec![0_u8; 64 * 1024];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| ErreurImport::Heritee(format!("{} : {e}", chemin.display())))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(h.finalize().iter().map(|b| format!("{b:02x}")).collect())
}

fn erreur_sqlite(e: rusqlite::Error) -> ErreurImport {
    ErreurImport::Heritee(e.to_string())
}

fn ouvrir_lecture_seule(chemin: &Path) -> Result<Connection, ErreurImport> {
    Connection::open_with_flags(
        chemin,
        OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .map_err(erreur_sqlite)
}

/// Vrai si le catalogue est lisible (base en clair, ou clé correcte).
fn est_lisible(conn: &Connection) -> bool {
    conn.query_row("SELECT count(*) FROM sqlite_master", [], |r| {
        r.get::<_, i64>(0)
    })
    .is_ok()
}

/// Refuse un journal de transaction (`-journal`) ou WAL (`-wal`) non vide.
fn refuser_journal_non_vide(chemin: &Path) -> Result<(), ErreurImport> {
    for suffixe in ["-journal", "-wal"] {
        let mut nom = chemin.as_os_str().to_os_string();
        nom.push(suffixe);
        let journal = PathBuf::from(nom);
        if std::fs::metadata(&journal).is_ok_and(|m| m.len() > 0) {
            return Err(ErreurImport::Heritee(format!(
                "{} présent et non vide : transaction interrompue ou pages non reportées. \
                 Ouvrir puis fermer proprement l'ancienne version du service, puis relancer \
                 l'import (la lecture seule ne peut pas rejouer ce journal).",
                journal.display()
            )));
        }
    }
    Ok(())
}

#[cfg(feature = "sqlcipher")]
fn ouvrir_chiffree(chemin: &Path) -> Result<BaseHeritee, ErreurImport> {
    use kindmother_db_key::KeyDerivation;

    // Même dérivation que les anciens chemins `legacy-sqlite` + `db-encryption`.
    let data_dir = chemin.parent().unwrap_or(chemin);
    let secret = data_dir.join(".kindmother_secret");
    if !secret.is_file() {
        return Err(ErreurImport::Heritee(format!(
            "{} illisible en clair (base chiffrée ou fichier non SQLite) et secret \
             d'installation {} absent : clé héritée impossible à dériver",
            chemin.display(),
            secret.display()
        )));
    }
    let nom = chemin.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
        ErreurImport::Heritee(format!("{} : nom de fichier invalide", chemin.display()))
    })?;
    let kd = KeyDerivation::new(data_dir).map_err(|e| ErreurImport::Heritee(e.0))?;
    let cle = kd
        .pragma_key_hex(nom)
        .map_err(|e| ErreurImport::Heritee(e.0))?;
    let conn = ouvrir_lecture_seule(chemin)?;
    conn.pragma_update(None, "key", &cle)
        .map_err(erreur_sqlite)?;
    if !est_lisible(&conn) {
        return Err(ErreurImport::Heritee(format!(
            "{} illisible, en clair comme avec la clé SQLCipher héritée \
             (fichier non SQLite, autre machine ou secret d'installation différent)",
            chemin.display()
        )));
    }
    Ok(BaseHeritee {
        conn,
        chiffree: true,
        chemin: chemin.to_path_buf(),
    })
}

#[cfg(not(feature = "sqlcipher"))]
fn ouvrir_chiffree(chemin: &Path) -> Result<BaseHeritee, ErreurImport> {
    Err(ErreurImport::Heritee(format!(
        "{} illisible en clair : base chiffrée (SQLCipher) ou fichier non SQLite. \
         Utiliser un outil compilé avec la feature `sqlcipher` (défaut de kindmother-import).",
        chemin.display()
    )))
}
