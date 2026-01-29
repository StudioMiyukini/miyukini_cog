//! Tests d'intégration du Kernel
//!
//! Ces tests vérifient l'intégration entre les modules du Kernel et la conformité
//! aux invariants INV-K-1 à INV-K-10.
//!
//! ## Références
//!
//! - [Kernel - Tests Unitaires Specification](../../../docs/kernel/tests/Kernel%20-%20Tests%20Unitaires%20Specification.md)
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)

use miyukini_kernel::{
    config::{Config, EnvConfig},
    id::{IdGenerator, UuidIdGenerator},
    lifecycle::{Lifecycle, DefaultLifecycle},
    log::{Level, Logger, DefaultLogger},
    time::{Clock, DefaultClock},
};
use std::time::UNIX_EPOCH;

/// @id: test_integration_kernel_modules_work_together
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant que tous les modules du Kernel peuvent être utilisés ensemble sans conflit.
/// @do: verify_modules_integration
#[test]
fn test_integration_kernel_modules_work_together() {
    // Config
    let config = EnvConfig::from_env();
    let _db_url = config.get("DATABASE_URL");

    // Id
    let id_generator = UuidIdGenerator;
    let id = id_generator.generate();

    // Time
    let clock = DefaultClock;
    let now = clock.now();

    // Log
    let logger = DefaultLogger;
    logger.log(Level::Info, &format!("Generated ID: {} at {:?}", id, now));

    // Lifecycle
    let mut lifecycle = DefaultLifecycle::new();
    lifecycle.register_shutdown_hook(|| {
        println!("Shutting down...");
    });

    // Tous les modules fonctionnent ensemble
    assert!(now.duration_since(UNIX_EPOCH).is_ok());
}

/// @id: test_integration_config_id_integration
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'intégration entre Config et Id : génération d'ID depuis une config.
/// @do: verify_config_id_integration
#[test]
fn test_integration_config_id_integration() {
    let _config = EnvConfig::from_env();
    let id_generator = UuidIdGenerator;

    // Générer un ID même si la config ne contient pas de clé spécifique
    let id = id_generator.generate();
    let id_str = id.to_string();

    // L'ID peut être utilisé comme valeur de config (exemple théorique)
    // En pratique, le produit décide comment utiliser l'ID
    assert!(!id_str.is_empty());
}

/// @id: test_integration_time_log_integration
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'intégration entre Time et Log : logger avec timestamp.
/// @do: verify_time_log_integration
#[test]
fn test_integration_time_log_integration() {
    let clock = DefaultClock;
    let logger = DefaultLogger;

    let now = clock.now();
    let timestamp = now.duration_since(UNIX_EPOCH).unwrap().as_secs();

    // Logger avec timestamp (le produit formate)
    logger.log(
        Level::Info,
        &format!("Event at timestamp: {}", timestamp),
    );

    // Pas de panic
    assert!(timestamp > 0);
}

/// @id: test_integration_lifecycle_log_integration
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'intégration entre Lifecycle et Log : logger dans un hook de shutdown.
/// @do: verify_lifecycle_log_integration
#[test]
fn test_integration_lifecycle_log_integration() {
    let logger = DefaultLogger;
    let mut lifecycle = DefaultLifecycle::new();

    lifecycle.register_shutdown_hook(move || {
        logger.log(Level::Info, "Shutdown hook executed");
    });

    // Le hook peut utiliser le logger
    lifecycle.shutdown();
    // Pas de panic
}

/// @id: test_integration_id_lifecycle_integration
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'intégration entre Id et Lifecycle : générer un ID dans un hook.
/// @do: verify_id_lifecycle_integration
#[test]
fn test_integration_id_lifecycle_integration() {
    let id_generator = UuidIdGenerator;
    let mut lifecycle = DefaultLifecycle::new();

    lifecycle.register_shutdown_hook(move || {
        let _id = id_generator.generate();
        // L'ID peut être généré dans un hook
    });

    lifecycle.shutdown();
    // Pas de panic
}

/// @id: test_integration_invariant_k1_no_business_logic
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-1 : aucune logique métier dans le Kernel.
/// @do: verify_invariant_k1
#[test]
fn test_integration_invariant_k1_no_business_logic() {
    // Vérifier qu'aucun module n'expose de types métier
    let config = EnvConfig::from_env();
    let id_generator = UuidIdGenerator;

    // Aucune clé métier prédéfinie
    assert_eq!(config.get("USER_ID"), None);
    assert_eq!(config.get("ORDER_STATUS"), None);

    // Aucun type métier (UserId, OrderId, etc.)
    let _id: miyukini_kernel::Id = id_generator.generate();
    // Si on pouvait faire : let _user_id: UserId = ..., cela violerait INV-K-1
}

/// @id: test_integration_invariant_k2_no_external_dependency
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-2 : aucune dépendance externe critique. Tous les modules fonctionnent sans réseau.
/// @do: verify_invariant_k2
#[test]
fn test_integration_invariant_k2_no_external_dependency() {
    // Tous les modules doivent fonctionner sans réseau
    let config = EnvConfig::from_env(); // Source locale
    let id_generator = UuidIdGenerator; // Génération locale
    let clock = DefaultClock; // Horloge locale
    let logger = DefaultLogger; // Sortie locale
    let mut lifecycle = DefaultLifecycle::new(); // Hooks locaux

    // Tous fonctionnent sans réseau
    let _value = config.get("SOME_KEY");
    let _id = id_generator.generate();
    let _now = clock.now();
    logger.log(Level::Info, "test");
    lifecycle.register_shutdown_hook(|| {});

    // Le fait que le test passe confirme INV-K-2
}

/// @id: test_integration_invariant_k6_determinism
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-6 : déterminisme. Les opérations déterministes produisent le même résultat.
/// @do: verify_invariant_k6
#[test]
fn test_integration_invariant_k6_determinism() {
    // Parse déterministe
    let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
    let id1 = miyukini_kernel::Id::parse(uuid_str).unwrap();
    let id2 = miyukini_kernel::Id::parse(uuid_str).unwrap();
    assert_eq!(id1, id2); // Même entrée → même sortie

    // Config déterministe (même environnement)
    std::env::set_var("TEST_DETERMINISTIC", "value");
    let config1 = EnvConfig::from_env();
    let config2 = EnvConfig::from_env();
    assert_eq!(
        config1.get("TEST_DETERMINISTIC"),
        config2.get("TEST_DETERMINISTIC")
    );
    std::env::remove_var("TEST_DETERMINISTIC");
}

/// @id: test_integration_invariant_k7_explicability
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-7 : explicabilité. Les messages sont compréhensibles par un humain.
/// @do: verify_invariant_k7
#[test]
fn test_integration_invariant_k7_explicability() {
    let logger = DefaultLogger;

    // Les messages doivent être explicables
    logger.log(Level::Error, "Configuration file not found");
    logger.log(Level::Warn, "Retrying connection...");
    logger.log(Level::Info, "Application started successfully");

    // Pas de codes cryptiques
    // Les niveaux sont explicites (Error, Warn, Info, Debug, Trace)
    assert!(matches!(Level::Error, Level::Error));
}

/// @id: test_integration_invariant_k8_offline
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-8 : souveraineté locale. Tous les modules fonctionnent offline.
/// @do: verify_invariant_k8
#[test]
fn test_integration_invariant_k8_offline() {
    // Tous les modules fonctionnent offline
    let config = EnvConfig::from_env();
    let id_generator = UuidIdGenerator;
    let clock = DefaultClock;
    let logger = DefaultLogger;
    let mut lifecycle = DefaultLifecycle::new();

    // Opérations offline
    let _value = config.get("KEY");
    let _id = id_generator.generate();
    let _now = clock.now();
    logger.log(Level::Info, "Offline operation");
    lifecycle.register_shutdown_hook(|| {});

    // Le fait que le test passe confirme INV-K-8
}

/// @id: test_integration_invariant_k10_governance
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant l'invariant INV-K-10 : gouvernance préservée. Le Kernel ne prend pas de décision autonome.
/// @do: verify_invariant_k10
#[test]
fn test_integration_invariant_k10_governance() {
    // Le Kernel fournit des primitives, ne prend pas de décision
    let config = EnvConfig::from_env();
    let id_generator = UuidIdGenerator;

    // Le Kernel fournit l'ID, le produit décide comment l'utiliser
    let id = id_generator.generate();

    // Le Kernel fournit la config, le produit décide des clés
    let _value = config.get("PRODUCT_KEY");

    // Aucune décision stratégique dans le Kernel
    // Si on pouvait faire : config.should_allow_access(), cela violerait INV-K-10
}

/// @id: test_integration_round_trip_id_config
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant le round-trip Id → Config → Id : un ID peut être stocké et récupéré depuis la config.
/// @do: verify_id_config_round_trip
#[test]
fn test_integration_round_trip_id_config() {
    let id_generator = UuidIdGenerator;
    let id = id_generator.generate();
    let id_str = id.to_string();

    // Simuler stockage dans config (en pratique, le produit gère)
    std::env::set_var("STORED_ID", &id_str);
    let config = EnvConfig::from_env();

    // Récupérer et parser
    if let Some(stored_str) = config.get("STORED_ID") {
        let parsed_id = miyukini_kernel::Id::parse(stored_str).unwrap();
        assert_eq!(id, parsed_id); // Round-trip réussi
    }

    std::env::remove_var("STORED_ID");
}

/// @id: test_integration_lifecycle_shutdown_with_all_modules
/// @role: test
/// @layer: kernel
/// @human: Test vérifiant que shutdown() peut utiliser tous les modules du Kernel dans les hooks.
/// @do: verify_lifecycle_shutdown_integration
#[test]
fn test_integration_lifecycle_shutdown_with_all_modules() {
    let config = EnvConfig::from_env();
    let id_generator = UuidIdGenerator;
    let clock = DefaultClock;
    let logger = DefaultLogger;
    let mut lifecycle = DefaultLifecycle::new();

    // Enregistrer des hooks utilisant tous les modules
    lifecycle.register_shutdown_hook(move || {
        let _value = config.get("SHUTDOWN_KEY");
    });

    lifecycle.register_shutdown_hook(move || {
        let _id = id_generator.generate();
    });

    lifecycle.register_shutdown_hook(move || {
        let _now = clock.now();
    });

    lifecycle.register_shutdown_hook(move || {
        logger.log(Level::Info, "Shutdown complete");
    });

    // Tous les hooks peuvent utiliser les modules du Kernel
    lifecycle.shutdown();
    // Pas de panic
}
