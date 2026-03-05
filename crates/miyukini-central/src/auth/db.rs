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
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
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
    /// Identifiant unique de la sauvegarde.
    pub id: String,
    /// Identifiant du profil Central.
    pub profile_id: String,
    /// Clé du service (ex. `lord_of_the_castle`).
    pub service_key: String,
    /// Numéro de slot (0, 1, 2…).
    pub slot: i64,
    /// Données sérialisées (BLOB).
    pub data: Vec<u8>,
    /// Date de création (ISO).
    pub created_at: String,
    /// Date de dernière mise à jour (ISO).
    pub updated_at: String,
}

/// Ligne allégée pour lister les sauvegardes (sans le BLOB).
#[derive(Debug, Clone)]
pub struct CentralProfileSaveRow {
    /// Identifiant unique de la sauvegarde.
    pub id: String,
    /// Numéro de slot.
    pub slot: i64,
    /// Date de création (ISO).
    pub created_at: String,
    /// Date de dernière mise à jour (ISO).
    pub updated_at: String,
}

/// Profil Miyukini COG (email = login, champs enrichis modifiables).
#[derive(Debug, Clone)]
#[allow(missing_docs)]
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
    /// Premier compte créé = admin de l'environnement (MiyukiniAdmin).
    pub is_admin: bool,
    /// Genre (Masculin, Féminin, Neutre) — Rite d'Entrée.
    pub genre: Option<String>,
    /// Statut relationnel (Célibataire, En couple, Marié) — Rite d'Entrée.
    pub statut_marital: Option<String>,
    /// Genre du/de la partenaire si non célibataire.
    pub partenaire_genre: Option<String>,
    /// Nom ou prénom du/de la partenaire si non célibataire.
    pub partenaire_nom: Option<String>,
    /// Nombre d'enfants (0 si aucun ou non renseigné).
    pub enfants_nombre: Option<i32>,
    /// Prénoms des enfants (JSON array ou séparés par virgule).
    pub enfants_noms: Option<String>,
    /// Profession ou activité — Rite d'Entrée.
    pub profession: Option<String>,
    /// Langue maternelle.
    pub langue_maternelle: Option<String>,
}

/// Base des profils Central (SQLite).
pub struct CentralAuthDb {
    conn: Mutex<Connection>,
}

impl CentralAuthDb {
    /// Ouvre ou crée la base SQLite à `path`.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, AuthDbError> {
        let path = path.as_ref();
        let conn = Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("central");
            let kd = KeyDerivation::new(data_dir).map_err(|e| AuthDbError(e.0))?;
            let pragma_key = kd.pragma_key_hex(db_name).map_err(|e| AuthDbError(e.0))?;
            conn.pragma_update(None, "key", &pragma_key)
                .map_err(|e| AuthDbError(e.to_string()))?;
        }

        let db = Self {
            conn: Mutex::new(conn),
        };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        conn.execute_batch(
            r"
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
            ",
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS cog_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )?;
        self.migrate_add_profile_columns(&conn)?;
        conn.execute_batch(
            r"
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
            ",
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
            ("is_admin", "INTEGER NOT NULL DEFAULT 0"),
            ("genre", "TEXT"),
            ("statut_marital", "TEXT"),
            ("partenaire_genre", "TEXT"),
            ("partenaire_nom", "TEXT"),
            ("enfants_nombre", "INTEGER"),
            ("enfants_noms", "TEXT"),
            ("profession", "TEXT"),
            ("langue_maternelle", "TEXT"),
        ];
        for (name, typ) in columns {
            let sql = format!("ALTER TABLE central_profiles ADD COLUMN {name} {typ}");
            if let Err(e) = conn.execute(&sql, []) {
                if !e.to_string().contains("duplicate column name") {
                    return Err(e.into());
                }
            }
        }
        Ok(())
    }

    /// Connexion : email + mot de passe. Retourne le profil complet si valide.
    pub fn sign_in(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<CentralProfile>, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Ok(None);
        }
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let sql = "SELECT id, email, password_hash, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin, genre, statut_marital, partenaire_genre, partenaire_nom, enfants_nombre, enfants_noms, profession, langue_maternelle FROM central_profiles WHERE email = ?1";
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
                row.get::<_, i64>(12)?,
                row.get::<_, Option<String>>(13)?,
                row.get::<_, Option<String>>(14)?,
                row.get::<_, Option<String>>(15)?,
                row.get::<_, Option<String>>(16)?,
                row.get::<_, Option<i32>>(17)?,
                row.get::<_, Option<String>>(18)?,
                row.get::<_, Option<String>>(19)?,
                row.get::<_, Option<String>>(20)?,
            ))
        });
        match row {
            Ok((
                id,
                email_val,
                hash,
                pseudonyme,
                nom,
                prenom,
                date_naissance,
                telephone,
                numero_voie,
                rue,
                code_postal,
                ville,
                is_admin,
                genre,
                statut_marital,
                partenaire_genre,
                partenaire_nom,
                enfants_nombre,
                enfants_noms,
                profession,
                langue_maternelle,
            )) => {
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
                        is_admin: is_admin != 0,
                        genre,
                        statut_marital,
                        partenaire_genre,
                        partenaire_nom,
                        enfants_nombre,
                        enfants_noms,
                        profession,
                        langue_maternelle,
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
        let sql = "SELECT id, email, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin, genre, statut_marital, partenaire_genre, partenaire_nom, enfants_nombre, enfants_noms, profession, langue_maternelle FROM central_profiles WHERE id = ?1";
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
                is_admin: row.get::<_, i64>(11)? != 0,
                genre: row.get::<_, Option<String>>(12).unwrap_or(None),
                statut_marital: row.get::<_, Option<String>>(13).unwrap_or(None),
                partenaire_genre: row.get::<_, Option<String>>(14).unwrap_or(None),
                partenaire_nom: row.get::<_, Option<String>>(15).unwrap_or(None),
                enfants_nombre: row.get::<_, Option<i32>>(16).unwrap_or(None),
                enfants_noms: row.get::<_, Option<String>>(17).unwrap_or(None),
                profession: row.get::<_, Option<String>>(18).unwrap_or(None),
                langue_maternelle: row.get::<_, Option<String>>(19).unwrap_or(None),
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
            "UPDATE central_profiles SET email = ?1, pseudonyme = ?2, nom = ?3, prenom = ?4, date_naissance = ?5, telephone = ?6, numero_voie = ?7, rue = ?8, code_postal = ?9, ville = ?10, genre = ?11, statut_marital = ?12, partenaire_genre = ?13, partenaire_nom = ?14, enfants_nombre = ?15, enfants_noms = ?16, profession = ?17, langue_maternelle = ?18, updated_at = ?19 WHERE id = ?20",
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
                profile.genre,
                profile.statut_marital,
                profile.partenaire_genre,
                profile.partenaire_nom,
                profile.enfants_nombre,
                profile.enfants_noms,
                profile.profession,
                profile.langue_maternelle,
                now,
                profile.id,
            ],
        )?;
        Ok(())
    }

    /// Liste tous les profils (id, email, is_admin pour la DB mère).
    pub fn list_profiles(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT id, email, is_admin FROM central_profiles")?;
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
                is_admin: row.get::<_, i64>(2)? != 0,
                genre: None,
                statut_marital: None,
                partenaire_genre: None,
                partenaire_nom: None,
                enfants_nombre: None,
                enfants_noms: None,
                profession: None,
                langue_maternelle: None,
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
            return Err(AuthDbError(format!("Profil non trouvé: {old_id}")));
        }
        Ok(())
    }

    /// Indique si le COG est vierge (aucun compte créé). Passe à false à la création du premier compte.
    pub fn is_cog_virgin(&self) -> Result<bool, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let v = match conn.query_row(
            "SELECT value FROM cog_meta WHERE key = 'cog_virgin'",
            [],
            |row| row.get::<_, String>(0),
        ) {
            Ok(s) => s,
            Err(rusqlite::Error::QueryReturnedNoRows) => return Ok(true),
            Err(e) => return Err(e.into()),
        };
        Ok(v == "1")
    }

    /// Marque le COG comme non vierge (appelé à la création du premier compte).
    pub fn set_cog_virgin(&self, virgin: bool) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        let value = if virgin { "1" } else { "0" };
        conn.execute(
            "INSERT OR REPLACE INTO cog_meta (key, value) VALUES ('cog_virgin', ?1)",
            [value],
        )?;
        Ok(())
    }

    /// ID du profil actuellement connecté (session persistée).
    pub fn get_current_profile_id(&self) -> Result<Option<String>, AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        match conn.query_row(
            "SELECT value FROM cog_meta WHERE key = 'current_profile_id'",
            [],
            |row| row.get::<_, String>(0),
        ) {
            Ok(id) => Ok(Some(id)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Enregistre le profil connecté (ou None pour déconnexion).
    pub fn set_current_profile_id(&self, profile_id: Option<&str>) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        match profile_id {
            Some(id) => {
                conn.execute(
                    "INSERT OR REPLACE INTO cog_meta (key, value) VALUES ('current_profile_id', ?1)",
                    [id],
                )?;
            }
            None => {
                conn.execute("DELETE FROM cog_meta WHERE key = 'current_profile_id'", [])?;
            }
        }
        Ok(())
    }

    /// Réinitialise le COG à l'état vierge (suppression de tous les comptes). Pour tests uniquement.
    pub fn reset_cog_to_virgin(&self) -> Result<(), AuthDbError> {
        let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
        conn.execute("DELETE FROM central_profile_saves", [])?;
        conn.execute("DELETE FROM profile_service_refs", [])?;
        conn.execute("DELETE FROM central_profiles", [])?;
        conn.execute(
            "INSERT OR REPLACE INTO cog_meta (key, value) VALUES ('cog_virgin', '1')",
            [],
        )?;
        conn.execute("DELETE FROM cog_meta WHERE key = 'current_profile_id'", [])?;
        Ok(())
    }

    /// Création de compte : email + mot de passe complexe. Retourne le profil créé.
    /// Le premier compte créé est admin de l'environnement (MiyukiniAdmin).
    /// `pseudonyme` : optionnel, utilisé pour le Rite d'Entrée (nom / pseudo).
    pub fn sign_up(
        &self,
        email: &str,
        password: &str,
        pseudonyme: Option<&str>,
    ) -> Result<CentralProfile, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Err(AuthDbError("L'email est requis.".into()));
        }
        validate_password(password).map_err(|e| AuthDbError(e.to_string()))?;
        let is_first = self.list_profiles()?.is_empty();
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password(password);
        let now = chrono::Utc::now().to_rfc3339();
        let is_admin = is_first;
        let pseudo_val = pseudonyme.map(|s| s.trim()).filter(|s| !s.is_empty());
        {
            let conn = self.conn.lock().map_err(|e| AuthDbError(e.to_string()))?;
            conn.execute(
                "INSERT INTO central_profiles (id, email, password_hash, created_at, updated_at, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin) VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, ?7)",
                params![id, email, hash, now, now, pseudo_val, if is_admin { 1i64 } else { 0i64 }],
            )?;
        } // conn libéré ici — évite le deadlock avec set_cog_virgin ci-dessous
        if is_first {
            self.set_cog_virgin(false)?;
        }
        Ok(CentralProfile {
            id: id.clone(),
            email: email.to_string(),
            pseudonyme: pseudo_val.map(String::from),
            nom: None,
            prenom: None,
            date_naissance: None,
            telephone: None,
            numero_voie: None,
            rue: None,
            code_postal: None,
            ville: None,
            is_admin,
            genre: None,
            statut_marital: None,
            partenaire_genre: None,
            partenaire_nom: None,
            enfants_nombre: None,
            enfants_noms: None,
            profession: None,
            langue_maternelle: None,
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
        let row = stmt.query_row(params![profile_id, service_key], |row| {
            row.get::<_, String>(0)
        });
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
            return Err(AuthDbError(format!("Sauvegarde non trouvée: {id}")));
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
        let n = conn.execute(
            "DELETE FROM central_profile_saves WHERE id = ?1",
            params![id],
        )?;
        if n == 0 {
            return Err(AuthDbError(format!("Sauvegarde non trouvée: {id}")));
        }
        Ok(())
    }
}
