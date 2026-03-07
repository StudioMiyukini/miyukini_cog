//! Contrat d'exposition Portal pour JayFestival.
//!
//! Implémente PortalContract pour exposer les pages publiques JayFestival
//! (éditions publiées, organisateurs, exposants) via le COG Web Portal.
//!
//! @id: jayfestival_portal_contract @do: implement_portal_contract
//! @role: api @layer: service
//! @human: JayFestivalPortalService — implémente PortalContract, expose pages publiques via Portal.

use cog_portal_contract::{PortalContract, PortalError, PublicPage};
use std::sync::Arc;

use crate::data::JayFestivalDb;

/// Service Portal JayFestival — fournit les pages publiques au COG Web Portal.
pub struct JayFestivalPortalService {
    db: Arc<JayFestivalDb>,
}

impl JayFestivalPortalService {
    /// Crée une instance du service Portal JayFestival.
    pub fn new(db: Arc<JayFestivalDb>) -> Self {
        Self { db }
    }
}

impl PortalContract for JayFestivalPortalService {
    fn service_slug(&self) -> &str {
        "jayfestival"
    }

    fn service_name(&self) -> &str {
        "JayFestival"
    }

    fn public_pages(&self) -> Result<Vec<PublicPage>, PortalError> {
        let editions = self
            .db
            .editions_published()
            .map_err(|e| PortalError::Db(e.to_string()))?;

        let editions_html = if editions.is_empty() {
            "<p>Aucune édition en cours.</p>".to_string()
        } else {
            let items: String = editions
                .iter()
                .map(|e| {
                    let name = e.name.as_deref().unwrap_or("Édition");
                    let dates = match (e.start_date.as_deref(), e.end_date.as_deref()) {
                        (Some(s), Some(e)) => format!("{s} → {e}"),
                        (Some(s), None) => s.to_string(),
                        _ => "dates à confirmer".to_string(),
                    };
                    let location = e.location.as_deref().unwrap_or("");
                    format!(
                        "<article class=\"jf-edition\"><h3>{name}</h3><p>{dates}</p><p>{location}</p></article>"
                    )
                })
                .collect();
            format!("<section class=\"jf-editions\">{items}</section>")
        };

        let exposants = self
            .db
            .exposants_public_list()
            .map_err(|e| PortalError::Db(e.to_string()))?;

        let exposants_html = if exposants.is_empty() {
            "<p>Aucun exposant dans le répertoire.</p>".to_string()
        } else {
            let items: String = exposants
                .iter()
                .map(|ex| {
                    let name = ex.company_name.as_deref().unwrap_or("Exposant");
                    let secteur = ex.secteur.as_deref().unwrap_or("");
                    format!("<li class=\"jf-exposant\"><strong>{name}</strong> — {secteur}</li>")
                })
                .collect();
            format!("<ul class=\"jf-exposants\">{items}</ul>")
        };

        Ok(vec![
            PublicPage {
                slug: "editions".to_string(),
                title: "Éditions JayFestival".to_string(),
                description: Some("Découvrez les festivals et événements organisés via JayFestival.".to_string()),
                html_content: editions_html,
                is_public: true,
                display_order: 0,
            },
            PublicPage {
                slug: "exposants".to_string(),
                title: "Répertoire des Exposants".to_string(),
                description: Some("Annuaire des exposants participants aux éditions JayFestival.".to_string()),
                html_content: exposants_html,
                is_public: true,
                display_order: 1,
            },
        ])
    }

    fn page_by_slug(&self, slug: &str) -> Result<Option<PublicPage>, PortalError> {
        let pages = self.public_pages()?;
        Ok(pages.into_iter().find(|p| p.slug == slug))
    }
}
