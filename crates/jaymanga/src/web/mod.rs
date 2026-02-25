//! Module web pour le Portail JayManga.
//!
//! Templates HTML (SSR) et configuration du serveur web Portail.
//! Inclut le portail vendeur (catalogue, liseuse, panier)
//! et le Portail Agrégé (vue inter-COG unifiée).

pub mod portal;
pub mod aggregator_portal;

// ---------------------------------------------------------------------------
// Configuration
// ---------------------------------------------------------------------------

/// Routes du portail web JayManga.
pub const PORTAL_PREFIX: &str = "/manga";

/// Liste des routes du portail.
#[must_use]
pub fn portal_routes() -> Vec<(&'static str, &'static str)> {
    vec![
        ("/manga", "Catalogue public"),
        ("/manga/{work_id}", "Fiche oeuvre"),
        ("/manga/read/{chapter_id}", "Liseuse web"),
        ("/manga/cart", "Panier"),
        ("/manga/checkout", "Checkout"),
        ("/manga/aggregate", "Portail Agrégé"),
        ("/manga/aggregate/seller/{cog_id}", "Page vendeur"),
    ]
}
