//! Endpoints REST du lecteur JayManga.
//!
//! Sauvegarde et récupération de la progression de lecture,
//! report d'événements XP.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Types de requête / réponse
// ---------------------------------------------------------------------------

/// Requête de sauvegarde de progression.
#[derive(Debug, Clone, Deserialize)]
pub struct SaveProgressRequest {
    pub work_id: String,
    pub chapter_id: String,
    pub page_number: i32,
    pub chapter_number: i32,
    pub total_pages_in_chapter: i32,
    pub read_duration_secs: Option<f64>,
}

/// Réponse de progression sauvegardée.
#[derive(Debug, Clone, Serialize)]
pub struct SaveProgressResponse {
    pub saved: bool,
    pub chapter_progress: f64,
    pub xp_events: Vec<XpEventResponse>,
}

/// Événement XP dans la réponse API.
#[derive(Debug, Clone, Serialize)]
pub struct XpEventResponse {
    pub xp_gained: i64,
    pub total_xp: i64,
    pub source: String,
    pub new_level: Option<LevelUpResponse>,
}

/// Notification de montée de niveau.
#[derive(Debug, Clone, Serialize)]
pub struct LevelUpResponse {
    pub level: i32,
    pub name: String,
}

/// Requête de progression.
#[derive(Debug, Clone, Deserialize)]
pub struct GetProgressQuery {
    pub work_id: String,
}

/// Réponse de progression.
#[derive(Debug, Clone, Serialize)]
pub struct ProgressResponse {
    pub work_id: String,
    pub last_chapter: Option<i32>,
    pub last_page: Option<i32>,
    pub progress_percent: f64,
}

/// Requête de report XP.
#[derive(Debug, Clone, Deserialize)]
pub struct ReportXpRequest {
    pub event_type: String, // "page_read", "chapter_complete", "work_complete"
    pub context: Option<String>,
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_progress_request_deserialize() {
        let json = r#"{
            "work_id": "w-1",
            "chapter_id": "ch-1",
            "page_number": 15,
            "chapter_number": 2,
            "total_pages_in_chapter": 30,
            "read_duration_secs": 4.5
        }"#;
        let req: SaveProgressRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.page_number, 15);
        assert_eq!(req.chapter_number, 2);
    }

    #[test]
    fn test_progress_response_serialize() {
        let resp = ProgressResponse {
            work_id: "w-1".to_string(),
            last_chapter: Some(3),
            last_page: Some(12),
            progress_percent: 0.45,
        };
        let json = serde_json::to_string(&resp).unwrap();
        assert!(json.contains("0.45"));
    }
}
