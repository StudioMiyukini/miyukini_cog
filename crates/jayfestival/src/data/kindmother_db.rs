//! Base de données fille SQLite JayFestival sous autorité KindMother.
//!
//! Instance Daughter : persistance locale via SQLite, identité KindMother pour gouvernance.

use crate::data::types::{Edition, EditionExposant, Exposant, Organisateur, Profile};
use kindmother::{InstanceIdentity, InstanceType};
use rusqlite::{params, Connection};
use sha2::{Digest, Sha256};
use std::path::Path;
use std::sync::Mutex;

/// Erreur de la base JayFestival (SQLite, sérialisation).
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JayFestival DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError(e.to_string())
    }
}

/// Base de données fille SQLite de JayFestival (KindMother Daughter).
pub struct JayFestivalDb {
    conn: Mutex<Connection>,
    /// Identité KindMother : instance Fille (base locale).
    pub instance: InstanceIdentity,
}

fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

impl JayFestivalDb {
    /// Ouvre ou crée la base SQLite à `path` et initialise le schéma.
    /// Identité KindMother : Daughter (base fille).
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let conn = Connection::open(path)?;
        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Crée le schéma des tables (profiles, editions, organisateurs, exposants, editions_exposants).
    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                username TEXT,
                user_type TEXT,
                email TEXT UNIQUE,
                avatar_url TEXT,
                theme TEXT,
                password_hash TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS editions (
                id TEXT PRIMARY KEY,
                name TEXT,
                slug TEXT,
                start_date TEXT,
                end_date TEXT,
                location TEXT,
                theme TEXT,
                status TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS organisateurs (
                id TEXT PRIMARY KEY,
                name TEXT,
                slug TEXT,
                region TEXT,
                description TEXT,
                contact_email TEXT,
                website TEXT,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS exposants (
                id TEXT PRIMARY KEY,
                company_name TEXT,
                stand_name TEXT,
                contact_email TEXT,
                contact_phone TEXT,
                adresse TEXT,
                logo_url TEXT,
                site_web TEXT,
                siret TEXT,
                secteur TEXT,
                category TEXT,
                description TEXT,
                visible_repertoire INTEGER,
                created_at TEXT,
                updated_at TEXT
            );
            CREATE TABLE IF NOT EXISTS editions_exposants (
                id TEXT PRIMARY KEY,
                exposant_id TEXT,
                edition_id TEXT,
                is_accepted INTEGER,
                is_validated INTEGER,
                is_paid INTEGER,
                assigned_stand TEXT,
                size_meters REAL,
                created_at TEXT,
                updated_at TEXT,
                FOREIGN KEY (exposant_id) REFERENCES exposants(id),
                FOREIGN KEY (edition_id) REFERENCES editions(id)
            );
            ",
        )?;
        Ok(())
    }

    // --- Profiles (auth) ---

    /// Retourne le profil par email si le mot de passe est valide.
    pub fn profile_by_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<Profile>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, username, user_type, email, avatar_url, theme, password_hash, created_at, updated_at FROM profiles WHERE email = ?1",
        )?;
        let row = stmt.query_row(params![email], |row| {
            Ok(Profile {
                id: row.get(0)?,
                username: row.get(1)?,
                user_type: row.get(2)?,
                email: row.get(3)?,
                avatar_url: row.get(4)?,
                theme: row.get(5)?,
                password_hash: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        });
        match row {
            Ok(profile) => {
                let expected = profile.password_hash.as_deref().unwrap_or("");
                if expected == hash_password(password) {
                    Ok(Some(profile))
                } else {
                    Ok(None)
                }
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Récupère un profil par id.
    pub fn profile_by_id(&self, id: &str) -> Result<Option<Profile>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, username, user_type, email, avatar_url, theme, password_hash, created_at, updated_at FROM profiles WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(Profile {
                id: row.get(0)?,
                username: row.get(1)?,
                user_type: row.get(2)?,
                email: row.get(3)?,
                avatar_url: row.get(4)?,
                theme: row.get(5)?,
                password_hash: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        });
        match row {
            Ok(p) => Ok(Some(p)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Crée un profil (inscription). Retourne l'id généré.
    pub fn profile_create(
        &self,
        email: &str,
        password: &str,
        user_type: &str,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let hash = hash_password(password);
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO profiles (id, email, password_hash, user_type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, email, hash, user_type, now, now],
        )?;
        Ok(id)
    }

    // --- Editions ---

    /// Liste toutes les éditions.
    pub fn editions_list(&self) -> Result<Vec<Edition>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions ORDER BY start_date DESC",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Edition {
                id: row.get(0)?,
                name: row.get(1)?,
                slug: row.get(2)?,
                start_date: row.get(3)?,
                end_date: row.get(4)?,
                location: row.get(5)?,
                theme: row.get(6)?,
                status: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Récupère une édition par id.
    pub fn edition_by_id(&self, id: &str) -> Result<Option<Edition>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(Edition {
                id: row.get(0)?,
                name: row.get(1)?,
                slug: row.get(2)?,
                start_date: row.get(3)?,
                end_date: row.get(4)?,
                location: row.get(5)?,
                theme: row.get(6)?,
                status: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        });
        match row {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // --- Organisateurs ---

    /// Liste tous les organisateurs.
    pub fn organisateurs_list(&self) -> Result<Vec<Organisateur>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, region, description, contact_email, website, created_at, updated_at FROM organisateurs ORDER BY name",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok(Organisateur {
                id: row.get(0)?,
                name: row.get(1)?,
                slug: row.get(2)?,
                region: row.get(3)?,
                description: row.get(4)?,
                contact_email: row.get(5)?,
                website: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Récupère un organisateur par id.
    pub fn organisateur_by_id(&self, id: &str) -> Result<Option<Organisateur>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, region, description, contact_email, website, created_at, updated_at FROM organisateurs WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(Organisateur {
                id: row.get(0)?,
                name: row.get(1)?,
                slug: row.get(2)?,
                region: row.get(3)?,
                description: row.get(4)?,
                contact_email: row.get(5)?,
                website: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        });
        match row {
            Ok(o) => Ok(Some(o)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // --- Exposants ---

    /// Liste les exposants (optionnellement filtrés par visible_repertoire).
    pub fn exposants_list(&self, visible_only: bool) -> Result<Vec<Exposant>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let sql = if visible_only {
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants WHERE visible_repertoire = 1 ORDER BY company_name"
        } else {
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants ORDER BY company_name"
        };
        let mut stmt = conn.prepare(sql)?;
        let rows = stmt.query_map([], |row| {
            Ok(Exposant {
                id: row.get(0)?,
                company_name: row.get(1)?,
                stand_name: row.get(2)?,
                contact_email: row.get(3)?,
                contact_phone: row.get(4)?,
                adresse: row.get(5)?,
                logo_url: row.get(6)?,
                site_web: row.get(7)?,
                siret: row.get(8)?,
                secteur: row.get(9)?,
                category: row.get(10)?,
                description: row.get(11)?,
                visible_repertoire: row.get::<_, Option<i32>>(12)?.map(|x| x != 0),
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Récupère un exposant par id.
    pub fn exposant_by_id(&self, id: &str) -> Result<Option<Exposant>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants WHERE id = ?1",
        )?;
        let row = stmt.query_row(params![id], |row| {
            Ok(Exposant {
                id: row.get(0)?,
                company_name: row.get(1)?,
                stand_name: row.get(2)?,
                contact_email: row.get(3)?,
                contact_phone: row.get(4)?,
                adresse: row.get(5)?,
                logo_url: row.get(6)?,
                site_web: row.get(7)?,
                siret: row.get(8)?,
                secteur: row.get(9)?,
                category: row.get(10)?,
                description: row.get(11)?,
                visible_repertoire: row.get::<_, Option<i32>>(12)?.map(|x| x != 0),
                created_at: row.get(13)?,
                updated_at: row.get(14)?,
            })
        });
        match row {
            Ok(e) => Ok(Some(e)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    // --- Editions-Exposants ---

    /// Liste les participations d'une édition.
    pub fn editions_exposants_by_edition(&self, edition_id: &str) -> Result<Vec<EditionExposant>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, edition_id, is_accepted, is_validated, is_paid, assigned_stand, size_meters, created_at, updated_at FROM editions_exposants WHERE edition_id = ?1",
        )?;
        let rows = stmt.query_map(params![edition_id], |row| {
            Ok(EditionExposant {
                id: row.get(0)?,
                exposant_id: row.get(1)?,
                edition_id: row.get(2)?,
                is_accepted: row.get::<_, Option<i32>>(3)?.map(|x| x != 0),
                is_validated: row.get::<_, Option<i32>>(4)?.map(|x| x != 0),
                is_paid: row.get::<_, Option<i32>>(5)?.map(|x| x != 0),
                assigned_stand: row.get(6)?,
                size_meters: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }
}
