//! Types de contrat pour l'intégration JayXpose.
//!
//! Définit les structures d'échange entre JayManga et JayXpose.

use serde::{Deserialize, Serialize};

/// Entrée de vitrine JayXpose provenant de JayManga.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XposeEntry {
    pub source_service: String,
    pub source_id: String,
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub price_cents: i64,
    pub currency: String,
    pub action_url: String,
    pub action_label: String,
    pub category: String,
}

/// Résultat de synchronisation avec JayXpose.
#[derive(Debug, Clone, Default)]
pub struct XposeSyncResult {
    pub entries_synced: i32,
    pub entries_removed: i32,
    pub errors: Vec<String>,
}
