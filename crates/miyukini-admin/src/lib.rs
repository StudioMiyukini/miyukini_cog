//! # MiyukiniAdmin
//!
//! Console souveraine d'administration du Miyukini Core System.
//!
//! MiyukiniAdmin est un Opérateur Souverain autonome qui observe, installe, arbitre,
//! mais ne vit pas dans le flux normal. Il est out-of-band, comme un BIOS/hyperviseur.

/// @id miyukiniadmin_module_config
/// @role infrastructure
/// @layer operator
/// @do expose_config_module
//@human / @human Module de configuration backend.
pub mod config;

/// @id miyukiniadmin_module_monitoring_service
/// @role infrastructure
/// @layer operator
/// @do expose_monitoring_service
//@human / @human Service de monitoring et collecte de métriques.
pub mod monitoring_service;

/// @id miyukiniadmin_module_database_service
/// @role infrastructure
/// @layer operator
/// @do expose_database_service
//@human / @human Service de gestion de base de données.
pub mod database_service;

/// @id miyukiniadmin_module_security_service
/// @role infrastructure
/// @layer operator
/// @do expose_security_service
//@human / @human Service de sécurité et contrôles.
pub mod security_service;

/// @id miyukiniadmin_module_admin_cell
/// @role data
/// @layer operator
/// @do expose_admin_cell_types
//@human / @human Types Cellule Admin, manifeste de test, cycle de vie (Module Testing and Lifecycle Contract).
pub mod admin_cell;

/// @id miyukiniadmin_module_module_testing_service
/// @role infrastructure
/// @layer operator
/// @do expose_module_testing_service
//@human / @human Service de tests des modules (découverte, cellule Admin, tests embarqués, intégrité TAMR).
pub mod module_testing_service;

/// @id miyukiniadmin_module_module_lifecycle_service
/// @role infrastructure
/// @layer operator
/// @do expose_module_lifecycle_service
//@human / @human Service de cycle de vie des modules (add, lock, unlock, delete).
pub mod module_lifecycle_service;

/// @id miyukiniadmin_module_testing_service
/// @role infrastructure
/// @layer operator
/// @do expose_testing_service
//@human / @human Service d'exécution de tests.
pub mod testing_service;

/// @id miyukiniadmin_module_db_unit_tests
/// @role infrastructure
/// @layer operator
/// @do expose_db_unit_tests
//@human / @human Tests unitaires DB (COH, CONF, STRUCT, SEC) — Unit Tests Contract.
pub mod db_unit_tests;

/// @id miyukiniadmin_module_cycle_tests_runner
/// @role infrastructure
/// @layer operator
/// @do expose_cycle_tests_runner
//@human / @human Runner des tests de cycle (PERF, LAT, LOAD, RES, MiyukiniSQLtest).
pub mod cycle_tests_runner;

/// @id miyukiniadmin_module_testing_suites
/// @role infrastructure
/// @layer operator
/// @do expose_testing_suites
//@human / @human Suites Quick/Standard/Full (unit + cycle) et rapports.
pub mod testing_suites;

/// @id miyukiniadmin_module_bonding_brother_bridge
/// @role infrastructure
/// @layer operator
/// @do expose_bonding_brother_bridge
//@human / @human Pont vers BondingBrother pour les connexions.
pub mod bonding_brother_bridge;

/// @id miyukiniadmin_module_audit_logger
/// @role infrastructure
/// @layer operator
/// @do expose_audit_logger
//@human / @human Journalisation d'audit de toutes les actions.
pub mod audit_logger;

/// @id miyukiniadmin_module_api_handlers
/// @role infrastructure
/// @layer operator
/// @do expose_api_handlers
//@human / @human Handlers API pour les requêtes backend.
pub mod api_handlers;

/// @id miyukiniadmin_module_api_routes
/// @role infrastructure
/// @layer operator
/// @do expose_api_routes
//@human / @human Routes API backend.
pub mod api_routes;

/// @id miyukiniadmin_module_crud_state
/// @role infrastructure
/// @layer operator
/// @do expose_crud_state
//@human / @human État partagé CRUD tables/données (demo).
pub mod crud_state;

/// @id miyukiniadmin_module_migration_service
/// @role infrastructure
/// @layer operator
/// @do expose_migration_service
//@human / @human Service de gestion des migrations de schéma (DB Operations §9.3).
pub mod migration_service;

/// @id miyukiniadmin_module_backup_service
/// @role infrastructure
/// @layer operator
/// @do expose_backup_service
//@human / @human Service de sauvegarde et restauration (Backup Restore Contract).
pub mod backup_service;

/// @id miyukiniadmin_module_models
/// @role data
/// @layer operator
/// @do expose_models
//@human / @human Modèles (environment_state, admin_account, session, corruption_memory).
pub mod models;

/// @id miyukiniadmin_module_services
/// @role infrastructure
/// @layer operator
/// @do expose_services
//@human / @human Services (environment_state, auth, permission, recovery).
pub mod services;

/// @id miyukiniadmin_module_gouvernance
/// @role infrastructure
/// @layer operator
/// @do expose_gouvernance
//@human / @human Porte StrongFather obligatoire des actions modifiantes (K-11).
pub mod gouvernance;
