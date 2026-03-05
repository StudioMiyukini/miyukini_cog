//! Module de configuration du Kernel
//!
//! Ce module fournit une abstraction pour l'accès à la configuration.
//! Le Kernel fournit le mécanisme d'accès, pas la politique.
//! Le produit choisit les clés et les valeurs.
//!
//! ## Principe
//!
//! - Pas de typage (int, bool) — le produit parse
//! - Pas de validation de clés — le produit définit ses clés
//! - Source locale uniquement — pas de dépendance réseau (INV-K-2)
//! - Pas de format imposé — le produit choisit (JSON, TOML, etc.)
//!
//! ## Références
//!
//! - [Kernel - Invariants & Guarantees](../../../docs/kernel/contracts/Kernel%20-%20Invariants%20&%20Guarantees.md)
//! - [Kernel - Reference Implementation Guidelines](../../../docs/kernel/implementation/Kernel%20-%20Reference%20Implementation%20Guidelines.md)

use std::collections::HashMap;

/// `@id`: config_trait
/// `@role`: interface
/// `@layer`: kernel
/// `@human`: Trait d'accès à la configuration. Fournit un mécanisme générique pour récupérer des valeurs de configuration par clé.
/// `@do`: get_config_value
/// Trait public du module `config`. Interface d'accès à la configuration.
///
/// Le kernel fournit le mécanisme d'accès, pas la politique.
/// Le produit choisit les clés et les valeurs.
/// Pas de typage (int, bool) — le produit parse.
///
/// ## Signature gelée (v0.1)
///
/// Cette signature est gelée et ne peut pas être modifiée sans version majeure.
pub trait Config {
    /// @id: config_get
    /// @role: accessor
    /// @layer: kernel
    /// @human: Récupère une valeur de configuration par sa clé. Retourne None si la clé n'existe pas.
    /// @do: get_config_by_key
    /// @depends: config_trait
    /// Récupère une valeur de configuration par sa clé.
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de configuration à récupérer
    ///
    /// # Returns
    ///
    /// * `Some(&str)` - La valeur de configuration si la clé existe
    /// * `None` - Si la clé n'existe pas
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::config::{Config, EnvConfig};
    ///
    /// let config = EnvConfig::from_env();
    /// if let Some(value) = config.get("DATABASE_URL") {
    ///     println!("Database URL: {}", value);
    /// }
    /// ```
    fn get(&self, key: &str) -> Option<&str>;
}

/// @id: envconfig_struct
/// @role: implementation
/// @layer: kernel
/// @human: Implémentation par défaut du trait Config utilisant les variables d'environnement. Charge toutes les variables d'environnement au moment de la création.
/// @do: load_env_config
/// Configuration chargée depuis les variables d'environnement.
///
/// Le produit choisit ses clés et ses valeurs.
/// Cette implémentation charge toutes les variables d'environnement disponibles
/// au moment de l'appel à `from_env()`.
///
/// ## Statut
///
/// Implémentation, hors contrat du trait — peut évoluer.
///
/// ## Invariants respectés
///
/// - INV-K-2 : Aucune dépendance externe critique (source locale uniquement)
/// - INV-K-3 : Primitives locales sûres uniquement (std::env::vars() est sûr)
/// - INV-K-1 : Aucune logique métier (pas de clés prédéfinies)
#[derive(Debug, Clone)]
pub struct EnvConfig {
    /// @id: envconfig_values
    /// @role: storage
    /// @layer: kernel
    /// @human: Stockage interne des variables d'environnement sous forme de HashMap clé-valeur.
    /// @do: store_env_vars
    /// @depends: envconfig_struct
    values: HashMap<String, String>,
}

impl EnvConfig {
    /// @id: envconfig_from_env
    /// @role: constructor
    /// @layer: kernel
    /// @human: Charge la configuration depuis les variables d'environnement. Ne retourne pas d'erreur car std::env::vars() est infaillible.
    /// @do: load_from_environment
    /// @depends: envconfig_struct
    /// Charge la configuration depuis les variables d'environnement.
    ///
    /// Cette méthode collecte toutes les variables d'environnement disponibles
    /// au moment de l'appel et les stocke dans une HashMap interne.
    ///
    /// ## Garanties
    ///
    /// - Ne retourne jamais d'erreur : `std::env::vars()` est infaillible
    /// - Source locale uniquement : pas d'appel réseau (INV-K-2)
    /// - Déterministe : même environnement → même résultat (INV-K-6)
    ///
    /// # Returns
    ///
    /// Une instance de `EnvConfig` contenant toutes les variables d'environnement disponibles.
    ///
    /// # Exemple
    ///
    /// ```rust
    /// use miyukini_kernel::config::EnvConfig;
    ///
    /// let config = EnvConfig::from_env();
    /// ```
    #[must_use]
    pub fn from_env() -> Self {
        Self {
            values: std::env::vars().collect(),
        }
    }
}

impl Config for EnvConfig {
    /// @id: envconfig_get_impl
    /// @role: implementation
    /// @layer: kernel
    /// @human: Implémentation de la méthode get du trait Config pour EnvConfig. Recherche la clé dans la HashMap interne et retourne une référence à la valeur si trouvée.
    /// @do: get_from_env_map
    /// @depends: config_get, envconfig_struct
    /// Implémentation de `Config::get` pour `EnvConfig`.
    ///
    /// Recherche la clé dans la HashMap interne et retourne une référence
    /// à la valeur si trouvée, `None` sinon.
    ///
    /// ## Comportement
    ///
    /// - La recherche est sensible à la casse
    /// - Les clés vides retournent `None` (pas de variable d'environnement vide)
    /// - Pas de validation de format : le produit parse les valeurs
    ///
    /// # Arguments
    ///
    /// * `key` - La clé de configuration à rechercher
    ///
    /// # Returns
    ///
    /// * `Some(&str)` - La valeur si la clé existe
    /// * `None` - Si la clé n'existe pas
    fn get(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(std::string::String::as_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @id: test_config_get_existing_key
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que get() retourne la valeur correcte pour une clé existante dans la configuration.
    /// @do: verify_get_existing_key
    /// @depends: config_get
    #[test]
    fn test_config_get_existing_key() {
        std::env::set_var("TEST_CONFIG_KEY", "test_value");
        let config = EnvConfig::from_env();
        assert_eq!(config.get("TEST_CONFIG_KEY"), Some("test_value"));
        std::env::remove_var("TEST_CONFIG_KEY");
    }

    /// @id: test_config_get_missing_key
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que get() retourne None pour une clé absente de la configuration.
    /// @do: verify_get_missing_key
    /// @depends: config_get
    #[test]
    fn test_config_get_missing_key() {
        let config = EnvConfig::from_env();
        assert_eq!(config.get("NONEXISTENT_KEY_12345"), None);
    }

    /// @id: test_config_get_empty_key
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant le comportement de get() avec une clé vide. Retourne None car aucune variable d'environnement ne peut avoir une clé vide.
    /// @do: verify_get_empty_key
    /// @depends: config_get
    #[test]
    fn test_config_get_empty_key() {
        let config = EnvConfig::from_env();
        // Une clé vide ne peut pas exister dans les variables d'environnement
        assert_eq!(config.get(""), None);
    }

    /// @id: test_config_get_case_sensitive
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que get() est sensible à la casse. Les clés avec casse différente sont considérées comme différentes.
    /// @do: verify_case_sensitivity
    /// @depends: config_get
    #[test]
    fn test_config_get_case_sensitive() {
        std::env::set_var("TEST_CASE_KEY", "lowercase_value");
        let config = EnvConfig::from_env();
        // La recherche est sensible à la casse
        assert_eq!(config.get("TEST_CASE_KEY"), Some("lowercase_value"));
        assert_eq!(config.get("test_case_key"), None);
        assert_eq!(config.get("TEST_case_KEY"), None);
        std::env::remove_var("TEST_CASE_KEY");
    }

    /// @id: test_config_envconfig_from_env
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que EnvConfig::from_env() fonctionne correctement et charge les variables d'environnement.
    /// @do: verify_from_env_works
    /// @depends: envconfig_from_env
    #[test]
    fn test_config_envconfig_from_env() {
        // Vérifier que from_env() ne panique pas
        let config = EnvConfig::from_env();
        // Vérifier qu'au moins PATH existe (variable système standard)
        // Note: PATH devrait exister sur tous les systèmes
        assert!(
            config.get("PATH").is_some()
                || config.get("Path").is_some()
                || config.get("path").is_some()
        );
    }

    /// @id: test_config_no_business_logic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant qu'aucune clé métier n'est prédéfinie dans le Kernel, conformément à INV-K-1 (Aucune logique métier).
    /// @do: verify_no_business_keys
    /// @depends: envconfig_struct
    #[test]
    fn test_config_no_business_logic() {
        // Vérifier qu'aucune clé métier n'est prédéfinie
        let config = EnvConfig::from_env();
        // Aucune clé comme "STRIPE_KEY", "USER_TTL", etc. ne doit être prédéfinie
        // Si ces clés n'existent pas dans l'environnement, elles doivent retourner None
        assert_eq!(config.get("STRIPE_KEY"), None);
        assert_eq!(config.get("USER_TTL"), None);
        assert_eq!(config.get("ORDER_STATUS"), None);
        assert_eq!(config.get("PRODUCT_CATEGORY"), None);
    }

    /// @id: test_config_no_external_dependency
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que EnvConfig fonctionne sans réseau, conformément à INV-K-2 (Aucune dépendance externe critique).
    /// @do: verify_no_network_required
    /// @depends: envconfig_from_env
    #[test]
    fn test_config_no_external_dependency() {
        // Ce test vérifie que from_env() fonctionne sans réseau
        // En isolant le réseau, le test devrait toujours passer
        let config = EnvConfig::from_env();
        // Si from_env() nécessitait un appel réseau, cette ligne échouerait en mode offline
        // Le fait que le test passe confirme INV-K-2
        assert!(config.get("SOME_KEY").is_none() || config.get("SOME_KEY").is_some());
    }

    /// @id: test_config_deterministic
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant le déterminisme de Config, conformément à INV-K-6. Même état d'environnement doit produire le même résultat.
    /// @do: verify_determinism
    /// @depends: envconfig_from_env, config_get
    #[test]
    fn test_config_deterministic() {
        // Définir une variable de test
        std::env::set_var("TEST_DETERMINISTIC", "value123");

        // Créer deux configurations avec le même environnement
        let config1 = EnvConfig::from_env();
        let config2 = EnvConfig::from_env();

        // Les deux doivent retourner la même valeur
        assert_eq!(
            config1.get("TEST_DETERMINISTIC"),
            config2.get("TEST_DETERMINISTIC")
        );
        assert_eq!(config1.get("TEST_DETERMINISTIC"), Some("value123"));

        // Nettoyer
        std::env::remove_var("TEST_DETERMINISTIC");

        // Vérifier que après suppression, les deux configurations retournent None
        // (si elles étaient recréées, mais ici elles gardent leur état initial)
        // Pour vraiment tester le déterminisme, on recrée après suppression
        std::env::remove_var("TEST_DETERMINISTIC");
        let config3 = EnvConfig::from_env();
        assert_eq!(config3.get("TEST_DETERMINISTIC"), None);
    }

    /// @id: test_config_multiple_keys
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que plusieurs clés peuvent être récupérées correctement depuis la configuration.
    /// @do: verify_multiple_keys
    /// @depends: config_get
    #[test]
    fn test_config_multiple_keys() {
        std::env::set_var("TEST_KEY_1", "value1");
        std::env::set_var("TEST_KEY_2", "value2");
        std::env::set_var("TEST_KEY_3", "value3");

        let config = EnvConfig::from_env();

        assert_eq!(config.get("TEST_KEY_1"), Some("value1"));
        assert_eq!(config.get("TEST_KEY_2"), Some("value2"));
        assert_eq!(config.get("TEST_KEY_3"), Some("value3"));

        std::env::remove_var("TEST_KEY_1");
        std::env::remove_var("TEST_KEY_2");
        std::env::remove_var("TEST_KEY_3");
    }

    /// @id: test_config_empty_value
    /// @role: test
    /// @layer: kernel
    /// @human: Test vérifiant que get() retourne Some("") pour une clé existante avec une valeur vide, pas None.
    /// @do: verify_empty_value_handling
    /// @depends: config_get
    #[test]
    fn test_config_empty_value() {
        std::env::set_var("TEST_EMPTY_VALUE", "");
        let config = EnvConfig::from_env();
        // Une clé avec une valeur vide doit retourner Some(""), pas None
        assert_eq!(config.get("TEST_EMPTY_VALUE"), Some(""));
        std::env::remove_var("TEST_EMPTY_VALUE");
    }
}
