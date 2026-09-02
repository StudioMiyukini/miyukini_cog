//! Modèle compte admin (Authentication Contract).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §11.1

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// @id miyukiniadmin_admin_account
/// @role data
/// @layer operator
/// @do represent_admin_account
//@human / @human Compte administrateur MiyukiniAdmin (registre admin).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminAccount {
    /// @id miyukiniadmin_admin_account_id
    /// @role data
    /// @layer operator
    /// @do store_account_identity
    //@human / @human Identifiant unique du compte.
    pub account_id: String,

    /// @id miyukiniadmin_admin_account_username
    /// @role data
    /// @layer operator
    /// @do store_username
    //@human / @human Nom d'utilisateur (unique).
    pub username: String,

    /// @id miyukiniadmin_admin_account_password_hash
    /// @role data
    /// @layer operator
    /// @do store_password_hash
    //@human / @human Hash mot de passe (Argon2id).
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// @id miyukiniadmin_admin_account_role
    /// @role data
    /// @layer operator
    /// @do store_role
    //@human / @human Rôle RBAC (Admin, Recovery, Audit).
    pub role: AdminRole,

    /// @id miyukiniadmin_admin_account_mfa_enabled
    /// @role data
    /// @layer operator
    /// @do store_mfa_flag
    //@human / @human MFA activée (TOTP ou WebAuthn).
    pub mfa_enabled: bool,

    /// @id miyukiniadmin_admin_account_mfa_secret_encrypted
    /// @role data
    /// @layer operator
    /// @do store_mfa_secret
    //@human / @human Secret MFA chiffré (AES-256-GCM) si TOTP.
    #[serde(skip_serializing)]
    pub mfa_secret_encrypted: Option<String>,

    /// @id miyukiniadmin_admin_account_created_at
    /// @role data
    /// @layer operator
    /// @do store_created_at
    //@human / @human Date de création du compte.
    pub created_at: DateTime<Utc>,

    /// @id miyukiniadmin_admin_account_updated_at
    /// @role data
    /// @layer operator
    /// @do store_updated_at
    //@human / @human Dernière mise à jour.
    pub updated_at: DateTime<Utc>,

    /// @id miyukiniadmin_admin_account_last_login_at
    /// @role data
    /// @layer operator
    /// @do store_last_login
    //@human / @human Dernière connexion réussie.
    pub last_login_at: Option<DateTime<Utc>>,

    /// @id miyukiniadmin_admin_account_locked_until
    /// @role data
    /// @layer operator
    /// @do store_locked_until
    //@human / @human Verrouillage jusqu'à (rate limit).
    pub locked_until: Option<DateTime<Utc>>,

    /// @id miyukiniadmin_admin_account_failed_attempts
    /// @role data
    /// @layer operator
    /// @do store_failed_attempts
    //@human / @human Nombre d'échecs de connexion (fenêtre).
    pub failed_attempts: u32,
}

/// @id miyukiniadmin_admin_role_enum
/// @role data
/// @layer operator
/// @do represent_admin_role
//@human / @human Rôle RBAC MiyukiniAdmin (Permission Contract §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[derive(Default)]
pub enum AdminRole {
    /// Administrateur standard.
    /// @id miyukiniadmin_role_admin
    /// @do documenter_admin
    #[default]
    Admin,

    /// Administrateur recovery (accès DB recovery, conditions cumulatives).
    /// @id miyukiniadmin_role_recovery
    /// @do documenter_recovery
    Recovery,

    /// Auditeur (lecture seule).
    /// @id miyukiniadmin_role_audit
    /// @do documenter_audit
    Audit,
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
