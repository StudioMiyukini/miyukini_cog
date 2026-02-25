//! Requêtes de présence MWS pour JayManga.
//!
//! Vérification de la disponibilité des COGs vendeurs
//! (unitaire et batch) via le participant MWS.

use crate::data::OnlineStatus;

// ---------------------------------------------------------------------------
// Erreurs
// ---------------------------------------------------------------------------

/// Erreur du module présence MWS.
#[derive(Debug, thiserror::Error)]
pub enum MwsPresenceError {
    #[error("MWS connection error: {0}")]
    Connection(String),
    #[error("Timeout querying presence for {0}")]
    Timeout(String),
}

// ---------------------------------------------------------------------------
// Types de résultat
// ---------------------------------------------------------------------------

/// Résultat d'une requête de présence pour un COG.
#[derive(Debug, Clone)]
pub struct PresenceResult {
    pub cog_id: String,
    pub status: OnlineStatus,
    pub last_seen: Option<String>,
}

// ---------------------------------------------------------------------------
// Fonctions de présence
// ---------------------------------------------------------------------------

/// Convertit un statut de présence brut (depuis le MWS) en `OnlineStatus`.
#[must_use]
pub fn presence_parse_status(raw: &str) -> OnlineStatus {
    match raw {
        "online" | "connected" | "active" => OnlineStatus::Online,
        "offline" | "disconnected" => OnlineStatus::Offline,
        _ => OnlineStatus::Unknown,
    }
}

/// Détermine le statut d'affichage (texte + couleur) pour l'UI.
#[must_use]
pub fn presence_display_info(status: &OnlineStatus) -> (&'static str, &'static str) {
    match status {
        OnlineStatus::Online => ("En ligne", "green"),
        OnlineStatus::Offline => ("Hors ligne", "gray"),
        OnlineStatus::Unknown => ("Inconnu", "white"),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_status() {
        assert_eq!(presence_parse_status("online"), OnlineStatus::Online);
        assert_eq!(presence_parse_status("connected"), OnlineStatus::Online);
        assert_eq!(presence_parse_status("offline"), OnlineStatus::Offline);
        assert_eq!(presence_parse_status("garbage"), OnlineStatus::Unknown);
    }

    #[test]
    fn test_display_info() {
        let (text, color) = presence_display_info(&OnlineStatus::Online);
        assert_eq!(text, "En ligne");
        assert_eq!(color, "green");
    }
}
