use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceMeta {
    pub slug: String,
    pub date: String,
    pub status: String,
    pub task_class: Option<String>,
    pub complexity: Option<String>,
    pub path: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtefactContent {
    pub path: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgressInfo {
    pub slug: String,
    pub phases: Vec<PhaseProgress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseProgress {
    pub phase: String,
    pub done: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptBuilderInput {
    pub title: String,
    pub task_class: String,
    pub domain: String,
    pub description: String,
    pub constraints: Option<String>,
    pub stack: Option<String>,
    pub tags: Vec<String>,
}
