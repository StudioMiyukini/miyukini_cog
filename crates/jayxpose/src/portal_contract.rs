//! Contrat d'exposition Portal pour JayXpose.
//!
//! Implémente PortalContract pour exposer les pages publiques JayXpose
//! (annuaire exposants, catalogue public) via le COG Web Portal.
//!
//! @id: jayxpose_portal_contract @do: implement_portal_contract
//! @role: api @layer: service
//! @human: JayXposePortalService — implémente PortalContract, expose annuaire et catalogue publics via Portal.

use cog_portal_contract::{PortalContract, PortalError, PublicPage};
use std::sync::Arc;

use crate::data::JayXposeDb;

/// Service Portal JayXpose — fournit les pages publiques au COG Web Portal.
pub struct JayXposePortalService {
    db: Arc<JayXposeDb>,
}

impl JayXposePortalService {
    /// Crée une instance du service Portal JayXpose.
    pub fn new(db: Arc<JayXposeDb>) -> Self {
        Self { db }
    }
}

impl PortalContract for JayXposePortalService {
    fn service_slug(&self) -> &str {
        "jayxpose"
    }

    fn service_name(&self) -> &str {
        "JayXpose"
    }

    fn public_pages(&self) -> Result<Vec<PublicPage>, PortalError> {
        let exposants = self
            .db
            .exposants_list_annuaire()
            .map_err(|e| PortalError::Db(e.to_string()))?;

        let annuaire_html = if exposants.is_empty() {
            "<p>Aucun exposant dans l'annuaire pour le moment.</p>".to_string()
        } else {
            let items: String = exposants
                .iter()
                .map(|ex| {
                    let name = ex.company_name.as_deref().unwrap_or("Exposant");
                    let desc = ex.description_short.as_deref().unwrap_or("");
                    let secteur = ex.secteur.as_deref().unwrap_or("");
                    let slug = ex.vitrine_slug.as_deref().unwrap_or("");
                    format!(
                        "<article class=\"jx-exposant\" data-slug=\"{slug}\">\
                         <h3>{name}</h3>\
                         <p class=\"secteur\">{secteur}</p>\
                         <p>{desc}</p>\
                         </article>"
                    )
                })
                .collect();
            format!("<section class=\"jx-annuaire\">{items}</section>")
        };

        Ok(vec![PublicPage {
            slug: "annuaire".to_string(),
            title: "Annuaire JayXpose".to_string(),
            description: Some("Découvrez les exposants et leurs catalogues.".to_string()),
            html_content: annuaire_html,
            is_public: true,
            display_order: 0,
        }])
    }

    fn page_by_slug(&self, slug: &str) -> Result<Option<PublicPage>, PortalError> {
        // Check annuaire first
        let base_pages = self.public_pages()?;
        if let Some(p) = base_pages.into_iter().find(|p| p.slug == slug) {
            return Ok(Some(p));
        }

        // Try vitrine by slug (individual exposant page)
        let exposant = self
            .db
            .exposant_by_vitrine_slug(slug)
            .map_err(|e| PortalError::Db(e.to_string()))?;

        let Some(ex) = exposant else {
            return Ok(None);
        };

        let name = ex.company_name.as_deref().unwrap_or("Exposant");
        let desc = ex.description_short.as_deref().unwrap_or("");
        let html = format!(
            "<article class=\"jx-vitrine\"><h2>{name}</h2><p>{desc}</p></article>"
        );

        Ok(Some(PublicPage {
            slug: slug.to_string(),
            title: name.to_string(),
            description: Some(desc.to_string()),
            html_content: html,
            is_public: true,
            display_order: 100,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::JayXposeDb;

    fn make_service() -> JayXposePortalService {
        let db = JayXposeDb::open(":memory:").expect("db in-memory");
        JayXposePortalService::new(Arc::new(db))
    }

    #[test]
    fn public_pages_returns_annuaire() {
        let svc = make_service();
        let pages = svc.public_pages().expect("public_pages ok");
        let slugs: Vec<&str> = pages.iter().map(|p| p.slug.as_str()).collect();
        assert!(slugs.contains(&"annuaire"), "slug annuaire absent: {slugs:?}");
    }

    #[test]
    fn service_slug_and_name() {
        let svc = make_service();
        assert_eq!(svc.service_slug(), "jayxpose");
        assert_eq!(svc.service_name(), "JayXpose");
    }

    #[test]
    fn page_by_slug_annuaire_found() {
        let svc = make_service();
        let page = svc.page_by_slug("annuaire").expect("no error");
        assert!(page.is_some());
        assert_eq!(page.unwrap().slug, "annuaire");
    }

    #[test]
    fn page_by_slug_unknown_returns_none() {
        let svc = make_service();
        let page = svc.page_by_slug("does-not-exist").expect("no error");
        assert!(page.is_none());
    }
}
