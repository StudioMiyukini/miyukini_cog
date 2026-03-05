//! Verification des permissions d'acces aux fichiers et dossiers.
//!
//! @id: miyucloud_auth_permissions
//! @do: verify_access_permissions_for_resources
//! @role: auth
//! @layer: domain
//!
//! Le proprietaire a toujours tous les droits. Les beneficiaires de partage
//! sont verifies selon leur niveau de permission (read, read_write).

use crate::data::types::{AccessAction, PermissionLevel, SharePermission};
use crate::errors::MiyucloudError;

/// Verifie si un utilisateur a le droit d'effectuer une action sur une ressource.
///
/// - Le proprietaire (`owner_id`) a toujours tous les droits.
/// - Pour les autres, les permissions sont verifiees dans la liste `permissions`.
/// - Les actions `Download` et `Preview` requierent au minimum `Read`.
/// - Les actions d'ecriture (`Upload`, suppression, etc.) requierent `ReadWrite`.
pub fn verify_access(
    owner_id: &str,
    requester_id: &str,
    permissions: &[SharePermission],
    action: AccessAction,
) -> Result<(), MiyucloudError> {
    // Le proprietaire a toujours tous les droits
    if owner_id == requester_id {
        return Ok(());
    }

    // Chercher une permission applicable
    let required_level = required_permission_level(action);

    for perm in permissions {
        if perm.grantee_id == requester_id && permission_satisfies(perm.permission, required_level)
        {
            return Ok(());
        }
    }

    Err(MiyucloudError::PermissionDenied(format!(
        "User {requester_id} does not have {required_level:?} permission for action {action:?}"
    )))
}

/// Determine le niveau de permission requis pour une action.
fn required_permission_level(action: AccessAction) -> PermissionLevel {
    match action {
        AccessAction::Download | AccessAction::Preview | AccessAction::AuthAttempt => {
            PermissionLevel::Read
        }
        AccessAction::AuthFail => PermissionLevel::Read,
    }
}

/// Verifie si un niveau de permission satisfait une exigence.
///
/// `ReadWrite` satisfait `Read` (hierarchique).
fn permission_satisfies(granted: PermissionLevel, required: PermissionLevel) -> bool {
    match granted {
        PermissionLevel::ReadWrite => true,
        PermissionLevel::Read => required == PermissionLevel::Read,
    }
}

/// Actions d'ecriture definies dans le domaine MiyuCloud.
/// Utilisee par la couche domaine pour verifier les permissions d'ecriture.
pub fn verify_write_access(
    owner_id: &str,
    requester_id: &str,
    permissions: &[SharePermission],
) -> Result<(), MiyucloudError> {
    if owner_id == requester_id {
        return Ok(());
    }

    for perm in permissions {
        if perm.grantee_id == requester_id && perm.permission == PermissionLevel::ReadWrite {
            return Ok(());
        }
    }

    Err(MiyucloudError::PermissionDenied(format!(
        "User {requester_id} does not have write permission"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::GranteeType;

    fn make_permission(grantee_id: &str, level: PermissionLevel) -> SharePermission {
        SharePermission {
            id: uuid::Uuid::new_v4().to_string(),
            file_id: Some("file-1".to_string()),
            folder_id: None,
            grantee_id: grantee_id.to_string(),
            grantee_type: GranteeType::Profile,
            permission: level,
            created_at: "2026-03-01T00:00:00Z".to_string(),
        }
    }

    #[test]
    fn test_owner_has_full_access() {
        let result = verify_access("owner-1", "owner-1", &[], AccessAction::Download);
        assert!(result.is_ok());
    }

    #[test]
    fn test_shared_read_can_download() {
        let perms = vec![make_permission("user-2", PermissionLevel::Read)];
        let result = verify_access("owner-1", "user-2", &perms, AccessAction::Download);
        assert!(result.is_ok());
    }

    #[test]
    fn test_shared_read_cannot_write() {
        let perms = vec![make_permission("user-2", PermissionLevel::Read)];
        let result = verify_write_access("owner-1", "user-2", &perms);
        assert!(result.is_err());
    }

    #[test]
    fn test_shared_readwrite_can_write() {
        let perms = vec![make_permission("user-3", PermissionLevel::ReadWrite)];
        let result = verify_write_access("owner-1", "user-3", &perms);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_permission_denied() {
        let result = verify_access("owner-1", "stranger", &[], AccessAction::Download);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Permission denied"));
    }

    #[test]
    fn test_readwrite_satisfies_read() {
        let perms = vec![make_permission("user-4", PermissionLevel::ReadWrite)];
        let result = verify_access("owner-1", "user-4", &perms, AccessAction::Download);
        assert!(result.is_ok());
    }
}
