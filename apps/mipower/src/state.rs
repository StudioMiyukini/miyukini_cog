//! État global de l'application MIPOWER.

use crate::models::SequenceMeta;
use dioxus::prelude::*;

// ── Navigation ───────────────────────────────────────────────────────────────

/// Vue active de l'application.
#[derive(Clone, PartialEq, Debug)]
pub enum View {
    Dashboard,
    Report {
        slug:      String,
        file_path: Option<String>,
        tab:       Option<String>,
    },
    /// MIPOWER Starter (ex Prompt Builder)
    Starter {
        prefill_title: Option<String>,
        prefill_desc:  Option<String>,
    },
    Settings,
}

// ── Contexte partagé ─────────────────────────────────────────────────────────

/// Contexte partagé par tous les composants.
#[derive(Clone)]
pub struct MipowerCtx {
    pub mip_root: Signal<String>,
}

impl MipowerCtx {
    pub fn new(mip_root: String) -> Self {
        Self { mip_root: Signal::new(mip_root) }
    }
}

// ── Notification ─────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
pub struct Notification {
    pub message: String,
    pub is_error: bool,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Charge les séquences depuis index.json dans mip_root.
pub fn load_sequences(mip_root: &str) -> Vec<SequenceMeta> {
    let candidates = [
        std::path::PathBuf::from(mip_root).join(".mip").join("sequences").join("index.json"),
        std::path::PathBuf::from(mip_root).join("sequences").join("index.json"),
    ];
    let Some(path) = candidates.into_iter().find(|p| p.exists()) else {
        return vec![];
    };
    let raw = std::fs::read_to_string(&path).unwrap_or_default();
    let raw = raw.trim_start_matches('\u{feff}');
    let Ok(index) = serde_json::from_str::<crate::models::SequencesIndex>(raw) else {
        return vec![];
    };
    index.sequences.into_iter().map(|e| {
        let mut meta = SequenceMeta::from(e);
        // Dériver statut depuis p6-trace.md si "active"
        if meta.status == "active" {
            let base = std::path::PathBuf::from(mip_root).join(".mip").join("sequences");
            let seq_dir = base.join(&meta.name);
            if seq_dir.exists() {
                meta.status = derive_status(&seq_dir);
            }
        }
        meta
    }).collect()
}

fn derive_status(seq_dir: &std::path::Path) -> String {
    let trace = seq_dir.join("phases").join("p6-trace.md");
    if let Ok(content) = std::fs::read_to_string(&trace) {
        if content.contains("SUCCES") { return "done".into(); }
        if content.contains("REFUSE") || content.contains("abandonne") { return "archived".into(); }
    }
    "active".into()
}
