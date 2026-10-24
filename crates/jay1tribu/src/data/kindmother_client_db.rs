//! Base de données Jay1Tribu via KindMother Client (feature kindmother-only, défaut).
//!
//! @id     jay1tribu_kindmother_client_db
//! @do     persist_jay1tribu_data_via_kindmother_client
//! @role   persistence
//! @layer  infra
//! @human  Délégation exclusive au service KindMother (K-13 : seul écrivain) ; aucun
//!         accès SQLite direct. Schéma = migrations versionnées ; écritures = intentions
//!         typées (EX-07 : aucun texte SQL d'écriture). API synchrone pour le domaine.
//!
//! Différences sémantiques avec le chemin HÉRITÉ (`kindmother_db.rs`) :
//! - KindMother active `PRAGMA foreign_keys = ON` : les suppressions en cascade
//!   (`tribe_delete`, `salon_delete`) retirent aussi les lignes dépendantes que le chemin
//!   HÉRITÉ laissait orphelines (livraisons en attente, pièces jointes, invitations),
//!   sinon la contrainte refuserait la suppression ; elles sont atomiques (lot unique).
//! - `tribe_create` et `tribe_invitation_accept` sont atomiques (`submit_batch`).
//! - Horodatages calculés côté client (RFC 3339, chrono), comme le chemin HÉRITÉ.
//! - `None` est écrit `NULL` (et non chaîne vide comme l'ancien client).
//!
//! Erreurs : toute méthode retourne `DbError` si le service est injoignable, refuse
//! l'intention (schéma, droits) ou échoue — d'où l'absence de section `# Errors`
//! répétée sur chaque méthode (API alignée sur le chemin HÉRITÉ).
#![allow(clippy::missing_errors_doc)]

use crate::data::types::{Friend, Message, Salon, SalonType, Tribe};
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{
    ClientError, IntentOutcome, KindMotherClient, OperatorCredential, SqlValue,
    TypedSelect, TypedWriteIntent,
};
use std::collections::HashMap;
use std::future::Future;
use std::path::Path;
use std::sync::{Arc, OnceLock};

/// Erreur de la base Jay1Tribu (KindMother Client).
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jay1Tribu DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<ClientError> for DbError {
    fn from(e: ClientError) -> Self {
        DbError(e.to_string())
    }
}

/// Adresse par défaut du service KindMother (surchargée par `KINDMOTHER_ADDR`).
const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";
/// Variable d'environnement de l'adresse du service KindMother.
const ENV_KINDMOTHER_ADDR: &str = "KINDMOTHER_ADDR";
/// Opérateur et base KindMother de Jay1Tribu.
const DB_NAME: &str = "jay1tribu";

use super::schema_kindmother::migrations;

type Row = HashMap<String, serde_json::Value>;

static CLIENT: OnceLock<Arc<KindMotherClient>> = OnceLock::new();
static RUNTIME: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

/// Runtime global (multi-thread) des appels bloquants hors contexte Tokio.
fn runtime() -> &'static tokio::runtime::Runtime {
    RUNTIME.get_or_init(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("Jay1Tribu : création du runtime Tokio impossible")
    })
}

/// Exécute un future de manière bloquante, quel que soit le contexte appelant.
///
/// - hors runtime : runtime global ;
/// - runtime multi-thread (Dioxus, tests multi_thread) : `block_in_place` ;
/// - runtime current-thread : `block_in_place` y paniquerait ; le future est exécuté
///   par le runtime global depuis un fil dédié (aucune récursion, aucune panique).
fn run_blocking<F>(f: F) -> F::Output
where
    F: Future + Send,
    F::Output: Send,
{
    use tokio::runtime::{Handle, RuntimeFlavor};
    match Handle::try_current() {
        Ok(handle) if handle.runtime_flavor() == RuntimeFlavor::MultiThread => {
            tokio::task::block_in_place(|| handle.block_on(f))
        }
        Ok(_) => std::thread::scope(|s| {
            s.spawn(|| runtime().block_on(f))
                .join()
                .unwrap_or_else(|e| std::panic::resume_unwind(e))
        }),
        Err(_) => runtime().block_on(f),
    }
}

/// Adresse du service : argument, sinon `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051`.
fn resolve_addr(addr: Option<&str>) -> String {
    addr.map(String::from)
        .or_else(|| std::env::var(ENV_KINDMOTHER_ADDR).ok())
        .filter(|a| !a.trim().is_empty())
        .unwrap_or_else(|| DEFAULT_KINDMOTHER_ADDR.to_string())
}

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn new_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

fn row_str(row: &Row, key: &str) -> String {
    row.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn row_opt_str(row: &Row, key: &str) -> Option<String> {
    row.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn row_to_friend(row: &Row) -> Friend {
    Friend {
        id: row_str(row, "id"),
        profile_id: row_str(row, "profile_id"),
        friend_cog_id: row_str(row, "friend_cog_id"),
        friend_pseudo: row_opt_str(row, "friend_pseudo"),
        created_at: row_str(row, "created_at"),
    }
}

fn row_to_tribe(row: &Row) -> Tribe {
    Tribe {
        id: row_str(row, "id"),
        name: row_str(row, "name"),
        description: row_opt_str(row, "description"),
        creator_cog_id: row_str(row, "creator_cog_id"),
        created_at: row_str(row, "created_at"),
        updated_at: row_str(row, "updated_at"),
    }
}

fn row_to_salon(row: &Row) -> Salon {
    Salon {
        id: row_str(row, "id"),
        tribe_id: row_opt_str(row, "tribe_id"),
        name: row_str(row, "name"),
        salon_type: SalonType::from_str(&row_str(row, "salon_type")),
        created_at: row_str(row, "created_at"),
    }
}

fn row_to_message(row: &Row) -> Message {
    Message {
        id: row_str(row, "id"),
        salon_id: row_str(row, "salon_id"),
        sender_cog_id: row_str(row, "sender_cog_id"),
        content: row_str(row, "content"),
        created_at: row_str(row, "created_at"),
    }
}

const FRIEND_COLS: [&str; 5] = [
    "id",
    "profile_id",
    "friend_cog_id",
    "friend_pseudo",
    "created_at",
];
const TRIBE_COLS: [&str; 6] = [
    "id",
    "name",
    "description",
    "creator_cog_id",
    "created_at",
    "updated_at",
];
const SALON_COLS: [&str; 5] = ["id", "tribe_id", "name", "salon_type", "created_at"];
const MESSAGE_COLS: [&str; 5] = ["id", "salon_id", "sender_cog_id", "content", "created_at"];

/// Base de données Jay1Tribu via KindMother Client.
#[derive(Clone)]
pub struct Jay1TribuDb {
    client: Arc<KindMotherClient>,
    pub instance: InstanceIdentity,
}

impl Jay1TribuDb {
    // ---------- Connexion ----------

    /// Initialise la connexion globale au service KindMother (synchrone).
    ///
    /// Adresse : `addr`, sinon `KINDMOTHER_ADDR`, sinon `127.0.0.1:50051` ;
    /// justificatif : `OperatorCredential::load("jay1tribu")`.
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = resolve_addr(addr);
        let cred = OperatorCredential::load(DB_NAME)?;
        let client = KindMotherClient::connect(&addr, &cred, DB_NAME).await?;
        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError("Client déjà initialisé".to_string()))?;
        Ok(())
    }

    /// Crée une instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT
            .get()
            .ok_or(DbError("Client KindMother non initialisé".to_string()))?
            .clone();
        Ok(Self::from_client(client))
    }

    fn from_client(client: Arc<KindMotherClient>) -> Self {
        Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        }
    }

    /// Ouvre la base (compatible API HÉRITÉE ; `_path` ignoré) : initialise le client
    /// global si besoin, crée l'instance et applique les migrations.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        kindmother_client::heritage::refuser_si_base_heritee(path.as_ref()).map_err(DbError)?;
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Connexion injectée (tests, outils) : client dédié hors singleton global,
    /// migrations appliquées (synchrone).
    pub fn connect_with(addr: &str, cred: &OperatorCredential) -> Result<Self, DbError> {
        run_blocking(Self::connect_with_async(addr, cred))
    }

    /// Connexion injectée (async) ; voir [`Self::connect_with`].
    pub async fn connect_with_async(
        addr: &str,
        cred: &OperatorCredential,
    ) -> Result<Self, DbError> {
        let client = KindMotherClient::connect(addr, cred, DB_NAME).await?;
        let db = Self::from_client(Arc::new(client));
        db.client.apply_migrations(migrations()).await?;
        Ok(db)
    }

    /// Applique les migrations versionnées (idempotent, synchrone).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.client.apply_migrations(migrations()))?;
        Ok(())
    }

    // ---------- Accès KindMother ----------

    fn submit(&self, intent: TypedWriteIntent) -> Result<IntentOutcome, DbError> {
        Ok(run_blocking(self.client.submit(intent))?)
    }

    fn submit_batch(&self, intents: Vec<TypedWriteIntent>) -> Result<Vec<IntentOutcome>, DbError> {
        Ok(run_blocking(self.client.submit_batch(intents))?)
    }

    fn select(&self, query: TypedSelect) -> Result<Vec<Row>, DbError> {
        Ok(run_blocking(self.client.select(query))?)
    }

    /// Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée.
    fn query_raw(&self, sql: &str, params: Vec<String>) -> Result<Vec<Row>, DbError> {
        Ok(run_blocking(self.client.query(sql, params))?)
    }

    // ---------- Friends ----------

    pub fn friend_list(&self, profile_id: &str) -> Result<Vec<Friend>, DbError> {
        let rows = self.select(
            TypedSelect::from("friends")
                .columns(FRIEND_COLS)
                .filter("profile_id", profile_id)
                .order_by("friend_pseudo", false)
                .order_by("created_at", false),
        )?;
        Ok(rows.iter().map(row_to_friend).collect())
    }

    pub fn friend_create(
        &self,
        profile_id: &str,
        friend_cog_id: &str,
        friend_pseudo: Option<&str>,
    ) -> Result<Friend, DbError> {
        if self.is_friend(profile_id, friend_cog_id)? {
            return Err(DbError(
                "Cet ami est déjà dans la liste (profile_id, friend_cog_id) uniques.".to_string(),
            ));
        }
        let id = new_id();
        let created_at = now_rfc3339();
        self.submit(
            TypedWriteIntent::insert("friends")
                .value("id", id.as_str())
                .value("profile_id", profile_id)
                .value("friend_cog_id", friend_cog_id)
                .value("friend_pseudo", friend_pseudo)
                .value("created_at", created_at.as_str())
                .purpose("friend_create"),
        )?;
        Ok(Friend {
            id,
            profile_id: profile_id.to_string(),
            friend_cog_id: friend_cog_id.to_string(),
            friend_pseudo: friend_pseudo.map(String::from),
            created_at,
        })
    }

    pub fn friend_delete(&self, id: &str) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::delete("friends")
                .key("id", id)
                .purpose("friend_delete"),
        )?;
        Ok(())
    }

    /// Vérifie si `friend_cog_id` est dans la liste d'amis de `profile_id`.
    pub fn is_friend(&self, profile_id: &str, friend_cog_id: &str) -> Result<bool, DbError> {
        let rows = self.select(
            TypedSelect::from("friends")
                .columns(["id"])
                .filter("profile_id", profile_id)
                .filter("friend_cog_id", friend_cog_id)
                .limit(1),
        )?;
        Ok(!rows.is_empty())
    }

    // ---------- Tribes ----------

    pub fn tribe_list(&self, cog_id: &str) -> Result<Vec<Tribe>, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT t.id, t.name, t.description, t.creator_cog_id, t.created_at, t.updated_at
             FROM tribes t INNER JOIN tribe_members m ON t.id = m.tribe_id WHERE m.cog_id = ?1 ORDER BY t.name",
            vec![cog_id.to_string()],
        )?;
        Ok(rows.iter().map(row_to_tribe).collect())
    }

    pub fn tribe_by_id(&self, id: &str) -> Result<Option<Tribe>, DbError> {
        let rows = self.select(
            TypedSelect::from("tribes")
                .columns(TRIBE_COLS)
                .filter("id", id)
                .limit(1),
        )?;
        Ok(rows.first().map(row_to_tribe))
    }

    /// Crée une tribu, ses rôles « Chef de tribu » et « Membre » et inscrit le créateur
    /// comme chef — en un lot atomique.
    pub fn tribe_create(
        &self,
        name: &str,
        description: Option<&str>,
        creator_cog_id: &str,
    ) -> Result<Tribe, DbError> {
        let id = new_id();
        let now = now_rfc3339();
        let chef_role_id = new_id();
        let role = |role_id: &str, role_name: &str| {
            TypedWriteIntent::insert("tribe_roles")
                .value("id", role_id)
                .value("tribe_id", id.as_str())
                .value("name", role_name)
                .value("permissions_json", SqlValue::Null)
                .value("created_at", now.as_str())
                .purpose("tribe_create_role")
        };
        self.submit_batch(vec![
            TypedWriteIntent::insert("tribes")
                .value("id", id.as_str())
                .value("name", name)
                .value("description", description)
                .value("creator_cog_id", creator_cog_id)
                .value("created_at", now.as_str())
                .value("updated_at", now.as_str())
                .purpose("tribe_create"),
            role(&chef_role_id, "Chef de tribu"),
            role(&new_id(), "Membre"),
            TypedWriteIntent::insert("tribe_members")
                .value("id", new_id())
                .value("tribe_id", id.as_str())
                .value("cog_id", creator_cog_id)
                .value("role_id", chef_role_id.as_str())
                .value("joined_at", now.as_str())
                .purpose("tribe_create_member"),
        ])?;
        Ok(Tribe {
            id,
            name: name.to_string(),
            description: description.map(String::from),
            creator_cog_id: creator_cog_id.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn tribe_update(
        &self,
        id: &str,
        name: &str,
        description: Option<&str>,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::update("tribes")
                .key("id", id)
                .value("name", name)
                .value("description", description)
                .value("updated_at", now_rfc3339())
                .purpose("tribe_update"),
        )?;
        Ok(())
    }

    /// Intentions de suppression des messages d'un salon et de leurs dépendances
    /// (livraisons en attente, pièces jointes), puis des membres du salon.
    fn salon_content_delete_intents(
        &self,
        salon_id: &str,
    ) -> Result<Vec<TypedWriteIntent>, DbError> {
        let messages = self.select(
            TypedSelect::from("messages")
                .columns(["id"])
                .filter("salon_id", salon_id),
        )?;
        let mut intents = Vec::with_capacity(messages.len() * 2 + 2);
        for row in &messages {
            let message_id = row_str(row, "id");
            intents.push(
                TypedWriteIntent::delete("pending_deliveries")
                    .key("message_id", message_id.as_str())
                    .purpose("salon_delete_pending"),
            );
            intents.push(
                TypedWriteIntent::delete("message_attachments")
                    .key("message_id", message_id.as_str())
                    .purpose("salon_delete_attachments"),
            );
        }
        intents.push(
            TypedWriteIntent::delete("messages")
                .key("salon_id", salon_id)
                .purpose("salon_delete_msgs"),
        );
        intents.push(
            TypedWriteIntent::delete("salon_members")
                .key("salon_id", salon_id)
                .purpose("salon_delete_members"),
        );
        Ok(intents)
    }

    /// Supprime une tribu et tout son contenu (lot atomique ; voir l'en-tête pour
    /// l'extension de cascade imposée par les clés étrangères).
    pub fn tribe_delete(&self, id: &str) -> Result<(), DbError> {
        let salons = self.select(
            TypedSelect::from("salons")
                .columns(["id"])
                .filter("tribe_id", id),
        )?;
        let mut intents = vec![
            TypedWriteIntent::delete("tribe_members")
                .key("tribe_id", id)
                .purpose("tribe_delete_members"),
            TypedWriteIntent::delete("tribe_roles")
                .key("tribe_id", id)
                .purpose("tribe_delete_roles"),
            TypedWriteIntent::delete("tribe_invitations")
                .key("tribe_id", id)
                .purpose("tribe_delete_invitations"),
        ];
        for row in &salons {
            intents.extend(self.salon_content_delete_intents(&row_str(row, "id"))?);
        }
        intents.push(
            TypedWriteIntent::delete("salons")
                .key("tribe_id", id)
                .purpose("tribe_delete_salons"),
        );
        intents.push(
            TypedWriteIntent::delete("tribes")
                .key("id", id)
                .purpose("tribe_delete"),
        );
        self.submit_batch(intents)?;
        Ok(())
    }

    // ---------- Salons ----------

    pub fn salon_list(&self, tribe_id: Option<&str>) -> Result<Vec<Salon>, DbError> {
        // `tribe_id = None` : filtre `IS NULL` (salons hors tribu), plus récents d'abord.
        let query = TypedSelect::from("salons")
            .columns(SALON_COLS)
            .filter("tribe_id", tribe_id);
        let query = if tribe_id.is_some() {
            query.order_by("name", false)
        } else {
            query.order_by("created_at", true)
        };
        Ok(self.select(query)?.iter().map(row_to_salon).collect())
    }

    pub fn salon_by_id(&self, id: &str) -> Result<Option<Salon>, DbError> {
        let rows = self.select(
            TypedSelect::from("salons")
                .columns(SALON_COLS)
                .filter("id", id)
                .limit(1),
        )?;
        Ok(rows.first().map(row_to_salon))
    }

    pub fn salon_create(
        &self,
        tribe_id: Option<&str>,
        name: &str,
        salon_type: SalonType,
    ) -> Result<Salon, DbError> {
        let id = new_id();
        let created_at = now_rfc3339();
        self.submit(
            TypedWriteIntent::insert("salons")
                .value("id", id.as_str())
                .value("tribe_id", tribe_id)
                .value("name", name)
                .value("salon_type", salon_type.as_str())
                .value("created_at", created_at.as_str())
                .purpose("salon_create"),
        )?;
        Ok(Salon {
            id,
            tribe_id: tribe_id.map(String::from),
            name: name.to_string(),
            salon_type,
            created_at,
        })
    }

    /// Ajoute un membre (sans effet s'il l'est déjà, comme `INSERT OR IGNORE`).
    pub fn salon_add_member(&self, salon_id: &str, cog_id: &str) -> Result<(), DbError> {
        // Pas d'upsert : sans valeur il n'insère que la clé (`joined_at` NOT NULL
        // manquerait) ; avec `joined_at` en valeur, il l'écraserait en cas de conflit.
        // D'où contrôle d'existence puis insertion.
        let exists = !self
            .select(
                TypedSelect::from("salon_members")
                    .columns(["cog_id"])
                    .filter("salon_id", salon_id)
                    .filter("cog_id", cog_id)
                    .limit(1),
            )?
            .is_empty();
        if exists {
            return Ok(());
        }
        self.submit(
            TypedWriteIntent::insert("salon_members")
                .value("salon_id", salon_id)
                .value("cog_id", cog_id)
                .value("joined_at", now_rfc3339())
                .purpose("salon_add_member"),
        )?;
        Ok(())
    }

    /// Liste les cog_ids des membres d'un salon (pour envoi MWS).
    pub fn salon_member_cog_ids(&self, salon_id: &str) -> Result<Vec<String>, DbError> {
        let rows = self.select(
            TypedSelect::from("salon_members")
                .columns(["cog_id"])
                .filter("salon_id", salon_id),
        )?;
        Ok(rows.iter().map(|r| row_str(r, "cog_id")).collect())
    }

    /// Supprime un salon, ses messages (et leurs dépendances) et ses membres (lot atomique).
    pub fn salon_delete(&self, id: &str) -> Result<(), DbError> {
        let mut intents = self.salon_content_delete_intents(id)?;
        intents.push(
            TypedWriteIntent::delete("salons")
                .key("id", id)
                .purpose("salon_delete"),
        );
        self.submit_batch(intents)?;
        Ok(())
    }

    /// Liste les salons dont le `cog_id` est membre (pour afficher les conversations).
    pub fn salon_list_for_cog(&self, cog_id: &str) -> Result<Vec<Salon>, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at
             FROM salons s INNER JOIN salon_members m ON s.id = m.salon_id WHERE m.cog_id = ?1 ORDER BY s.created_at DESC",
            vec![cog_id.to_string()],
        )?;
        Ok(rows.iter().map(row_to_salon).collect())
    }

    /// Trouve le salon direct existant entre deux COG (DAT-01 : au plus un salon direct
    /// par paire). Sémantique HÉRITÉE : salon hors tribu comptant exactement ces deux membres.
    pub fn salon_find_direct_between(
        &self,
        cog_id_1: &str,
        cog_id_2: &str,
    ) -> Result<Option<Salon>, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at
             FROM salons s
             WHERE s.tribe_id IS NULL
             AND (SELECT COUNT(*) FROM salon_members m WHERE m.salon_id = s.id) = 2
             AND EXISTS (SELECT 1 FROM salon_members m WHERE m.salon_id = s.id AND m.cog_id = ?1)
             AND EXISTS (SELECT 1 FROM salon_members m WHERE m.salon_id = s.id AND m.cog_id = ?2)
             LIMIT 1",
            vec![cog_id_1.to_string(), cog_id_2.to_string()],
        )?;
        Ok(rows.first().map(row_to_salon))
    }

    // ---------- Messages ----------

    /// Derniers `limit` messages d'un salon, du plus ancien au plus récent
    /// (`limit < 0` : sans limite, comme SQLite).
    pub fn message_list(&self, salon_id: &str, limit: i64) -> Result<Vec<Message>, DbError> {
        let mut query = TypedSelect::from("messages")
            .columns(MESSAGE_COLS)
            .filter("salon_id", salon_id)
            .order_by("created_at", true);
        if limit >= 0 {
            query = query.limit(u32::try_from(limit).unwrap_or(u32::MAX));
        }
        let mut msgs: Vec<Message> = self.select(query)?.iter().map(row_to_message).collect();
        msgs.reverse();
        Ok(msgs)
    }

    pub fn message_create(
        &self,
        salon_id: &str,
        sender_cog_id: &str,
        content: &str,
    ) -> Result<Message, DbError> {
        let is_member = !self
            .select(
                TypedSelect::from("salon_members")
                    .columns(["cog_id"])
                    .filter("salon_id", salon_id)
                    .filter("cog_id", sender_cog_id)
                    .limit(1),
            )?
            .is_empty();
        if !is_member {
            return Err(DbError(
                "L'expéditeur n'est pas membre du salon.".to_string(),
            ));
        }
        let id = new_id();
        let created_at = now_rfc3339();
        self.submit(
            TypedWriteIntent::insert("messages")
                .value("id", id.as_str())
                .value("salon_id", salon_id)
                .value("sender_cog_id", sender_cog_id)
                .value("content", content)
                .value("created_at", created_at.as_str())
                .purpose("message_create"),
        )?;
        Ok(Message {
            id,
            salon_id: salon_id.to_string(),
            sender_cog_id: sender_cog_id.to_string(),
            content: content.to_string(),
            created_at,
        })
    }

    /// Récupère un message par ID (pour livraison différée).
    pub fn message_by_id(&self, message_id: &str) -> Result<Option<Message>, DbError> {
        let rows = self.select(
            TypedSelect::from("messages")
                .columns(MESSAGE_COLS)
                .filter("id", message_id)
                .limit(1),
        )?;
        Ok(rows.first().map(row_to_message))
    }

    // ---------- Pending deliveries (H2 livraison différée) ----------

    /// Enregistre une livraison en attente pour un destinataire (tribu).
    pub fn pending_delivery_enqueue(
        &self,
        message_id: &str,
        recipient_cog_id: &str,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("pending_deliveries")
                .value("id", new_id())
                .value("message_id", message_id)
                .value("recipient_cog_id", recipient_cog_id)
                .value("created_at", now_rfc3339())
                .purpose("pending_delivery_enqueue"),
        )?;
        Ok(())
    }

    /// Liste les (pending_id, message_id) en attente pour un destinataire donné.
    pub fn pending_delivery_message_ids_for(
        &self,
        recipient_cog_id: &str,
    ) -> Result<Vec<(String, String)>, DbError> {
        let rows = self.select(
            TypedSelect::from("pending_deliveries")
                .columns(["id", "message_id"])
                .filter("recipient_cog_id", recipient_cog_id)
                .order_by("created_at", false),
        )?;
        Ok(rows
            .iter()
            .map(|r| (row_str(r, "id"), row_str(r, "message_id")))
            .collect())
    }

    /// Supprime une entrée de livraison en attente.
    pub fn pending_delivery_delete(&self, pending_id: &str) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::delete("pending_deliveries")
                .key("id", pending_id)
                .purpose("pending_delivery_delete"),
        )?;
        Ok(())
    }

    /// Liste les livraisons en attente pour les messages envoyés par un COG donné.
    /// Retourne (pending_id, message_id, recipient_cog_id) pour reprise à la reconnexion.
    pub fn pending_deliveries_for_sender(
        &self,
        sender_cog_id: &str,
    ) -> Result<Vec<(String, String, String)>, DbError> {
        // Lecture SQL brut transitoire (lot C) : à passer en lecture typée/nommée
        let rows = self.query_raw(
            "SELECT pd.id, pd.message_id, pd.recipient_cog_id FROM pending_deliveries pd
             INNER JOIN messages m ON m.id = pd.message_id
             WHERE m.sender_cog_id = ?1 ORDER BY pd.created_at",
            vec![sender_cog_id.to_string()],
        )?;
        Ok(rows
            .iter()
            .map(|r| {
                (
                    row_str(r, "id"),
                    row_str(r, "message_id"),
                    row_str(r, "recipient_cog_id"),
                )
            })
            .collect())
    }

    // ---------- Tribe invitations (M4) ----------

    /// Crée une invitation à une tribu (status = pending).
    #[allow(clippy::similar_names)] // noms de paramètres de l'API HÉRITÉE
    pub fn tribe_invitation_create(
        &self,
        tribe_id: &str,
        inviter_cog_id: &str,
        invitee_cog_id: &str,
    ) -> Result<String, DbError> {
        let existing = self.select(
            TypedSelect::from("tribe_invitations")
                .columns(["id"])
                .filter("tribe_id", tribe_id)
                .filter("invitee_cog_id", invitee_cog_id)
                .filter("status", "pending")
                .limit(1),
        )?;
        if !existing.is_empty() {
            return Err(DbError(
                "Invitation déjà en attente pour ce COG.".to_string(),
            ));
        }
        let id = new_id();
        self.submit(
            TypedWriteIntent::insert("tribe_invitations")
                .value("id", id.as_str())
                .value("tribe_id", tribe_id)
                .value("inviter_cog_id", inviter_cog_id)
                .value("invitee_cog_id", invitee_cog_id)
                .value("status", "pending")
                .value("created_at", now_rfc3339())
                .purpose("tribe_invitation_create"),
        )?;
        Ok(id)
    }

    /// Accepte une invitation : statut `accepted` et inscription du membre (lot atomique).
    pub fn tribe_invitation_accept(&self, invitation_id: &str) -> Result<(), DbError> {
        let rows = self.select(
            TypedSelect::from("tribe_invitations")
                .columns(["tribe_id", "invitee_cog_id"])
                .filter("id", invitation_id)
                .filter("status", "pending")
                .limit(1),
        )?;
        let row = rows.first().ok_or(DbError(
            "Invitation introuvable ou déjà traitée.".to_string(),
        ))?;
        let tribe_id = row_str(row, "tribe_id");
        let invitee_cog_id = row_str(row, "invitee_cog_id");
        let role_id = self
            .tribe_default_member_role_id(&tribe_id)?
            .ok_or(DbError("Aucun rôle pour cette tribu.".to_string()))?;
        self.submit_batch(vec![
            TypedWriteIntent::update("tribe_invitations")
                .key("id", invitation_id)
                .key("status", "pending")
                .value("status", "accepted")
                .purpose("tribe_invitation_accept"),
            TypedWriteIntent::insert("tribe_members")
                .value("id", new_id())
                .value("tribe_id", tribe_id)
                .value("cog_id", invitee_cog_id)
                .value("role_id", role_id)
                .value("joined_at", now_rfc3339())
                .purpose("tribe_invitation_accept_member"),
        ])?;
        Ok(())
    }

    /// Refuse une invitation (status = refused, si elle est encore en attente).
    pub fn tribe_invitation_refuse(&self, invitation_id: &str) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::update("tribe_invitations")
                .key("id", invitation_id)
                .key("status", "pending")
                .value("status", "refused")
                .purpose("tribe_invitation_refuse"),
        )?;
        Ok(())
    }

    /// Liste les invitations en attente pour un COG (invitee).
    pub fn tribe_invitations_pending_for(
        &self,
        invitee_cog_id: &str,
    ) -> Result<Vec<(String, String, String)>, DbError> {
        let rows = self.select(
            TypedSelect::from("tribe_invitations")
                .columns(["id", "tribe_id", "inviter_cog_id"])
                .filter("invitee_cog_id", invitee_cog_id)
                .filter("status", "pending")
                .order_by("created_at", false),
        )?;
        Ok(rows
            .iter()
            .map(|r| {
                (
                    row_str(r, "id"),
                    row_str(r, "tribe_id"),
                    row_str(r, "inviter_cog_id"),
                )
            })
            .collect())
    }

    /// Récupère le role_id par défaut (premier rôle « Membre » ou premier rôle) d'une tribu.
    pub fn tribe_default_member_role_id(&self, tribe_id: &str) -> Result<Option<String>, DbError> {
        let rows = self.select(
            TypedSelect::from("tribe_roles")
                .columns(["id"])
                .filter("tribe_id", tribe_id)
                .filter("name", "Membre")
                .limit(1),
        )?;
        if let Some(r) = rows.first() {
            return Ok(Some(row_str(r, "id")));
        }
        let rows = self.select(
            TypedSelect::from("tribe_roles")
                .columns(["id"])
                .filter("tribe_id", tribe_id)
                .limit(1),
        )?;
        Ok(rows.first().map(|r| row_str(r, "id")))
    }

    // ---------- Message attachments (M2) ----------

    /// Enregistre une pièce jointe (fichier ou image) sur un message.
    pub fn message_attachment_create(
        &self,
        message_id: &str,
        kind: &str,
        local_path: &str,
    ) -> Result<(), DbError> {
        self.submit(
            TypedWriteIntent::insert("message_attachments")
                .value("id", new_id())
                .value("message_id", message_id)
                .value("kind", kind)
                .value("local_path", local_path)
                .value("created_at", now_rfc3339())
                .purpose("message_attachment_create"),
        )?;
        Ok(())
    }
}

#[cfg(test)]
#[allow(clippy::assert_is_empty)]
mod tests {
    //! Tests contre un kindmother-service en processus (opérateur `jay1tribu`, Admin
    //! de la base `jay1tribu`).

    use super::*;

    fn test_db() -> (Jay1TribuDb, tempfile::TempDir) {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (addr, creds) =
            kindmother_service::spawn_ephemeral_blocking(tmp.path()).expect("serveur éphémère");
        let cred = OperatorCredential::load_from_dir(DB_NAME, &creds).expect("justificatif");
        let db = Jay1TribuDb::connect_with(&addr.to_string(), &cred).expect("connexion");
        (db, tmp)
    }

    #[test]
    fn migrations_idempotentes() {
        let (db, _tmp) = test_db();
        db.init_schema_sync().expect("2e application");
        let (before, after) =
            run_blocking(db.client.apply_migrations(migrations())).expect("3e application");
        assert_eq!((before, after), (1, 1));
    }

    #[test]
    fn friend_crud_et_doublon() {
        let (db, _tmp) = test_db();
        let f = db.friend_create("p1", "cog-a", Some("Alice")).unwrap();
        db.friend_create("p1", "cog-b", None).unwrap();
        let err = db.friend_create("p1", "cog-a", None).unwrap_err();
        assert!(err.0.contains("déjà dans la liste"));
        assert!(db.is_friend("p1", "cog-a").unwrap());
        let list = db.friend_list("p1").unwrap();
        assert_eq!(list.len(), 2);
        // NULL trié en premier (comme SQLite HÉRITÉ).
        assert_eq!(list[0].friend_pseudo, None);
        assert_eq!(list[1].friend_pseudo.as_deref(), Some("Alice"));
        db.friend_delete(&f.id).unwrap();
        assert!(!db.is_friend("p1", "cog-a").unwrap());
    }

    #[test]
    fn tribe_create_update_list_delete() {
        let (db, _tmp) = test_db();
        let t = db.tribe_create("Ma tribu", None, "creator").unwrap();
        let found = db.tribe_by_id(&t.id).unwrap().expect("tribu");
        assert_eq!(found.name, "Ma tribu");
        assert_eq!(found.description, None);
        assert_eq!(db.tribe_list("creator").unwrap().len(), 1);
        assert!(db.tribe_default_member_role_id(&t.id).unwrap().is_some());

        db.tribe_update(&t.id, "Renommée", Some("desc")).unwrap();
        let found = db.tribe_by_id(&t.id).unwrap().unwrap();
        assert_eq!(found.name, "Renommée");
        assert_eq!(found.description.as_deref(), Some("desc"));

        // Contenu dépendant : salon, message, livraison, pièce jointe, invitation.
        let s = db
            .salon_create(Some(&t.id), "général", SalonType::Collective)
            .unwrap();
        db.salon_add_member(&s.id, "creator").unwrap();
        let m = db.message_create(&s.id, "creator", "hello").unwrap();
        db.pending_delivery_enqueue(&m.id, "cog-x").unwrap();
        db.message_attachment_create(&m.id, "image", "/tmp/a.png")
            .unwrap();
        db.tribe_invitation_create(&t.id, "creator", "cog-y")
            .unwrap();
        assert_eq!(db.salon_list(Some(&t.id)).unwrap().len(), 1);

        db.tribe_delete(&t.id).unwrap();
        assert!(db.tribe_by_id(&t.id).unwrap().is_none());
        assert!(db.salon_by_id(&s.id).unwrap().is_none());
        assert!(db.message_by_id(&m.id).unwrap().is_none());
        assert!(db
            .pending_delivery_message_ids_for("cog-x")
            .unwrap()
            .is_empty());
        assert!(db.tribe_list("creator").unwrap().is_empty());
    }

    #[test]
    fn salons_direct_et_messages() {
        let (db, _tmp) = test_db();
        assert!(db.salon_find_direct_between("a", "b").unwrap().is_none());
        let s = db.salon_create(None, "DM", SalonType::Direct).unwrap();
        db.salon_add_member(&s.id, "a").unwrap();
        db.salon_add_member(&s.id, "b").unwrap();
        db.salon_add_member(&s.id, "b").unwrap(); // idempotent (INSERT OR IGNORE)
        assert_eq!(db.salon_member_cog_ids(&s.id).unwrap().len(), 2);
        assert_eq!(
            db.salon_find_direct_between("a", "b")
                .unwrap()
                .map(|s| s.id),
            Some(s.id.clone())
        );
        assert_eq!(db.salon_list(None).unwrap().len(), 1);
        assert_eq!(db.salon_list_for_cog("a").unwrap().len(), 1);

        let err = db.message_create(&s.id, "intrus", "x").unwrap_err();
        assert!(err.0.contains("n'est pas membre"));
        let m1 = db.message_create(&s.id, "a", "un").unwrap();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let m2 = db.message_create(&s.id, "b", "deux").unwrap();
        let list = db.message_list(&s.id, 10).unwrap();
        assert_eq!(
            list.iter().map(|m| m.id.clone()).collect::<Vec<_>>(),
            vec![m1.id.clone(), m2.id.clone()]
        );
        let dernier = db.message_list(&s.id, 1).unwrap();
        assert_eq!(dernier.len(), 1);
        assert_eq!(dernier[0].id, m2.id);
        assert_eq!(db.message_by_id(&m1.id).unwrap().unwrap().content, "un");

        db.pending_delivery_enqueue(&m1.id, "b").unwrap();
        let pending = db.pending_delivery_message_ids_for("b").unwrap();
        assert_eq!(pending.len(), 1);
        assert_eq!(db.pending_deliveries_for_sender("a").unwrap().len(), 1);
        db.pending_delivery_delete(&pending[0].0).unwrap();
        assert!(db.pending_delivery_message_ids_for("b").unwrap().is_empty());

        db.message_attachment_create(&m2.id, "file", "/tmp/f")
            .unwrap();
        db.pending_delivery_enqueue(&m2.id, "a").unwrap();
        db.salon_delete(&s.id).unwrap();
        assert!(db.salon_by_id(&s.id).unwrap().is_none());
        assert!(db.message_list(&s.id, -1).unwrap().is_empty());
    }

    #[test]
    fn invitations_accept_refuse() {
        let (db, _tmp) = test_db();
        let t = db.tribe_create("T", Some("d"), "chef").unwrap();
        let inv = db.tribe_invitation_create(&t.id, "chef", "invite").unwrap();
        let err = db
            .tribe_invitation_create(&t.id, "chef", "invite")
            .unwrap_err();
        assert!(err.0.contains("déjà en attente"));
        assert_eq!(db.tribe_invitations_pending_for("invite").unwrap().len(), 1);
        db.tribe_invitation_accept(&inv).unwrap();
        assert!(db
            .tribe_invitations_pending_for("invite")
            .unwrap()
            .is_empty());
        assert_eq!(db.tribe_list("invite").unwrap().len(), 1);
        assert!(db.tribe_invitation_accept(&inv).is_err());

        let inv2 = db.tribe_invitation_create(&t.id, "chef", "autre").unwrap();
        db.tribe_invitation_refuse(&inv2).unwrap();
        assert!(db
            .tribe_invitations_pending_for("autre")
            .unwrap()
            .is_empty());
        assert!(db.tribe_list("autre").unwrap().is_empty());
    }

    #[tokio::test(flavor = "current_thread")]
    async fn run_blocking_sous_runtime_current_thread() {
        // block_in_place paniquerait ici : run_blocking passe par le runtime global.
        assert_eq!(run_blocking(async { 41 + 1 }), 42);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn connexion_async_injectee() {
        let tmp = tempfile::tempdir().expect("tempdir");
        let (addr, creds) = kindmother_service::spawn_ephemeral(tmp.path())
            .await
            .expect("serveur éphémère");
        let cred = OperatorCredential::load_from_dir(DB_NAME, &creds).expect("justificatif");
        let db = Jay1TribuDb::connect_with_async(&addr.to_string(), &cred)
            .await
            .expect("connexion");
        // API synchrone depuis un runtime multi-thread (block_in_place).
        let f = db.friend_create("p", "c", None).unwrap();
        assert_eq!(db.friend_list("p").unwrap()[0].id, f.id);
    }
}
