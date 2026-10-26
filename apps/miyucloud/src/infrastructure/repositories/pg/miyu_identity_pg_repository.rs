//! @id     miyucloud_infrastructure_repositories_pg_miyu_identity_pg_repository
//! @do     stocker_liaisons_et_cles_appareil
//! @role   donnee
//! @layer  app
//! @human  miyucloud — liaisons identité Miyukini → compte et clés d'appareil (PostgreSQL).

//! Stockage PostgreSQL de [`MiyuIdentityStore`] : tables `auth.miyu_identities`
//! et `auth.device_keys` (migration `20261024000001_lot2_sans_secret_partage`).

use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;

use crate::common::errors::{DomainError, ErrorKind};
use crate::infrastructure::services::miyu_auth_service::{
    CleAppareil, IdentiteLiee, MiyuIdentityStore,
};

/// Dépôt PostgreSQL des identités Miyukini et clés d'appareil.
pub struct MiyuIdentityPgRepository {
    pool: Arc<PgPool>,
}

impl MiyuIdentityPgRepository {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

fn erreur(contexte: &str, e: sqlx::Error) -> DomainError {
    if let sqlx::Error::Database(db) = &e
        && db.is_unique_violation()
    {
        return DomainError::new(
            ErrorKind::AlreadyExists,
            "MiyuIdentity",
            format!("{contexte}: already exists"),
        );
    }
    DomainError::new(
        ErrorKind::DatabaseError,
        "MiyuIdentity",
        format!("{contexte}: {e}"),
    )
}

fn ligne_identite(row: &sqlx::postgres::PgRow) -> Result<IdentiteLiee, DomainError> {
    let lire = |e: sqlx::Error| erreur("read identity", e);
    let sujet: Vec<u8> = row.try_get("sujet").map_err(lire)?;
    Ok(IdentiteLiee {
        id: row.try_get("id").map_err(lire)?,
        user_id: row.try_get("user_id").map_err(lire)?,
        cog_id: row.try_get("cog_id").map_err(lire)?,
        sujet: hex::encode(sujet),
        label: row.try_get("label").map_err(lire)?,
        created_at: row.try_get("created_at").map_err(lire)?,
        last_used_at: row.try_get("last_used_at").map_err(lire)?,
    })
}

fn ligne_cle(row: &sqlx::postgres::PgRow) -> Result<CleAppareil, DomainError> {
    let lire = |e: sqlx::Error| erreur("read device key", e);
    let cle: Vec<u8> = row.try_get("public_key").map_err(lire)?;
    Ok(CleAppareil {
        id: row.try_get("id").map_err(lire)?,
        user_id: row.try_get("user_id").map_err(lire)?,
        public_key: miyauth::http::b64_encoder(&cle),
        client_name: row.try_get("client_name").map_err(lire)?,
        scopes: row.try_get("scopes").map_err(lire)?,
        created_at: row.try_get("created_at").map_err(lire)?,
        last_used_at: row.try_get("last_used_at").map_err(lire)?,
    })
}

const COLONNES_IDENTITE: &str = "id, user_id, cog_id, sujet, label, created_at, last_used_at";
const COLONNES_CLE: &str = "id, user_id, public_key, client_name, scopes, created_at, last_used_at";

impl MiyuIdentityStore for MiyuIdentityPgRepository {
    async fn user_for_identity(
        &self,
        cog_id: &str,
        sujet: &[u8; 32],
    ) -> Result<Option<Uuid>, DomainError> {
        // Met à jour last_used_at au passage (une seule requête).
        let row = sqlx::query(
            "UPDATE auth.miyu_identities SET last_used_at = NOW() \
             WHERE cog_id = $1 AND sujet = $2 RETURNING user_id",
        )
        .bind(cog_id)
        .bind(&sujet[..])
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| erreur("lookup identity", e))?;
        row.map(|r| r.try_get("user_id"))
            .transpose()
            .map_err(|e| erreur("lookup identity", e))
    }

    async fn link_identity(
        &self,
        user_id: Uuid,
        cog_id: &str,
        sujet: &[u8; 32],
        label: &str,
    ) -> Result<IdentiteLiee, DomainError> {
        let row = sqlx::query(&format!(
            "INSERT INTO auth.miyu_identities (user_id, cog_id, sujet, label) \
             VALUES ($1, $2, $3, $4) RETURNING {COLONNES_IDENTITE}"
        ))
        .bind(user_id)
        .bind(cog_id)
        .bind(&sujet[..])
        .bind(label)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| erreur("link identity", e))?;
        ligne_identite(&row)
    }

    async fn list_identities(&self, user_id: Uuid) -> Result<Vec<IdentiteLiee>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {COLONNES_IDENTITE} FROM auth.miyu_identities \
             WHERE user_id = $1 ORDER BY created_at"
        ))
        .bind(user_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| erreur("list identities", e))?;
        rows.iter().map(ligne_identite).collect()
    }

    async fn unlink_identity(&self, user_id: Uuid, id: Uuid) -> Result<bool, DomainError> {
        let r = sqlx::query("DELETE FROM auth.miyu_identities WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(self.pool.as_ref())
            .await
            .map_err(|e| erreur("unlink identity", e))?;
        Ok(r.rows_affected() > 0)
    }

    async fn register_device_key(
        &self,
        user_id: Uuid,
        public_key: &[u8; 32],
        client_name: &str,
        scopes: &str,
    ) -> Result<CleAppareil, DomainError> {
        let row = sqlx::query(&format!(
            "INSERT INTO auth.device_keys (user_id, public_key, client_name, scopes) \
             VALUES ($1, $2, $3, $4) RETURNING {COLONNES_CLE}"
        ))
        .bind(user_id)
        .bind(&public_key[..])
        .bind(client_name)
        .bind(scopes)
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| erreur("register device key", e))?;
        ligne_cle(&row)
    }

    async fn active_device_key(
        &self,
        public_key: &[u8; 32],
    ) -> Result<Option<CleAppareil>, DomainError> {
        let row = sqlx::query(&format!(
            "UPDATE auth.device_keys SET last_used_at = NOW() \
             WHERE public_key = $1 AND NOT revoked RETURNING {COLONNES_CLE}"
        ))
        .bind(&public_key[..])
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| erreur("lookup device key", e))?;
        row.as_ref().map(ligne_cle).transpose()
    }

    async fn list_device_keys(&self, user_id: Uuid) -> Result<Vec<CleAppareil>, DomainError> {
        let rows = sqlx::query(&format!(
            "SELECT {COLONNES_CLE} FROM auth.device_keys \
             WHERE user_id = $1 AND NOT revoked ORDER BY created_at"
        ))
        .bind(user_id)
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| erreur("list device keys", e))?;
        rows.iter().map(ligne_cle).collect()
    }

    async fn revoke_device_key(&self, user_id: Uuid, id: Uuid) -> Result<bool, DomainError> {
        let r = sqlx::query(
            "UPDATE auth.device_keys SET revoked = TRUE \
             WHERE id = $1 AND user_id = $2 AND NOT revoked",
        )
        .bind(id)
        .bind(user_id)
        .execute(self.pool.as_ref())
        .await
        .map_err(|e| erreur("revoke device key", e))?;
        Ok(r.rows_affected() > 0)
    }

    async fn device_key_revoked(&self, public_key: &[u8; 32]) -> Result<bool, DomainError> {
        let row = sqlx::query(
            "SELECT EXISTS (SELECT 1 FROM auth.device_keys              WHERE public_key = $1 AND revoked) AS retiree",
        )
        .bind(&public_key[..])
        .fetch_one(self.pool.as_ref())
        .await
        .map_err(|e| erreur("lookup revoked device key", e))?;
        row.try_get("retiree")
            .map_err(|e| erreur("lookup revoked device key", e))
    }

    async fn rotate_device_key(
        &self,
        user_id: Uuid,
        ancienne_id: Uuid,
        nouvelle: &[u8; 32],
    ) -> Result<Option<CleAppareil>, DomainError> {
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|e| erreur("rotate device key", e))?;
        // Verrou de ligne : deux rotations concurrentes de la même clé ne
        // peuvent pas toutes deux réussir.
        let ancienne = sqlx::query(
            "UPDATE auth.device_keys SET revoked = TRUE              WHERE id = $1 AND user_id = $2 AND NOT revoked              RETURNING client_name, scopes",
        )
        .bind(ancienne_id)
        .bind(user_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| erreur("rotate device key", e))?;
        let Some(ancienne) = ancienne else {
            // Rien n'a changé : la transaction est abandonnée.
            return Ok(None);
        };
        let lire = |e: sqlx::Error| erreur("rotate device key", e);
        let nom: String = ancienne.try_get("client_name").map_err(lire)?;
        let portees: String = ancienne.try_get("scopes").map_err(lire)?;
        let row = sqlx::query(&format!(
            "INSERT INTO auth.device_keys (user_id, public_key, client_name, scopes)              VALUES ($1, $2, $3, $4) RETURNING {COLONNES_CLE}"
        ))
        .bind(user_id)
        .bind(&nouvelle[..])
        .bind(&nom)
        .bind(&portees)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| erreur("rotate device key", e))?;
        let cle = ligne_cle(&row)?;
        tx.commit()
            .await
            .map_err(|e| erreur("rotate device key", e))?;
        Ok(Some(cle))
    }
}
