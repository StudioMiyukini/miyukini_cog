//! Adaptateur de synchronisation catalogue JayManga → JayXpose.
//!
//! Convertit les oeuvres publiées en entrées de vitrine JayXpose
//! pour l'affichage sur la surface web du COG vendeur.

use super::contract::XposeEntry;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur de l'adaptateur JayXpose.
#[derive(Debug, thiserror::Error)]
pub enum JayXposeAdapterError {
    #[error("Sync error: {0}")]
    Sync(String),
    #[error("JayXpose not available")]
    NotAvailable,
}

// ---------------------------------------------------------------------------
// Conversion
// ---------------------------------------------------------------------------

/// Convertit une oeuvre JayManga en entrée JayXpose.
#[must_use]
pub fn xpose_entry_from_work(
    work_id: &str,
    title: &str,
    synopsis: Option<&str>,
    cover_path: Option<&str>,
    price: i64,
    currency: &str,
    portal_url: &str,
) -> XposeEntry {
    XposeEntry {
        source_service: "jaymanga".to_string(),
        source_id: work_id.to_string(),
        title: title.to_string(),
        description: synopsis.map(ToString::to_string),
        image_url: cover_path.map(ToString::to_string),
        price_cents: price,
        currency: currency.to_string(),
        action_url: portal_url.to_string(),
        action_label: "Lire".to_string(),
        category: "manga".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_from_work() {
        let entry = xpose_entry_from_work(
            "work-123",
            "One Piece Vol.1",
            Some("Un manga d'aventure"),
            Some("/covers/op.jpg"),
            799,
            "EUR",
            "/manga/work-123",
        );
        assert_eq!(entry.source_service, "jaymanga");
        assert_eq!(entry.source_id, "work-123");
        assert_eq!(entry.price_cents, 799);
        assert_eq!(entry.action_label, "Lire");
        assert_eq!(entry.category, "manga");
    }
}
