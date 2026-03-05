//! Base de données fille SQLite Jay1Tribu (KindMother Daughter).
//!
//! @id: jay1tribu_kindmother_db
//! @do: provide_sqlite_persistence_for_jay1tribu
//! @role: persistence
//! @layer: infra
//!
//! Archives locales uniquement ; envoi/réception en temps réel requièrent le Webway.

use crate::data::types::{Friend, Message, Salon, SalonType, Tribe};
use kindmother::{InstanceIdentity, InstanceType};
#[cfg(feature = "db-encryption")]
use kindmother_db_key::KeyDerivation;
use rusqlite::params;
use std::path::Path;
use std::sync::Mutex;

/// Erreur de la base Jay1Tribu.
#[derive(Debug)]
pub struct DbError(pub String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jay1Tribu DB: {}", self.0)
    }
}

impl std::error::Error for DbError {}

impl From<rusqlite::Error> for DbError {
    fn from(e: rusqlite::Error) -> Self {
        DbError(e.to_string())
    }
}

fn now_rfc3339() -> String {
    chrono::Utc::now().to_rfc3339()
}

/// Base de données Jay1Tribu (Instance Daughter).
pub struct Jay1TribuDb {
    conn: Mutex<rusqlite::Connection>,
    pub instance: InstanceIdentity,
}

impl Jay1TribuDb {
    /// Ouvre ou crée la base à `path` et initialise le schéma.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref();
        let conn = rusqlite::Connection::open(path)?;

        #[cfg(feature = "db-encryption")]
        {
            let data_dir = path.parent().unwrap_or(path);
            let db_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("jay1tribu");
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

    fn init_schema(&self) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute_batch(
            r"
            CREATE TABLE IF NOT EXISTS friends (
                id TEXT PRIMARY KEY,
                profile_id TEXT NOT NULL,
                friend_cog_id TEXT NOT NULL,
                friend_pseudo TEXT,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tribes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                creator_cog_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS tribe_roles (
                id TEXT PRIMARY KEY,
                tribe_id TEXT NOT NULL,
                name TEXT NOT NULL,
                permissions_json TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (tribe_id) REFERENCES tribes(id)
            );
            CREATE TABLE IF NOT EXISTS tribe_members (
                id TEXT PRIMARY KEY,
                tribe_id TEXT NOT NULL,
                cog_id TEXT NOT NULL,
                role_id TEXT NOT NULL,
                joined_at TEXT NOT NULL,
                FOREIGN KEY (tribe_id) REFERENCES tribes(id),
                FOREIGN KEY (role_id) REFERENCES tribe_roles(id)
            );
            CREATE TABLE IF NOT EXISTS salons (
                id TEXT PRIMARY KEY,
                tribe_id TEXT,
                name TEXT NOT NULL,
                salon_type TEXT NOT NULL DEFAULT 'direct',
                created_at TEXT NOT NULL,
                FOREIGN KEY (tribe_id) REFERENCES tribes(id)
            );
            CREATE TABLE IF NOT EXISTS salon_members (
                salon_id TEXT NOT NULL,
                cog_id TEXT NOT NULL,
                joined_at TEXT NOT NULL,
                PRIMARY KEY (salon_id, cog_id),
                FOREIGN KEY (salon_id) REFERENCES salons(id)
            );
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                salon_id TEXT NOT NULL,
                sender_cog_id TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (salon_id) REFERENCES salons(id)
            );
            CREATE TABLE IF NOT EXISTS pending_deliveries (
                id TEXT PRIMARY KEY,
                message_id TEXT NOT NULL,
                recipient_cog_id TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (message_id) REFERENCES messages(id)
            );
            CREATE TABLE IF NOT EXISTS tribe_invitations (
                id TEXT PRIMARY KEY,
                tribe_id TEXT NOT NULL,
                inviter_cog_id TEXT NOT NULL,
                invitee_cog_id TEXT NOT NULL,
                status TEXT NOT NULL DEFAULT 'pending',
                created_at TEXT NOT NULL,
                FOREIGN KEY (tribe_id) REFERENCES tribes(id)
            );
            CREATE TABLE IF NOT EXISTS message_attachments (
                id TEXT PRIMARY KEY,
                message_id TEXT NOT NULL,
                kind TEXT NOT NULL,
                local_path TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (message_id) REFERENCES messages(id)
            );
            CREATE UNIQUE INDEX IF NOT EXISTS idx_friends_profile_cog ON friends(profile_id, friend_cog_id);
            CREATE INDEX IF NOT EXISTS idx_pending_deliveries_recipient ON pending_deliveries(recipient_cog_id);
            CREATE INDEX IF NOT EXISTS idx_tribe_invitations_invitee ON tribe_invitations(invitee_cog_id);
            CREATE INDEX IF NOT EXISTS idx_friends_profile ON friends(profile_id);
            CREATE INDEX IF NOT EXISTS idx_tribe_members_tribe ON tribe_members(tribe_id);
            CREATE INDEX IF NOT EXISTS idx_salons_tribe ON salons(tribe_id);
            CREATE INDEX IF NOT EXISTS idx_messages_salon ON messages(salon_id);
            ",
        )?;
        Ok(())
    }

    // ---------- Friends ----------
    pub fn friend_list(&self, profile_id: &str) -> Result<Vec<Friend>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, profile_id, friend_cog_id, friend_pseudo, created_at FROM friends WHERE profile_id = ?1 ORDER BY friend_pseudo, created_at",
        )?;
        let rows = stmt.query_map(params![profile_id], |row| {
            Ok(Friend {
                id: row.get(0)?,
                profile_id: row.get(1)?,
                friend_cog_id: row.get(2)?,
                friend_pseudo: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    pub fn friend_create(
        &self,
        profile_id: &str,
        friend_cog_id: &str,
        friend_pseudo: Option<&str>,
    ) -> Result<Friend, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let exists = conn
            .query_row(
                "SELECT 1 FROM friends WHERE profile_id = ?1 AND friend_cog_id = ?2 LIMIT 1",
                params![profile_id, friend_cog_id],
                |row| row.get::<_, i32>(0),
            )
            .is_ok();
        if exists {
            return Err(DbError(
                "Cet ami est déjà dans la liste (profile_id, friend_cog_id) uniques.".to_string(),
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        conn.execute(
            "INSERT INTO friends (id, profile_id, friend_cog_id, friend_pseudo, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, profile_id, friend_cog_id, friend_pseudo, created_at],
        )?;
        Ok(Friend {
            id: id.clone(),
            profile_id: profile_id.to_string(),
            friend_cog_id: friend_cog_id.to_string(),
            friend_pseudo: friend_pseudo.map(String::from),
            created_at,
        })
    }

    pub fn friend_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM friends WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Vérifie si `friend_cog_id` est dans la liste d'amis de `profile_id`.
    pub fn is_friend(&self, profile_id: &str, friend_cog_id: &str) -> Result<bool, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let count: i32 = conn.query_row(
            "SELECT COUNT(1) FROM friends WHERE profile_id = ?1 AND friend_cog_id = ?2",
            params![profile_id, friend_cog_id],
            |row| row.get(0),
        )?;
        Ok(count > 0)
    }

    // ---------- Tribes ----------
    pub fn tribe_list(&self, cog_id: &str) -> Result<Vec<Tribe>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT t.id, t.name, t.description, t.creator_cog_id, t.created_at, t.updated_at
             FROM tribes t INNER JOIN tribe_members m ON t.id = m.tribe_id WHERE m.cog_id = ?1 ORDER BY t.name",
        )?;
        let rows = stmt.query_map(params![cog_id], |row| {
            Ok(Tribe {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                creator_cog_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    pub fn tribe_by_id(&self, id: &str) -> Result<Option<Tribe>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, creator_cog_id, created_at, updated_at FROM tribes WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(Tribe {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                creator_cog_id: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })?;
        Ok(rows.next().transpose()?)
    }

    pub fn tribe_create(
        &self,
        name: &str,
        description: Option<&str>,
        creator_cog_id: &str,
    ) -> Result<Tribe, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = now_rfc3339();
        let mut conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let tx = conn.transaction().map_err(|e| DbError(e.to_string()))?;
        tx.execute(
            "INSERT INTO tribes (id, name, description, creator_cog_id, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![id, name, description, creator_cog_id, now, now],
        )?;
        let chef_role_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO tribe_roles (id, tribe_id, name, permissions_json, created_at) VALUES (?1, ?2, 'Chef de tribu', NULL, ?3)",
            params![chef_role_id, id, now],
        )?;
        let membre_role_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO tribe_roles (id, tribe_id, name, permissions_json, created_at) VALUES (?1, ?2, 'Membre', NULL, ?3)",
            params![membre_role_id, id, now],
        )?;
        let member_id = uuid::Uuid::new_v4().to_string();
        tx.execute(
            "INSERT INTO tribe_members (id, tribe_id, cog_id, role_id, joined_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![member_id, id, creator_cog_id, chef_role_id, now],
        )?;
        tx.commit().map_err(|e| DbError(e.to_string()))?;
        Ok(Tribe {
            id: id.clone(),
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
        let now = now_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE tribes SET name = ?1, description = ?2, updated_at = ?3 WHERE id = ?4",
            params![name, description, now, id],
        )?;
        Ok(())
    }

    pub fn tribe_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM tribe_members WHERE tribe_id = ?1", params![id])?;
        conn.execute("DELETE FROM tribe_roles WHERE tribe_id = ?1", params![id])?;
        let salon_ids: Vec<String> = conn
            .prepare("SELECT id FROM salons WHERE tribe_id = ?1")?
            .query_map(params![id], |row| row.get(0))?
            .collect::<Result<Vec<_>, _>>()?;
        for salon_id in &salon_ids {
            conn.execute(
                "DELETE FROM messages WHERE salon_id = ?1",
                params![salon_id],
            )?;
            conn.execute(
                "DELETE FROM salon_members WHERE salon_id = ?1",
                params![salon_id],
            )?;
        }
        conn.execute("DELETE FROM salons WHERE tribe_id = ?1", params![id])?;
        conn.execute("DELETE FROM tribes WHERE id = ?1", params![id])?;
        Ok(())
    }

    // ---------- Salons ----------
    fn row_to_salon(row: &rusqlite::Row<'_>) -> rusqlite::Result<Salon> {
        Ok(Salon {
            id: row.get(0)?,
            tribe_id: row.get(1)?,
            name: row.get(2)?,
            salon_type: SalonType::from_str(&row.get::<_, String>(3).unwrap_or_default()),
            created_at: row.get(4)?,
        })
    }

    pub fn salon_list(&self, tribe_id: Option<&str>) -> Result<Vec<Salon>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let out = if let Some(tid) = tribe_id {
            let mut stmt = conn.prepare(
                "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE tribe_id = ?1 ORDER BY name",
            )?;
            let rows: Vec<Salon> = stmt
                .query_map(params![tid], Self::row_to_salon)?
                .collect::<Result<Vec<_>, _>>()?;
            rows
        } else {
            let mut stmt = conn.prepare(
                "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE tribe_id IS NULL ORDER BY created_at DESC",
            )?;
            let rows: Vec<Salon> = stmt
                .query_map([], Self::row_to_salon)?
                .collect::<Result<Vec<_>, _>>()?;
            rows
        };
        Ok(out)
    }

    pub fn salon_by_id(&self, id: &str) -> Result<Option<Salon>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, tribe_id, name, salon_type, created_at FROM salons WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(Salon {
                id: row.get(0)?,
                tribe_id: row.get(1)?,
                name: row.get(2)?,
                salon_type: SalonType::from_str(&row.get::<_, String>(3).unwrap_or_default()),
                created_at: row.get(4)?,
            })
        })?;
        Ok(rows.next().transpose()?)
    }

    pub fn salon_create(
        &self,
        tribe_id: Option<&str>,
        name: &str,
        salon_type: SalonType,
    ) -> Result<Salon, DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO salons (id, tribe_id, name, salon_type, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, tribe_id, name, salon_type.as_str(), created_at],
        )?;
        Ok(Salon {
            id: id.clone(),
            tribe_id: tribe_id.map(String::from),
            name: name.to_string(),
            salon_type,
            created_at: created_at.clone(),
        })
    }

    pub fn salon_add_member(&self, salon_id: &str, cog_id: &str) -> Result<(), DbError> {
        let joined_at = now_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT OR IGNORE INTO salon_members (salon_id, cog_id, joined_at) VALUES (?1, ?2, ?3)",
            params![salon_id, cog_id, joined_at],
        )?;
        Ok(())
    }

    /// Liste les cog_ids des membres d'un salon (pour envoi MWS).
    pub fn salon_member_cog_ids(&self, salon_id: &str) -> Result<Vec<String>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare("SELECT cog_id FROM salon_members WHERE salon_id = ?1")?;
        let rows = stmt.query_map(params![salon_id], |row| row.get(0))?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    pub fn salon_delete(&self, id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute("DELETE FROM messages WHERE salon_id = ?1", params![id])?;
        conn.execute("DELETE FROM salon_members WHERE salon_id = ?1", params![id])?;
        conn.execute("DELETE FROM salons WHERE id = ?1", params![id])?;
        Ok(())
    }

    /// Liste les salons dont le `cog_id` est membre (pour afficher les conversations).
    pub fn salon_list_for_cog(&self, cog_id: &str) -> Result<Vec<Salon>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at
             FROM salons s INNER JOIN salon_members m ON s.id = m.salon_id WHERE m.cog_id = ?1 ORDER BY s.created_at DESC",
        )?;
        let rows = stmt.query_map(params![cog_id], |row| {
            Ok(Salon {
                id: row.get(0)?,
                tribe_id: row.get(1)?,
                name: row.get(2)?,
                salon_type: SalonType::from_str(&row.get::<_, String>(3).unwrap_or_default()),
                created_at: row.get(4)?,
            })
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    /// Trouve le salon direct existant entre deux COG (DAT-01 : au plus un salon direct par paire).
    pub fn salon_find_direct_between(
        &self,
        cog_id_1: &str,
        cog_id_2: &str,
    ) -> Result<Option<Salon>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT s.id, s.tribe_id, s.name, s.salon_type, s.created_at
             FROM salons s
             WHERE s.tribe_id IS NULL
             AND (SELECT COUNT(*) FROM salon_members m WHERE m.salon_id = s.id) = 2
             AND EXISTS (SELECT 1 FROM salon_members m WHERE m.salon_id = s.id AND m.cog_id = ?1)
             AND EXISTS (SELECT 1 FROM salon_members m WHERE m.salon_id = s.id AND m.cog_id = ?2)
             LIMIT 1",
        )?;
        let mut rows = stmt.query_map(params![cog_id_1, cog_id_2], Self::row_to_salon)?;
        Ok(rows.next().transpose()?)
    }

    // ---------- Messages ----------
    pub fn message_list(&self, salon_id: &str, limit: i64) -> Result<Vec<Message>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, salon_id, sender_cog_id, content, created_at FROM messages WHERE salon_id = ?1 ORDER BY created_at DESC LIMIT ?2",
        )?;
        let rows = stmt.query_map(params![salon_id, limit], |row| {
            Ok(Message {
                id: row.get(0)?,
                salon_id: row.get(1)?,
                sender_cog_id: row.get(2)?,
                content: row.get(3)?,
                created_at: row.get(4)?,
            })
        })?;
        let mut msgs: Vec<Message> = rows.collect::<Result<Vec<_>, _>>()?;
        msgs.reverse();
        Ok(msgs)
    }

    pub fn message_create(
        &self,
        salon_id: &str,
        sender_cog_id: &str,
        content: &str,
    ) -> Result<Message, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let is_member: bool = conn
            .query_row(
                "SELECT 1 FROM salon_members WHERE salon_id = ?1 AND cog_id = ?2 LIMIT 1",
                params![salon_id, sender_cog_id],
                |row| row.get::<_, i32>(0),
            )
            .map(|_| true)
            .or_else(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => Ok(false),
                e => Err(e),
            })?;
        if !is_member {
            return Err(DbError(
                "L'expéditeur n'est pas membre du salon.".to_string(),
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        conn.execute(
            "INSERT INTO messages (id, salon_id, sender_cog_id, content, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, salon_id, sender_cog_id, content, created_at],
        )?;
        Ok(Message {
            id: id.clone(),
            salon_id: salon_id.to_string(),
            sender_cog_id: sender_cog_id.to_string(),
            content: content.to_string(),
            created_at: created_at.clone(),
        })
    }

    /// Récupère un message par ID (pour livraison différée).
    pub fn message_by_id(&self, message_id: &str) -> Result<Option<Message>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let opt: Option<Message> = conn
            .query_row(
                "SELECT id, salon_id, sender_cog_id, content, created_at FROM messages WHERE id = ?1",
                params![message_id],
                |row| {
                    Ok(Message {
                        id: row.get(0)?,
                        salon_id: row.get(1)?,
                        sender_cog_id: row.get(2)?,
                        content: row.get(3)?,
                        created_at: row.get(4)?,
                    })
                },
            )
            .ok();
        Ok(opt)
    }

    // ---------- Pending deliveries (H2 livraison différée) ----------
    /// Enregistre une livraison en attente pour un destinataire (tribu).
    pub fn pending_delivery_enqueue(
        &self,
        message_id: &str,
        recipient_cog_id: &str,
    ) -> Result<(), DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO pending_deliveries (id, message_id, recipient_cog_id, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id, message_id, recipient_cog_id, created_at],
        )?;
        Ok(())
    }

    /// Liste les IDs de messages en attente pour un destinataire donné.
    pub fn pending_delivery_message_ids_for(
        &self,
        recipient_cog_id: &str,
    ) -> Result<Vec<(String, String)>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, message_id FROM pending_deliveries WHERE recipient_cog_id = ?1 ORDER BY created_at",
        )?;
        let rows = stmt.query_map(params![recipient_cog_id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    /// Supprime une entrée de livraison en attente.
    pub fn pending_delivery_delete(&self, pending_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "DELETE FROM pending_deliveries WHERE id = ?1",
            params![pending_id],
        )?;
        Ok(())
    }

    /// Liste les livraisons en attente pour les messages envoyés par un COG donné.
    /// Retourne (pending_id, message_id, recipient_cog_id) pour reprise à la reconnexion.
    pub fn pending_deliveries_for_sender(
        &self,
        sender_cog_id: &str,
    ) -> Result<Vec<(String, String, String)>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT pd.id, pd.message_id, pd.recipient_cog_id FROM pending_deliveries pd
             INNER JOIN messages m ON m.id = pd.message_id
             WHERE m.sender_cog_id = ?1 ORDER BY pd.created_at",
        )?;
        let rows = stmt.query_map(params![sender_cog_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    // ---------- Tribe invitations (M4) ----------
    /// Crée une invitation à une tribu (status = pending).
    pub fn tribe_invitation_create(
        &self,
        tribe_id: &str,
        inviter_cog_id: &str,
        invitee_cog_id: &str,
    ) -> Result<String, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let exists = conn
            .query_row(
                "SELECT 1 FROM tribe_invitations WHERE tribe_id = ?1 AND invitee_cog_id = ?2 AND status = 'pending' LIMIT 1",
                params![tribe_id, invitee_cog_id],
                |row| row.get::<_, i32>(0),
            )
            .is_ok();
        if exists {
            return Err(DbError(
                "Invitation déjà en attente pour ce COG.".to_string(),
            ));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        conn.execute(
            "INSERT INTO tribe_invitations (id, tribe_id, inviter_cog_id, invitee_cog_id, status, created_at) VALUES (?1, ?2, ?3, ?4, 'pending', ?5)",
            params![id, tribe_id, inviter_cog_id, invitee_cog_id, created_at],
        )?;
        Ok(id)
    }

    /// Accepte une invitation (ajoute le membre à la tribu, met status = accepted).
    pub fn tribe_invitation_accept(&self, invitation_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let (tribe_id, invitee_cog_id): (String, String) = conn.query_row(
            "SELECT tribe_id, invitee_cog_id FROM tribe_invitations WHERE id = ?1 AND status = 'pending'",
            params![invitation_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )?;
        let role_id: String = conn
            .query_row(
                "SELECT id FROM tribe_roles WHERE tribe_id = ?1 AND name = 'Membre' LIMIT 1",
                params![tribe_id],
                |row| row.get(0),
            )
            .or_else(|_| {
                conn.query_row(
                    "SELECT id FROM tribe_roles WHERE tribe_id = ?1 LIMIT 1",
                    params![tribe_id],
                    |row| row.get(0),
                )
            })
            .map_err(|e| DbError(format!("Aucun rôle pour cette tribu: {}", e)))?;
        conn.execute(
            "UPDATE tribe_invitations SET status = 'accepted' WHERE id = ?1",
            params![invitation_id],
        )?;
        let member_id = uuid::Uuid::new_v4().to_string();
        let joined_at = now_rfc3339();
        conn.execute(
            "INSERT INTO tribe_members (id, tribe_id, cog_id, role_id, joined_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![member_id, tribe_id, invitee_cog_id, role_id, joined_at],
        )?;
        Ok(())
    }

    /// Refuse une invitation (status = refused).
    pub fn tribe_invitation_refuse(&self, invitation_id: &str) -> Result<(), DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "UPDATE tribe_invitations SET status = 'refused' WHERE id = ?1 AND status = 'pending'",
            params![invitation_id],
        )?;
        Ok(())
    }

    /// Liste les invitations en attente pour un COG (invitee).
    pub fn tribe_invitations_pending_for(
        &self,
        invitee_cog_id: &str,
    ) -> Result<Vec<(String, String, String)>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let mut stmt = conn.prepare(
            "SELECT id, tribe_id, inviter_cog_id FROM tribe_invitations WHERE invitee_cog_id = ?1 AND status = 'pending' ORDER BY created_at",
        )?;
        let rows = stmt.query_map(params![invitee_cog_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(DbError::from)
    }

    /// Récupère le role_id par défaut (premier rôle « Membre » ou premier rôle) d'une tribu.
    pub fn tribe_default_member_role_id(&self, tribe_id: &str) -> Result<Option<String>, DbError> {
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        let r: Option<String> = conn
            .query_row(
                "SELECT id FROM tribe_roles WHERE tribe_id = ?1 AND name = 'Membre' LIMIT 1",
                params![tribe_id],
                |row| row.get(0),
            )
            .ok();
        if r.is_some() {
            return Ok(r);
        }
        let r2: Option<String> = conn
            .query_row(
                "SELECT id FROM tribe_roles WHERE tribe_id = ?1 LIMIT 1",
                params![tribe_id],
                |row| row.get(0),
            )
            .ok();
        Ok(r2)
    }

    // ---------- Message attachments (M2) ----------
    /// Enregistre une pièce jointe (fichier ou image) sur un message.
    pub fn message_attachment_create(
        &self,
        message_id: &str,
        kind: &str,
        local_path: &str,
    ) -> Result<(), DbError> {
        let id = uuid::Uuid::new_v4().to_string();
        let created_at = now_rfc3339();
        let conn = self.conn.lock().map_err(|e| DbError(e.to_string()))?;
        conn.execute(
            "INSERT INTO message_attachments (id, message_id, kind, local_path, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, message_id, kind, local_path, created_at],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::SalonType;

    fn mem_db() -> Jay1TribuDb {
        Jay1TribuDb::open(":memory:").unwrap()
    }

    #[test]
    fn friend_create_duplicate_returns_error() {
        let db = mem_db();
        db.friend_create("profile1", "cog-a", Some("Pseudo"))
            .unwrap();
        let err = db.friend_create("profile1", "cog-a", None).unwrap_err();
        assert!(err.0.contains("déjà dans la liste"));
    }

    #[test]
    fn tribe_create_and_read() {
        let db = mem_db();
        let tribe = db.tribe_create("Ma tribu", None, "creator-cog").unwrap();
        assert_eq!(tribe.name, "Ma tribu");
        let found = db.tribe_by_id(&tribe.id).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Ma tribu");
    }

    #[test]
    fn salon_find_direct_between_and_dat01() {
        let db = mem_db();
        assert!(db
            .salon_find_direct_between("cog-a", "cog-b")
            .unwrap()
            .is_none());
        let s1 = db.salon_create(None, "DM", SalonType::Direct).unwrap();
        db.salon_add_member(&s1.id, "cog-a").unwrap();
        db.salon_add_member(&s1.id, "cog-b").unwrap();
        let found = db.salon_find_direct_between("cog-a", "cog-b").unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, s1.id);
    }

    #[test]
    fn message_create_rejects_non_member() {
        let db = mem_db();
        let salon = db.salon_create(None, "DM", SalonType::Direct).unwrap();
        db.salon_add_member(&salon.id, "cog-a").unwrap();
        let err = db
            .message_create(&salon.id, "cog-other", "Hello")
            .unwrap_err();
        assert!(err.0.contains("n'est pas membre"));
    }

    #[test]
    fn message_create_ok_when_member() {
        let db = mem_db();
        let salon = db.salon_create(None, "DM", SalonType::Direct).unwrap();
        db.salon_add_member(&salon.id, "cog-a").unwrap();
        let msg = db.message_create(&salon.id, "cog-a", "Hello").unwrap();
        assert_eq!(msg.content, "Hello");
        let list = db.message_list(&salon.id, 10).unwrap();
        assert_eq!(list.len(), 1);
    }
}
