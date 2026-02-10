//! Gouvernance locale JayXpose pour operations non-SQL.
//!
//! Règle: toute création non-SQL (brouillon en mémoire, template runtime, etc.)
//! doit être validée par mandat puis tracée en audit.

use crate::data::{JayXposeDb, SyncLog};
use serde_json::Value;

/// Erreurs de gouvernance JayXpose.
#[derive(Debug)]
pub enum GovernanceError {
    /// Opération refusée car mandat absent.
    NoMandate,
}

impl std::fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoMandate => write!(f, "Execution refused: no governed mandate"),
        }
    }
}

impl std::error::Error for GovernanceError {}

/// Contexte de gouvernance pour opérations métier.
#[derive(Debug, Clone, Default)]
pub struct GovernanceContext {
    /// Identifiant de mandat actif (StrongFather).
    pub mandate_id: String,
    /// Niveau de sécurité opération.
    pub security_level: u8,
}

impl GovernanceContext {
    /// Retourne true si un mandat est présent.
    pub fn has_mandate(&self) -> bool {
        !self.mandate_id.is_empty()
    }
}

/// Valide et trace une mutation non-SQL.
pub fn govern_non_sql_create(
    ctx: &GovernanceContext,
    db: &JayXposeDb,
    exposant_id: Option<&str>,
    action: &str,
    payload: Value,
) -> Result<(), GovernanceError> {
    if !ctx.has_mandate() {
        return Err(GovernanceError::NoMandate);
    }

    let audit_payload = serde_json::json!({
        "kind": "non_sql_create",
        "action": action,
        "mandate_id": ctx.mandate_id,
        "security_level": ctx.security_level,
        "payload": payload,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });

    let _ = db.sync_log_insert(&SyncLog {
        id: None,
        exposant_id: exposant_id.map(str::to_string),
        sync_source: Some("strongfather".to_string()),
        sync_type: Some("governed_non_sql_create".to_string()),
        status: Some("ok".to_string()),
        payload_json: Some(audit_payload.to_string()),
        error_message: None,
        created_at: None,
    });
    Ok(())
}

