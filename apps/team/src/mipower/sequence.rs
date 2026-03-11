//! Parsing des séquences MIPOWER depuis `.mip/sequences/`.
//!
//! Chaque séquence est un dossier contenant :
//! - Un `brief.md` décrivant les objectifs
//! - Un `index.json` listant les étapes
//! - Des fichiers `eN.md` pour chaque étape

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Entrée dans l'index.json d'une séquence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceStep {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub depends_on: Vec<String>,
    #[serde(default)]
    pub agent_id: Option<String>,
    #[serde(default)]
    pub model_pattern: Option<String>,
    #[serde(default)]
    pub min_tier: Option<u8>,
}

/// Séquence MIPOWER complète.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiPowerSequence {
    pub name: String,
    pub path: PathBuf,
    pub steps: Vec<SequenceStep>,
    pub brief: String,
}

/// Erreur de parsing de séquence.
#[derive(Debug)]
pub enum SequenceError {
    IoError(String),
    ParseError(String),
    StepFileNotFound(String),
}

impl std::fmt::Display for SequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(e) => write!(f, "I/O : {e}"),
            Self::ParseError(e) => write!(f, "Parse : {e}"),
            Self::StepFileNotFound(s) => write!(f, "Étape introuvable : {s}"),
        }
    }
}

/// Charge toutes les séquences depuis un dossier `.mip/sequences/`.
pub fn load_sequences(sequences_dir: &Path) -> Vec<MiPowerSequence> {
    let mut sequences = Vec::new();

    let entries = match std::fs::read_dir(sequences_dir) {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!("Dossier séquences introuvable ({sequences_dir:?}) : {e}");
            return sequences;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        match load_sequence(&path) {
            Ok(seq) => {
                tracing::info!("Séquence chargée : {} ({} étapes)", seq.name, seq.steps.len());
                sequences.push(seq);
            }
            Err(e) => {
                tracing::warn!("Impossible de charger la séquence {path:?} : {e}");
            }
        }
    }

    // Trier par nom
    sequences.sort_by(|a, b| a.name.cmp(&b.name));
    sequences
}

/// Charge une séquence depuis son dossier.
pub fn load_sequence(dir: &Path) -> Result<MiPowerSequence, SequenceError> {
    let name = dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    // Lire index.json
    let index_path = dir.join("index.json");
    let index_content = std::fs::read_to_string(&index_path)
        .map_err(|e| SequenceError::IoError(format!("{index_path:?}: {e}")))?;

    let steps: Vec<SequenceStep> = serde_json::from_str(&index_content)
        .map_err(|e| SequenceError::ParseError(format!("index.json: {e}")))?;

    // Lire brief.md (optionnel)
    let brief = dir
        .join("brief.md")
        .pipe(|p| std::fs::read_to_string(&p).unwrap_or_default());

    Ok(MiPowerSequence {
        name,
        path: dir.to_path_buf(),
        steps,
        brief,
    })
}

trait PathPipe: Sized {
    fn pipe<T, F: FnOnce(Self) -> T>(self, f: F) -> T {
        f(self)
    }
}
impl PathPipe for PathBuf {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_sequence(dir: &Path, name: &str, steps: usize) -> PathBuf {
        let seq_dir = dir.join(name);
        fs::create_dir_all(&seq_dir).unwrap();

        let index: Vec<serde_json::Value> = (1..=steps)
            .map(|i| serde_json::json!({
                "id": format!("E{i:02}"),
                "title": format!("Étape {i}"),
                "description": format!("Description de l'étape {i}"),
            }))
            .collect();

        fs::write(
            seq_dir.join("index.json"),
            serde_json::to_string_pretty(&index).unwrap(),
        ).unwrap();

        fs::write(
            seq_dir.join("brief.md"),
            format!("# {name}\n\nObjectif de test."),
        ).unwrap();

        seq_dir
    }

    #[test]
    fn test_load_sequence() {
        let dir = TempDir::new().unwrap();
        create_test_sequence(dir.path(), "test-seq", 3);
        let sequences = load_sequences(dir.path());
        assert_eq!(sequences.len(), 1);
        assert_eq!(sequences[0].steps.len(), 3);
        assert_eq!(sequences[0].steps[0].id, "E01");
    }

    #[test]
    fn test_load_empty_dir() {
        let dir = TempDir::new().unwrap();
        let sequences = load_sequences(dir.path());
        assert!(sequences.is_empty());
    }

    #[test]
    fn test_load_multiple_sequences() {
        let dir = TempDir::new().unwrap();
        create_test_sequence(dir.path(), "seq-a", 2);
        create_test_sequence(dir.path(), "seq-b", 5);
        let sequences = load_sequences(dir.path());
        assert_eq!(sequences.len(), 2);
        // Triées par nom
        assert_eq!(sequences[0].name, "seq-a");
        assert_eq!(sequences[1].name, "seq-b");
    }
}
