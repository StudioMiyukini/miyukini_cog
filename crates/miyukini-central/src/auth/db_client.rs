//! @id     central_auth_kindmother_client_db
//! @do     persist_central_auth_data_via_kindmother_client
//! @role   securite
//! @layer  infra
//! @human  miyukini-central — profils Central persistés via KindMother (seul écrivain, K-13).

//! Base de données des profils Miyukini Central via KindMother Client.
//!
//! Cette implémentation utilise exclusivement `kindmother-client` : KindMother est le
//! seul interlocuteur autorisé de la base `central` (K-13).
//!
//! - **Schéma** : migrations versionnées (`SchemaMigration`, module `schema_kindmother`) appliquées par le service,
//!   sous politique non destructive.
//! - **Écritures** : intentions typées uniquement ([`TypedWriteIntent`]) — aucun texte SQL
//!   d'écriture ne quitte ce module (EX-07). Les opérations multi-étapes passent par un lot
//!   atomique (`submit_batch`).
//! - **Lectures** : lectures typées ([`TypedSelect`]).
//!
//! L'API publique est **synchrone** et identique à celle du chemin legacy (`db.rs`).
//! En interne, les appels async s'exécutent sur un runtime Tokio global multi-thread.
//!
//! Différences assumées avec le legacy :
//! - le service active `PRAGMA foreign_keys` : lier une sauvegarde, une référence de
//!   service ou un hash Connect à un profil inexistant est refusé ;
//! - [`CentralAuthDb::update_profile_id`] déplace aussi les lignes liées (références,
//!   sauvegardes, hash Connect, session courante) vers le nouvel ID ;
//! - l'unicité du premier admin repose sur un jeton `cog_meta.first_admin` (voir `sign_up`).

use super::hachage::{hash_password_argon2, verify_password};
use super::{CentralProfile, CentralProfileSave, CentralProfileSaveRow};
use crate::auth::password::validate_password;
use kindmother::intent::MAX_INTENTS_PER_BATCH;
use kindmother_client::{
    ClientError, IntentOutcome, KindMotherClient, OperatorCredential, SqlValue,
    TypedSelect, TypedWriteIntent,
};
use std::collections::HashMap;
use std::future::Future;
use std::sync::{Arc, OnceLock};
use thiserror::Error;
use tokio::sync::OnceCell;

/// Ligne renvoyée par KindMother (colonne → valeur JSON).
type Row = HashMap<String, serde_json::Value>;

/// Erreur de la base auth Central (KindMother Client).
#[derive(Error, Debug)]
pub enum AuthDbError {
    /// Erreur du client KindMother (transport, refus ou échec d'une intention).
    #[error("KindMother client error: {0}")]
    Client(#[from] ClientError),

    /// Erreur de sérialisation.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Erreur interne (ligne mal formée, hachage…).
    #[error("Internal error: {0}")]
    Internal(String),

    /// Client non initialisé.
    #[error("Client not initialized")]
    NotInitialized,

    /// Erreur de validation (entrée refusée avant tout accès).
    #[error("Validation error: {0}")]
    Validation(String),

    /// Ligne attendue absente (équivalent legacy : « 0 ligne affectée »).
    #[error("Not found: {0}")]
    NotFound(String),

    /// Opération refusée (preuve d'administrateur invalide…).
    #[error("Refused: {0}")]
    Refused(String),
}

// ---------------------------------------------------------------------------
// Connexion
// ---------------------------------------------------------------------------

/// Opérateur KindMother de Central (Admin de la base `central`).
const OPERATOR_ID: &str = "miyukini-central";

/// Base KindMother de Central.
const DATABASE: &str = "central";

/// Variable d'environnement de l'adresse du service KindMother.
const ENV_KINDMOTHER_ADDR: &str = "KINDMOTHER_ADDR";

/// Adresse par défaut du service KindMother.
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";

/// Client global partagé (singleton) du chemin applicatif (`open`).
static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();

/// Runtime Tokio global (multi-thread) qui exécute tous les appels async.
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("central-auth-kindmother")
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

/// Exécute un future de manière bloquante sur le runtime global.
///
/// - Hors runtime : `block_on` direct sur le runtime global.
/// - Depuis un runtime (Dioxus, `#[tokio::test]`, current-thread ou multi-thread) :
///   `block_on` y paniquerait, et `block_in_place` panique sur un runtime current-thread ;
///   le future est donc exécuté sur un fil dédié (portée bornée) qui bloque sur le
///   runtime global. Les E/S du client restent ainsi toujours pilotées par le même runtime.
fn run_blocking<F>(f: F) -> F::Output
where
    F: Future + Send,
    F::Output: Send,
{
    if tokio::runtime::Handle::try_current().is_err() {
        return runtime().block_on(f);
    }
    std::thread::scope(|scope| {
        scope
            .spawn(|| runtime().block_on(f))
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    })
}

// ---------------------------------------------------------------------------
// Schéma
// ---------------------------------------------------------------------------

use super::schema_kindmother::migrations;

const T_PROFILES: &str = "central_profiles";
const T_META: &str = "cog_meta";
const T_REFS: &str = "profile_service_refs";
const T_SAVES: &str = "central_profile_saves";
const T_CONNECT: &str = "connect_credentials";

/// Clés de `cog_meta`.
const META_VIRGIN: &str = "cog_virgin";
const META_CURRENT_PROFILE: &str = "current_profile_id";
/// Jeton d'unicité du premier admin (clé primaire : un seul candidat peut l'insérer).
const META_FIRST_ADMIN: &str = "first_admin";

/// Colonnes d'un profil (hors `password_hash`, jamais renvoyé hors `sign_in`).
const PROFILE_COLUMNS: [&str; 20] = [
    "id",
    "email",
    "pseudonyme",
    "nom",
    "prenom",
    "date_naissance",
    "telephone",
    "numero_voie",
    "rue",
    "code_postal",
    "ville",
    "is_admin",
    "genre",
    "statut_marital",
    "partenaire_genre",
    "partenaire_nom",
    "enfants_nombre",
    "enfants_noms",
    "profession",
    "langue_maternelle",
];

/// Colonnes d'une sauvegarde complète.
const SAVE_COLUMNS: [&str; 7] = [
    "id",
    "profile_id",
    "service_key",
    "slot",
    "data",
    "created_at",
    "updated_at",
];

// ---------------------------------------------------------------------------
// Base
// ---------------------------------------------------------------------------

/// Base des profils Central via KindMother Client.
///
/// Fournit une API synchrone identique à celle du chemin legacy.
#[derive(Clone)]
pub struct CentralAuthDb {
    client: Arc<KindMotherClient>,
}

impl CentralAuthDb {
    /// Connexion injectable (tests, déploiements particuliers) : se connecte à `addr`
    /// avec `credential` sur la base `central`, puis applique les migrations.
    pub fn connect_with(addr: &str, credential: &OperatorCredential) -> Result<Self, AuthDbError> {
        run_blocking(async {
            let client = KindMotherClient::connect(addr, credential, DATABASE).await?;
            let db = Self {
                client: Arc::new(client),
            };
            db.init_schema_async().await?;
            Ok(db)
        })
    }

    /// Initialise la connexion globale au service KindMother (synchrone).
    ///
    /// `addr` : adresse explicite, sinon `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051`.
    /// Sans effet si la connexion globale existe déjà.
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), AuthDbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale au service KindMother (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), AuthDbError> {
        let addr = addr.map_or_else(
            || {
                std::env::var(ENV_KINDMOTHER_ADDR)
                    .unwrap_or_else(|_| DEFAULT_KINDMOTHER_ADDR.to_string())
            },
            str::to_string,
        );
        CLIENT
            .get_or_try_init(|| async {
                let credential = OperatorCredential::load(OPERATOR_ID)?;
                let client = KindMotherClient::connect(&addr, &credential, DATABASE).await?;
                Ok::<_, AuthDbError>(Arc::new(client))
            })
            .await?;
        Ok(())
    }

    /// Crée une nouvelle instance utilisant le client global.
    pub fn new() -> Result<Self, AuthDbError> {
        let client = CLIENT.get().ok_or(AuthDbError::NotInitialized)?.clone();
        Ok(Self { client })
    }

    /// Ouvre la base (API compatible avec le legacy) : `path` est ignoré, la base
    /// `central` est servie par KindMother (client global, migrations appliquées).
    pub fn open(path: impl AsRef<std::path::Path>) -> Result<Self, AuthDbError> {
        kindmother_client::heritage::refuser_si_base_heritee(path.as_ref())
            .map_err(AuthDbError::Refused)?;
        Self::init_global_sync(None)?;
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Applique les migrations de schéma (synchrone, idempotent).
    pub fn init_schema_sync(&self) -> Result<(), AuthDbError> {
        run_blocking(self.init_schema_async())
    }

    /// Applique les migrations de schéma (async, idempotent).
    pub async fn init_schema_async(&self) -> Result<(), AuthDbError> {
        self.client.apply_migrations(migrations()).await?;
        Ok(())
    }

    /// Vérification de la santé de la connexion (synchrone).
    pub fn health_check(&self) -> Result<bool, AuthDbError> {
        run_blocking(self.client.health_check()).map_err(AuthDbError::from)
    }

    // -----------------------------------------------------------------------
    // Profils
    // -----------------------------------------------------------------------

    /// Connexion : email + mot de passe. Retourne le profil complet si valide.
    pub fn sign_in(
        &self,
        email: &str,
        password: &str,
    ) -> Result<Option<CentralProfile>, AuthDbError> {
        run_blocking(self.sign_in_async(email, password))
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
            .select(
                TypedSelect::from(T_PROFILES)
                    .columns(PROFILE_COLUMNS.iter().copied().chain(["password_hash"]))
                    .filter("email", email)
                    .limit(1),
            )
            .await?;
        let Some(row) = rows.first() else {
            return Ok(None);
        };
        let hash = req_str(row, "password_hash")?;
        if !verify_password(password, &hash) {
            return Ok(None);
        }
        let profile = row_to_profile(row)?;
        // Migration progressive SHA-256 → Argon2id (échec non bloquant, comme le legacy).
        if !hash.starts_with("$argon2") {
            if let Ok(new_hash) = hash_password_argon2(password) {
                let upgrade = TypedWriteIntent::update(T_PROFILES)
                    .key("id", profile.id.as_str())
                    .value("password_hash", new_hash)
                    .purpose("central.auth.sign_in: migration du hash SHA-256 vers Argon2id");
                if let Err(e) = self.client.submit(upgrade).await {
                    tracing::warn!("migration du hash du profil {} : {e}", profile.id);
                }
            }
        }
        Ok(Some(profile))
    }

    /// Charge un profil par ID (pour édition dans la fenêtre Profil).
    pub fn get_profile(&self, id: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        run_blocking(self.get_profile_async(id))
    }

    async fn get_profile_async(&self, id: &str) -> Result<Option<CentralProfile>, AuthDbError> {
        let rows = self
            .select(
                TypedSelect::from(T_PROFILES)
                    .columns(PROFILE_COLUMNS)
                    .filter("id", id)
                    .limit(1),
            )
            .await?;
        rows.first().map(row_to_profile).transpose()
    }

    /// Enregistre les champs modifiables du profil en base.
    pub fn update_profile(&self, profile: &CentralProfile) -> Result<(), AuthDbError> {
        run_blocking(self.update_profile_async(profile))
    }

    async fn update_profile_async(&self, profile: &CentralProfile) -> Result<(), AuthDbError> {
        let now = chrono::Utc::now().to_rfc3339();
        let p = profile.clone();
        let intent = TypedWriteIntent::update(T_PROFILES)
            .key("id", p.id)
            .value("email", p.email)
            .value("pseudonyme", p.pseudonyme)
            .value("nom", p.nom)
            .value("prenom", p.prenom)
            .value("date_naissance", p.date_naissance)
            .value("telephone", p.telephone)
            .value("numero_voie", p.numero_voie)
            .value("rue", p.rue)
            .value("code_postal", p.code_postal)
            .value("ville", p.ville)
            .value("genre", p.genre)
            .value("statut_marital", p.statut_marital)
            .value("partenaire_genre", p.partenaire_genre)
            .value("partenaire_nom", p.partenaire_nom)
            .value("enfants_nombre", p.enfants_nombre)
            .value("enfants_noms", p.enfants_noms)
            .value("profession", p.profession)
            .value("langue_maternelle", p.langue_maternelle)
            .value("updated_at", now)
            .purpose("central.auth.update_profile: champs modifiables du profil");
        // Comme le legacy : aucun contrôle du nombre de lignes (profil absent = sans effet).
        self.client.submit(intent).await?;
        Ok(())
    }

    /// Liste tous les profils (id, email, is_admin pour la DB mère).
    pub fn list_profiles(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        run_blocking(self.list_profiles_async())
    }

    async fn list_profiles_async(&self) -> Result<Vec<CentralProfile>, AuthDbError> {
        let rows = self
            .select(TypedSelect::from(T_PROFILES).columns(["id", "email", "is_admin"]))
            .await?;
        rows.iter()
            .map(|row| {
                Ok(CentralProfile::minimal(
                    req_str(row, "id")?,
                    req_str(row, "email")?,
                    req_i64(row, "is_admin")? != 0,
                ))
            })
            .collect()
    }

    /// Met à jour l'ID d'un profil (après réattribution par la DB mère en cas de conflit).
    ///
    /// Les clés étrangères étant actives côté service (sans `ON UPDATE CASCADE`), la ligne
    /// est recréée sous le nouvel ID, les lignes liées y sont rattachées puis l'ancienne
    /// ligne est supprimée — le tout en **un lot atomique**.
    pub fn update_profile_id(&self, old_id: &str, new_id: &str) -> Result<(), AuthDbError> {
        run_blocking(self.update_profile_id_async(old_id, new_id))
    }

    async fn update_profile_id_async(&self, old_id: &str, new_id: &str) -> Result<(), AuthDbError> {
        let now = chrono::Utc::now().to_rfc3339();
        let rows = self
            .select(TypedSelect::from(T_PROFILES).filter("id", old_id).limit(1))
            .await?;
        let Some(row) = rows.into_iter().next() else {
            return Err(AuthDbError::NotFound(format!(
                "Profil non trouvé: {old_id}"
            )));
        };
        if old_id == new_id {
            let touch = TypedWriteIntent::update(T_PROFILES)
                .key("id", old_id)
                .value("updated_at", now)
                .purpose("central.auth.update_profile_id: ID inchangé");
            return self
                .expect_rows(touch, || format!("Profil non trouvé: {old_id}"))
                .await;
        }
        let email = req_str(&row, "email")?;
        let purpose = "central.auth.update_profile_id: réattribution d'ID par la DB mère";

        // 1. Libère l'email (UNIQUE) de l'ancienne ligne, supprimée en fin de lot.
        let free_email = TypedWriteIntent::update(T_PROFILES)
            .key("id", old_id)
            .value("email", format!("reattribution:{new_id}:{email}"))
            .purpose(purpose);
        // 2. Copie intégrale (toutes colonnes) sous le nouvel ID.
        let mut copy = TypedWriteIntent::insert(T_PROFILES).purpose(purpose);
        for (column, value) in &row {
            let value = match column.as_str() {
                "id" => SqlValue::from(new_id),
                "updated_at" => SqlValue::from(now.as_str()),
                _ => json_to_sql(column, value)?,
            };
            copy = copy.value(column.as_str(), value);
        }
        let mut batch = vec![free_email, copy];
        // 3. Rattache les lignes liées.
        for table in [T_REFS, T_SAVES, T_CONNECT] {
            batch.push(
                TypedWriteIntent::update(table)
                    .key("profile_id", old_id)
                    .value("profile_id", new_id)
                    .purpose(purpose),
            );
        }
        batch.push(
            TypedWriteIntent::update(T_META)
                .key("key", META_CURRENT_PROFILE)
                .key("value", old_id)
                .value("value", new_id)
                .purpose(purpose),
        );
        // 4. Supprime l'ancienne ligne (plus aucune référence).
        batch.push(
            TypedWriteIntent::delete(T_PROFILES)
                .key("id", old_id)
                .purpose(purpose),
        );
        let outcomes = self.client.submit_batch(batch).await?;
        if outcomes.last().map_or(0, |o| o.rows_affected) == 0 {
            // Profil supprimé entre la lecture et le lot : rien de cohérent n'a été déplacé.
            return Err(AuthDbError::NotFound(format!(
                "Profil non trouvé: {old_id}"
            )));
        }
        Ok(())
    }

    /// Création de compte : email + mot de passe complexe. Retourne le profil créé.
    /// Le premier compte créé est admin de l'environnement (MiyukiniAdmin).
    /// `pseudonyme` : optionnel, utilisé pour le Rite d'Entrée (nom / pseudo).
    ///
    /// Unicité du premier admin : le legacy compte les profils dans une transaction
    /// IMMEDIATE. Ici, le candidat « premier compte » insère (INSERT simple, pas upsert)
    /// le jeton `cog_meta.first_admin` **dans le même lot atomique** que son profil admin :
    /// un candidat concurrent échoue sur la clé primaire, tout son lot est annulé, et il
    /// est alors créé comme simple compte. Le jeton est retiré par `reset_cog_to_virgin`.
    pub fn sign_up(
        &self,
        email: &str,
        password: &str,
        pseudonyme: Option<&str>,
    ) -> Result<CentralProfile, AuthDbError> {
        run_blocking(self.sign_up_async(email, password, pseudonyme))
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
        let id = uuid::Uuid::new_v4().to_string();
        let hash = hash_password_argon2(password)
            .map_err(|e| AuthDbError::Internal(format!("hash error: {e}")))?;
        let now = chrono::Utc::now().to_rfc3339();
        let pseudo_val = pseudonyme.map(str::trim).filter(|s| !s.is_empty());

        let insert_profile = |is_admin: bool| {
            TypedWriteIntent::insert(T_PROFILES)
                .value("id", id.as_str())
                .value("email", email)
                .value("password_hash", hash.as_str())
                .value("created_at", now.as_str())
                .value("updated_at", now.as_str())
                .value("pseudonyme", pseudo_val)
                .value("is_admin", i64::from(is_admin))
                .purpose("central.auth.sign_up: création de compte")
        };

        let table_empty = self
            .select(TypedSelect::from(T_PROFILES).columns(["id"]).limit(1))
            .await?
            .is_empty();
        let is_admin = if table_empty {
            let first = vec![
                TypedWriteIntent::insert(T_META)
                    .value("key", META_FIRST_ADMIN)
                    .value("value", id.as_str())
                    .purpose("central.auth.sign_up: jeton d'unicité du premier admin"),
                insert_profile(true),
                TypedWriteIntent::upsert(T_META)
                    .key("key", META_VIRGIN)
                    .value("value", "0")
                    .purpose("central.auth.sign_up: le COG n'est plus vierge"),
            ];
            match self.client.submit_batch(first).await {
                Ok(_) => true,
                Err(e) => {
                    // Jeton déjà pris (inscription concurrente) : lot annulé, compte simple.
                    tracing::debug!("premier admin non attribué ({e}) : création en compte simple");
                    self.client.submit(insert_profile(false)).await?;
                    false
                }
            }
        } else {
            self.client.submit(insert_profile(false)).await?;
            false
        };

        let mut profile = CentralProfile::minimal(id, email.to_string(), is_admin);
        profile.pseudonyme = pseudo_val.map(String::from);
        Ok(profile)
    }

    // -----------------------------------------------------------------------
    // État du COG (cog_meta)
    // -----------------------------------------------------------------------

    /// Indique si le COG est vierge (aucun compte créé). Passe à false à la création du premier compte.
    pub fn is_cog_virgin(&self) -> Result<bool, AuthDbError> {
        run_blocking(self.is_cog_virgin_async())
    }

    async fn is_cog_virgin_async(&self) -> Result<bool, AuthDbError> {
        Ok(self.get_meta(META_VIRGIN).await?.is_none_or(|v| v == "1"))
    }

    /// Marque le COG comme vierge ou non (appelé à la création du premier compte).
    pub fn set_cog_virgin(&self, virgin: bool) -> Result<(), AuthDbError> {
        run_blocking(self.set_cog_virgin_async(virgin))
    }

    /// Alias historique de [`Self::set_cog_virgin`].
    pub fn set_cog_virgin_sync(&self, virgin: bool) -> Result<(), AuthDbError> {
        self.set_cog_virgin(virgin)
    }

    async fn set_cog_virgin_async(&self, virgin: bool) -> Result<(), AuthDbError> {
        let intent = TypedWriteIntent::upsert(T_META)
            .key("key", META_VIRGIN)
            .value("value", if virgin { "1" } else { "0" })
            .purpose("central.auth.set_cog_virgin");
        self.client.submit(intent).await?;
        Ok(())
    }

    /// ID du profil actuellement connecté (session persistée).
    pub fn get_current_profile_id(&self) -> Result<Option<String>, AuthDbError> {
        run_blocking(self.get_meta(META_CURRENT_PROFILE))
    }

    /// Enregistre le profil connecté (ou None pour déconnexion).
    pub fn set_current_profile_id(&self, profile_id: Option<&str>) -> Result<(), AuthDbError> {
        run_blocking(self.set_current_profile_id_async(profile_id))
    }

    async fn set_current_profile_id_async(
        &self,
        profile_id: Option<&str>,
    ) -> Result<(), AuthDbError> {
        let intent = match profile_id {
            Some(id) => TypedWriteIntent::upsert(T_META)
                .key("key", META_CURRENT_PROFILE)
                .value("value", id)
                .purpose("central.auth.set_current_profile_id: connexion"),
            None => TypedWriteIntent::delete(T_META)
                .key("key", META_CURRENT_PROFILE)
                .purpose("central.auth.set_current_profile_id: déconnexion"),
        };
        self.client.submit(intent).await?;
        Ok(())
    }

    /// Réinitialise le COG à l'état vierge (suppression de tous les comptes).
    ///
    /// Opération destructrice réservée à un administrateur : exige l'e-mail et le
    /// mot de passe d'un profil `is_admin` (ré-authentification). Toute preuve
    /// invalide, ou un profil non admin, est refusée sans rien supprimer.
    ///
    /// Les intentions typées interdisant toute suppression sans clé, les lignes sont
    /// d'abord lues puis supprimées **une à une par clé**, enfants avant parents (clés
    /// étrangères), dans un lot atomique. Au-delà de `MAX_INTENTS_PER_BATCH` intentions,
    /// le lot est découpé dans le même ordre : chaque tranche reste atomique et un nouvel
    /// appel termine le travail en cas d'interruption. Une ligne insérée entre la lecture
    /// et le lot n'est pas supprimée (comme toute écriture postérieure à la réinitialisation).
    pub fn reset_cog_to_virgin(
        &self,
        admin_email: &str,
        admin_password: &str,
    ) -> Result<(), AuthDbError> {
        run_blocking(self.reset_cog_to_virgin_async(admin_email, admin_password))
    }

    async fn reset_cog_to_virgin_async(
        &self,
        admin_email: &str,
        admin_password: &str,
    ) -> Result<(), AuthDbError> {
        // Preuve d'admin.
        match self.sign_in_async(admin_email, admin_password).await? {
            Some(p) if p.is_admin => {}
            Some(_) => {
                return Err(AuthDbError::Refused(
                    "Réinitialisation refusée : droits administrateur requis.".into(),
                ))
            }
            None => {
                return Err(AuthDbError::Refused(
                    "Réinitialisation refusée : authentification invalide.".into(),
                ))
            }
        }
        let purpose = "central.auth.reset_cog_to_virgin: réinitialisation par un administrateur";
        let mut batch = Vec::new();
        for row in self
            .select(TypedSelect::from(T_SAVES).columns(["id"]))
            .await?
        {
            batch.push(
                TypedWriteIntent::delete(T_SAVES)
                    .key("id", req_str(&row, "id")?)
                    .purpose(purpose),
            );
        }
        for row in self
            .select(TypedSelect::from(T_REFS).columns(["profile_id", "service_key"]))
            .await?
        {
            batch.push(
                TypedWriteIntent::delete(T_REFS)
                    .key("profile_id", req_str(&row, "profile_id")?)
                    .key("service_key", req_str(&row, "service_key")?)
                    .purpose(purpose),
            );
        }
        for row in self
            .select(TypedSelect::from(T_CONNECT).columns(["profile_id"]))
            .await?
        {
            batch.push(
                TypedWriteIntent::delete(T_CONNECT)
                    .key("profile_id", req_str(&row, "profile_id")?)
                    .purpose(purpose),
            );
        }
        for row in self
            .select(TypedSelect::from(T_PROFILES).columns(["id"]))
            .await?
        {
            batch.push(
                TypedWriteIntent::delete(T_PROFILES)
                    .key("id", req_str(&row, "id")?)
                    .purpose(purpose),
            );
        }
        batch.push(
            TypedWriteIntent::upsert(T_META)
                .key("key", META_VIRGIN)
                .value("value", "1")
                .purpose(purpose),
        );
        for key in [META_CURRENT_PROFILE, META_FIRST_ADMIN] {
            batch.push(
                TypedWriteIntent::delete(T_META)
                    .key("key", key)
                    .purpose(purpose),
            );
        }
        let mut rest = batch;
        while !rest.is_empty() {
            let tail = rest.split_off(rest.len().min(MAX_INTENTS_PER_BATCH));
            self.client.submit_batch(rest).await?;
            rest = tail;
        }
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Hash Miyukini Connect
    // -----------------------------------------------------------------------

    /// Enregistre (ou met a jour) un hash Argon2id Miyukini Connect pour un profil.
    pub fn set_connect_password_hash(
        &self,
        profile_id: &str,
        password_hash: &str,
    ) -> Result<(), AuthDbError> {
        if !password_hash.starts_with("$argon2id$") {
            return Err(AuthDbError::Validation(
                "Le hash Connect doit etre au format Argon2id.".to_string(),
            ));
        }
        let intent = TypedWriteIntent::upsert(T_CONNECT)
            .key("profile_id", profile_id)
            .value("password_hash", password_hash)
            .value("updated_at", chrono::Utc::now().to_rfc3339())
            .purpose("central.auth.set_connect_password_hash");
        run_blocking(self.client.submit(intent))?;
        Ok(())
    }

    /// Retourne le hash Argon2id Miyukini Connect associe a un profil.
    pub fn get_connect_password_hash(
        &self,
        profile_id: &str,
    ) -> Result<Option<String>, AuthDbError> {
        run_blocking(
            self.select_one_str(
                TypedSelect::from(T_CONNECT)
                    .columns(["password_hash"])
                    .filter("profile_id", profile_id),
                "password_hash",
            ),
        )
    }

    // -----------------------------------------------------------------------
    // Références de service
    // -----------------------------------------------------------------------

    /// Retourne l’ID de référence du profil vers la table du service (ex. id de sauvegarde de jeu).
    /// Chaque service utilise une `service_key` unique (ex. `"jaykoa"`, `"jaykonta"`).
    pub fn get_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Option<String>, AuthDbError> {
        run_blocking(
            self.select_one_str(
                TypedSelect::from(T_REFS)
                    .columns(["ref_id"])
                    .filter("profile_id", profile_id)
                    .filter("service_key", service_key),
                "ref_id",
            ),
        )
    }

    /// Lie le profil à une ligne de la table du service (ex. sauvegarde de jeu).
    /// `ref_id` = id de la ligne dans la table dédiée au service. Passer `None` pour supprimer le lien.
    pub fn set_profile_service_ref(
        &self,
        profile_id: &str,
        service_key: &str,
        ref_id: Option<&str>,
    ) -> Result<(), AuthDbError> {
        let intent = match ref_id {
            Some(id) => TypedWriteIntent::upsert(T_REFS)
                .key("profile_id", profile_id)
                .key("service_key", service_key)
                .value("ref_id", id)
                .purpose("central.auth.set_profile_service_ref: lien"),
            None => TypedWriteIntent::delete(T_REFS)
                .key("profile_id", profile_id)
                .key("service_key", service_key)
                .purpose("central.auth.set_profile_service_ref: suppression du lien"),
        };
        run_blocking(self.client.submit(intent))?;
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
        let intent = TypedWriteIntent::insert(T_SAVES)
            .value("id", id.as_str())
            .value("profile_id", profile_id)
            .value("service_key", service_key)
            .value("slot", slot)
            .value("data", data.to_vec())
            .value("created_at", now.as_str())
            .value("updated_at", now.as_str())
            .purpose("central.auth.insert_profile_save");
        run_blocking(self.client.submit(intent))?;
        Ok(id)
    }

    /// Charge une sauvegarde par id.
    pub fn get_profile_save(&self, id: &str) -> Result<Option<CentralProfileSave>, AuthDbError> {
        run_blocking(
            self.select_save(
                TypedSelect::from(T_SAVES)
                    .columns(SAVE_COLUMNS)
                    .filter("id", id)
                    .limit(1),
            ),
        )
    }

    /// Charge une sauvegarde par (profile_id, service_key, slot).
    pub fn get_profile_save_by_slot(
        &self,
        profile_id: &str,
        service_key: &str,
        slot: i64,
    ) -> Result<Option<CentralProfileSave>, AuthDbError> {
        run_blocking(
            self.select_save(
                TypedSelect::from(T_SAVES)
                    .columns(SAVE_COLUMNS)
                    .filter("profile_id", profile_id)
                    .filter("service_key", service_key)
                    .filter("slot", slot)
                    .limit(1),
            ),
        )
    }

    /// Met à jour le blob et updated_at d’une sauvegarde existante.
    pub fn update_profile_save(&self, id: &str, data: &[u8]) -> Result<(), AuthDbError> {
        let intent = TypedWriteIntent::update(T_SAVES)
            .key("id", id)
            .value("data", data.to_vec())
            .value("updated_at", chrono::Utc::now().to_rfc3339())
            .purpose("central.auth.update_profile_save");
        run_blocking(self.expect_rows(intent, || format!("Sauvegarde non trouvée: {id}")))
    }

    /// Liste les sauvegardes (sans BLOB) pour un profil et un service.
    pub fn list_profile_saves(
        &self,
        profile_id: &str,
        service_key: &str,
    ) -> Result<Vec<CentralProfileSaveRow>, AuthDbError> {
        let rows = run_blocking(
            self.select(
                TypedSelect::from(T_SAVES)
                    .columns(["id", "slot", "created_at", "updated_at"])
                    .filter("profile_id", profile_id)
                    .filter("service_key", service_key)
                    .order_by("slot", false),
            ),
        )?;
        rows.iter()
            .map(|row| {
                Ok(CentralProfileSaveRow {
                    id: req_str(row, "id")?,
                    slot: req_i64(row, "slot")?,
                    created_at: req_str(row, "created_at")?,
                    updated_at: req_str(row, "updated_at")?,
                })
            })
            .collect()
    }

    /// Supprime une sauvegarde par id. Ne modifie pas `profile_service_refs` (à faire côté appelant si ref_id pointait ici).
    pub fn delete_profile_save(&self, id: &str) -> Result<(), AuthDbError> {
        let intent = TypedWriteIntent::delete(T_SAVES)
            .key("id", id)
            .purpose("central.auth.delete_profile_save");
        run_blocking(self.expect_rows(intent, || format!("Sauvegarde non trouvée: {id}")))
    }

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    async fn select(&self, query: TypedSelect) -> Result<Vec<Row>, AuthDbError> {
        Ok(self.client.select(query).await?)
    }

    /// Première ligne de `query`, colonne texte `column`.
    async fn select_one_str(
        &self,
        query: TypedSelect,
        column: &str,
    ) -> Result<Option<String>, AuthDbError> {
        let rows = self.select(query.limit(1)).await?;
        rows.first().map(|r| req_str(r, column)).transpose()
    }

    async fn get_meta(&self, key: &str) -> Result<Option<String>, AuthDbError> {
        self.select_one_str(
            TypedSelect::from(T_META)
                .columns(["value"])
                .filter("key", key),
            "value",
        )
        .await
    }

    async fn select_save(
        &self,
        query: TypedSelect,
    ) -> Result<Option<CentralProfileSave>, AuthDbError> {
        let rows = self.select(query).await?;
        rows.first().map(row_to_profile_save).transpose()
    }

    /// Soumet `intent` ; `NotFound` si aucune ligne n'est affectée (sémantique legacy).
    async fn expect_rows(
        &self,
        intent: TypedWriteIntent,
        not_found: impl FnOnce() -> String + Send,
    ) -> Result<(), AuthDbError> {
        let outcome: IntentOutcome = self.client.submit(intent).await?;
        if outcome.rows_affected == 0 {
            return Err(AuthDbError::NotFound(not_found()));
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Conversion des lignes
// ---------------------------------------------------------------------------

fn row_to_profile(row: &Row) -> Result<CentralProfile, AuthDbError> {
    let enfants_nombre = opt_i64(row, "enfants_nombre")?
        .map(i32::try_from)
        .transpose()
        .map_err(|_| AuthDbError::Internal("enfants_nombre hors limites".into()))?;
    Ok(CentralProfile {
        id: req_str(row, "id")?,
        email: req_str(row, "email")?,
        pseudonyme: opt_str(row, "pseudonyme")?,
        nom: opt_str(row, "nom")?,
        prenom: opt_str(row, "prenom")?,
        date_naissance: opt_str(row, "date_naissance")?,
        telephone: opt_str(row, "telephone")?,
        numero_voie: opt_str(row, "numero_voie")?,
        rue: opt_str(row, "rue")?,
        code_postal: opt_str(row, "code_postal")?,
        ville: opt_str(row, "ville")?,
        is_admin: req_i64(row, "is_admin")? != 0,
        genre: opt_str(row, "genre")?,
        statut_marital: opt_str(row, "statut_marital")?,
        partenaire_genre: opt_str(row, "partenaire_genre")?,
        partenaire_nom: opt_str(row, "partenaire_nom")?,
        enfants_nombre,
        enfants_noms: opt_str(row, "enfants_noms")?,
        profession: opt_str(row, "profession")?,
        langue_maternelle: opt_str(row, "langue_maternelle")?,
    })
}

fn row_to_profile_save(row: &Row) -> Result<CentralProfileSave, AuthDbError> {
    use base64::Engine;
    // KindMother transporte les BLOB en base64 (alphabet standard, avec remplissage).
    let data = base64::engine::general_purpose::STANDARD
        .decode(req_str(row, "data")?)
        .map_err(|e| AuthDbError::Internal(format!("data de sauvegarde illisible : {e}")))?;
    Ok(CentralProfileSave {
        id: req_str(row, "id")?,
        profile_id: req_str(row, "profile_id")?,
        service_key: req_str(row, "service_key")?,
        slot: req_i64(row, "slot")?,
        data,
        created_at: req_str(row, "created_at")?,
        updated_at: req_str(row, "updated_at")?,
    })
}

fn bad_column(column: &str, expected: &str) -> AuthDbError {
    AuthDbError::Internal(format!("colonne {column} : {expected} attendu"))
}

/// Texte non nul (équivalent de `row.get::<_, String>`).
fn req_str(row: &Row, column: &str) -> Result<String, AuthDbError> {
    opt_str(row, column)?.ok_or_else(|| bad_column(column, "texte non nul"))
}

/// Texte nullable (chaîne vide conservée, comme le legacy).
fn opt_str(row: &Row, column: &str) -> Result<Option<String>, AuthDbError> {
    match row.get(column) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(s)) => Ok(Some(s.clone())),
        Some(_) => Err(bad_column(column, "texte")),
    }
}

/// Entier non nul.
fn req_i64(row: &Row, column: &str) -> Result<i64, AuthDbError> {
    opt_i64(row, column)?.ok_or_else(|| bad_column(column, "entier non nul"))
}

/// Entier nullable.
fn opt_i64(row: &Row, column: &str) -> Result<Option<i64>, AuthDbError> {
    match row.get(column) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(v) => v
            .as_i64()
            .map(Some)
            .ok_or_else(|| bad_column(column, "entier")),
    }
}

/// Valeur JSON d'une ligne lue → valeur typée réinsérable (copie de ligne).
fn json_to_sql(column: &str, value: &serde_json::Value) -> Result<SqlValue, AuthDbError> {
    match value {
        serde_json::Value::Null => Ok(SqlValue::Null),
        serde_json::Value::Bool(b) => Ok(SqlValue::from(*b)),
        serde_json::Value::String(s) => Ok(SqlValue::from(s.as_str())),
        serde_json::Value::Number(n) => n
            .as_i64()
            .map(SqlValue::from)
            .or_else(|| n.as_f64().map(SqlValue::from))
            .ok_or_else(|| bad_column(column, "nombre représentable")),
        _ => Err(bad_column(column, "scalaire")),
    }
}
