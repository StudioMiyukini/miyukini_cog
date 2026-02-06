//! Base de données des profils Miyukini COG (Central).
//!
//! Le profil central est la base pour les autres services. Chaque service peut
//! lier le profil à sa table dédiée via `profile_service_refs` (une « colonne »
//! par service : clé de service → id de la ligne dans la table du service).
//! Exemple : sauvegarde de jeu → `service_key = "lord_of_the_castle"`, `ref_id` = id
//! de la ligne dans la table des sauvegardes du service.
//!
//! **Structure de sauvegarde liée au profil Central :**
//! - `central_profile_saves` : sauvegardes par (profile_id, service_key, slot).
//!   `ref_id` dans `profile_service_refs` pointe vers `central_profile_saves.id` (sauvegarde courante).
//! - Un profil peut avoir plusieurs slots par service (slot 0, 1, 2…).

use crate::auth::password::validate_password;
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Mutex;

fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

/// Erreur de la base auth Central.
#[derive(Debug)]
pub struct AuthDbError(pub String);

impl std::fmt::Display for AuthDbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Auth DB: {}", self.0)
    }
}

impl std::error::Error for AuthDbError {}

impl From<rusqlite::Error> for AuthDbError {
    fn from(e: rusqlite::Error) -> Self {
        AuthDbError(e.to_string())
    }
}

/// Ligne de sauvegarde liée au profil Central (données complètes).
#[derive(Debug, Clone)]
pub struct CentralProfileSave {
    pub id: String,
    pub profile_id: String,
    pub service_key: String,
    pub slot: i64,
    pub data: Vec<u8>,
    pub created_at: String,
    pub updated_at: String,
}

/// Ligne allégée pour lister les sauvegardes (sans le BLOB).
#[derive(Debug, Clone)]
pub struct CentralProfileSaveRow {
    pub id: String,
    pub slot: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// Profil Miyukini COG (email = login, champs enrichis modifiables).
#[derive(Debug, Clone)]
pub struct CentralProfile {
    pub id: String,
    pub email: String,
    pub pseudonyme: Option<String>,
    pub nom: Option<String>,
    pub prenom: Option<String>,
    pub date_naissance: Option<String>,
    pub telephone: Option<String>,
    pub numero_voie: Option<String>,
    pub rue: Option<String>,
    pub code_postal: Option<String>,
    pub ville: Option<String>,
}

/// Base des profils Central (SQLite).
pub struct CentralAuthDb {
    conn: Mutex<Connection>,
}

impl CentralAuthDb {
    /// Ouvre ou crée la base SQLite à `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AuthDbError> {
        let conn = Connection::open(path)?;
        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        conn.execute_batch(
            r#"
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
            "#,
        )?;
        self.migrate_add_profile_columns(&conn)?;
        conn.execute_batch(
            r#"
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
            "#,
        )?;
        Ok(())
    }

    /// Ajoute les colonnes profil si absentes (migration pour bases existantes).
    fn migrate_add_profile_columns(&self, conn: &Connection) -> Result<(), AuthDbError> {
        let columns = [
            ("pseudonyme", "TEXT"),
            ("nom", "TEXT"),
            ("prenom", "TEXT"),
            ("date_naissance", "TEXT"),
            ("telephone", "TEXT"),
            ("numero_voie", "TEXT"),
            ("rue", "TEXT"),
            ("code_postal", "TEXT"),
            ("ville", "TEXT"),
        ];
        for (name, typ) in columns {
            let sql = format!("ALTER TABLE central_profiles ADD COLUMN {} {}", name, typ);
            if let Err(e) = conn.execute(&sql, []) {
                if !e.to_string().contains("duplicate column name") {
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }

    /// Connexion : email + mot de passe. Retourne le profil complet si valide.
    pub fn sign_in(&self, email: &str, password: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Ok(None);
        }
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let sql = "SELECT id, email, password_hash, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville FROM central_profiles WHERE email = ?1";
        let mut stmt = conn.prepare(sql)?;
        let row = stmt.query_row(params![email], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
                row.get::<_, Option<String>>(10)?,
                row.get::<_, Option<String>>(11)?,
            ))
        });
        match row {
            Ok((id, email_val, hash, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville)) => {
                if hash == hash_password(password) {
                    Ok(Some(CentralProfile {
                        id,
                        email: email_val,
                        pseudonyme,
                        nom,
                        prenom,
                        date_naissance,
                        telephone,
                        numero_voie,
                        rue,
                        code_postal,
                        ville,
                    }))
                } else {
                    Ok(None)
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Charge un profil par ID (pour édition dans la fenêtre Profil).
    pub fn get_profile(&self, id: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let sql = "SELECT id, email, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville FROM central_profiles WHERE id = ?1";
        let mut stmt = conn.prepare(sql)?;
        let row = stmt.query_row(params![id], |row| {
            Ok(CentralProfile {
                id: row.get::<_, String>(0)?,
                email: row.get::<_, String>(1)?,
                pseudonyme: row.get::<_, Option<String>>(2)?,
                nom: row.get::<_, Option<String>>(3)?,
                prenom: row.get::<_, Option<String>>(4)?,
                date_naissance: row.get::<_, Option<String>>(5)?,
                telephone: row.get::<_, Option<String>>(6)?,
                numero_voie: row.get::<_, Option<String>>(7)?,
                rue: row.get::<_, Option<String>>(8)?,
                code_postal: row.get::<_, Option<String>>(9)?,
                ville: row.get::<_, Option<String>>(10)?,
            })
        });
        match row {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Enregistre les champs modifiables du profil en base.
    pub fn update_profile(&self, profile: &CentralProfile) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let now = chrono::Utc::now().to_rfc3339();
        conn.execute(
            "UPDATE central_profiles SET email = ?1, pseudonyme = ?2, nom = ?3, prenom = ?4, date_naissance = ?5, telephone = ?6, numero_voie = ?7, rue = ?8, code_postal = ?9, ville = ?10, updated_at = ?11 WHERE id = ?12",
            params![
                profile.email,
                profile.pseudonyme,
                profile.nom,
                profile.prenom,
                profile.date_naissance,
                profile.telephone,
                profile.numero_voie,
                profile.rue,
                profile.code_postal,
                profile.ville,
                now,
                profile.id,
            ],
        )?;
        Ok(())
    }

    /// Liste tous les profils (id, email uniquement pour la DB mère).
    pub fn list_profiles(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT id, email FROM central_profiles")?;
        let rows = stmt.query_map([], |row| {
            Ok(CentralProfile {
                id: row.get::<_, String>(0)?,
                email: row.get::<_, String>(1)?,
                pseudonyme: None,
                nom: None,
                prenom: None,
                date_naissance: None,
                telephone: None,
                numero_voie: None,
                rue: None,
                code_postal: None,
                ville: None,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Met à jour l'ID d'un profil (après réattribution par la DB mère en cas de conflit).
    pub fn update_profile_id(&self, old_id: &str, new_id: &str) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let n = conn.execute(
            "UPDATE central_profiles SET id = ?1, updated_at = ?2 WHERE id = ?3",
            params![new_id, chrono::Utc::now().to_rfc3339(), old_id],
        )?;
        if n == 0 {
            return Err(AuthDbError(format!("Profil non trouvé: {}", old_id)));
        }
        Ok(())
    }

    /// Création de compte : email + mot de passe complexe. Retourne le profil créé.
    pub fn sign_up(
        &self,
        email: &str,
        password: &str,
    ) -> Result<CentralProfile, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Err(AuthDbError("L'email est requis.".into()));
        }
        validate_password(password).map_err(|e| AuthDbError(e.to_string()))?;
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password(password);
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO central_profiles (id, email, password_hash, created_at, updated_at, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville) VALUES (?1, ?2, ?3, ?4, ?5, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL)",
            params![id, email, hash, now, now],
        )?;
        Ok(CentralProfile {
            id: id.clone(),
            email: email.to_string(),
            pseudonyme: None,
            nom: None,
            prenom: None,
            date_naissance: None,
            telephone: None,
            numero_voie: None,
            rue: None,
            code_postal: None,
            ville: None,
        })
    }

    /// Retourne l’ID de référence du profil vers la table du service (ex. id de sauvegarde de jeu).
    /// Chaque service utilise une `service_key` unique (ex. `"lord_of_the_castle"`, `"miyuclicker"`).
    pub fn get_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Option<String>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT ref_id FROM profile_service_refs WHERE profile_id = ?1 AND service_key = ?2",
        )?;
        let row = stmt.query_row(params![profile_id, service_key], |row| row.get::<_, String>(0));
        match row {
            Ok(ref_id) => Ok(Some(ref_id)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Lie le profil à une ligne de la table du service (ex. sauvegarde de jeu).
    /// `ref_id` = id de la ligne dans la table dédiée au service. Passer `None` pour supprimer le lien.
    pub fn set_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
        ref_id: Option<&str>,
    ) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        match ref_id {
            Some(id) => {
                conn.execute(
                    "INSERT INTO profile_service_refs (profile_id, service_key, ref_id) VALUES (?1, ?2, ?3) ON CONFLICT(profile_id, service_key) DO UPDATE SET ref_id = excluded.ref_id",
                    params![profile_id, service_key, id],
                )?;
            }
            None => {
                conn.execute(
                    "DELETE FROM profile_service_refs WHERE profile_id = ?1 AND service_key = ?2",
                    params![profile_id, service_key],
                )?;
            }
        }
        Ok(())
    }

    // ---------- Sauvegardes liées au profil (central_profile_saves) ----------

    /// Crée une sauvegarde pour un profil et un service (slot 0 par défaut). Retourne l’id de la ligne.
    pub fn insert_profile_save(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
        data: &[u8],
    ) -> Result<String, AuthDbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO central_profile_saves (id, profile_id, service_key, slot, data, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, profile_id, service_key, slot, data, now, now],
        )?;
        Ok(id)
    }

    /// Charge une sauvegarde par id.
    pub fn get_profile_save(&self, id: &str) -> Result<Option<CentralProfileSave>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, service_key, slot, data, created_at, updated_at FROM central_profile_saves WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(CentralProfileSave {
                id: row.get::<_, String>(0)?,
                profile_id: row.get::<_, String>(1)?,
                service_key: row.get::<_, String>(2)?,
                slot: row.get::<_, i64>(3)?,
                data: row.get::<_, Vec<u8>>(4)?,
                created_at: row.get::<_, String>(5)?,
                updated_at: row.get::<_, String>(6)?,
            })
        });
        match row {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Charge une sauvegarde par (profile_id, service_key, slot).
    pub fn get_profile_save_by_slot(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
    ) -> Result<Option<CentralProfileSave>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, service_key, slot, data, created_at, updated_at FROM central_profile_saves WHERE profile_id = ?1 AND service_key = ?2 AND slot = ?3",
        )?;
        let row = stmt.query_row(params![profile_id, service_key, slot], |row| {
            Ok(CentralProfileSave {
                id: row.get::<_, String>(0)?,
                profile_id: row.get::<_, String>(1)?,
                service_key: row.get::<_, String>(2)?,
                slot: row.get::<_, i64>(3)?,
                data: row.get::<_, Vec<u8>>(4)?,
                created_at: row.get::<_, String>(5)?,
                updated_at: row.get::<_, String>(6)?,
            })
        });
        match row {
            Ok(s) => Ok(Some(s)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Met à jour le blob et updated_at d’une sauvegarde existante.
    pub fn update_profile_save(&self, id: &str, data: &[u8]) -> Result<(), AuthDbError> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let n = conn.execute(
            "UPDATE central_profile_saves SET data = ?1, updated_at = ?2 WHERE id = ?3",
            params![data, now, id],
        )?;
        if n == 0 {
            return Err(AuthDbError(format!("Sauvegarde non trouvée: {}", id)));
        }
        Ok(())
    }

    /// Liste les sauvegardes (sans BLOB) pour un profil et un service.
    pub fn list_profile_saves(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Vec<CentralProfileSaveRow>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, slot, created_at, updated_at FROM central_profile_saves WHERE profile_id = ?1 AND service_key = ?2 ORDER BY slot",
        )?;
        let rows = stmt.query_map(params![profile_id, service_key], |row| {
            Ok(CentralProfileSaveRow {
                id: row.get::<_, String>(0)?,
                slot: row.get::<_, i64>(1)?,
                created_at: row.get::<_, String>(2)?,
                updated_at: row.get::<_, String>(3)?,
            })
        })?;
        let mut out = Vec::new();
        for row in rows {
            out.push(row?);
        }
        Ok(out)
    }

    /// Supprime une sauvegarde par id. Ne modifie pas `profile_service_refs` (à faire côté appelant si ref_id pointait ici).
    pub fn delete_profile_save(&self, id: &str) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let n = conn.execute("DELETE FROM central_profile_saves WHERE id = ?1", params![id])?;
        if n == 0 {
            return Err(AuthDbError(format!("Sauvegarde non trouvée: {}", id)));
        }
        Ok(())
    }
}
