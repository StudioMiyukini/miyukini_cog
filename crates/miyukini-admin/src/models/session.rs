//! Modèle session admin (Authentication Contract §session).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11.4

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::AdminRole;

/// @id miyukiniadmin_admin_session
/// @role data
/// @layer operator
/// @do represent_admin_session
//@human / @human Session administrateur (après login réussi + MFA si requis).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSession {
    /// @id miyukiniadmin_session_id
    /// @role data
    /// @layer operator
    /// @do store_session_identity
    //@human / @human Identifiant unique de la session.
    pub session_id: String,

    /// @id miyukiniadmin_session_account_id
    /// @role data
    /// @layer operator
    /// @do store_account_ref
    //@human / @human Compte associé.
    pub account_id: String,

    /// @id miyukiniadmin_session_role
    /// @role data
    /// @layer operator
    /// @do store_role_snapshot
    //@human / @human Rôle au moment de la création (snapshot).
    pub role: AdminRole,

    /// @id miyukiniadmin_session_expires_at
    /// @role data
    /// @layer operator
    /// @do store_expiry
    //@human / @human Expiration (idle + absolu).
    pub expires_at: DateTime<Utc>,

    /// @id miyukiniadmin_session_created_at
    /// @role data
    /// @layer operator
    /// @do store_created_at
    //@human / @human Création de la session.
    pub created_at: DateTime<Utc>,

    /// @id miyukiniadmin_session_ip
    /// @role data
    /// @layer operator
    /// @do store_ip
    //@human / @human IP d'origine (optionnel, pour audit).
    pub ip: Option<String>,

    /// @id miyukiniadmin_session_user_agent
    /// @role data
    /// @layer operator
    /// @do store_user_agent
    //@human / @human User-Agent (optionnel, pour audit).
    pub user_agent: Option<String>,
}

impl AdminSession {
    /// Vérifie si la session est encore valide (non expirée).
    /// @id miyukiniadmin_session_is_valid
    /// @role accessor
    /// @layer operator
    /// @do check_session_valid
    //@human / @human Indique si la session est valide.
    #[must_use]
    pub fn is_valid(&self, now: DateTime<Utc>) -> bool {
        now < self.expires_at
    }
}
