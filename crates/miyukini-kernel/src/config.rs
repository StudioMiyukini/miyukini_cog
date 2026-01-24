//! Mécanismes de chargement et d'accès à la configuration.
//! Le kernel ne définit aucune politique (noms de variables, structure métier).

use std::collections::HashMap;

/// Contrat : chargement et accès à des paires clé/valeur.
/// Le produit décide des clés et des valeurs.
pub trait Config {
    /// Retourne la valeur associée à la clé, si présente.
    fn get(&self, key: &str) -> Option<&str>;
}

/// Implémentation par défaut : charge depuis les variables d'environnement.
/// Minimal et remplaçable.
#[derive(Debug)]
pub struct EnvConfig {
    map: HashMap<String, String>,
}

impl EnvConfig {
    /// Construit une config à partir de `std::env::vars()`.
    pub fn from_env() -> Self {
        EnvConfig {
            map: std::env::vars().collect(),
        }
    }
}

impl Config for EnvConfig {
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key).map(|s| s.as_str())
    }
}
