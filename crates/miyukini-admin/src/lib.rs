//! # MiyukiniAdmin
//!
//! Console souveraine d'administration du Miyukini Core System.
//!
//! MiyukiniAdmin est un Opérateur Souverain autonome qui observe, installe, arbitre,
//! mais ne vit pas dans le flux normal. Il est out-of-band, comme un BIOS/hyperviseur.

/// @id: miyukiniadmin_module_config
/// @role: infrastructure
/// @layer: operator
/// @human: Module de configuration backend.
/// @do: expose_config_module
pub mod config;

/// @id: miyukiniadmin_module_monitoring_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de monitoring et collecte de métriques.
/// @do: expose_monitoring_service
pub mod monitoring_service;

/// @id: miyukiniadmin_module_database_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de gestion de base de données.
/// @do: expose_database_service
pub mod database_service;

/// @id: miyukiniadmin_module_security_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de sécurité et contrôles.
/// @do: expose_security_service
pub mod security_service;

/// @id: miyukiniadmin_module_admin_cell
/// @role: data
/// @layer: operator
/// @human: Types Cellule Admin, manifeste de test, cycle de vie (Module Testing and Lifecycle Contract).
/// @do: expose_admin_cell_types
pub mod admin_cell;

/// @id: miyukiniadmin_module_module_testing_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de tests des modules (découverte, cellule Admin, tests embarqués, intégrité TAMR).
/// @do: expose_module_testing_service
pub mod module_testing_service;

/// @id: miyukiniadmin_module_module_lifecycle_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de cycle de vie des modules (add, lock, unlock, delete).
/// @do: expose_module_lifecycle_service
pub mod module_lifecycle_service;

/// @id: miyukiniadmin_module_testing_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service d'exécution de tests.
/// @do: expose_testing_service
pub mod testing_service;

/// @id: miyukiniadmin_module_bonding_brother_bridge
/// @role: infrastructure
/// @layer: operator
/// @human: Pont vers BondingBrother pour les connexions.
/// @do: expose_bonding_brother_bridge
pub mod bonding_brother_bridge;

/// @id: miyukiniadmin_module_audit_logger
/// @role: infrastructure
/// @layer: operator
/// @human: Journalisation d'audit de toutes les actions.
/// @do: expose_audit_logger
pub mod audit_logger;

/// @id: miyukiniadmin_module_api_handlers
/// @role: infrastructure
/// @layer: operator
/// @human: Handlers API pour les requêtes backend.
/// @do: expose_api_handlers
pub mod api_handlers;

/// @id: miyukiniadmin_module_api_routes
/// @role: infrastructure
/// @layer: operator
/// @human: Routes API backend.
/// @do: expose_api_routes
pub mod api_routes;

/// @id: miyukiniadmin_module_crud_state
/// @role: infrastructure
/// @layer: operator
/// @human: État partagé CRUD tables/données (demo).
/// @do: expose_crud_state
pub mod crud_state;

/// @id: miyukiniadmin_module_migration_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de gestion des migrations de schéma (DB Operations §9.3).
/// @do: expose_migration_service
pub mod migration_service;

/// @id: miyukiniadmin_module_backup_service
/// @role: infrastructure
/// @layer: operator
/// @human: Service de sauvegarde et restauration (Backup Restore Contract).
/// @do: expose_backup_service
pub mod backup_service;

/// @id: miyukiniadmin_module_models
/// @role: data
/// @layer: operator
/// @human: Modèles (environment_state, admin_account, session, corruption_memory).
/// @do: expose_models
pub mod models;

/// @id: miyukiniadmin_module_services
/// @role: infrastructure
/// @layer: operator
/// @human: Services (environment_state, auth, permission, recovery).
/// @do: expose_services
pub mod services;
