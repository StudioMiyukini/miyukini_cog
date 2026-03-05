//! Base de données JayFestival via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` pour accéder aux données.
//! KindMother est le seul interlocuteur autorisé de la base de données.
//!
//! L'API publique est **synchrone** pour maintenir la compatibilité avec le code existant.
//! En interne, les appels async sont exécutés via le runtime Tokio.
//!
//! @id: jayfestival_kindmother_client_db
//! @do: persist_jayfestival_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{Edition, EditionExposant, Exposant, Organisateur, Profile};
use kindmother_client::{ClientError, KindMotherClient};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use thiserror::Error;
use tokio::sync::OnceCell;

/// Erreur de la base JayFestival (KindMother Client).
#[derive(Error, Debug)]
pub enum DbError {
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

/// Exécute un future de manière bloquante, compatible avec un runtime tokio existant (Dioxus).
fn run_blocking<F: std::future::Future>(f: F) -> F::Output {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => tokio::task::block_in_place(|| handle.block_on(f)),
        Err(_) => run_blocking(f),
    }
}

fn hash_password(password: &str) -> String {
    let mut h = Sha256::new();
    h.update(password.as_bytes());
    format!("{:x}", h.finalize())
}

/// Base de données JayFestival via KindMother Client.
///
/// Fournit une API synchrone pour la compatibilité avec le code existant.
#[derive(Clone)]
pub struct JayFestivalDb {
    client: Arc<KindMotherClient>,
}

impl JayFestivalDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, "jayfestival", "jayfestival").await?;

        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError::Internal("Client already initialized".to_string()))?;

        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError::NotInitialized)?.clone();
        Ok(Self { client })
    }

    /// Ouvre une connexion à la base de données (API compatible avec l'ancienne implémentation).
    pub fn open(_path: impl AsRef<std::path::Path>) -> Result<Self, DbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Initialise le schéma de la base de données (synchrone).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    /// Initialise le schéma de la base de données (async).
    pub async fn init_schema_async(&self) -> Result<(), DbError> {
        let schema = r#"
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

    // -----------------------------------------------------------------------
    // Profiles (auth)
    // -----------------------------------------------------------------------

    /// Retourne le profil par email si le mot de passe est valide (synchrone).
    pub fn profile_by_email_password(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<Profile>, DbError> {
        run_blocking(self.profile_by_email_password_async(email, password))
    }

    async fn profile_by_email_password_async(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<Profile>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, username, user_type, email, avatar_url, theme, password_hash, created_at, updated_at FROM profiles WHERE email = ?1",
                vec![email],
            )
            .await?;

        if let Some(row) = rows.first() {
            let profile = Self::row_to_profile(row)?;
            let expected = profile.password_hash.as_deref().unwrap_or("");
            if expected == hash_password(password) {
                Ok(Some(profile))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }

    /// Récupère un profil par id (synchrone).
    pub fn profile_by_id(&self, id: &str) -> Result<Option<Profile>, DbError> {
        run_blocking(self.profile_by_id_async(id))
    }

    async fn profile_by_id_async(&self, id: &str) -> Result<Option<Profile>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, username, user_type, email, avatar_url, theme, password_hash, created_at, updated_at FROM profiles WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first().map(|r| Self::row_to_profile(r)).transpose()
    }

    /// Crée un profil (inscription). Retourne l'id généré (synchrone).
    pub fn profile_create(
        &self,
        email: &str,
        password: &str,
        user_type: &str,
    ) -> Result<String, DbError> {
        run_blocking(self.profile_create_async(email, password, user_type))
    }

    async fn profile_create_async(
        &self,
        email: &str,
        password: &str,
        user_type: &str,
    ) -> Result<String, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().to_rfc3339();
        let hash = hash_password(password);

        self.client
            .execute(
                "INSERT INTO profiles (id, email, password_hash, user_type, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                vec![&id, email, &hash, user_type, &now, &now],
                "profile_create",
            )
            .await?;

        Ok(id)
    }

    fn row_to_profile(row: &HashMap<String, serde_json::Value>) -> Result<Profile, DbError> {
        Ok(Profile {
            id: Self::get_opt_string(row, "id"),
            username: Self::get_opt_string(row, "username"),
            user_type: Self::get_opt_string(row, "user_type"),
            email: Self::get_opt_string(row, "email"),
            avatar_url: Self::get_opt_string(row, "avatar_url"),
            theme: Self::get_opt_string(row, "theme"),
            password_hash: Self::get_opt_string(row, "password_hash"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Editions
    // -----------------------------------------------------------------------

    /// Liste toutes les éditions (synchrone).
    pub fn editions_list(&self) -> Result<Vec<Edition>, DbError> {
        run_blocking(self.editions_list_async())
    }

    async fn editions_list_async(&self) -> Result<Vec<Edition>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions ORDER BY start_date DESC",
                Vec::<String>::new(),
            )
            .await?;

        rows.iter().map(|r| Self::row_to_edition(r)).collect()
    }

    /// Récupère une édition par id (synchrone).
    pub fn edition_by_id(&self, id: &str) -> Result<Option<Edition>, DbError> {
        run_blocking(self.edition_by_id_async(id))
    }

    async fn edition_by_id_async(&self, id: &str) -> Result<Option<Edition>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, name, slug, start_date, end_date, location, theme, status, created_at, updated_at FROM editions WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first().map(|r| Self::row_to_edition(r)).transpose()
    }

    fn row_to_edition(row: &HashMap<String, serde_json::Value>) -> Result<Edition, DbError> {
        Ok(Edition {
            id: Self::get_opt_string(row, "id"),
            name: Self::get_opt_string(row, "name"),
            slug: Self::get_opt_string(row, "slug"),
            start_date: Self::get_opt_string(row, "start_date"),
            end_date: Self::get_opt_string(row, "end_date"),
            location: Self::get_opt_string(row, "location"),
            theme: Self::get_opt_string(row, "theme"),
            status: Self::get_opt_string(row, "status"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Organisateurs
    // -----------------------------------------------------------------------

    /// Liste tous les organisateurs (synchrone).
    pub fn organisateurs_list(&self) -> Result<Vec<Organisateur>, DbError> {
        run_blocking(self.organisateurs_list_async())
    }

    async fn organisateurs_list_async(&self) -> Result<Vec<Organisateur>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, name, slug, region, description, contact_email, website, created_at, updated_at FROM organisateurs ORDER BY name",
                Vec::<String>::new(),
            )
            .await?;

        rows.iter().map(|r| Self::row_to_organisateur(r)).collect()
    }

    /// Récupère un organisateur par id (synchrone).
    pub fn organisateur_by_id(&self, id: &str) -> Result<Option<Organisateur>, DbError> {
        run_blocking(self.organisateur_by_id_async(id))
    }

    async fn organisateur_by_id_async(&self, id: &str) -> Result<Option<Organisateur>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, name, slug, region, description, contact_email, website, created_at, updated_at FROM organisateurs WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first()
            .map(|r| Self::row_to_organisateur(r))
            .transpose()
    }

    fn row_to_organisateur(
        row: &HashMap<String, serde_json::Value>,
    ) -> Result<Organisateur, DbError> {
        Ok(Organisateur {
            id: Self::get_opt_string(row, "id"),
            name: Self::get_opt_string(row, "name"),
            slug: Self::get_opt_string(row, "slug"),
            region: Self::get_opt_string(row, "region"),
            description: Self::get_opt_string(row, "description"),
            contact_email: Self::get_opt_string(row, "contact_email"),
            website: Self::get_opt_string(row, "website"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Exposants
    // -----------------------------------------------------------------------

    /// Liste les exposants (optionnellement filtrés par visible_repertoire) (synchrone).
    pub fn exposants_list(&self, visible_only: bool) -> Result<Vec<Exposant>, DbError> {
        run_blocking(self.exposants_list_async(visible_only))
    }

    async fn exposants_list_async(&self, visible_only: bool) -> Result<Vec<Exposant>, DbError> {
        let sql = if visible_only {
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants WHERE visible_repertoire = 1 ORDER BY company_name"
        } else {
            "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants ORDER BY company_name"
        };

        let rows = self.client.query(sql, Vec::<String>::new()).await?;
        rows.iter().map(|r| Self::row_to_exposant(r)).collect()
    }

    /// Récupère un exposant par id (synchrone).
    pub fn exposant_by_id(&self, id: &str) -> Result<Option<Exposant>, DbError> {
        run_blocking(self.exposant_by_id_async(id))
    }

    async fn exposant_by_id_async(&self, id: &str) -> Result<Option<Exposant>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, company_name, stand_name, contact_email, contact_phone, adresse, logo_url, site_web, siret, secteur, category, description, visible_repertoire, created_at, updated_at FROM exposants WHERE id = ?1",
                vec![id],
            )
            .await?;

        rows.first().map(|r| Self::row_to_exposant(r)).transpose()
    }

    fn row_to_exposant(row: &HashMap<String, serde_json::Value>) -> Result<Exposant, DbError> {
        Ok(Exposant {
            id: Self::get_opt_string(row, "id"),
            company_name: Self::get_opt_string(row, "company_name"),
            stand_name: Self::get_opt_string(row, "stand_name"),
            contact_email: Self::get_opt_string(row, "contact_email"),
            contact_phone: Self::get_opt_string(row, "contact_phone"),
            adresse: Self::get_opt_string(row, "adresse"),
            logo_url: Self::get_opt_string(row, "logo_url"),
            site_web: Self::get_opt_string(row, "site_web"),
            siret: Self::get_opt_string(row, "siret"),
            secteur: Self::get_opt_string(row, "secteur"),
            category: Self::get_opt_string(row, "category"),
            description: Self::get_opt_string(row, "description"),
            visible_repertoire: Self::get_opt_bool(row, "visible_repertoire"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    // -----------------------------------------------------------------------
    // Editions-Exposants
    // -----------------------------------------------------------------------

    /// Liste les participations d'une édition (synchrone).
    pub fn editions_exposants_by_edition(
        &self,
        edition_id: &str,
    ) -> Result<Vec<EditionExposant>, DbError> {
        run_blocking(self.editions_exposants_by_edition_async(edition_id))
    }

    async fn editions_exposants_by_edition_async(
        &self,
        edition_id: &str,
    ) -> Result<Vec<EditionExposant>, DbError> {
        let rows = self
            .client
            .query(
                "SELECT id, exposant_id, edition_id, is_accepted, is_validated, is_paid, assigned_stand, size_meters, created_at, updated_at FROM editions_exposants WHERE edition_id = ?1",
                vec![edition_id],
            )
            .await?;

        rows.iter()
            .map(|r| Self::row_to_edition_exposant(r))
            .collect()
    }

    fn row_to_edition_exposant(
        row: &HashMap<String, serde_json::Value>,
    ) -> Result<EditionExposant, DbError> {
        Ok(EditionExposant {
            id: Self::get_opt_string(row, "id"),
            exposant_id: Self::get_opt_string(row, "exposant_id"),
            edition_id: Self::get_opt_string(row, "edition_id"),
            is_accepted: Self::get_opt_bool(row, "is_accepted"),
            is_validated: Self::get_opt_bool(row, "is_validated"),
            is_paid: Self::get_opt_bool(row, "is_paid"),
            assigned_stand: Self::get_opt_string(row, "assigned_stand"),
            size_meters: Self::get_opt_f64(row, "size_meters"),
            created_at: Self::get_opt_string(row, "created_at"),
            updated_at: Self::get_opt_string(row, "updated_at"),
        })
    }

    /// Vérification de la santé de la connexion (synchrone).
    pub fn health_check(&self) -> Result<bool, DbError> {
        run_blocking(self.client.health_check()).map_err(DbError::from)
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn get_opt_string(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
        row.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    }

    fn get_opt_bool(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<bool> {
        row.get(key).and_then(|v| {
            if let Some(i) = v.as_i64() {
                Some(i != 0)
            } else if let Some(b) = v.as_bool() {
                Some(b)
            } else {
                None
            }
        })
    }

    fn get_opt_f64(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<f64> {
        row.get(key).and_then(|v| v.as_f64())
    }
}
