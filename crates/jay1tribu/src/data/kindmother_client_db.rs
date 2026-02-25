//! Base de données Jay1Tribu via KindMother Client (feature kindmother-only).
//!
//! Délégation exclusive au service KindMother ; pas d'accès SQLite direct.
//! API synchrone (block_on sur le runtime Tokio) pour compatibilité avec le domaine.
//!
//! @id: jay1tribu_kindmother_client_db
//! @do: persist_jay1tribu_data_via_kindmother_client
//! @layer: infra

use crate::data::types::{Friend, Message, Salon, SalonType, Tribe};
use kindmother::{InstanceIdentity, InstanceType};
use kindmother_client::{ClientError, KindMotherClient};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;
use tokio::sync::OnceCell;

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

const DEFAULT_KINDMOTHER_ADDR: &str = "127.0.0.1:50051";
const DB_NAME: &str = "jay1tribu";

static CLIENT: OnceCell<Arc<KindMotherClient>> = OnceCell::const_new();
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

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

fn row_str(row: &HashMap<String, serde_json::Value>, key: &str) -> String {
    row.get(key)
        .and_then(|v| v.as_str())
        .unwrap_or_default()
        .to_string()
}

fn row_opt_str(row: &HashMap<String, serde_json::Value>, key: &str) -> Option<String> {
    row.get(key).and_then(|v| v.as_str()).map(String::from)
}

/// Base de données Jay1Tribu via KindMother Client.
#[derive(Clone)]
pub struct Jay1TribuDb {
    client: Arc<KindMotherClient>,
    pub instance: InstanceIdentity,
}

impl Jay1TribuDb {
    /// Initialise la connexion globale au service KindMother (synchrone).
    pub fn init_global_sync(addr: Option<&str>) -> Result<(), DbError> {
        run_blocking(Self::init_global_async(addr))
    }

    /// Initialise la connexion globale (async).
    pub async fn init_global_async(addr: Option<&str>) -> Result<(), DbError> {
        let addr = addr.unwrap_or(DEFAULT_KINDMOTHER_ADDR);
        let client = KindMotherClient::connect(addr, DB_NAME, DB_NAME).await?;
        CLIENT
            .set(Arc::new(client))
            .map_err(|_| DbError("Client déjà initialisé".to_string()))?;
        Ok(())
    }

    /// Crée une instance utilisant le client global.
    pub fn new() -> Result<Self, DbError> {
        let client = CLIENT.get().ok_or(DbError("Client KindMother non initialisé".to_string()))?.clone();
        Ok(Self {
            client,
            instance: InstanceIdentity::new(InstanceType::Daughter),
        })
    }

    /// Ouvre la base (compatible API legacy) : initialise le client global si besoin puis crée l'instance.
    pub fn open(_path: impl AsRef<Path>) -> Result<Self, DbError> {
        if CLIENT.get().is_none() {
            Self::init_global_sync(None)?;
        }
        let db = Self::new()?;
        db.init_schema_sync()?;
        Ok(db)
    }

    /// Initialise le schéma (synchrone).
    pub fn init_schema_sync(&self) -> Result<(), DbError> {
        run_blocking(self.init_schema_async())
    }

    async fn init_schema_async(&self) -> Result<(), DbError> {
        let statements = [
            "CREATE TABLE IF NOT EXISTS friends (id TEXT PRIMARY KEY, profile_id TEXT NOT NULL, friend_cog_id TEXT NOT NULL, friend_pseudo TEXT, created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS tribes (id TEXT PRIMARY KEY, name TEXT NOT NULL, description TEXT, creator_cog_id TEXT NOT NULL, created_at TEXT NOT NULL, updated_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS tribe_roles (id TEXT PRIMARY KEY, tribe_id TEXT NOT NULL, name TEXT NOT NULL, permissions_json TEXT, created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS tribe_members (id TEXT PRIMARY KEY, tribe_id TEXT NOT NULL, cog_id TEXT NOT NULL, role_id TEXT NOT NULL, joined_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS salons (id TEXT PRIMARY KEY, tribe_id TEXT, name TEXT NOT NULL, salon_type TEXT NOT NULL DEFAULT 'direct', created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS salon_members (salon_id TEXT NOT NULL, cog_id TEXT NOT NULL, joined_at TEXT NOT NULL, PRIMARY KEY (salon_id, cog_id))",
            "CREATE TABLE IF NOT EXISTS messages (id TEXT PRIMARY KEY, salon_id TEXT NOT NULL, sender_cog_id TEXT NOT NULL, content TEXT NOT NULL, created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS pending_deliveries (id TEXT PRIMARY KEY, message_id TEXT NOT NULL, recipient_cog_id TEXT NOT NULL, created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS tribe_invitations (id TEXT PRIMARY KEY, tribe_id TEXT NOT NULL, inviter_cog_id TEXT NOT NULL, invitee_cog_id TEXT NOT NULL, status TEXT NOT NULL DEFAULT 'pending', created_at TEXT NOT NULL)",
            "CREATE TABLE IF NOT EXISTS message_attachments (id TEXT PRIMARY KEY, message_id TEXT NOT NULL, kind TEXT NOT NULL, local_path TEXT NOT NULL, created_at TEXT NOT NULL)",
        ];
        for sql in &statements {
            self.client
                .execute(*sql, Vec::<String>::new(), "init_schema")
                .await?;
        }
        Ok(())
    }

    pub fn friend_list(&self, profile_id: &str) -> Result<Vec<Friend>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, profile_id, friend_cog_id, friend_pseudo, created_at FROM friends WHERE profile_id = ?1 ORDER BY friend_pseudo, created_at",
            vec![profile_id.to_string()],
        ))?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Friend {
                id: row_str(&row, "id"),
                profile_id: row_str(&row, "profile_id"),
                friend_cog_id: row_str(&row, "friend_cog_id"),
                friend_pseudo: row_opt_str(&row, "friend_pseudo"),
                created_at: row_str(&row, "created_at"),
            });
        }
        Ok(out)
    }

    pub fn friend_create(&self, profile_id: &str, friend_cog_id: &str, friend_pseudo: Option<&str>) -> Result<Friend, DbError> {
        let existing = run_blocking(self.client.query(
            "SELECT 1 FROM friends WHERE profile_id = ?1 AND friend_cog_id = ?2 LIMIT 1",
            vec![profile_id.to_string(), friend_cog_id.to_string()],
        ))?;
        if !existing.is_empty() {
            return Err(DbError("Cet ami est déjà dans la liste.".to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        let pseudo = friend_pseudo.map(String::from).unwrap_or_default();
        run_blocking(self.client.execute(
            "INSERT INTO friends (id, profile_id, friend_cog_id, friend_pseudo, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![id.clone(), profile_id.to_string(), friend_cog_id.to_string(), pseudo.clone(), created_at.clone()],
            "friend_create",
        ))?;
        Ok(Friend {
            id,
            profile_id: profile_id.to_string(),
            friend_cog_id: friend_cog_id.to_string(),
            friend_pseudo: if pseudo.is_empty() { None } else { Some(pseudo) },
            created_at,
        })
    }

    pub fn friend_delete(&self, id: &str) -> Result<(), DbError> {
        run_blocking(self.client.execute(
            "DELETE FROM friends WHERE id = ?1",
            vec![id.to_string()],
            "friend_delete",
        ))?;
        Ok(())
    }

    pub fn is_friend(&self, profile_id: &str, friend_cog_id: &str) -> Result<bool, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT 1 FROM friends WHERE profile_id = ?1 AND friend_cog_id = ?2 LIMIT 1",
            vec![profile_id.to_string(), friend_cog_id.to_string()],
        ))?;
        Ok(!rows.is_empty())
    }

    pub fn tribe_list(&self, cog_id: &str) -> Result<Vec<Tribe>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT t.id, t.name, t.description, t.creator_cog_id, t.created_at, t.updated_at FROM tribes t INNER JOIN tribe_members m ON t.id = m.tribe_id WHERE m.cog_id = ?1 ORDER BY t.name",
            vec![cog_id.to_string()],
        ))?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Tribe {
                id: row_str(&row, "id"),
                name: row_str(&row, "name"),
                description: row_opt_str(&row, "description"),
                creator_cog_id: row_str(&row, "creator_cog_id"),
                created_at: row_str(&row, "created_at"),
                updated_at: row_str(&row, "updated_at"),
            });
        }
        Ok(out)
    }

    pub fn tribe_by_id(&self, id: &str) -> Result<Option<Tribe>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, name, description, creator_cog_id, created_at, updated_at FROM tribes WHERE id = ?1",
            vec![id.to_string()],
        ))?;
        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };
        Ok(Some(Tribe {
            id: row_str(&row, "id"),
            name: row_str(&row, "name"),
            description: row_opt_str(&row, "description"),
            creator_cog_id: row_str(&row, "creator_cog_id"),
            created_at: row_str(&row, "created_at"),
            updated_at: row_str(&row, "updated_at"),
        }))
    }

    pub fn tribe_create(&self, name: &str, description: Option<&str>, creator_cog_id: &str) -> Result<Tribe, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_rfc3339();
        let desc = description.map(String::from).unwrap_or_default();
        run_blocking(self.client.execute(
            "INSERT INTO tribes (id, name, description, creator_cog_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            vec![id.clone(), name.to_string(), desc, creator_cog_id.to_string(), now.clone(), now.clone()],
            "tribe_create",
        ))?;
        let chef_role_id = uuid::Uuid::new_v4().to_string();
        run_blocking(self.client.execute(
            "INSERT INTO tribe_roles (id, tribe_id, name, permissions_json, created_at) VALUES (?1, ?2, 'Chef de tribu', NULL, ?3)",
            vec![chef_role_id.clone(), id.clone(), now.clone()],
            "tribe_create_chef",
        ))?;
        let membre_role_id = uuid::Uuid::new_v4().to_string();
        run_blocking(self.client.execute(
            "INSERT INTO tribe_roles (id, tribe_id, name, permissions_json, created_at) VALUES (?1, ?2, 'Membre', NULL, ?3)",
            vec![membre_role_id, id.clone(), now.clone()],
            "tribe_create_membre",
        ))?;
        let member_id = uuid::Uuid::new_v4().to_string();
        run_blocking(self.client.execute(
            "INSERT INTO tribe_members (id, tribe_id, cog_id, role_id, joined_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![member_id, id.clone(), creator_cog_id.to_string(), chef_role_id, now.clone()],
            "tribe_create_member",
        ))?;
        Ok(Tribe {
            id: id.clone(),
            name: name.to_string(),
            description: description.map(String::from),
            creator_cog_id: creator_cog_id.to_string(),
            created_at: now.clone(),
            updated_at: now,
        })
    }

    pub fn tribe_update(&self, id: &str, name: &str, description: Option<&str>) -> Result<(), DbError> {
        let now = now_rfc3339();
        let desc = description.map(String::from).unwrap_or_default();
        run_blocking(self.client.execute(
            "UPDATE tribes SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
            vec![name.to_string(), desc, now, id.to_string()],
            "tribe_update",
        ))?;
        Ok(())
    }

    pub fn tribe_delete(&self, id: &str) -> Result<(), DbError> {
        run_blocking(self.client.execute("DELETE FROM tribe_members WHERE tribe_id = ?1", vec![id.to_string()], "tribe_delete_members"))?;
        run_blocking(self.client.execute("DELETE FROM tribe_roles WHERE tribe_id = ?1", vec![id.to_string()], "tribe_delete_roles"))?;
        let salons: Vec<HashMap<String, serde_json::Value>> = run_blocking(self.client.query("SELECT id FROM salons WHERE tribe_id = ?1", vec![id.to_string()]))?;
        for row in &salons {
            let sid = row_str(row, "id");
            let _ = run_blocking(self.client.execute("DELETE FROM messages WHERE salon_id = ?1", vec![sid.clone()], "tribe_delete_msgs"));
            let _ = run_blocking(self.client.execute("DELETE FROM salon_members WHERE salon_id = ?1", vec![sid], "tribe_delete_sm"));
        }
        run_blocking(self.client.execute("DELETE FROM salons WHERE tribe_id = ?1", vec![id.to_string()], "tribe_delete_salons"))?;
        run_blocking(self.client.execute("DELETE FROM tribes WHERE id = ?1", vec![id.to_string()], "tribe_delete"))?;
        Ok(())
    }

    pub fn salon_list(&self, tribe_id: Option<&str>) -> Result<Vec<Salon>, DbError> {
        let (sql, params) = match tribe_id {
            Some(tid) => (
                "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE tribe_id = ?1 ORDER BY name",
                vec![tid.to_string()],
            ),
            None => (
                "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE tribe_id IS NULL ORDER BY created_at DESC",
                vec![],
            ),
        };
        let rows = run_blocking(self.client.query(sql, params))?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Salon {
                id: row_str(&row, "id"),
                tribe_id: row_opt_str(&row, "tribe_id"),
                name: row_str(&row, "name"),
                salon_type: SalonType::from_str(&row_str(&row, "salon_type")),
                created_at: row_str(&row, "created_at"),
            });
        }
        Ok(out)
    }

    pub fn salon_by_id(&self, id: &str) -> Result<Option<Salon>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE id = ?1",
            vec![id.to_string()],
        ))?;
        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };
        Ok(Some(Salon {
            id: row_str(&row, "id"),
            tribe_id: row_opt_str(&row, "tribe_id"),
            name: row_str(&row, "name"),
            salon_type: SalonType::from_str(&row_str(&row, "salon_type")),
            created_at: row_str(&row, "created_at"),
        }))
    }

    pub fn salon_create(&self, tribe_id: Option<&str>, name: &str, salon_type: SalonType) -> Result<Salon, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        let tid = tribe_id.map(String::from).unwrap_or_default();
        run_blocking(self.client.execute(
            "INSERT INTO salons (id, tribe_id, name, salon_type, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![id.clone(), tid.clone(), name.to_string(), salon_type.as_str().to_string(), created_at.clone()],
            "salon_create",
        ))?;
        Ok(Salon {
            id: id.clone(),
            tribe_id: if tid.is_empty() { None } else { Some(tid) },
            name: name.to_string(),
            salon_type,
            created_at: created_at.clone(),
        })
    }

    pub fn salon_add_member(&self, salon_id: &str, cog_id: &str) -> Result<(), DbError> {
        let joined_at = now_rfc3339();
        let _ = run_blocking(self.client.execute(
            "INSERT OR IGNORE INTO salon_members (salon_id, cog_id, joined_at) VALUES (?1, ?2, ?3)",
            vec![salon_id.to_string(), cog_id.to_string(), joined_at],
            "salon_add_member",
        ))?;
        Ok(())
    }

    pub fn salon_member_cog_ids(&self, salon_id: &str) -> Result<Vec<String>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT cog_id FROM salon_members WHERE salon_id = ?1",
            vec![salon_id.to_string()],
        ))?;
        Ok(rows.iter().map(|r| row_str(r, "cog_id")).collect())
    }

    pub fn salon_delete(&self, id: &str) -> Result<(), DbError> {
        run_blocking(self.client.execute("DELETE FROM messages WHERE salon_id = ?1", vec![id.to_string()], "salon_delete_msgs"))?;
        run_blocking(self.client.execute("DELETE FROM salon_members WHERE salon_id = ?1", vec![id.to_string()], "salon_delete_sm"))?;
        run_blocking(self.client.execute("DELETE FROM salons WHERE id = ?1", vec![id.to_string()], "salon_delete"))?;
        Ok(())
    }

    pub fn salon_list_for_cog(&self, cog_id: &str) -> Result<Vec<Salon>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at FROM salons s INNER JOIN salon_members m ON s.id = m.salon_id WHERE m.cog_id = ?1 ORDER BY s.created_at DESC",
            vec![cog_id.to_string()],
        ))?;
        let mut out = Vec::with_capacity(rows.len());
        for row in rows {
            out.push(Salon {
                id: row_str(&row, "id"),
                tribe_id: row_opt_str(&row, "tribe_id"),
                name: row_str(&row, "name"),
                salon_type: SalonType::from_str(&row_str(&row, "salon_type")),
                created_at: row_str(&row, "created_at"),
            });
        }
        Ok(out)
    }

    pub fn salon_find_direct_between(&self, cog_id_1: &str, cog_id_2: &str) -> Result<Option<Salon>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at FROM salons s
             INNER JOIN salon_members m1 ON s.id = m1.salon_id AND m1.cog_id = ?1
             INNER JOIN salon_members m2 ON s.id = m2.salon_id AND m2.cog_id = ?2
             WHERE s.salon_type = 'direct' LIMIT 1",
            vec![cog_id_1.to_string(), cog_id_2.to_string()],
        ))?;
        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };
        Ok(Some(Salon {
            id: row_str(&row, "id"),
            tribe_id: row_opt_str(&row, "tribe_id"),
            name: row_str(&row, "name"),
            salon_type: SalonType::Direct,
            created_at: row_str(&row, "created_at"),
        }))
    }

    pub fn message_list(&self, salon_id: &str, limit: i64) -> Result<Vec<Message>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, salon_id, sender_cog_id, content, created_at FROM messages WHERE salon_id = ?1 ORDER BY created_at DESC LIMIT ?2",
            vec![salon_id.to_string(), limit.to_string()],
        ))?;
        let mut msgs: Vec<Message> = rows
            .iter()
            .map(|row| Message {
                id: row_str(row, "id"),
                salon_id: row_str(row, "salon_id"),
                sender_cog_id: row_str(row, "sender_cog_id"),
                content: row_str(row, "content"),
                created_at: row_str(row, "created_at"),
            })
            .collect();
        msgs.reverse();
        Ok(msgs)
    }

    pub fn message_create(&self, salon_id: &str, sender_cog_id: &str, content: &str) -> Result<Message, DbError> {
        let check = run_blocking(self.client.query(
            "SELECT 1 FROM salon_members WHERE salon_id = ?1 AND cog_id = ?2 LIMIT 1",
            vec![salon_id.to_string(), sender_cog_id.to_string()],
        ))?;
        if check.is_empty() {
            return Err(DbError("L'expéditeur n'est pas membre du salon.".to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        run_blocking(self.client.execute(
            "INSERT INTO messages (id, salon_id, sender_cog_id, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![id.clone(), salon_id.to_string(), sender_cog_id.to_string(), content.to_string(), created_at.clone()],
            "message_create",
        ))?;
        Ok(Message {
            id,
            salon_id: salon_id.to_string(),
            sender_cog_id: sender_cog_id.to_string(),
            content: content.to_string(),
            created_at,
        })
    }

    pub fn message_by_id(&self, message_id: &str) -> Result<Option<Message>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, salon_id, sender_cog_id, content, created_at FROM messages WHERE id = ?1",
            vec![message_id.to_string()],
        ))?;
        let row = match rows.into_iter().next() {
            Some(r) => r,
            None => return Ok(None),
        };
        Ok(Some(Message {
            id: row_str(&row, "id"),
            salon_id: row_str(&row, "salon_id"),
            sender_cog_id: row_str(&row, "sender_cog_id"),
            content: row_str(&row, "content"),
            created_at: row_str(&row, "created_at"),
        }))
    }

    pub fn pending_delivery_enqueue(&self, message_id: &str, recipient_cog_id: &str) -> Result<(), DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        run_blocking(self.client.execute(
            "INSERT INTO pending_deliveries (id, message_id, recipient_cog_id, created_at) VALUES (?1, ?2, ?3, ?4)",
            vec![id, message_id.to_string(), recipient_cog_id.to_string(), created_at],
            "pending_delivery_enqueue",
        ))?;
        Ok(())
    }

    pub fn pending_delivery_message_ids_for(&self, recipient_cog_id: &str) -> Result<Vec<(String, String)>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, message_id FROM pending_deliveries WHERE recipient_cog_id = ?1 ORDER BY created_at",
            vec![recipient_cog_id.to_string()],
        ))?;
        Ok(rows
            .iter()
            .map(|r| (row_str(r, "id"), row_str(r, "message_id")))
            .collect())
    }

    pub fn pending_delivery_delete(&self, pending_id: &str) -> Result<(), DbError> {
        run_blocking(self.client.execute(
            "DELETE FROM pending_deliveries WHERE id = ?1",
            vec![pending_id.to_string()],
            "pending_delivery_delete",
        ))?;
        Ok(())
    }

    pub fn pending_deliveries_for_sender(&self, sender_cog_id: &str) -> Result<Vec<(String, String, String)>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT pd.id, pd.message_id, pd.recipient_cog_id FROM pending_deliveries pd INNER JOIN messages m ON m.id = pd.message_id WHERE m.sender_cog_id = ?1 ORDER BY pd.created_at",
            vec![sender_cog_id.to_string()],
        ))?;
        Ok(rows
            .iter()
            .map(|r| (row_str(r, "id"), row_str(r, "message_id"), row_str(r, "recipient_cog_id")))
            .collect())
    }

    pub fn tribe_invitation_create(&self, tribe_id: &str, inviter_cog_id: &str, invitee_cog_id: &str) -> Result<String, DbError> {
        let existing = run_blocking(self.client.query(
            "SELECT 1 FROM tribe_invitations WHERE tribe_id = ?1 AND invitee_cog_id = ?2 AND status = 'pending' LIMIT 1",
            vec![tribe_id.to_string(), invitee_cog_id.to_string()],
        ))?;
        if !existing.is_empty() {
            return Err(DbError("Invitation déjà en attente pour ce COG.".to_string()));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        run_blocking(self.client.execute(
            "INSERT INTO tribe_invitations (id, tribe_id, inviter_cog_id, invitee_cog_id, status, created_at) VALUES (?1, ?2, ?3, ?4, 'pending', ?5)",
            vec![id.clone(), tribe_id.to_string(), inviter_cog_id.to_string(), invitee_cog_id.to_string(), created_at],
            "tribe_invitation_create",
        ))?;
        Ok(id)
    }

    pub fn tribe_invitation_accept(&self, invitation_id: &str) -> Result<(), DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT tribe_id, invitee_cog_id FROM tribe_invitations WHERE id = ?1 AND status = 'pending'",
            vec![invitation_id.to_string()],
        ))?;
        let row = rows.into_iter().next().ok_or(DbError("Invitation introuvable ou déjà traitée.".to_string()))?;
        let tribe_id = row_str(&row, "tribe_id");
        let invitee_cog_id = row_str(&row, "invitee_cog_id");
        let role_rows = run_blocking(self.client.query(
            "SELECT id FROM tribe_roles WHERE tribe_id = ?1 AND name = 'Membre' LIMIT 1",
            vec![tribe_id.clone()],
        ))?;
        let role_id = role_rows
            .first()
            .map(|r| row_str(r, "id"))
            .or_else(|| {
                run_blocking(self.client.query("SELECT id FROM tribe_roles WHERE tribe_id = ?1 LIMIT 1", vec![tribe_id.clone()]))
                    .ok()
                    .and_then(|v| v.into_iter().next())
                    .map(|r| row_str(&r, "id"))
            })
            .ok_or(DbError("Aucun rôle pour cette tribu.".to_string()))?;
        run_blocking(self.client.execute(
            "UPDATE tribe_invitations SET status = 'accepted' WHERE id = ?1",
            vec![invitation_id.to_string()],
            "tribe_invitation_accept",
        ))?;
        let member_id = uuid::Uuid::new_v4().to_string();
        let joined_at = now_rfc3339();
        run_blocking(self.client.execute(
            "INSERT INTO tribe_members (id, tribe_id, cog_id, role_id, joined_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![member_id, tribe_id, invitee_cog_id, role_id, joined_at],
            "tribe_invitation_accept_member",
        ))?;
        Ok(())
    }

    pub fn tribe_invitation_refuse(&self, invitation_id: &str) -> Result<(), DbError> {
        run_blocking(self.client.execute(
            "UPDATE tribe_invitations SET status = 'refused' WHERE id = ?1 AND status = 'pending'",
            vec![invitation_id.to_string()],
            "tribe_invitation_refuse",
        ))?;
        Ok(())
    }

    pub fn tribe_invitations_pending_for(&self, invitee_cog_id: &str) -> Result<Vec<(String, String, String)>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id, tribe_id, inviter_cog_id FROM tribe_invitations WHERE invitee_cog_id = ?1 AND status = 'pending' ORDER BY created_at",
            vec![invitee_cog_id.to_string()],
        ))?;
        Ok(rows
            .iter()
            .map(|r| (row_str(r, "id"), row_str(r, "tribe_id"), row_str(r, "inviter_cog_id")))
            .collect())
    }

    pub fn tribe_default_member_role_id(&self, tribe_id: &str) -> Result<Option<String>, DbError> {
        let rows = run_blocking(self.client.query(
            "SELECT id FROM tribe_roles WHERE tribe_id = ?1 AND name = 'Membre' LIMIT 1",
            vec![tribe_id.to_string()],
        ))?;
        if let Some(r) = rows.first() {
            return Ok(Some(row_str(r, "id")));
        }
        let rows2 = run_blocking(self.client.query(
            "SELECT id FROM tribe_roles WHERE tribe_id = ?1 LIMIT 1",
            vec![tribe_id.to_string()],
        ))?;
        Ok(rows2.into_iter().next().map(|r| row_str(&r, "id")))
    }

    pub fn message_attachment_create(&self, message_id: &str, kind: &str, local_path: &str) -> Result<(), DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        run_blocking(self.client.execute(
            "INSERT INTO message_attachments (id, message_id, kind, local_path, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            vec![id, message_id.to_string(), kind.to_string(), local_path.to_string(), created_at],
            "message_attachment_create",
        ))?;
        Ok(())
    }
}
