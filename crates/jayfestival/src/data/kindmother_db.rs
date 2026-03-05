//! Base de données fille SQLite JayFestival sous autorité KindMother.
//!
//! Instance Daughter : persistance locale via SQLite, identité KindMother pour gouvernance.

use crate::data::types::{
    Animation, BudgetEntry, BudgetSummary, Edition, EditionExposant, Exposant, Organisateur,
    Profile,
};
use kindmother::{InstanceIdentity, InstanceType};
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
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
    /// Ouvre ou crée la base SQLite chiffrée à `path` et initialise le schéma.
    /// Identité KindMother : Daughter (base fille).
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref();
        let conn = Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("jayfestival");
            let kd = KeyDerivation::new(data_dir).map_err(|e| DbError(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| DbError(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)
                .map_err(|e| DbError(e.to_string()))?;
        }

        let instance = InstanceIdentity::new(InstanceType::Daughter);
        let db = Self {
            conn: Mutex::new(conn),
            instance,
        };
        db.init_schema()?;
        Ok(db)
    }

    /// Crée le schéma des tables (profiles, editions, organisateurs, exposants,
    /// editions_exposants, animations, budget_entries).
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
                candidature_date TEXT,
                motif_refus TEXT,
                status_candidature TEXT DEFAULT 'en_attente',
                created_at TEXT,
                updated_at TEXT,
                FOREIGN KEY (exposant_id) REFERENCES exposants(id),
                FOREIGN KEY (edition_id) REFERENCES editions(id)
            );
            CREATE TABLE IF NOT EXISTS animations (
                id TEXT PRIMARY KEY,
                edition_id TEXT NOT NULL,
                name TEXT,
                animation_type TEXT,
                start_time TEXT,
                end_time TEXT,
                room TEXT,
                description TEXT,
                status TEXT DEFAULT 'brouillon',
                created_at TEXT,
                updated_at TEXT,
                FOREIGN KEY (edition_id) REFERENCES editions(id)
            );
            CREATE TABLE IF NOT EXISTS budget_entries (
                id TEXT PRIMARY KEY,
                edition_id TEXT NOT NULL,
                label TEXT,
                category TEXT,
                amount REAL,
                entry_type TEXT,
                date TEXT,
                notes TEXT,
                created_at TEXT,
                updated_at TEXT,
                FOREIGN KEY (edition_id) REFERENCES editions(id)
            );
            ",
        )?;
        // Migration : ajouter les colonnes manquantes sur editions_exposants
        // (tolère l'erreur si la colonne existe déjà).
        for col in [
            "ALTER TABLE editions_exposants ADD COLUMN candidature_date TEXT",
            "ALTER TABLE editions_exposants ADD COLUMN motif_refus TEXT",
            "ALTER TABLE editions_exposants ADD COLUMN status_candidature TEXT DEFAULT 'en_attente'",
        ] {
            let _ = conn.execute_batch(col);
        }
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

    // --- Editions : mutations ---

    /// Crée une nouvelle édition. Retourne l'id généré.
    pub fn edition_create(
        &self,
        name: &str,
        slug: &str,
        start_date: Option<&str>,
        end_date: Option<&str>,
        location: Option<&str>,
        theme: Option<&str>,
        status: Option<&str>,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO editions (id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![id, name, slug, start_date, end_date, location, theme, status.unwrap_or("brouillon"), now, now],
        )?;
        Ok(id)
    }

    /// Met à jour une édition existante.
    pub fn edition_update(&self, edition: &Edition) -> Result<(), DbError> {
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE editions SET name = ?2, slug = ?3, start_date = ?4, end_date = ?5, location = ?6, theme = ?7, status = ?8, updated_at = ?9 WHERE id = ?1",
            params![
                edition.id,
                edition.name,
                edition.slug,
                edition.start_date,
                edition.end_date,
                edition.location,
                edition.theme,
                edition.status,
                now,
            ],
        )?;
        Ok(())
    }

    // --- Exposants : mutations ---

    /// Crée un nouvel exposant. Retourne l'id généré.
    pub fn exposant_create(
        &self,
        company_name: &str,
        contact_email: Option<&str>,
        category: Option<&str>,
        description: Option<&str>,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO exposants (id, company_name, contact_email, category, description, visible_repertoire, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, 1, ?6, ?7)",
            params![id, company_name, contact_email, category, description, now, now],
        )?;
        Ok(id)
    }

    // --- Editions-Exposants ---

    /// Liste les participations d'une édition.
    pub fn editions_exposants_by_edition(
        &self,
        edition_id: &str,
    ) -> Result<Vec<EditionExposant>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, exposant_id, edition_id, is_accepted, is_validated, is_paid, assigned_stand, size_meters, candidature_date, motif_refus, status_candidature, created_at, updated_at FROM editions_exposants WHERE edition_id = ?1",
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
                candidature_date: row.get(8)?,
                motif_refus: row.get(9)?,
                status_candidature: row.get(10)?,
                created_at: row.get(11)?,
                updated_at: row.get(12)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Crée une candidature exposant pour une édition. Retourne l'id généré.
    pub fn editions_exposants_create(
        &self,
        exposant_id: &str,
        edition_id: &str,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO editions_exposants (id, exposant_id, edition_id, is_accepted, is_validated, is_paid, candidature_date, status_candidature, created_at, updated_at) VALUES (?1, ?2, ?3, 0, 0, 0, ?4, 'en_attente', ?5, ?6)",
            params![id, exposant_id, edition_id, now, now, now],
        )?;
        Ok(id)
    }

    /// Met à jour le statut d'une candidature (acceptee, refusee, annulee).
    pub fn editions_exposants_update_status(
        &self,
        id: &str,
        status: &str,
        motif_refus: Option<&str>,
    ) -> Result<(), DbError> {
        let now = chrono::Utc::now().to_rfc3339();
        let is_accepted = matches!(status, "acceptee");
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE editions_exposants SET status_candidature = ?2, motif_refus = ?3, is_accepted = ?4, updated_at = ?5 WHERE id = ?1",
            params![id, status, motif_refus, is_accepted as i32, now],
        )?;
        Ok(())
    }

    /// Compte les candidatures en attente pour toutes les éditions.
    pub fn candidatures_pending_count(&self) -> Result<usize, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM editions_exposants WHERE status_candidature = 'en_attente'",
            [],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    // --- Animations ---

    /// Liste les animations d'une édition (ordonnées par horaire).
    pub fn animations_by_edition(&self, edition_id: &str) -> Result<Vec<Animation>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, edition_id, name, animation_type, start_time, end_time, room, description, status, created_at, updated_at FROM animations WHERE edition_id = ?1 ORDER BY start_time",
        )?;
        let rows = stmt.query_map(params![edition_id], |row| {
            Ok(Animation {
                id: row.get(0)?,
                edition_id: row.get(1)?,
                name: row.get(2)?,
                animation_type: row.get(3)?,
                start_time: row.get(4)?,
                end_time: row.get(5)?,
                room: row.get(6)?,
                description: row.get(7)?,
                status: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Crée une animation. Retourne l'id généré.
    pub fn animation_create(
        &self,
        edition_id: &str,
        name: &str,
        animation_type: Option<&str>,
        start_time: Option<&str>,
        end_time: Option<&str>,
        room: Option<&str>,
        description: Option<&str>,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO animations (id, edition_id, name, animation_type, start_time, end_time, room, description, status, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, 'brouillon', ?9, ?10)",
            params![id, edition_id, name, animation_type, start_time, end_time, room, description, now, now],
        )?;
        Ok(id)
    }

    /// Supprime une animation par id.
    pub fn animation_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM animations WHERE id = ?1", params![id])?;
        Ok(())
    }

    // --- Budget ---

    /// Liste les entrées budgétaires d'une édition.
    pub fn budget_entries_by_edition(&self, edition_id: &str) -> Result<Vec<BudgetEntry>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, edition_id, label, category, amount, entry_type, date, notes, created_at, updated_at FROM budget_entries WHERE edition_id = ?1 ORDER BY date",
        )?;
        let rows = stmt.query_map(params![edition_id], |row| {
            Ok(BudgetEntry {
                id: row.get(0)?,
                edition_id: row.get(1)?,
                label: row.get(2)?,
                category: row.get(3)?,
                amount: row.get(4)?,
                entry_type: row.get(5)?,
                date: row.get(6)?,
                notes: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    /// Crée une entrée budgétaire. Retourne l'id généré.
    pub fn budget_entry_create(
        &self,
        edition_id: &str,
        label: &str,
        category: Option<&str>,
        amount: f64,
        entry_type: &str,
        date: Option<&str>,
        notes: Option<&str>,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO budget_entries (id, edition_id, label, category, amount, entry_type, date, notes, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![id, edition_id, label, category, amount, entry_type, date, notes, now, now],
        )?;
        Ok(id)
    }

    /// Calcule le résumé budgétaire d'une édition (revenus, dépenses, balance).
    pub fn budget_summary(&self, edition_id: &str) -> Result<BudgetSummary, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let revenus: f64 = conn
            .query_row(
                "SELECT COALESCE(SUM(amount), 0.0) FROM budget_entries WHERE edition_id = ?1 AND entry_type = 'revenu'",
                params![edition_id],
                |row| row.get(0),
            )
            .unwrap_or(0.0);
        let depenses: f64 = conn
            .query_row(
                "SELECT COALESCE(SUM(amount), 0.0) FROM budget_entries WHERE edition_id = ?1 AND entry_type = 'depense'",
                params![edition_id],
                |row| row.get(0),
            )
            .unwrap_or(0.0);
        Ok(BudgetSummary {
            total_revenus: revenus,
            total_depenses: depenses,
            balance: revenus - depenses,
        })
    }

    // --- Phase 1 UNC : Méthodes publiques ---

    /// Liste les éditions publiées (status = 'publie' ou 'en_cours').
    pub fn editions_published(&self) -> Result<Vec<Edition>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions WHERE status IN ('publie', 'en_cours') ORDER BY start_date DESC",
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

    /// Liste les organisateurs publics (pour annuaire).
    pub fn organisateurs_public_list(&self) -> Result<Vec<Organisateur>, DbError> {
        self.organisateurs_list()
    }

    /// Liste les exposants publics (visible_repertoire = 1).
    pub fn exposants_public_list(&self) -> Result<Vec<Exposant>, DbError> {
        self.exposants_list(true)
    }

    /// Recherche textuelle sur éditions, organisateurs, exposants.
    pub fn search_all(
        &self,
        query: &str,
    ) -> Result<(Vec<Edition>, Vec<Organisateur>, Vec<Exposant>), DbError> {
        let q = format!("%{}%", query.to_lowercase());
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;

        // Recherche éditions
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions WHERE LOWER(name) LIKE ?1 OR LOWER(location) LIKE ?1 OR LOWER(theme) LIKE ?1 ORDER BY start_date DESC LIMIT 20",
        )?;
        let editions: Vec<Edition> = stmt
            .query_map(params![&q], |row| {
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
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Recherche organisateurs
        let mut stmt = conn.prepare(
            "SELECT id, name, slug, region, description, contact_email, website, created_at, updated_at FROM organisateurs WHERE LOWER(name) LIKE ?1 OR LOWER(region) LIKE ?1 ORDER BY name LIMIT 20",
        )?;
        let organisateurs: Vec<Organisateur> = stmt
            .query_map(params![&q], |row| {
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
            })?
            .filter_map(|r| r.ok())
            .collect();

        // Recherche exposants
        let mut stmt = conn.prepare(
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants WHERE visible_repertoire = 1 AND (LOWER(company_name) LIKE ?1 OR LOWER(secteur) LIKE ?1 OR LOWER(category) LIKE ?1) ORDER BY company_name LIMIT 20",
        )?;
        let exposants: Vec<Exposant> = stmt
            .query_map(params![&q], |row| {
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
            })?
            .filter_map(|r| r.ok())
            .collect();

        Ok((editions, organisateurs, exposants))
    }

    /// Compte les exposants validés d'une édition.
    pub fn exposants_count_by_edition(&self, edition_id: &str) -> Result<usize, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM editions_exposants WHERE edition_id = ?1 AND status_candidature = 'acceptee'",
            params![edition_id],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    /// Liste les exposants validés d'une édition (pour affichage public).
    pub fn exposants_by_edition(&self, edition_id: &str) -> Result<Vec<Exposant>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT e.id, e.company_name, e.stand_name, e.contact_email, e.contact_phone, e.adresse, e.logo_url, e.site_web, e.siret, e.secteur, e.category, e.description, e.visible_repertoire, e.created_at, e.updated_at 
             FROM exposants e 
             INNER JOIN editions_exposants ee ON e.id = ee.exposant_id 
             WHERE ee.edition_id = ?1 AND ee.status_candidature = 'acceptee' 
             ORDER BY e.company_name",
        )?;
        let rows = stmt.query_map(params![edition_id], |row| {
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

    /// Crée un organisateur. Retourne l'id généré.
    pub fn organisateur_create(
        &self,
        name: &str,
        slug: &str,
        region: Option<&str>,
        description: Option<&str>,
        contact_email: Option<&str>,
        website: Option<&str>,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO organisateurs (id, name, slug, region, description, contact_email, website, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![id, name, slug, region, description, contact_email, website, now, now],
        )?;
        Ok(id)
    }
}
