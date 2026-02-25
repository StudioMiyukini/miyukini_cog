//! Endpoints REST pour le Portail web JayManga.
//!
//! Routes publiques (catalogue, media, liseuse, fédération)
//! et routes authentifiées (panier, checkout, progression, présence).

pub mod catalog_api;
pub mod reader_api;
pub mod payment_api;
pub mod media_api;
pub mod federation_api;
pub mod presence_api;

// ---------------------------------------------------------------------------
// Router principal
// ---------------------------------------------------------------------------

/// Préfixe de toutes les routes API JayManga.
pub const API_PREFIX: &str = "/api/jaymanga";

/// Liste des routes enregistrées (pour documentation / healthcheck).
#[must_use]
pub fn api_routes_list() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        // Catalogue
        ("GET", "/api/jaymanga/catalog", "Liste des oeuvres publiées"),
        ("GET", "/api/jaymanga/catalog/works/{work_id}", "Fiche oeuvre"),
        ("GET", "/api/jaymanga/catalog/series/{series_id}", "Série avec volumes"),
        ("GET", "/api/jaymanga/catalog/search", "Recherche textuelle"),
        // Media
        ("GET", "/api/jaymanga/media/page/{page_id}", "Image de page (variante auto)"),
        ("GET", "/api/jaymanga/media/page/{page_id}/{profile}", "Variante spécifique"),
        ("GET", "/api/jaymanga/media/cover/{work_id}", "Couverture"),
        // Lecteur
        ("POST", "/api/jaymanga/reader/progress", "Sauvegarder progression"),
        ("GET", "/api/jaymanga/reader/progress/{work_id}", "Récupérer progression"),
        ("POST", "/api/jaymanga/reader/xp", "Reporter événement XP"),
        // Paiement
        ("POST", "/api/jaymanga/cart/add", "Ajouter au panier"),
        ("GET", "/api/jaymanga/cart", "Contenu du panier"),
        ("POST", "/api/jaymanga/checkout", "Initier checkout"),
        ("POST", "/api/jaymanga/webhook/{gateway}", "Callback passerelle"),
        ("GET", "/api/jaymanga/license/verify/{content_id}", "Vérifier licence"),
        // Fédération
        ("GET", "/api/jaymanga/federation/info", "Infos vendeur"),
        ("GET", "/api/jaymanga/federation/catalog", "Catalogue fédéré"),
        ("GET", "/api/jaymanga/federation/catalog/since/{ts}", "Delta catalogue"),
        // Présence
        ("GET", "/api/jaymanga/presence/{cog_id}", "Statut présence"),
        ("POST", "/api/jaymanga/presence/batch", "Présence batch"),
        ("GET", "/api/jaymanga/discover", "COGs JayManga connus"),
    ]
}
