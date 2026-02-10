//! Base de données des profils Miyukini Central via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! L'API publique est **synchrone** pour maintenir la compatibilité avec le code existant.
//! En interne, les appels async sont exécutés via le runtime Tokio.
//!
//! @id: central_auth_kindmother_client_db
//! @do: persist_central_auth_data_via_kindmother_client
//! @layer: infra

use crate::auth::password::validate_password;
use kindmother_client::{ClientError, KindMotherClient};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use thiserror::Error;
use tokio::sync::OnceCell;

fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

/// Erreur de la base auth Central (KindMother Client).
#[derive(Error, Debug)]
pub enum AuthDbError {
    /// Erreur du client KindMother.
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),

    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erreur interne.
    #[error("Internal error: {0}")]
    Internal(String),

    /// Client non initialisé.
    #[error("Client not initialized")]
    NotInitialized,

    /// Erreur de validation.
    #[error("Validation error: {0}")]
    Validation(String),
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
}

/// Adresse par défaut du service KindMother.
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";

/// Client global partagé (singleton).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Runtime Tokio global pour exécuter les appels async.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn get_runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

/// Base des profils Central via KindMother Client.
///
/// Fournit une API synchrone pour la compatibilité avec le code existant.
#[derive(Clone)]
pub struct CentralAuthDb {
    client: Arc<KindMotherClient>,
}

impl CentralAuthDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), AuthDbError> {
        get_runtime().block_on(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), AuthDbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "miyukini-central", "central").await?;

        CLIENT
            .set(Arc::new(client))
            .map_err(|_| AuthDbError::Internal("Client already initialized".to_string()))?;

        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, AuthDbError> {
        let client = CLIENT.get().ok_or(AuthDbError::NotInitialized)?.clone();
        Ok(Self { client })
    }

    /// Ouvre une connexion à la base de données (API compatible avec l'ancienne implémentation).
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, AuthDbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Initialise le schéma de la base de données (synchrone).
    pub fn init_schema_sync(&self) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.init_schema_async())
    }

    /// Initialise le schéma de la base de données (async).
    pub async fn init_schema_async(&self) -> Result<(), AuthDbError> {
        let schema = r#"
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
                ville TEXT,
                is_admin INTEGER NOT NULL DEFAULT 0
            );
            CREATE TABLE IF NOT EXISTS cog_meta (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
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
                data_base64 TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                UNIQUE(profile_id, service_key, slot),
                FOREIGN KEY (profile_id) REFERENCES central_profiles(id)
            );
            CREATE INDEX IF NOT EXISTS idx_central_profile_saves_profile_service
                ON central_profile_saves(profile_id, service_key);
        "#;

        let statements: Vec<&str> = schema
            .split(';')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        for sql in statements {
            self.client
                .execute(sql, Vec::<String>::new(), "init_schema")
                .await?;
        }

        Ok(())
    }

    /// Connexion : email + mot de passe. Retourne le profil complet si valide (synchrone).
    pub fn sign_in(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<CentralProfile>, AuthDbError> {
        get_runtime().block_on(self.sign_in_async(email, password))
    }

    async fn sign_in_async(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<CentralProfile>, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Ok(None);
        }

        let rows = self
            .client
            .query(
                "SELECT id, email, password_hash, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin FROM central_profiles WHERE email = ?1",
                vec![email],
            )
            .await?;

        if let Some(row) = rows.first() {
            let hash = Self::get_string(row, "password_hash");
            if hash == hash_password(password) {
                Ok(Some(Self::row_to_profile(row)?))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Charge un profil par ID (synchrone).
    pub fn get_profile(&self, id: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        get_runtime().block_on(self.get_profile_async(id))
    }

    async fn get_profile_async(&self, id: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT id, email, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin FROM central_profiles WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first().map(|r| Self::row_to_profile(r)).transpose()
    }

    /// Enregistre les champs modifiables du profil en base (synchrone).
    pub fn update_profile(&self, profile: &CentralProfile) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.update_profile_async(profile))
    }

    async fn update_profile_async(&self, profile: &CentralProfile) -> Result<(), AuthDbError> {
        let now = chrono::Utc::now().to_rfc3339();

        self.client
            .execute(
                "UPDATE central_profiles SET email = ?1, pseudonyme = ?2, nom = ?3, prenom = ?4, date_naissance = ?5, telephone = ?6, numero_voie = ?7, rue = ?8, code_postal = ?9, ville = ?10, updated_at = ?11 WHERE id = ?12",
                vec![
                    &profile.email,
                    profile.pseudonyme.as_deref().unwrap_or(""),
                    profile.nom.as_deref().unwrap_or(""),
                    profile.prenom.as_deref().unwrap_or(""),
                    profile.date_naissance.as_deref().unwrap_or(""),
                    profile.telephone.as_deref().unwrap_or(""),
                    profile.numero_voie.as_deref().unwrap_or(""),
                    profile.rue.as_deref().unwrap_or(""),
                    profile.code_postal.as_deref().unwrap_or(""),
                    profile.ville.as_deref().unwrap_or(""),
                    &now,
                    &profile.id,
                ],
                "update_profile",
            )
            .await?;

        Ok(())
    }

    /// Liste tous les profils (id, email uniquement) (synchrone).
    pub fn list_profiles(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        get_runtime().block_on(self.list_profiles_async())
    }

    async fn list_profiles_async(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT id, email, is_admin FROM central_profiles",
                Vec::<String>::new(),
            )
            .await?;

        rows.iter()
            .map(|row| {
                Ok(CentralProfile {
                    id: Self::get_string(row, "id"),
                    email: Self::get_string(row, "email"),
                    pseudonyme: None,
                    nom: None,
                    prenom: None,
                    date_naissance: None,
                    telephone: None,
                    numero_voie: None,
                    rue: None,
                    code_postal: None,
                    ville: None,
                    is_admin: Self::get_opt_i64(row, "is_admin").unwrap_or(0) != 0,
                })
            })
            .collect()
    }

    /// Met à jour l'ID d'un profil (synchrone).
    pub fn update_profile_id(&self, old_id: &str, new_id: &str) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.update_profile_id_async(old_id, new_id))
    }

    async fn update_profile_id_async(&self, old_id: &str, new_id: &str) -> Result<(), AuthDbError> {
        let result = self
            .client
            .execute(
                "UPDATE central_profiles SET id = ?1, updated_at = ?2 WHERE id = ?3",
                vec![new_id, &chrono::Utc::now().to_rfc3339(), old_id],
                "update_profile_id",
            )
            .await?;

        if result == 0 {
            return Err(AuthDbError::Internal(format!(
                "Profil non trouvé: {old_id}"
            )));
        }
        Ok(())
    }

    /// Création de compte : email + mot de passe complexe (synchrone).
    /// `pseudonyme` : optionnel, pour le Rite d'Entrée (nom / pseudo).
    pub fn sign_up(
        &self,
        email: &str,
        password: &str,
        pseudonyme: Option<&str>,
    ) -> Result<CentralProfile, AuthDbError> {
        get_runtime().block_on(self.sign_up_async(email, password, pseudonyme))
    }

    async fn sign_up_async(
        &self,
        email: &str,
        password: &str,
        pseudonyme: Option<&str>,
    ) -> Result<CentralProfile, AuthDbError> {
        let email = email.trim();
        if email.is_empty() {
            return Err(AuthDbError::Validation("L'email est requis.".into()));
        }
        validate_password(password).map_err(|e| AuthDbError::Validation(e.to_string()))?;

        let is_first = self.list_profiles_async().await?.is_empty();
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password(password);
        let now = chrono::Utc::now().to_rfc3339();
        let is_admin = is_first;
        let pseudo_val = pseudonyme.map(|s| s.trim()).filter(|s| !s.is_empty());
        let is_admin_param = if is_admin { "1" } else { "0" };

        self.client
            .execute(
                "INSERT INTO central_profiles (id, email, password_hash, created_at, updated_at, pseudonyme, nom, prenom, date_naissance, telephone, numero_voie, rue, code_postal, ville, is_admin) VALUES (?1, ?2, ?3, ?4, ?5, ?6, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL, ?7)",
                vec![
                    id.as_str(),
                    email,
                    hash.as_str(),
                    now.as_str(),
                    now.as_str(),
                    pseudo_val.unwrap_or(""),
                    is_admin_param,
                ],
                "sign_up",
            )
            .await?;

        if is_first {
            self.set_cog_virgin_sync(false)?;
        }

        Ok(CentralProfile {
            id,
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
        })
    }

    /// Indique si le COG est vierge (aucun compte créé) (synchrone).
    pub fn is_cog_virgin(&self) -> Result<bool, AuthDbError> {
        get_runtime().block_on(self.is_cog_virgin_async())
    }

    async fn is_cog_virgin_async(&self) -> Result<bool, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT value FROM cog_meta WHERE key = 'cog_virgin'",
                Vec::<String>::new(),
            )
            .await?;
        let v = rows.first().and_then(|r| r.get("value")).and_then(|v| v.as_str());
        Ok(v.map_or(true, |s| s == "1"))
    }

    /// Marque le COG comme non vierge (synchrone).
    pub fn set_cog_virgin_sync(&self, virgin: bool) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.set_cog_virgin_async(virgin))
    }

    async fn set_cog_virgin_async(&self, virgin: bool) -> Result<(), AuthDbError> {
        let value = if virgin { "1" } else { "0" };
        self.client
            .execute(
                "INSERT OR REPLACE INTO cog_meta (key, value) VALUES ('cog_virgin', ?1)",
                vec![value],
                "set_cog_virgin",
            )
            .await?;
        Ok(())
    }

    /// ID du profil actuellement connecté (synchrone).
    pub fn get_current_profile_id(&self) -> Result<Option<String>, AuthDbError> {
        get_runtime().block_on(self.get_current_profile_id_async())
    }

    async fn get_current_profile_id_async(&self) -> Result<Option<String>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT value FROM cog_meta WHERE key = 'current_profile_id'",
                Vec::<String>::new(),
            )
            .await?;
        Ok(rows
            .first()
            .and_then(|r| r.get("value"))
            .and_then(|v| v.as_str())
            .map(String::from))
    }

    /// Enregistre le profil connecté ou None pour déconnexion (synchrone).
    pub fn set_current_profile_id(&self, profile_id: Option<&str>) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.set_current_profile_id_async(profile_id))
    }

    async fn set_current_profile_id_async(&self, profile_id: Option<&str>) -> Result<(), AuthDbError> {
        match profile_id {
            Some(id) => {
                self.client
                    .execute(
                        "INSERT OR REPLACE INTO cog_meta (key, value) VALUES ('current_profile_id', ?1)",
                        vec![id],
                        "set_current_profile_id",
                    )
                    .await?;
            }
            None => {
                self.client
                    .execute("DELETE FROM cog_meta WHERE key = 'current_profile_id'", Vec::<String>::new(), "delete_current_profile_id")
                    .await?;
            }
        }
        Ok(())
    }

    /// Réinitialise le COG à l'état vierge (suppression de tous les comptes). Pour tests uniquement.
    pub fn reset_cog_to_virgin(&self) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.reset_cog_to_virgin_async())
    }

    async fn reset_cog_to_virgin_async(&self) -> Result<(), AuthDbError> {
        self.client
            .execute("DELETE FROM central_profile_saves", Vec::<String>::new(), "reset_cog")
            .await?;
        self.client
            .execute("DELETE FROM profile_service_refs", Vec::<String>::new(), "reset_cog")
            .await?;
        self.client
            .execute("DELETE FROM central_profiles", Vec::<String>::new(), "reset_cog")
            .await?;
        self.set_cog_virgin_async(true).await?;
        self.set_current_profile_id_async(None).await?;
        Ok(())
    }

    /// Retourne l'ID de référence du profil vers la table du service (synchrone).
    pub fn get_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Option<String>, AuthDbError> {
        get_runtime().block_on(self.get_profile_service_ref_async(profile_id, service_key))
    }

    async fn get_profile_service_ref_async(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Option<String>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT ref_id FROM profile_service_refs WHERE profile_id = ?1 AND service_key = ?2",
                vec![profile_id, service_key],
            )
            .await?;

        Ok(rows
            .first()
            .and_then(|r| Self::get_opt_string(r, "ref_id")))
    }

    /// Lie le profil à une ligne de la table du service (synchrone).
    pub fn set_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
        ref_id: Option<&str>,
    ) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.set_profile_service_ref_async(profile_id, service_key, ref_id))
    }

    async fn set_profile_service_ref_async(
        &self,
        profile_id: &str,
        service_key: &str,
        ref_id: Option<&str>,
    ) -> Result<(), AuthDbError> {
        match ref_id {
            Some(id) => {
                self.client
                    .execute(
                        "INSERT INTO profile_service_refs (profile_id, service_key, ref_id) VALUES (?1, ?2, ?3) ON CONFLICT(profile_id, service_key) DO UPDATE SET ref_id = excluded.ref_id",
                        vec![profile_id, service_key, id],
                        "set_profile_service_ref",
                    )
                    .await?;
            }
            None => {
                self.client
                    .execute(
                        "DELETE FROM profile_service_refs WHERE profile_id = ?1 AND service_key = ?2",
                        vec![profile_id, service_key],
                        "delete_profile_service_ref",
                    )
                    .await?;
            }
        }
        Ok(())
    }

    // ---------- Sauvegardes liées au profil (central_profile_saves) ----------

    /// Crée une sauvegarde pour un profil et un service. Retourne l'id de la ligne (synchrone).
    /// Note: Les données sont encodées en base64 pour le transport JSON.
    pub fn insert_profile_save(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
        data: &[u8],
    ) -> Result<String, AuthDbError> {
        get_runtime().block_on(self.insert_profile_save_async(profile_id, service_key, slot, data))
    }

    async fn insert_profile_save_async(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
        data: &[u8],
    ) -> Result<String, AuthDbError> {
        use base64::Engine;
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);

        self.client
            .execute(
                "INSERT INTO central_profile_saves (id, profile_id, service_key, slot, data_base64, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                vec![&id, profile_id, service_key, &slot.to_string(), &data_base64, &now, &now],
                "insert_profile_save",
            )
            .await?;

        Ok(id)
    }

    /// Charge une sauvegarde par id (synchrone).
    pub fn get_profile_save(&self, id: &str) -> Result<Option<CentralProfileSave>, AuthDbError> {
        get_runtime().block_on(self.get_profile_save_async(id))
    }

    async fn get_profile_save_async(&self, id: &str) -> Result<Option<CentralProfileSave>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT id, profile_id, service_key, slot, data_base64, created_at, updated_at FROM central_profile_saves WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first()
            .map(|r| Self::row_to_profile_save(r))
            .transpose()
    }

    /// Charge une sauvegarde par (profile_id, service_key, slot) (synchrone).
    pub fn get_profile_save_by_slot(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
    ) -> Result<Option<CentralProfileSave>, AuthDbError> {
        get_runtime().block_on(self.get_profile_save_by_slot_async(profile_id, service_key, slot))
    }

    async fn get_profile_save_by_slot_async(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
    ) -> Result<Option<CentralProfileSave>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT id, profile_id, service_key, slot, data_base64, created_at, updated_at FROM central_profile_saves WHERE profile_id = ?1 AND service_key = ?2 AND slot = ?3",
                vec![profile_id, service_key, &slot.to_string()],
            )
            .await?;

        rows.first()
            .map(|r| Self::row_to_profile_save(r))
            .transpose()
    }

    /// Met à jour le blob et updated_at d'une sauvegarde existante (synchrone).
    pub fn update_profile_save(&self, id: &str, data: &[u8]) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.update_profile_save_async(id, data))
    }

    async fn update_profile_save_async(&self, id: &str, data: &[u8]) -> Result<(), AuthDbError> {
        use base64::Engine;
        let now = chrono::Utc::now().to_rfc3339();
        let data_base64 = base64::engine::general_purpose::STANDARD.encode(data);

        let n = self
            .client
            .execute(
                "UPDATE central_profile_saves SET data_base64 = ?1, updated_at = ?2 WHERE id = ?3",
                vec![&data_base64, &now, id],
                "update_profile_save",
            )
            .await?;

        if n == 0 {
            return Err(AuthDbError::Internal(format!(
                "Sauvegarde non trouvée: {id}"
            )));
        }
        Ok(())
    }

    /// Liste les sauvegardes (sans BLOB) pour un profil et un service (synchrone).
    pub fn list_profile_saves(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Vec<CentralProfileSaveRow>, AuthDbError> {
        get_runtime().block_on(self.list_profile_saves_async(profile_id, service_key))
    }

    async fn list_profile_saves_async(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Vec<CentralProfileSaveRow>, AuthDbError> {
        let rows = self
            .client
            .query(
                "SELECT id, slot, created_at, updated_at FROM central_profile_saves WHERE profile_id = ?1 AND service_key = ?2 ORDER BY slot",
                vec![profile_id, service_key],
            )
            .await?;

        rows.iter()
            .map(|row| {
                Ok(CentralProfileSaveRow {
                    id: Self::get_string(row, "id"),
                    slot: Self::get_opt_i64(row, "slot").unwrap_or(0),
                    created_at: Self::get_string(row, "created_at"),
                    updated_at: Self::get_string(row, "updated_at"),
                })
            })
            .collect()
    }

    /// Supprime une sauvegarde par id (synchrone).
    pub fn delete_profile_save(&self, id: &str) -> Result<(), AuthDbError> {
        get_runtime().block_on(self.delete_profile_save_async(id))
    }

    async fn delete_profile_save_async(&self, id: &str) -> Result<(), AuthDbError> {
        let n = self
            .client
            .execute(
                "DELETE FROM central_profile_saves WHERE id = ?1",
                vec![id],
                "delete_profile_save",
            )
            .await?;

        if n == 0 {
            return Err(AuthDbError::Internal(format!(
                "Sauvegarde non trouvée: {id}"
            )));
        }
        Ok(())
    }

    /// Vérification de la santé de la connexion (synchrone).
    pub fn health_check(&self) -> Result<bool, AuthDbError> {
        get_runtime().block_on(self.client.health_check()).map_err(AuthDbError::from)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn row_to_profile(row: &HashMap<String, serde_json::Value>) -> Result<CentralProfile, AuthDbError> {
        Ok(CentralProfile {
            id: Self::get_string(row, "id"),
            email: Self::get_string(row, "email"),
            pseudonyme: Self::get_opt_string(row, "pseudonyme"),
            nom: Self::get_opt_string(row, "nom"),
            prenom: Self::get_opt_string(row, "prenom"),
            date_naissance: Self::get_opt_string(row, "date_naissance"),
            telephone: Self::get_opt_string(row, "telephone"),
            numero_voie: Self::get_opt_string(row, "numero_voie"),
            rue: Self::get_opt_string(row, "rue"),
            code_postal: Self::get_opt_string(row, "code_postal"),
            ville: Self::get_opt_string(row, "ville"),
            is_admin: Self::get_opt_i64(row, "is_admin").unwrap_or(0) != 0,
        })
    }

    fn row_to_profile_save(
        row: &HashMap<String, serde_json::Value>,
    ) -> Result<CentralProfileSave, AuthDbError> {
        use base64::Engine;
        let data_base64 = Self::get_string(row, "data_base64");
        let data = base64::engine::general_purpose::STANDARD
            .decode(&data_base64)
            .unwrap_or_default();

        Ok(CentralProfileSave {
            id: Self::get_string(row, "id"),
            profile_id: Self::get_string(row, "profile_id"),
            service_key: Self::get_string(row, "service_key"),
            slot: Self::get_opt_i64(row, "slot").unwrap_or(0),
            data,
            created_at: Self::get_string(row, "created_at"),
            updated_at: Self::get_string(row, "updated_at"),
        })
    }

    fn get_string(row: &HashMap<String, serde_json::Value>, key: &str) -> String {
        row.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    }

    fn get_opt_string(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    }

    fn get_opt_i64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<i64> {
        row.get(key).and_then(|v| v.as_i64())
    }
}
