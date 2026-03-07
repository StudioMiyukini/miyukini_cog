//! Contrat d'exposition Portal — PortalContract trait + types partagés.
//!
//! Ce crate définit l'interface que tout service Jay doit implémenter
//! pour être exposé via le COG Web Portal.
//!
//! @id: cog_portal_contract_lib @do: define_portal_contract_interface
//! @role: api @layer: service
//! @human: PortalContract trait + PublicPage + PortalError — contrat d'exposition Portal partagé.

use serde::{Deserialize, Serialize};

/// Erreur d'exposition Portal.
#[derive(Debug, thiserror::Error)]
pub enum PortalError {
    /// Erreur de base de données.
    #[error("Database error: {0}")]
    Db(String),
    /// Page introuvable.
    #[error("Page not found: {slug}")]
    NotFound { slug: String },
    /// Données invalides.
    #[error("Invalid data: {0}")]
    Invalid(String),
}

/// Page publique exposée par un service Jay via le Portal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PublicPage {
    /// Slug unique de la page (ex: `editions`, `exposants`).
    pub slug: String,
    /// Titre affiché dans le Portal.
    pub title: String,
    /// Description courte (pour meta + listing).
    pub description: Option<String>,
    /// Contenu HTML inline (rendu côté Portal).
    pub html_content: String,
    /// La page est-elle accessible sans auth ?
    pub is_public: bool,
    /// Priorité d'affichage dans le menu (0 = premier).
    pub display_order: u32,
}

/// Contrat d'exposition Portal — tout service Jay l'implémente.
///
/// Le COG Web Portal utilise ce trait via `Arc<dyn PortalContract>` pour
/// déléguer le rendu des pages de chaque service.
pub trait PortalContract: Send + Sync {
    /// Slug du service (ex: `jayfestival`, `jayxpose`).
    fn service_slug(&self) -> &'static str;
    /// Nom affiché du service (ex: `JayFestival`).
    fn service_name(&self) -> &'static str;
    /// Liste toutes les pages publiques du service.
    fn public_pages(&self) -> Result<Vec<PublicPage>, PortalError>;
    /// Retourne une page par son slug.
    fn page_by_slug(&self, slug: &str) -> Result<Option<PublicPage>, PortalError>;
}
