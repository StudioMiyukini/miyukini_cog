//! Registre des slugs de sous-domaines COG.
//!
//! Associe un slug (sous-domaine) à un `cog_id` pour le routage
//! `xxx.miyukini.com` → COG correspondant.
//!
//! - Validation : `[a-z0-9-]`, 3-32 caractères, pas de slugs réservés.
//! - Unicité : un slug ne peut être attribué qu'à un seul COG.
//! - Fallback : si aucun slug personnalisé, le `cog_id` est sanitisé en slug.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Slugs réservés (sous-domaines système, ne peuvent pas être pris par un COG).
const RESERVED_SLUGS: &[&str] = &[
    "www",
    "api",
    "admin",
    "origin",
    "relay",
    "tracker",
    "mail",
    "smtp",
    "imap",
    "ftp",
    "ssh",
    "ns1",
    "ns2",
    "test",
    "dev",
    "staging",
    "cdn",
    "static",
    "docs",
    "blog",
    "status",
    "miyukini",
    "webway",
    "mws",
];

/// Erreurs du registre de slugs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SlugError {
    /// Le slug est invalide (caractères interdits, longueur, etc.).
    InvalidFormat(String),
    /// Le slug est réservé (sous-domaine système).
    Reserved(String),
    /// Le slug est déjà pris par un autre COG.
    AlreadyTaken { slug: String, owner: String },
}

impl std::fmt::Display for SlugError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFormat(msg) => write!(f, "invalid slug format: {}", msg),
            Self::Reserved(slug) => write!(f, "slug '{}' is reserved", slug),
            Self::AlreadyTaken { slug, owner } => {
                write!(f, "slug '{}' is already taken by {}", slug, owner)
            }
        }
    }
}

/// Registre des slugs de sous-domaines.
///
/// Thread-safe via `Arc<RwLock<...>>`.
#[derive(Debug, Clone)]
pub struct SlugRegistry {
    /// Mapping slug → cog_id.
    slug_to_cog: Arc<RwLock<HashMap<String, String>>>,
    /// Mapping inverse cog_id → slug (pour nettoyage rapide).
    cog_to_slug: Arc<RwLock<HashMap<String, String>>>,
}

impl SlugRegistry {
    /// Crée un nouveau registre vide.
    #[must_use]
    pub fn new() -> Self {
        Self {
            slug_to_cog: Arc::new(RwLock::new(HashMap::new())),
            cog_to_slug: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Valide le format d'un slug.
    ///
    /// Règles :
    /// - 3 à 32 caractères
    /// - Uniquement `[a-z0-9-]`
    /// - Ne commence ni ne finit par `-`
    /// - Pas de `--` consécutifs
    pub fn validate_slug(slug: &str) -> Result<(), SlugError> {
        if slug.len() < 3 {
            return Err(SlugError::InvalidFormat(
                "slug must be at least 3 characters".to_string(),
            ));
        }
        if slug.len() > 32 {
            return Err(SlugError::InvalidFormat(
                "slug must be at most 32 characters".to_string(),
            ));
        }
        if !slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        {
            return Err(SlugError::InvalidFormat(
                "slug must contain only lowercase letters, digits, and hyphens".to_string(),
            ));
        }
        if slug.starts_with('-') || slug.ends_with('-') {
            return Err(SlugError::InvalidFormat(
                "slug must not start or end with a hyphen".to_string(),
            ));
        }
        if slug.contains("--") {
            return Err(SlugError::InvalidFormat(
                "slug must not contain consecutive hyphens".to_string(),
            ));
        }

        if RESERVED_SLUGS.contains(&slug) {
            return Err(SlugError::Reserved(slug.to_string()));
        }

        Ok(())
    }

    /// Sanitise un `cog_id` en slug valide (fallback si pas de slug personnalisé).
    ///
    /// - Met en minuscules
    /// - Remplace les caractères non valides par `-`
    /// - Tronque à 32 caractères
    /// - Supprime les `-` en début/fin
    #[must_use]
    pub fn sanitize_cog_id(cog_id: &str) -> String {
        let slug: String = cog_id
            .to_lowercase()
            .chars()
            .map(|c| {
                if c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' {
                    c
                } else {
                    '-'
                }
            })
            .collect();

        // Supprimer les doubles tirets
        let mut result = String::with_capacity(slug.len());
        let mut prev_dash = false;
        for c in slug.chars() {
            if c == '-' {
                if !prev_dash {
                    result.push(c);
                }
                prev_dash = true;
            } else {
                result.push(c);
                prev_dash = false;
            }
        }

        // Tronquer et nettoyer
        let result = result.trim_matches('-');
        let result = if result.len() > 32 {
            &result[..32]
        } else {
            result
        };
        let result = result.trim_end_matches('-');

        // Si le résultat est trop court, ajouter un suffixe
        if result.len() < 3 {
            format!("cog-{}", &cog_id.chars().take(8).collect::<String>())
                .to_lowercase()
                .chars()
                .filter(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == '-')
                .collect::<String>()
        } else {
            result.to_string()
        }
    }

    /// Enregistre un slug pour un COG.
    ///
    /// Si `requested_slug` est `None`, un slug est dérivé du `cog_id`.
    /// Retourne le slug effectivement attribué, ou une erreur.
    pub async fn register(
        &self,
        cog_id: &str,
        requested_slug: Option<&str>,
    ) -> Result<String, SlugError> {
        // Retirer l'ancien slug de ce COG s'il en avait un
        self.unregister(cog_id).await;

        let slug = match requested_slug {
            Some(s) => {
                let s = s.to_lowercase();
                Self::validate_slug(&s)?;
                s
            }
            None => {
                let s = Self::sanitize_cog_id(cog_id);
                // Le slug auto-généré peut être réservé, ajouter un suffixe
                if RESERVED_SLUGS.contains(&s.as_str()) || Self::validate_slug(&s).is_err() {
                    format!("{}-cog", &s[..s.len().min(28)])
                } else {
                    s
                }
            }
        };

        // Vérifier l'unicité
        {
            let slugs = self.slug_to_cog.read().await;
            if let Some(existing_owner) = slugs.get(&slug) {
                if existing_owner != cog_id {
                    return Err(SlugError::AlreadyTaken {
                        slug,
                        owner: existing_owner.clone(),
                    });
                }
            }
        }

        // Enregistrer
        {
            let mut slugs = self.slug_to_cog.write().await;
            let mut cogs = self.cog_to_slug.write().await;
            slugs.insert(slug.clone(), cog_id.to_string());
            cogs.insert(cog_id.to_string(), slug.clone());
        }

        info!(
            "Slug '{}' registered for COG '{}'",
            slug, cog_id
        );

        Ok(slug)
    }

    /// Désenregistre le slug d'un COG.
    pub async fn unregister(&self, cog_id: &str) {
        let old_slug = {
            let cogs = self.cog_to_slug.read().await;
            cogs.get(cog_id).cloned()
        };

        if let Some(slug) = old_slug {
            let mut slugs = self.slug_to_cog.write().await;
            let mut cogs = self.cog_to_slug.write().await;
            slugs.remove(&slug);
            cogs.remove(cog_id);
            debug!("Slug '{}' unregistered (COG '{}')", slug, cog_id);
        }
    }

    /// Recherche le `cog_id` associé à un slug.
    pub async fn lookup(&self, slug: &str) -> Option<String> {
        let slugs = self.slug_to_cog.read().await;
        slugs.get(slug).cloned()
    }

    /// Recherche le slug associé à un `cog_id`.
    pub async fn get_slug(&self, cog_id: &str) -> Option<String> {
        let cogs = self.cog_to_slug.read().await;
        cogs.get(cog_id).cloned()
    }

    /// Nombre de slugs enregistrés.
    pub async fn count(&self) -> usize {
        let slugs = self.slug_to_cog.read().await;
        slugs.len()
    }
}

impl Default for SlugRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_slug_valid() {
        assert!(SlugRegistry::validate_slug("mon-cog").is_ok());
        assert!(SlugRegistry::validate_slug("abc").is_ok());
        assert!(SlugRegistry::validate_slug("boutique-alice-42").is_ok());
        assert!(SlugRegistry::validate_slug("a1b").is_ok());
    }

    #[test]
    fn test_validate_slug_too_short() {
        assert!(matches!(
            SlugRegistry::validate_slug("ab"),
            Err(SlugError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_validate_slug_too_long() {
        let long = "a".repeat(33);
        assert!(matches!(
            SlugRegistry::validate_slug(&long),
            Err(SlugError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_validate_slug_invalid_chars() {
        assert!(matches!(
            SlugRegistry::validate_slug("Mon-COG"),
            Err(SlugError::InvalidFormat(_))
        ));
        assert!(matches!(
            SlugRegistry::validate_slug("mon_cog"),
            Err(SlugError::InvalidFormat(_))
        ));
        assert!(matches!(
            SlugRegistry::validate_slug("mon.cog"),
            Err(SlugError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_validate_slug_leading_trailing_hyphen() {
        assert!(matches!(
            SlugRegistry::validate_slug("-abc"),
            Err(SlugError::InvalidFormat(_))
        ));
        assert!(matches!(
            SlugRegistry::validate_slug("abc-"),
            Err(SlugError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_validate_slug_consecutive_hyphens() {
        assert!(matches!(
            SlugRegistry::validate_slug("ab--cd"),
            Err(SlugError::InvalidFormat(_))
        ));
    }

    #[test]
    fn test_validate_slug_reserved() {
        assert!(matches!(
            SlugRegistry::validate_slug("www"),
            Err(SlugError::Reserved(_))
        ));
        assert!(matches!(
            SlugRegistry::validate_slug("admin"),
            Err(SlugError::Reserved(_))
        ));
        assert!(matches!(
            SlugRegistry::validate_slug("origin"),
            Err(SlugError::Reserved(_))
        ));
    }

    #[test]
    fn test_sanitize_cog_id() {
        assert_eq!(SlugRegistry::sanitize_cog_id("Mon-PC"), "mon-pc");
        assert_eq!(
            SlugRegistry::sanitize_cog_id("Alice's_COG"),
            "alice-s-cog"
        );
        assert_eq!(
            SlugRegistry::sanitize_cog_id("test.machine.local"),
            "test-machine-local"
        );
    }

    #[tokio::test]
    async fn test_register_and_lookup() {
        let registry = SlugRegistry::new();
        let slug = registry.register("cog-001", Some("boutique-alice")).await;
        assert_eq!(slug, Ok("boutique-alice".to_string()));
        assert_eq!(
            registry.lookup("boutique-alice").await,
            Some("cog-001".to_string())
        );
    }

    #[tokio::test]
    async fn test_register_auto_slug() {
        let registry = SlugRegistry::new();
        let slug = registry.register("Mon-Super-COG", None).await;
        assert!(slug.is_ok());
        let slug = slug.unwrap();
        assert_eq!(registry.lookup(&slug).await, Some("Mon-Super-COG".to_string()));
    }

    #[tokio::test]
    async fn test_register_duplicate_slug() {
        let registry = SlugRegistry::new();
        registry
            .register("cog-001", Some("boutique"))
            .await
            .unwrap();
        let result = registry.register("cog-002", Some("boutique")).await;
        assert!(matches!(result, Err(SlugError::AlreadyTaken { .. })));
    }

    #[tokio::test]
    async fn test_unregister() {
        let registry = SlugRegistry::new();
        registry
            .register("cog-001", Some("boutique"))
            .await
            .unwrap();
        registry.unregister("cog-001").await;
        assert_eq!(registry.lookup("boutique").await, None);
    }

    #[tokio::test]
    async fn test_re_register_replaces_old_slug() {
        let registry = SlugRegistry::new();
        registry
            .register("cog-001", Some("old-slug"))
            .await
            .unwrap();
        let new = registry
            .register("cog-001", Some("new-slug"))
            .await
            .unwrap();
        assert_eq!(new, "new-slug");
        assert_eq!(registry.lookup("old-slug").await, None);
        assert_eq!(
            registry.lookup("new-slug").await,
            Some("cog-001".to_string())
        );
    }
}
