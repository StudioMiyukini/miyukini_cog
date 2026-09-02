//! Service de gestion des migrations de schéma (DB Operations Contract §9.3).
//!
//! Exécution via KindMother sous validation StrongFather ; historique et format
//! des scripts conformes à la documentation.

use serde::{Deserialize, Serialize};

/// @id miyukiniadmin_migration_service_migration_record
/// @role data
/// @layer operator
/// @do represent_migration_history_entry
//@human / @human Enregistrement d'une migration appliquée (table d'historique).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationRecord {
    /// @id miyukiniadmin_migration_record_id
    /// @role data
    /// @layer operator
    /// @do store_migration_identity
    //@human / @human Identifiant unique (nom fichier ou hash).
    /// @depends miyukiniadmin_migration_service_migration_record
    pub migration_id: String,
    /// @id miyukiniadmin_migration_record_applied_at
    /// @role data
    /// @layer operator
    /// @do store_applied_timestamp
    //@human / @human Horodatage d'application.
    /// @depends miyukiniadmin_migration_service_migration_record
    pub applied_at: String,
    /// @id miyukiniadmin_migration_record_checksum
    /// @role data
    /// @layer operator
    /// @do store_checksum
    //@human / @human Hash du contenu du script (intégrité).
    /// @depends miyukiniadmin_migration_service_migration_record
    pub checksum: Option<String>,
    /// @id miyukiniadmin_migration_record_duration_ms
    /// @role data
    /// @layer operator
    /// @do store_duration
    //@human / @human Durée d'exécution en millisecondes.
    /// @depends miyukiniadmin_migration_service_migration_record
    pub duration_ms: Option<u64>,
    /// @id miyukiniadmin_migration_record_result
    /// @role data
    /// @layer operator
    /// @do store_result
    //@human / @human Résultat (SUCCESS / FAIL).
    /// @depends miyukiniadmin_migration_service_migration_record
    pub result: String,
    /// @id miyukiniadmin_migration_record_operator_id
    /// @role data
    /// @layer operator
    /// @do store_operator_id
    //@human / @human Opérateur humain ayant déclenché (MiyukiniAdmin).
    /// @depends miyukiniadmin_migration_service_migration_record
    pub operator_id: Option<String>,
}

/// @id miyukiniadmin_migration_service_pending_migration
/// @role data
/// @layer operator
/// @do represent_pending_migration
//@human / @human Migration en attente d'application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingMigration {
    /// @id miyukiniadmin_pending_migration_id
    /// @role data
    /// @layer operator
    /// @do store_migration_identity
    //@human / @human Identifiant (nom fichier ou version).
    /// @depends miyukiniadmin_migration_service_pending_migration
    pub migration_id: String,
    /// @id miyukiniadmin_pending_migration_description
    /// @role data
    /// @layer operator
    /// @do store_description
    //@human / @human Description courte.
    /// @depends miyukiniadmin_migration_service_pending_migration
    pub description: Option<String>,
}

/// @id miyukiniadmin_migration_service_trait
/// @role infrastructure
/// @layer operator
/// @do define_migration_service_contract
//@human / @human Contrat du service de migrations (KindMother / StrongFather en production).
pub trait MigrationService {
    /// @id miyukiniadmin_migration_service_list_applied
    /// @role accessor
    /// @layer operator
    /// @do list_applied_migrations
    //@human / @human Liste des migrations déjà appliquées.
    /// @depends miyukiniadmin_migration_service_trait
    fn list_applied(&self) -> Vec<MigrationRecord>;

    /// @id miyukiniadmin_migration_service_list_pending
    /// @role accessor
    /// @layer operator
    /// @do list_pending_migrations
    //@human / @human Liste des migrations en attente.
    /// @depends miyukiniadmin_migration_service_trait
    fn list_pending(&self) -> Vec<PendingMigration>;

    /// @id miyukiniadmin_migration_service_history
    /// @role accessor
    /// @layer operator
    /// @do get_migration_history
    //@human / @human Historique complet (appliquées, ordre chronologique).
    /// @depends miyukiniadmin_migration_service_trait
    fn history(&self) -> Vec<MigrationRecord>;

    /// @id miyukiniadmin_migration_service_apply
    /// @role mutator
    /// @layer operator
    /// @do apply_pending_migrations
    //@human / @human Applique les migrations en attente (validation StrongFather + KindMother en production).
    /// @depends miyukiniadmin_migration_service_trait
    fn apply_pending(&self) -> ApplyMigrationsResult;
}

/// @id miyukiniadmin_migration_service_apply_result
/// @role data
/// @layer operator
/// @do represent_apply_result
//@human / @human Résultat de l'application des migrations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyMigrationsResult {
    /// @id miyukiniadmin_apply_result_status
    /// @role data
    /// @layer operator
    /// @do store_status
    //@human / @human SUCCESS / FAIL / PARTIAL.
    /// @depends miyukiniadmin_migration_service_apply_result
    pub status: String,
    /// @id miyukiniadmin_apply_result_applied_count
    /// @role data
    /// @layer operator
    /// @do store_applied_count
    //@human / @human Nombre de migrations appliquées.
    /// @depends miyukiniadmin_migration_service_apply_result
    pub applied_count: u32,
    /// @id miyukiniadmin_apply_result_message
    /// @role data
    /// @layer operator
    /// @do store_message
    //@human / @human Message ou erreur.
    /// @depends miyukiniadmin_migration_service_apply_result
    pub message: Option<String>,
}

/// @id miyukiniadmin_migration_service_impl
/// @role infrastructure
/// @layer operator
/// @do implement_migration_service_stub
//@human / @human Implémentation stub du service migrations (historique en mémoire pour démo).
/// @depends miyukiniadmin_migration_service_trait
#[derive(Debug, Default)]
pub struct MigrationServiceImpl {
    /// @id miyukiniadmin_migration_impl_applied
    /// @role data
    /// @layer operator
    /// @do store_applied_migrations
    //@human / @human Migrations appliquées (stub).
    /// @depends miyukiniadmin_migration_service_impl
    applied: Vec<MigrationRecord>,
    /// @id miyukiniadmin_migration_impl_pending
    /// @role data
    /// @layer operator
    /// @do store_pending_migrations
    //@human / @human Migrations en attente (stub).
    /// @depends miyukiniadmin_migration_service_impl
    pending: Vec<PendingMigration>,
}

impl MigrationServiceImpl {
    /// @id miyukiniadmin_migration_impl_new
    /// @role infrastructure
    /// @layer operator
    /// @do create_migration_service
    //@human / @human Crée le service avec état stub (aucune migration par défaut).
    /// @depends miyukiniadmin_migration_service_impl
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// @id miyukiniadmin_migration_impl_with_seed
    /// @role mutator
    /// @layer operator
    /// @do seed_demo_migrations
    //@human / @human Initialise avec des enregistrements de démo.
    /// @depends miyukiniadmin_migration_service_impl
    #[must_use]
    pub fn with_seed(mut self) -> Self {
        self.applied = vec![MigrationRecord {
            migration_id: "20260101000000_init_schema".to_string(),
            applied_at: "2026-01-01T00:00:00Z".to_string(),
            checksum: Some("abc123".to_string()),
            duration_ms: Some(150),
            result: "SUCCESS".to_string(),
            operator_id: Some("miyukini-admin".to_string()),
        }];
        self.pending = vec![
            PendingMigration {
                migration_id: "20260129000001_add_users_roles".to_string(),
                description: Some("Add users.roles column".to_string()),
            },
            PendingMigration {
                migration_id: "20260129000002_add_audit_table".to_string(),
                description: Some("Create audit_log table".to_string()),
            },
        ];
        self
    }
}

impl MigrationService for MigrationServiceImpl {
    fn list_applied(&self) -> Vec<MigrationRecord> {
        self.applied.clone()
    }

    fn list_pending(&self) -> Vec<PendingMigration> {
        self.pending.clone()
    }

    fn history(&self) -> Vec<MigrationRecord> {
        self.applied.clone()
    }

    fn apply_pending(&self) -> ApplyMigrationsResult {
        // Stub : en production, passage par BondingBrother → StrongFather → KindMother
        ApplyMigrationsResult {
            status: "SUCCESS".to_string(),
            applied_count: self.pending.len() as u32,
            message: Some("Stub: apply non exécuté (voir KindMother en production).".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id miyukiniadmin_migration_test_list
    /// @role test
    /// @layer operator
    /// @do verify_migration_list
    //@human / @human Test liste appliquées et en attente.
    /// @depends miyukiniadmin_migration_service_impl_with_seed
    #[test]
    fn test_migration_list() {
        let svc = MigrationServiceImpl::new().with_seed();
        assert_eq!(svc.list_applied().len(), 1);
        assert_eq!(svc.list_pending().len(), 2);
    }

    /// @id miyukiniadmin_migration_test_apply_result
    /// @role test
    /// @layer operator
    /// @do verify_apply_result_stub
    //@human / @human Test résultat apply (stub).
    /// @depends miyukiniadmin_migration_service_apply
    #[test]
    fn test_apply_result() {
        let svc = MigrationServiceImpl::new().with_seed();
        let r = svc.apply_pending();
        assert_eq!(r.status, "SUCCESS");
        assert_eq!(r.applied_count, 2);
    }
}
