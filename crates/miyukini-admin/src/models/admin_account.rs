//! Modèle compte admin (Authentication Contract).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// @id: miyukiniadmin_admin_account
/// @role: data
/// @layer: operator
/// @human: Compte administrateur MiyukiniAdmin (registre admin).
/// @do: represent_admin_account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccount {
    /// @id: miyukiniadmin_admin_account_id
    /// @role: data
    /// @layer: operator
    /// @human: Identifiant unique du compte.
    /// @do: store_account_identity
    pub account_id: String,

    /// @id: miyukiniadmin_admin_account_username
    /// @role: data
    /// @layer: operator
    /// @human: Nom d'utilisateur (unique).
    /// @do: store_username
    pub username: String,

    /// @id: miyukiniadmin_admin_account_password_hash
    /// @role: data
    /// @layer: operator
    /// @human: Hash mot de passe (Argon2id).
    /// @do: store_password_hash
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// @id: miyukiniadmin_admin_account_role
    /// @role: data
    /// @layer: operator
    /// @human: Rôle RBAC (Admin, Recovery, Audit).
    /// @do: store_role
    pub role: AdminRole,

    /// @id: miyukiniadmin_admin_account_mfa_enabled
    /// @role: data
    /// @layer: operator
    /// @human: MFA activée (TOTP ou WebAuthn).
    /// @do: store_mfa_flag
    pub mfa_enabled: bool,

    /// @id: miyukiniadmin_admin_account_mfa_secret_encrypted
    /// @role: data
    /// @layer: operator
    /// @human: Secret MFA chiffré (AES-256-GCM) si TOTP.
    /// @do: store_mfa_secret
    #[serde(skip_serializing)]
    pub mfa_secret_encrypted: Option<String>,

    /// @id: miyukiniadmin_admin_account_created_at
    /// @role: data
    /// @layer: operator
    /// @human: Date de création du compte.
    /// @do: store_created_at
    pub created_at: DateTime<Utc>,

    /// @id: miyukiniadmin_admin_account_updated_at
    /// @role: data
    /// @layer: operator
    /// @human: Dernière mise à jour.
    /// @do: store_updated_at
    pub updated_at: DateTime<Utc>,

    /// @id: miyukiniadmin_admin_account_last_login_at
    /// @role: data
    /// @layer: operator
    /// @human: Dernière connexion réussie.
    /// @do: store_last_login
    pub last_login_at: Option<DateTime<Utc>>,

    /// @id: miyukiniadmin_admin_account_locked_until
    /// @role: data
    /// @layer: operator
    /// @human: Verrouillage jusqu'à (rate limit).
    /// @do: store_locked_until
    pub locked_until: Option<DateTime<Utc>>,

    /// @id: miyukiniadmin_admin_account_failed_attempts
    /// @role: data
    /// @layer: operator
    /// @human: Nombre d'échecs de connexion (fenêtre).
    /// @do: store_failed_attempts
    pub failed_attempts: u32,
}

/// @id: miyukiniadmin_admin_role_enum
/// @role: data
/// @layer: operator
/// @human: Rôle RBAC MiyukiniAdmin (Permission Contract §3).
/// @do: represent_admin_role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum AdminRole {
    /// Administrateur standard.
    /// @id: miyukiniadmin_role_admin
    Admin,

    /// Administrateur recovery (accès DB recovery, conditions cumulatives).
    /// @id: miyukiniadmin_role_recovery
    Recovery,

    /// Auditeur (lecture seule).
    /// @id: miyukiniadmin_role_audit
    Audit,
}

impl Default for AdminRole {
    fn default() -> Self {
        Self::Admin
    }
}

impl std::fmt::Display for AdminRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Admin => write!(f, "Admin"),
            Self::Recovery => write!(f, "Recovery"),
            Self::Audit => write!(f, "Audit"),
        }
    }
}
