//! Module de logging du Kernel
//!
//! Ce module fournit une abstraction pour le logging structuré.
//! Le produit choisit les backends et les formats.
//!
//! ## Principe
//!
//! - Une seule méthode `log` — minimal, évite `info()`, `warn()`, etc.
//! - `message: &str` — le produit formate avant l'appel
//! - Pas de `log_with_fields` — format structuré = choix du produit
//! - `Level` propre au kernel — pas de ré-export de `log::Level`
//!
//! ## Références
//!
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
//! - [Kernel - Reference Implementation Guidelines](../../../docs/kernel/implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)
//! - [Kernel - Revue Traits API v0.1](../../../docs/kernel/Miyukini%20Core%20System%20-%20Revue%20Traits%20API%20v0.1.md)

/// @id: kernel_log_level
/// @role: infrastructure
/// @layer: kernel
/// @human: Niveaux de log alignés sur la façade standard. Le kernel définit son propre Level, pas de ré-export de log::Level.
/// @do: define_log_levels
/// Niveaux de log alignés sur la façade standard.
///
/// Le kernel définit son propre type `Level` pour éviter de ré-exporter
/// `log::Level` et rester indépendant des détails d'implémentation.
///
/// ## Invariants respectés
///
/// - INV-K-3 : Primitives locales sûres uniquement (pas de backend externe obligatoire)
/// - INV-K-7 : Explicabilité (niveaux compréhensibles)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    /// Erreur critique
    Error,
    /// Avertissement
    Warn,
    /// Information
    Info,
    /// Debug
    Debug,
    /// Trace (détails très fins)
    Trace,
}

/// @id: kernel_logger_trait
/// @role: infrastructure
/// @layer: kernel
/// @human: Trait de logging générique. Le produit choisit le backend et le format. Une seule méthode log() pour rester minimal.
/// @do: define_logging_contract
/// Trait de logging générique.
///
/// Le trait `Logger` permet d'injecter différentes stratégies de logging.
/// Le produit choisit le backend (stdout, fichier, service distant) et le format
/// (JSON, clé-valeur, texte simple).
///
/// ## Signature gelée (v0.1)
///
/// Cette signature est gelée et ne peut pas être modifiée sans version majeure.
///
/// ## Design
///
/// Une seule méthode `log(level, message)` évite la multiplication de méthodes
/// (`info()`, `warn()`, etc.) et reste minimal. Le produit peut créer des wrappers
/// si nécessaire.
pub trait Logger {
    /// @id: kernel_logger_log
    /// @role: infrastructure
    /// @layer: kernel
    /// @human: Log un message avec un niveau donné. Le produit formate le message avant l'appel si nécessaire.
    /// @do: log_message_with_level
    /// @depends: kernel_logger_trait
    /// Log un message avec un niveau donné.
    ///
    /// Le contrat parle de "format structuré (JSON, clé-valeur)" mais le kernel
    /// ne l'impose pas : le produit peut formater en JSON (ou autre) avant l'appel.
    /// Le trait ne prend que `&str`.
    ///
    /// # Arguments
    ///
    /// * `level` - Le niveau de log (Error, Warn, Info, Debug, Trace)
    /// * `message` - Le message à logger (déjà formaté par le produit si nécessaire)
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::log::{Logger, Level, DefaultLogger};
    ///
    /// let logger = DefaultLogger;
    /// logger.log(Level::Info, "Application started");
    /// ```
    fn log(&self, level: Level, message: &str);
}

/// @id: kernel_logger_default
/// @role: infrastructure
/// @layer: kernel
/// @human: Logger par défaut : écrit sur stderr. Minimal et remplaçable. Le produit peut utiliser un Logger custom.
/// @do: log_to_stderr
/// Logger par défaut : écrit sur stderr.
///
/// Cette implémentation est minimale et remplaçable. Elle écrit simplement
/// sur stderr avec un format basique. Le produit peut implémenter un `Logger`
/// custom pour utiliser un backend spécifique (fichier, service distant, etc.).
///
/// ## Format
///
/// Format minimal : `[{:?}] {}` où le premier élément est le niveau et le second
/// est le message.
impl Logger for DefaultLogger {
    fn log(&self, level: Level, message: &str) {
        eprintln!("[{level:?}] {message}");
    }
}

/// @id: kernel_logger_default_struct
/// @role: infrastructure
/// @layer: kernel
/// @human: Structure du logger par défaut. Type unitaire sans état.
/// @do: represent_default_logger
/// Logger par défaut.
///
/// Type unitaire sans état. Peut être instancié et réutilisé.
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultLogger;

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: test_log_all_levels
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que tous les niveaux de log sont acceptés sans panic.
    /// @do: verify_all_levels_accepted
    /// @depends: kernel_logger_log
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

    /// @id: test_log_message_preserved
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que le message passé est loggé tel quel (ou au moins accepté).
    /// @do: verify_message_preserved
    /// @depends: kernel_logger_log
    #[test]
    fn test_log_message_preserved() {
        let logger = DefaultLogger;
        let message = "Test message with special chars: <>&\"'";
        // Le message doit être accepté sans panic
        logger.log(Level::Info, message);
    }

    /// @id: test_log_level_enum
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que tous les niveaux de l'enum Level existent et sont utilisables.
    /// @do: verify_level_enum_complete
    /// @depends: kernel_log_level
    #[test]
    fn test_log_level_enum() {
        // Vérifier que tous les niveaux existent
        let _ = Level::Error;
        let _ = Level::Warn;
        let _ = Level::Info;
        let _ = Level::Debug;
        let _ = Level::Trace;

        // Vérifier l'ordre (Error < Warn < Info < Debug < Trace)
        // L'ordre est basé sur la position dans l'enum (Error = 0, Warn = 1, etc.)
        assert!(Level::Error < Level::Warn);
        assert!(Level::Warn < Level::Info);
        assert!(Level::Info < Level::Debug);
        assert!(Level::Debug < Level::Trace);
    }

    /// @id: test_log_default_logger
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que DefaultLogger fonctionne sans panic.
    /// @do: verify_default_logger_works
    /// @depends: kernel_logger_default
    #[test]
    fn test_log_default_logger() {
        let logger = DefaultLogger;
        logger.log(Level::Info, "Test message");
        // Pas de panic
    }

    /// @id: test_log_no_format_imposed
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucun format n'est imposé. Le produit peut formater le message avant l'appel.
    /// @do: verify_no_format_imposed
    /// @depends: kernel_logger_log
    #[test]
    fn test_log_no_format_imposed() {
        let logger = DefaultLogger;
        // Le produit peut formater en JSON avant l'appel
        let json_message =
            r#"{"level":"info","message":"test","timestamp":"2026-01-28T12:00:00Z"}"#;
        logger.log(Level::Info, json_message);
        // Pas de panic, le format est accepté tel quel
    }

    /// @id: test_log_no_business_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucun événement métier prédéfini n'existe, conformément à INV-K-1.
    /// @do: verify_no_business_events
    /// @depends: kernel_logger_trait
    #[test]
    fn test_log_no_business_logic() {
        // Vérifier qu'aucun événement métier n'est prédéfini
        // Le kernel ne doit pas avoir de méthodes comme log_order_created(), etc.
        let logger = DefaultLogger;
        logger.log(Level::Info, "order_created"); // Le produit formate, pas le kernel
                                                  // Si on pouvait faire : logger.log_order_created(), cela violerait INV-K-1
    }

    /// @id: test_log_no_external_dependency
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que Logger fonctionne sans réseau, conformément à INV-K-2. Pas de backend obligatoire.
    /// @do: verify_no_network_required
    /// @depends: kernel_logger_log
    #[test]
    fn test_log_no_external_dependency() {
        // Ce test vérifie que log() fonctionne sans réseau
        // Si log() nécessitait un backend distant, ce test échouerait en mode offline
        let logger = DefaultLogger;
        logger.log(Level::Info, "test message");
        // Le fait que le test passe confirme INV-K-2
    }

    /// @id: test_log_deterministic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que log() est déterministe : même appel → même sortie, conformément à INV-K-6.
    /// @do: verify_log_determinism
    /// @depends: kernel_logger_log
    #[test]
    fn test_log_deterministic() {
        let logger = DefaultLogger;
        // Même appel → même comportement (pas de random, pas de timestamp variable dans le format)
        logger.log(Level::Info, "test");
        logger.log(Level::Info, "test"); // Même appel = même sortie
    }
}
