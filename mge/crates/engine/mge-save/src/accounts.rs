// @id: MGE-Save-Accounts @do: dal-accounts @role: back-end @layer: 3 @human: denis
//! DAL pour les comptes utilisateurs.
//!
//! CRUD : creation, recherche par username, mise a jour du dernier login.

use rusqlite::params;
use uuid::Uuid;
use chrono::Utc;
use crate::{DbPool, PersistResult, PersistenceError};

/// DAL pour les comptes utilisateurs.
pub struct AccountDal<'a>(pub &'a DbPool);

/// Representation d'un compte utilisateur en base.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Account {
    /// Identifiant unique (UUID v4).
    pub id: String,
    /// Nom d'utilisateur (unique).
    pub username: String,
    /// Hash bcrypt du mot de passe.
    pub password_hash: String,
    /// Adresse email (unique).
    pub email: String,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Dernier login (ISO 8601) ou `None`.
    pub last_login: Option<String>,
    /// Compte banni.
    pub is_banned: bool,
    /// Raison du bannissement (si applicable).
    pub ban_reason: Option<String>,
}

/// Parametres de creation d'un compte.
#[derive(Debug)]
pub struct CreateAccountParams<'a> {
    /// Nom d'utilisateur souhaite.
    pub username: &'a str,
    /// Hash bcrypt du mot de passe.
    pub password_hash: &'a str,
    /// Adresse email.
    pub email: &'a str,
}

impl AccountDal<'_> {
    /// Cree un nouveau compte et retourne la struct `Account` correspondante.
    ///
    /// Retourne `PersistenceError::Duplicate` si le username ou l'email existe deja.
    pub fn create(&self, params: &CreateAccountParams<'_>) -> PersistResult<Account> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "INSERT INTO accounts (id, username, password_hash, email, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
                params![id, params.username, params.password_hash, params.email, now],
            )
            .map_err(|e| {
                if e.to_string().contains("UNIQUE") {
                    PersistenceError::Duplicate(params.username.to_string())
                } else {
                    e.into()
                }
            })?;
            Ok(Account {
                id,
                username: params.username.to_string(),
                password_hash: params.password_hash.to_string(),
                email: params.email.to_string(),
                created_at: now,
                last_login: None,
                is_banned: false,
                ban_reason: None,
            })
        })
    }

    /// Recherche un compte par son nom d'utilisateur.
    ///
    /// Retourne `PersistenceError::NotFound` si aucun compte ne correspond.
    pub fn find_by_username(&self, username: &str) -> PersistResult<Account> {
        self.0.with(|conn| {
            conn.query_row(
                "SELECT id, username, password_hash, email, created_at, last_login,
                         is_banned, ban_reason
                 FROM accounts WHERE username = ?1",
                params![username],
                |row| {
                    Ok(Account {
                        id: row.get(0)?,
                        username: row.get(1)?,
                        password_hash: row.get(2)?,
                        email: row.get(3)?,
                        created_at: row.get(4)?,
                        last_login: row.get(5)?,
                        is_banned: row.get::<_, i64>(6)? != 0,
                        ban_reason: row.get(7)?,
                    })
                },
            )
            .map_err(|e| match e {
                rusqlite::Error::QueryReturnedNoRows => PersistenceError::NotFound {
                    entity: "Account",
                    id: username.to_string(),
                },
                other => other.into(),
            })
        })
    }

    /// Met a jour le dernier login d'un compte a l'instant courant.
    pub fn update_last_login(&self, account_id: &str) -> PersistResult<()> {
        let now = Utc::now().to_rfc3339();
        self.0.with(|conn| {
            conn.execute(
                "UPDATE accounts SET last_login = ?1 WHERE id = ?2",
                params![now, account_id],
            )?;
            Ok(())
        })
    }
}
