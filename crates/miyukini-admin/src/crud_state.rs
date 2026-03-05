//! État partagé pour le CRUD tables / données (demo — à remplacer par KindMother en production).

use std::collections::HashMap;
use tokio::sync::RwLock;

/// État partagé des tables (nom -> lignes). Chaque ligne est un objet JSON.
#[derive(Debug, Default)]
pub struct CrudState {
    /// Tables : nom -> liste de lignes (chaque ligne = map colonne -> valeur).
    pub tables: RwLock<HashMap<String, Vec<serde_json::Value>>>,
}

impl CrudState {
    /// Crée l'état avec des tables de démo.
    #[must_use]
    pub fn with_demo_tables() -> Self {
        let tables = RwLock::new(Self::demo_tables());
        Self { tables }
    }

    fn demo_tables() -> HashMap<String, Vec<serde_json::Value>> {
        let mut m = HashMap::new();
        m.insert(
            "demo_users".to_string(),
            vec![
                serde_json::json!({"id": "1", "email": "admin@miyukini.local", "created_at": "2026-01-28T10:00:00Z"}),
                serde_json::json!({"id": "2", "email": "ops@miyukini.local", "created_at": "2026-01-29T09:00:00Z"}),
            ],
        );
        m.insert(
            "demo_audit".to_string(),
            vec![
                serde_json::json!({"id": "1", "action": "login", "actor": "admin", "at": "2026-01-29T08:00:00Z"}),
            ],
        );
        m
    }
}
