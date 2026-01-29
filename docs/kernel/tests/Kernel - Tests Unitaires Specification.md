# Kernel - Tests Unitaires Specification

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Spécification des tests à implémenter  
**Portée :** Modules id, time, log, config, lifecycle

---

## 1. Contexte

Ce document spécifie les **tests unitaires obligatoires** pour le crate `miyukini-kernel`, conformément à l'invariant **INV-K-6** (Déterminisme) et aux garanties de qualité du Kernel.

**Objectif :** Garantir que chaque module respecte les invariants INV-K-1 à INV-K-10 et que les traits publics fonctionnent conformément à l'API v0.1 gelée.

**Références contractuelles :**
- [Kernel - Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
- [Kernel - Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)
- [Kernel - Reference Implementation Guidelines](../implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)

---

## 2. Portée / Scope

Ce document couvre :
- Tests unitaires pour les 5 modules du Kernel (id, time, log, config, lifecycle)
- Vérification des invariants INV-K-1 à INV-K-10
- Validation des signatures d'API v0.1 gelées
- Tests de déterminisme et de pureté fonctionnelle

Ce document **ne couvre pas** :
- Tests d'intégration entre modules
- Tests de performance (voir tests techniques MiyukiniAdmin)
- Tests de conformité contractuelle inter-cores (voir contrats d'intégration)

---

## 3. Structure des tests

### 3.1 Organisation

Les tests doivent être organisés dans `crates/miyukini-kernel/tests/` avec la structure suivante :

```
tests/
├── mod.rs                    # Module principal des tests
├── config_tests.rs           # Tests module config
├── id_tests.rs               # Tests module id
├── time_tests.rs             # Tests module time
├── log_tests.rs              # Tests module log
├── lifecycle_tests.rs        # Tests module lifecycle
└── invariants_tests.rs       # Tests de vérification des invariants
```

### 3.2 Conventions de nommage

- Nom de test : `test_<module>_<fonctionnalité>_<scénario>`
- Exemple : `test_id_generate_unique_ids()`, `test_config_get_missing_key_returns_none()`

---

## 4. Tests par module

### 4.1 Module `config`

**Trait testé :** `Config::get(&self, key: &str) -> Option<&str>`

#### Tests obligatoires

| Test | Description | Vérifie |
|------|-------------|---------|
| `test_config_get_existing_key()` | Récupère une clé présente | Retourne `Some(&str)` avec la valeur |
| `test_config_get_missing_key()` | Récupère une clé absente | Retourne `None` |
| `test_config_get_empty_key()` | Récupère une clé vide | Retourne `None` ou `Some("")` selon impl |
| `test_config_get_case_sensitive()` | Vérifie sensibilité à la casse | Comportement cohérent (doc à préciser) |
| `test_config_envconfig_from_env()` | Charge depuis variables d'environnement | `EnvConfig::from_env()` fonctionne |
| `test_config_no_business_logic()` | Vérifie absence de logique métier | Aucune clé prédéfinie métier (INV-K-1) |
| `test_config_no_external_dependency()` | Fonctionne sans réseau | Pas d'appel réseau (INV-K-2) |
| `test_config_deterministic()` | Même état → même résultat | Déterminisme (INV-K-6) |

**Exemple de test :**

```rust
#[cfg(test)]
mod config_tests {
    use super::*;
    use miyukini_kernel::config::{Config, EnvConfig};

    #[test]
    fn test_config_get_existing_key() {
        std::env::set_var("TEST_KEY", "test_value");
        let config = EnvConfig::from_env();
        assert_eq!(config.get("TEST_KEY"), Some("test_value"));
        std::env::remove_var("TEST_KEY");
    }

    #[test]
    fn test_config_get_missing_key() {
        let config = EnvConfig::from_env();
        assert_eq!(config.get("NONEXISTENT_KEY"), None);
    }

    #[test]
    fn test_config_no_business_logic() {
        // Vérifier qu'aucune clé métier n'est prédéfinie
        let config = EnvConfig::from_env();
        // Aucune clé comme "STRIPE_KEY", "USER_TTL", etc.
        assert_eq!(config.get("STRIPE_KEY"), None);
        assert_eq!(config.get("USER_TTL"), None);
    }
}
```

---

### 4.2 Module `id`

**Trait testé :** `IdGenerator::generate(&self) -> Id`

#### Tests obligatoires

| Test | Description | Vérifie |
|------|-------------|---------|
| `test_id_generate_unique()` | Génère des IDs uniques | Deux IDs générés sont différents |
| `test_id_display_roundtrip()` | Display → parse → même Id | `Id::parse(id.to_string()) == id` |
| `test_id_parse_valid_uuid()` | Parse un UUID valide | `Id::parse("...")` réussit |
| `test_id_parse_invalid_format()` | Parse un format invalide | Retourne `Err(IdParseError)` |
| `test_id_equality()` | Comparaison d'égalité | `Id` implémente `Eq`, `PartialEq` |
| `test_id_hash()` | Utilisation comme clé | `Id` implémente `Hash` |
| `test_id_opaque()` | Format interne non exposé | Pas d'accès direct à `uuid::Uuid` |
| `test_id_no_business_logic()` | Pas de sémantique métier | Pas de `UserId`, `OrderId`, etc. (INV-K-1) |
| `test_id_no_external_dependency()` | Génération locale | Pas d'appel réseau (INV-K-2) |
| `test_id_deterministic_parse()` | Parse déterministe | Même string → même Id (INV-K-6) |

**Exemple de test :**

```rust
#[cfg(test)]
mod id_tests {
    use super::*;
    use miyukini_kernel::id::{Id, IdGenerator, UuidIdGenerator};

    #[test]
    fn test_id_generate_unique() {
        let generator = UuidIdGenerator;
        let id1 = generator.generate();
        let id2 = generator.generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_display_roundtrip() {
        let generator = UuidIdGenerator;
        let id = generator.generate();
        let id_str = id.to_string();
        let parsed = Id::parse(&id_str).expect("Should parse valid UUID");
        assert_eq!(id, parsed);
    }

    #[test]
    fn test_id_parse_invalid_format() {
        assert!(Id::parse("not-a-uuid").is_err());
        assert!(Id::parse("").is_err());
    }
}
```

---

### 4.3 Module `time`

**Trait testé :** `Clock::now(&self) -> SystemTime`

#### Tests obligatoires

| Test | Description | Vérifie |
|------|-------------|---------|
| `test_time_now_returns_systemtime()` | Retourne un SystemTime | Type correct |
| `test_time_now_monotonic()` | Temps monotone | `now()` croît avec le temps |
| `test_time_fake_clock_injection()` | Injection en test | FakeClock avec `now()` constant |
| `test_time_no_timezone_logic()` | Pas de logique timezone | Retourne `SystemTime`, pas de conversion |
| `test_time_no_business_logic()` | Pas de logique métier | Pas de calcul de dates métier (INV-K-1) |
| `test_time_no_external_dependency()` | Pas de NTP obligatoire | Fonctionne sans réseau (INV-K-2) |
| `test_time_deterministic_fake()` | FakeClock déterministe | Même FakeClock → même résultat (INV-K-6) |

**Exemple de test :**

```rust
#[cfg(test)]
mod time_tests {
    use super::*;
    use miyukini_kernel::time::{Clock, DefaultClock};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn test_time_now_returns_systemtime() {
        let clock = DefaultClock;
        let now = clock.now();
        assert!(now.duration_since(UNIX_EPOCH).is_ok());
    }

    #[test]
    fn test_time_fake_clock_injection() {
        let fixed_time = SystemTime::UNIX_EPOCH + Duration::from_secs(1234567890);
        let fake_clock = FakeClock::new(fixed_time);
        assert_eq!(fake_clock.now(), fixed_time);
    }
}
```

---

### 4.4 Module `log`

**Trait testé :** `Logger::log(&self, level: Level, message: &str)`

#### Tests obligatoires

| Test | Description | Vérifie |
|------|-------------|---------|
| `test_log_all_levels()` | Log tous les niveaux | Error, Warn, Info, Debug, Trace |
| `test_log_message_preserved()` | Message préservé | Le message passé est loggé tel quel |
| `test_log_level_enum()` | Enum Level correct | 5 niveaux définis |
| `test_log_default_logger()` | DefaultLogger fonctionne | Pas de panic |
| `test_log_no_format_imposed()` | Pas de format imposé | Le produit formate `message` |
| `test_log_no_business_logic()` | Pas de logique métier | Pas d'événements métier prédéfinis (INV-K-1) |
| `test_log_no_external_dependency()` | Pas de backend obligatoire | Fonctionne sans réseau (INV-K-2) |
| `test_log_deterministic()` | Log déterministe | Même appel → même sortie (INV-K-6) |

**Exemple de test :**

```rust
#[cfg(test)]
mod log_tests {
    use super::*;
    use miyukini_kernel::log::{Logger, Level, DefaultLogger};

    #[test]
    fn test_log_all_levels() {
        let logger = DefaultLogger;
        logger.log(Level::Error, "error message");
        logger.log(Level::Warn, "warn message");
        logger.log(Level::Info, "info message");
        logger.log(Level::Debug, "debug message");
        logger.log(Level::Trace, "trace message");
        // Pas de panic
    }

    #[test]
    fn test_log_level_enum() {
        // Vérifier que tous les niveaux existent
        let _ = Level::Error;
        let _ = Level::Warn;
        let _ = Level::Info;
        let _ = Level::Debug;
        let _ = Level::Trace;
    }
}
```

---

### 4.5 Module `lifecycle`

**Trait testé :** `Lifecycle::register_shutdown_hook<F>(&mut self, f: F)` et `Lifecycle::shutdown(&mut self)`

#### Tests obligatoires

| Test | Description | Vérifie |
|------|-------------|---------|
| `test_lifecycle_register_hook()` | Enregistre un hook | Hook enregistré sans erreur |
| `test_lifecycle_shutdown_executes_hooks()` | Exécute les hooks | Hooks appelés lors de `shutdown()` |
| `test_lifecycle_shutdown_lifo_order()` | Ordre LIFO | Dernier enregistré = premier exécuté |
| `test_lifecycle_multiple_hooks()` | Plusieurs hooks | Tous les hooks exécutés |
| `test_lifecycle_shutdown_idempotent()` | Idempotence | Second `shutdown()` = no-op ou comportement défini |
| `test_lifecycle_no_init_hooks()` | Pas de hooks d'init | Seulement shutdown (pas d'orchestration) |
| `test_lifecycle_no_business_logic()` | Pas de logique métier | Pas de hooks métier prédéfinis (INV-K-1) |
| `test_lifecycle_deterministic()` | Ordre déterministe | Même ordre d'enregistrement → même ordre d'exécution (INV-K-6) |

**Exemple de test :**

```rust
#[cfg(test)]
mod lifecycle_tests {
    use super::*;
    use miyukini_kernel::lifecycle::{Lifecycle, DefaultLifecycle};

    #[test]
    fn test_lifecycle_shutdown_lifo_order() {
        let mut lifecycle = DefaultLifecycle::new();
        let mut order = Vec::new();

        lifecycle.register_shutdown_hook(|| order.push(1));
        lifecycle.register_shutdown_hook(|| order.push(2));
        lifecycle.register_shutdown_hook(|| order.push(3));

        lifecycle.shutdown();

        assert_eq!(order, vec![3, 2, 1]); // LIFO
    }

    #[test]
    fn test_lifecycle_shutdown_executes_hooks() {
        let mut lifecycle = DefaultLifecycle::new();
        let mut called = false;

        lifecycle.register_shutdown_hook(|| called = true);
        assert!(!called);

        lifecycle.shutdown();
        assert!(called);
    }
}
```

---

## 5. Tests d'invariants transversaux

### 5.1 INV-K-1 : Aucune logique métier

**Tests :**
- Vérifier qu'aucun module n'expose de types métier (`UserId`, `OrderId`, `StripeKey`, etc.)
- Vérifier qu'aucun module ne contient de règles de gestion métier
- Vérifier qu'aucun module ne référence des concepts domaine

**Exemple :**

```rust
#[cfg(test)]
mod invariants_tests {
    #[test]
    fn test_inv_k1_no_business_logic() {
        // Vérifier qu'aucun type métier n'existe
        // Ce test échoue si on trouve UserId, OrderId, etc.
    }
}
```

### 5.2 INV-K-2 : Aucune dépendance externe critique

**Tests :**
- Vérifier que tous les modules fonctionnent sans réseau
- Vérifier qu'aucun appel réseau n'est fait au démarrage
- Vérifier que les tests passent en mode offline

### 5.3 INV-K-3 : Primitives locales sûres uniquement

**Tests :**
- Vérifier l'absence d'effets de bord cachés
- Vérifier l'absence de variables globales mutables
- Vérifier que les fonctions sont pures ou explicites

### 5.4 INV-K-6 : Déterminisme

**Tests :**
- Vérifier que les mêmes entrées produisent les mêmes sorties
- Vérifier la reproductibilité des tests
- Vérifier l'absence de sources de non-déterminisme (sauf injection explicite)

### 5.5 INV-K-9 : Coût proportionnel au hardware

**Tests :**
- Vérifier que les allocations mémoire sont raisonnables
- Vérifier l'absence de consommation excessive de ressources
- Tests de charge basiques (optionnel, voir tests techniques)

---

## 6. Structure de test recommandée

### 6.1 Utilisation de `cargo test`

```bash
# Exécuter tous les tests
cargo test

# Exécuter les tests d'un module spécifique
cargo test --test config_tests

# Exécuter avec affichage des sorties
cargo test -- --nocapture

# Exécuter les tests en mode offline (vérification INV-K-2)
cargo test --offline
```

### 6.2 Coverage minimal

**Objectif :** 100% de couverture pour les traits publics et les invariants.

**Modules à couvrir :**
- Toutes les méthodes publiques des traits
- Tous les chemins d'erreur (parsing, validation)
- Tous les invariants vérifiables par test

---

## 7. Intégration dans le workflow

### 7.1 Avant commit

Les tests doivent passer avant tout commit :

```bash
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### 7.2 CI/CD

Les tests doivent être exécutés dans le pipeline CI/CD :
- Tests unitaires
- Tests de déterminisme (réexécution multiple)
- Tests offline (vérification INV-K-2)

---

## 8. Références

- [Kernel - Invariants & Guarantees](../contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
- [Kernel - Reference Implementation Guidelines](../implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)
- [Kernel - Revue Traits API v0.1](../Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)

---

**Version :** 1.0  
**Date :** 2026-01-28  
**Statut :** Normatif — Spécification des tests à implémenter  
**Action requise :** Implémenter les tests selon cette spécification lors de l'implémentation du Kernel
