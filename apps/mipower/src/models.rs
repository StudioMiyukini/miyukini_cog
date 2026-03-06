use serde::{Deserialize, Serialize};

/// Entree dans sequences/index.json (format MIP index v2)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name:           String,
    pub date:           String,
    pub slug:           String,
    #[serde(rename = "type")]
    pub seq_type:       Option<String>,
    pub status:         Option<String>,
    pub current_phase:  Option<String>,
    pub brief_path:     Option<String>,
    pub ui_path:        Option<String>,
    pub metrics_path:   Option<String>,
    pub security_score: Option<i64>,
}

/// Format retourne par /api/sequences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMeta {
    pub slug:          String,
    pub date:          String,
    pub name:          String,
    pub status:        String,
    pub task_class:    Option<String>,
    pub complexity:    Option<String>,
    pub current_phase: Option<String>,
    pub brief_path:    Option<String>,
    pub path:          String,
}

impl From<IndexEntry> for SequenceMeta {
    fn from(e: IndexEntry) -> Self {
        let path = format!(".mip/sequences/{}", e.name);
        SequenceMeta {
            slug:          e.slug,
            date:          e.date,
            name:          e.name.clone(),
            status:        e.status.filter(|s| !s.is_empty()).unwrap_or_else(|| "active".into()),
            task_class:    e.seq_type.filter(|s| s != "UNKNOWN" && !s.is_empty()),
            complexity:    None,
            current_phase: e.current_phase.filter(|s| !s.is_empty()),
            brief_path:    e.brief_path,
            path,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactContent {
    pub path:    String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub slug:   String,
    pub phases: Vec<PhaseProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseProgress {
    pub phase: String,
    pub done:  usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptBuilderInput {
    pub title:       String,
    pub task_class:  String,
    pub domain:      String,
    pub description: String,
    pub constraints: Option<String>,
    pub stack:       Option<String>,
    pub tags:        Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencesIndex {
    pub sequences: Vec<IndexEntry>,
}
