//! Endpoints REST de présence MWS pour JayManga.
//!
//! Vérification de la disponibilité des COGs vendeurs,
//! requêtes unitaires et batch.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types de requête / réponse
// ---------------------------------------------------------------------------

/// Requête de vérification de présence batch.
#[derive(Debug, Clone, Deserialize)]
pub struct PresenceBatchRequest {
    pub cog_ids: Vec<String>,
}

/// Réponse de présence pour un COG.
#[derive(Debug, Clone, Serialize)]
pub struct PresenceResponse {
    pub cog_id: String,
    pub status: String,       // "online", "offline", "unknown"
    pub last_seen: Option<String>,
}

/// Réponse batch de présence.
#[derive(Debug, Clone, Serialize)]
pub struct PresenceBatchResponse {
    pub results: Vec<PresenceResponse>,
    pub checked_at: String,
}

/// COG découvert avec service JayManga.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoverResponse {
    pub cogs: Vec<DiscoverEntry>,
    pub total: i32,
}

/// Entrée d'un COG découvert.
#[derive(Debug, Clone, Serialize)]
pub struct DiscoverEntry {
    pub cog_id: String,
    pub shop_name: String,
    pub work_count: i32,
    pub online: bool,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_request_deserialize() {
        let json = r#"{"cog_ids": ["cog-1", "cog-2", "cog-3"]}"#;
        let req: PresenceBatchRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.cog_ids.len(), 3);
    }

    #[test]
    fn test_presence_response_serialize() {
        let resp = PresenceResponse {
            cog_id: "cog-1".to_string(),
            status: "online".to_string(),
            last_seen: Some("2026-02-25T10:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("online"));
    }
}
