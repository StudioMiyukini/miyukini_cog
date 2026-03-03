//! Gestion des sessions serveur MiyuCloud.
//!
//! @id: miyucloud_auth_sessions
//! @do: provide_session_create_validate_revoke
//! @role: auth
//! @layer: domain
//!
//! Sessions avec token 256-bit, expiration, inactivite,
//! et flag totp_verified pour la 2FA.

use crate::data::types::SessionRow;
use crate::errors::MiyucloudError;
use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use std::fmt::Write;

/// Maximum number of concurrent active sessions per owner.
const MAX_SESSIONS_PER_OWNER: u64 = 5;

/// Session expiration duration (24 hours from creation).
const SESSION_EXPIRY_HOURS: i64 = 24;

/// Inactivity timeout duration (2 hours since last activity).
const INACTIVITY_TIMEOUT_HOURS: i64 = 2;

/// Informations d'une session active.
#[derive(Debug)]
pub struct SessionInfo {
    /// Identifiant unique de la session (UUID v4).
    pub id: String,
    /// Identifiant du proprietaire.
    pub owner_id: String,
    /// Date de creation (ISO 8601).
    pub created_at: String,
    /// Date d'expiration (ISO 8601).
    pub expires_at: String,
    /// Date de derniere activite (ISO 8601).
    pub last_activity_at: String,
    /// Le code TOTP a ete verifie pour cette session.
    pub totp_verified: bool,
    /// Hash de l'IP de creation (RGPD).
    pub created_ip_hash: Option<String>,
    /// User-Agent de creation (tronque).
    pub created_ua: Option<String>,
}

/// Generate a 256-bit (32 byte) random token encoded as 64 hex chars.
fn generate_session_token() -> String {
    let mut bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut bytes);
    bytes.iter().fold(String::with_capacity(64), |mut acc, b| {
        let _ = write!(acc, "{b:02x}");
        acc
    })
}

/// Map a `SessionRow` (DB layer) to a `SessionInfo` (public API).
fn session_row_to_info(row: &SessionRow) -> SessionInfo {
    SessionInfo {
        id: row.id.clone(),
        owner_id: row.owner_id.clone(),
        created_at: row.created_at.clone(),
        expires_at: row.expires_at.clone(),
        last_activity_at: row.last_activity_at.clone(),
        totp_verified: row.totp_verified,
        created_ip_hash: row.created_ip_hash.clone(),
        created_ua: row.created_ua.clone(),
    }
}

/// Parse an RFC 3339 date string into a `DateTime<Utc>`.
fn parse_rfc3339(s: &str) -> Result<DateTime<Utc>, MiyucloudError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| MiyucloudError::InvalidInput(format!("Invalid date: {e}")))
}

/// Cree une nouvelle session pour un proprietaire.
///
/// Genere un token 256-bit (64 chars hex), stocke en DB.
/// Max 5 sessions par proprietaire (evicte la plus ancienne).
pub fn create_session(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
    ip_hash: Option<&str>,
    user_agent: Option<&str>,
) -> Result<String, MiyucloudError> {
    let token = generate_session_token();
    let session_id = uuid::Uuid::new_v4().to_string();
    let now = Utc::now();
    let expires_at = now + Duration::hours(SESSION_EXPIRY_HOURS);
    let expires_at_str = expires_at.to_rfc3339();

    // Evict oldest session if at capacity.
    let count = db.session_count_by_owner(owner_id)?;
    if count >= MAX_SESSIONS_PER_OWNER {
        if let Some(oldest_id) = db.session_oldest_by_owner(owner_id)? {
            db.session_revoke(&oldest_id)?;
        }
    }

    db.session_insert(
        &session_id,
        owner_id,
        &token,
        ip_hash,
        user_agent,
        &expires_at_str,
    )?;

    Ok(token)
}

/// Valide un token de session et retourne les infos.
///
/// Verifie : existence, non-revoquee, non-expiree, activite recente.
/// Met a jour `last_activity_at` si valide.
pub fn validate_session(
    db: &crate::data::MiyucloudDb,
    token: &str,
) -> Result<SessionInfo, MiyucloudError> {
    let row = db
        .session_by_token(token)?
        .ok_or_else(|| MiyucloudError::NotFound("Session not found".into()))?;

    if row.is_revoked {
        return Err(MiyucloudError::PermissionDenied("Session revoked".into()));
    }

    let now = Utc::now();

    let expires_at = parse_rfc3339(&row.expires_at)?;
    if now >= expires_at {
        return Err(MiyucloudError::PermissionDenied("Session expired".into()));
    }

    let last_activity = parse_rfc3339(&row.last_activity_at)?;
    let inactivity_limit = Duration::hours(INACTIVITY_TIMEOUT_HOURS);
    if now - last_activity > inactivity_limit {
        return Err(MiyucloudError::PermissionDenied(
            "Session inactive".into(),
        ));
    }

    db.session_update_activity(&row.id)?;

    Ok(session_row_to_info(&row))
}

/// Revoque une session specifique.
pub fn revoke_session(
    db: &crate::data::MiyucloudDb,
    session_id: &str,
) -> Result<(), MiyucloudError> {
    db.session_revoke(session_id)
}

/// Revoque toutes les sessions d'un proprietaire.
pub fn revoke_all_sessions(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<u64, MiyucloudError> {
    db.session_revoke_all(owner_id)
}

/// Liste les sessions actives d'un proprietaire.
pub fn list_sessions(
    db: &crate::data::MiyucloudDb,
    owner_id: &str,
) -> Result<Vec<SessionInfo>, MiyucloudError> {
    let rows = db.sessions_by_owner(owner_id)?;
    let active: Vec<SessionInfo> = rows
        .iter()
        .filter(|r| !r.is_revoked)
        .map(session_row_to_info)
        .collect();
    Ok(active)
}

/// Marque une session comme ayant complete la verification TOTP.
pub fn mark_totp_verified(
    db: &crate::data::MiyucloudDb,
    session_id: &str,
) -> Result<(), MiyucloudError> {
    db.session_mark_totp_verified(session_id)
}

/// Purge les sessions expirees de la DB.
pub fn purge_expired_sessions(
    db: &crate::data::MiyucloudDb,
) -> Result<u64, MiyucloudError> {
    db.session_purge_expired()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "legacy-sqlite")]
mod tests {
    use super::*;
    use crate::data::MiyucloudDb;

    /// Open an in-memory DB for testing.
    fn mem_db() -> MiyucloudDb {
        MiyucloudDb::open(":memory:").expect("in-memory DB should open")
    }

    /// Helper: create a session and return (db, token, owner_id).
    fn create_test_session() -> (MiyucloudDb, String, String) {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();
        let token = create_session(&db, &owner, Some("iphash"), Some("TestAgent/1.0"))
            .expect("create_session should succeed");
        (db, token, owner)
    }

    #[test]
    fn test_create_session_returns_hex_token() {
        let (_db, token, _owner) = create_test_session();
        assert_eq!(token.len(), 64, "Token should be 64 hex chars");
        assert!(
            token.chars().all(|c| c.is_ascii_hexdigit()),
            "Token should only contain hex chars"
        );
    }

    #[test]
    fn test_create_session_evicts_oldest_at_max() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();

        // Create MAX_SESSIONS_PER_OWNER sessions.
        let mut tokens = Vec::new();
        for _ in 0..MAX_SESSIONS_PER_OWNER {
            let t = create_session(&db, &owner, None, None)
                .expect("create_session should succeed");
            tokens.push(t);
        }

        // All 5 should be active.
        let active = list_sessions(&db, &owner).expect("list should work");
        assert_eq!(active.len(), MAX_SESSIONS_PER_OWNER as usize);

        // Create a 6th session — should evict the oldest.
        let t6 = create_session(&db, &owner, None, None)
            .expect("6th create_session should succeed");
        tokens.push(t6);

        // The first token should now be revoked.
        let result = validate_session(&db, &tokens[0]);
        assert!(result.is_err(), "Oldest session should have been revoked");

        // The newest should still be valid.
        let result = validate_session(&db, &tokens[5]);
        assert!(result.is_ok(), "Newest session should be valid");
    }

    #[test]
    fn test_validate_session_succeeds() {
        let (db, token, owner) = create_test_session();
        let info = validate_session(&db, &token).expect("validate should succeed");
        assert_eq!(info.owner_id, owner);
        assert!(!info.totp_verified);
        assert_eq!(info.created_ip_hash.as_deref(), Some("iphash"));
        assert_eq!(info.created_ua.as_deref(), Some("TestAgent/1.0"));
    }

    #[test]
    fn test_validate_session_unknown_token() {
        let db = mem_db();
        let result = validate_session(&db, "nonexistent_token");
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("not found"),
            "Error should mention not found: {err_msg}"
        );
    }

    #[test]
    fn test_validate_session_revoked() {
        let (db, token, _owner) = create_test_session();

        // Find the session ID to revoke it.
        let row = db
            .session_by_token(&token)
            .expect("lookup should work")
            .expect("session should exist");
        revoke_session(&db, &row.id).expect("revoke should succeed");

        let result = validate_session(&db, &token);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("revoked"),
            "Error should mention revoked: {err_msg}"
        );
    }

    #[test]
    fn test_validate_session_expired() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();
        let session_id = uuid::Uuid::new_v4().to_string();
        let token = generate_session_token();

        // Insert a session that expired 1 hour ago.
        let expired = (Utc::now() - Duration::hours(1)).to_rfc3339();
        db.session_insert(&session_id, &owner, &token, None, None, &expired)
            .expect("insert should succeed");

        let result = validate_session(&db, &token);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("expired"),
            "Error should mention expired: {err_msg}"
        );
    }

    #[test]
    fn test_validate_session_inactive() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();
        let session_id = uuid::Uuid::new_v4().to_string();
        let token = generate_session_token();

        // Insert a session with valid expiry but created 3 hours ago.
        // The DB sets last_activity_at = created_at = now_iso() internally,
        // so we need to manually update last_activity_at to simulate inactivity.
        let far_future = (Utc::now() + Duration::hours(48)).to_rfc3339();
        db.session_insert(&session_id, &owner, &token, None, None, &far_future)
            .expect("insert should succeed");

        // Manually set last_activity_at to 3 hours ago via a direct update.
        // We use session_by_token to confirm the session exists, then use
        // a raw approach: revoke + re-insert is not clean. Instead, we accept
        // that the DB sets last_activity_at to now, so we test the timeout
        // boundary by checking that a session just created passes validation.
        // For a true inactivity test, we insert via raw SQL.
        {
            let three_hours_ago = (Utc::now() - Duration::hours(3)).to_rfc3339();
            // Access the underlying connection to force-set last_activity_at.
            // Since MiyucloudDb wraps a Mutex<Connection>, we use the lock.
            let conn = db.lock_conn().expect("lock should succeed");
            conn.execute(
                "UPDATE cloud_sessions SET last_activity_at = ?1 WHERE id = ?2",
                rusqlite::params![three_hours_ago, session_id],
            )
            .expect("raw update should succeed");
        }

        let result = validate_session(&db, &token);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("inactive"),
            "Error should mention inactive: {err_msg}"
        );
    }

    #[test]
    fn test_revoke_session_works() {
        let (db, token, _owner) = create_test_session();
        let row = db
            .session_by_token(&token)
            .expect("lookup should work")
            .expect("session should exist");

        revoke_session(&db, &row.id).expect("revoke should succeed");

        let row_after = db
            .session_by_token(&token)
            .expect("lookup should work")
            .expect("session should still exist in DB");
        assert!(row_after.is_revoked, "Session should be revoked");
    }

    #[test]
    fn test_revoke_all_sessions_returns_count() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();

        for _ in 0..3 {
            create_session(&db, &owner, None, None).expect("create should succeed");
        }

        let count = revoke_all_sessions(&db, &owner).expect("revoke_all should succeed");
        assert_eq!(count, 3, "Should have revoked 3 sessions");

        let active = list_sessions(&db, &owner).expect("list should work");
        assert!(active.is_empty(), "No sessions should be active after revoke_all");
    }

    #[test]
    fn test_list_sessions_excludes_revoked() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();

        let t1 = create_session(&db, &owner, None, None).expect("create should succeed");
        let _t2 = create_session(&db, &owner, None, None).expect("create should succeed");

        // Revoke the first session.
        let row = db.session_by_token(&t1).expect("ok").expect("exists");
        revoke_session(&db, &row.id).expect("revoke ok");

        let active = list_sessions(&db, &owner).expect("list should work");
        assert_eq!(active.len(), 1, "Only 1 active session after revoking 1 of 2");
        assert_ne!(active[0].id, row.id, "Active session should not be the revoked one");
    }

    #[test]
    fn test_mark_totp_verified() {
        let (db, token, _owner) = create_test_session();
        let row = db.session_by_token(&token).expect("ok").expect("exists");

        assert!(!row.totp_verified, "Newly created session should not be totp verified");

        mark_totp_verified(&db, &row.id).expect("mark_totp should succeed");

        let row_after = db.session_by_token(&token).expect("ok").expect("exists");
        assert!(row_after.totp_verified, "Session should be totp verified after marking");
    }

    #[test]
    fn test_purge_expired_removes_old_sessions() {
        let db = mem_db();
        let owner = uuid::Uuid::new_v4().to_string();

        // Insert 2 already-expired sessions.
        for _ in 0..2 {
            let sid = uuid::Uuid::new_v4().to_string();
            let tok = generate_session_token();
            let expired = (Utc::now() - Duration::hours(1)).to_rfc3339();
            db.session_insert(&sid, &owner, &tok, None, None, &expired)
                .expect("insert should succeed");
        }

        // Insert 1 valid session.
        let _valid_token = create_session(&db, &owner, None, None).expect("create ok");

        let purged = purge_expired_sessions(&db).expect("purge should succeed");
        assert_eq!(purged, 2, "Should have purged 2 expired sessions");

        let active = list_sessions(&db, &owner).expect("list should work");
        assert_eq!(active.len(), 1, "Only the valid session should remain");
    }

    #[test]
    fn test_generate_session_token_uniqueness() {
        let t1 = generate_session_token();
        let t2 = generate_session_token();
        assert_ne!(t1, t2, "Two generated tokens should be different");
    }

    #[test]
    fn test_validate_session_updates_activity() {
        let (db, token, _owner) = create_test_session();

        let info1 = validate_session(&db, &token).expect("first validate ok");
        let info2 = validate_session(&db, &token).expect("second validate ok");

        // The second validation should have a last_activity_at >= the first.
        let dt1 = parse_rfc3339(&info1.last_activity_at).expect("parse ok");
        let dt2 = parse_rfc3339(&info2.last_activity_at).expect("parse ok");
        assert!(
            dt2 >= dt1,
            "Second validation should have equal or later activity timestamp"
        );
    }
}
