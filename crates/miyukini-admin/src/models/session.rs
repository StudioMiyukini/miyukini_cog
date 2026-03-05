//! Modèle session admin (Authentication Contract §session).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11.4

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::AdminRole;

/// @id: miyukiniadmin_admin_session
/// @role: data
/// @layer: operator
/// @human: Session administrateur (après login réussi + MFA si requis).
/// @do: represent_admin_session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminSession {
    /// @id: miyukiniadmin_session_id
    /// @role: data
    /// @layer: operator
    /// @human: Identifiant unique de la session.
    /// @do: store_session_identity
    pub session_id: String,

    /// @id: miyukiniadmin_session_account_id
    /// @role: data
    /// @layer: operator
    /// @human: Compte associé.
    /// @do: store_account_ref
    pub account_id: String,

    /// @id: miyukiniadmin_session_role
    /// @role: data
    /// @layer: operator
    /// @human: Rôle au moment de la création (snapshot).
    /// @do: store_role_snapshot
    pub role: AdminRole,

    /// @id: miyukiniadmin_session_expires_at
    /// @role: data
    /// @layer: operator
    /// @human: Expiration (idle + absolu).
    /// @do: store_expiry
    pub expires_at: DateTime<Utc>,

    /// @id: miyukiniadmin_session_created_at
    /// @role: data
    /// @layer: operator
    /// @human: Création de la session.
    /// @do: store_created_at
    pub created_at: DateTime<Utc>,

    /// @id: miyukiniadmin_session_ip
    /// @role: data
    /// @layer: operator
    /// @human: IP d'origine (optionnel, pour audit).
    /// @do: store_ip
    pub ip: Option<String>,

    /// @id: miyukiniadmin_session_user_agent
    /// @role: data
    /// @layer: operator
    /// @human: User-Agent (optionnel, pour audit).
    /// @do: store_user_agent
    pub user_agent: Option<String>,
}

impl AdminSession {
    /// Vérifie si la session est encore valide (non expirée).
    /// @id: miyukiniadmin_session_is_valid
    /// @role: accessor
    /// @layer: operator
    /// @human: Indique si la session est valide.
    /// @do: check_session_valid
    #[must_use]
    pub fn is_valid(&self, now: DateTime<Utc>) -> bool {
        now < self.expires_at
    }
}
