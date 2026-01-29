//! Service RBAC MiyukiniAdmin (Permission Contract §5).
//!
//! Référence : MiyukiniAdmin - Implementation Security and Controls §12

use std::collections::HashSet;

use crate::models::AdminRole;

/// @id: miyukiniadmin_permission_service
/// @role: infrastructure
/// @layer: operator
/// @human: Vérification des capacités par rôle (RBAC).
/// @do: check_capability_for_role
/// @depends: miyukiniadmin_models_admin_role
#[derive(Clone, Default)]
pub struct PermissionService;

/// Capacités Admin (Permission Contract §5.1) — sans admin.db.recovery ni création Recovery.
/// @id: miyukiniadmin_capabilities_admin
const CAPABILITIES_ADMIN: &[&str] = &[
    "admin.dashboard.read",
    "admin.navigation.read",
    "admin.metrics.read",
    "admin.metrics.db.read",
    "admin.logs.read",
    "admin.security.level.read",
    "admin.security.level.write",
    "admin.security.degradation.activate",
    "admin.security.trust.read",
    "admin.db.read",
    "admin.db.write",
    "admin.db.migrate",
    "admin.db.repair",
    "admin.db.backup.read",
    "admin.db.backup.trigger",
    "admin.db.restore",
    "admin.operators.list",
    "admin.operators.isolate",
    "admin.tests.coherence",
    "admin.tests.load",
    "admin.tests.flux.read",
    "admin.accounts.read",
    "admin.accounts.create",
    "admin.accounts.write",
    "admin.accounts.reset_password",
    "admin.accounts.lock",
    "admin.accounts.unlock",
    "admin.accounts.revoke",
    "admin.accounts.revoke_session",
    "admin.config.read",
    "admin.config.write",
];

/// Capacités supplémentaires Recovery (Permission Contract §5.2).
/// @id: miyukiniadmin_capabilities_recovery_extra
const CAPABILITIES_RECOVERY_EXTRA: &[&str] = &["admin.db.recovery"];

/// Capacités Audit (Permission Contract §5.3) — lecture seule.
/// @id: miyukiniadmin_capabilities_audit
const CAPABILITIES_AUDIT: &[&str] = &[
    "admin.dashboard.read",
    "admin.navigation.read",
    "admin.metrics.read",
    "admin.metrics.db.read",
    "admin.logs.read",
    "admin.security.level.read",
    "admin.security.trust.read",
    "admin.operators.list",
    "admin.tests.flux.read",
    "admin.accounts.read",
    "admin.config.read",
];

impl PermissionService {
    /// Construit le service.
    /// @id: miyukiniadmin_permission_service_new
    /// @role: constructor
    /// @layer: operator
    /// @human: Crée le service RBAC.
    /// @do: create_permission_service
    pub fn new() -> Self {
        Self
    }

    /// Retourne true si le rôle possède la capacité (Permission Contract §5).
    /// @id: miyukiniadmin_permission_has_capability
    /// @role: accessor
    /// @layer: operator
    /// @human: Vérifie si le rôle a la capacité demandée.
    /// @do: has_capability
    /// @depends: miyukiniadmin_permission_service
    pub fn has_capability(&self, role: AdminRole, capability: &str) -> bool {
        let set = self.capabilities_for_role(role);
        set.contains(capability)
    }

    /// Ensemble des capacités pour un rôle.
    /// @id: miyukiniadmin_permission_capabilities_for_role
    fn capabilities_for_role(&self, role: AdminRole) -> HashSet<&'static str> {
        match role {
            AdminRole::Admin => CAPABILITIES_ADMIN.iter().copied().collect(),
            AdminRole::Recovery => CAPABILITIES_ADMIN
                .iter()
                .copied()
                .chain(CAPABILITIES_RECOVERY_EXTRA.iter().copied())
                .collect(),
            AdminRole::Audit => CAPABILITIES_AUDIT.iter().copied().collect(),
        }
    }
}
