//! Gestion des modèles GGUF locaux — scan, métadonnées, chargement.

use serde::Serialize;
use std::path::{Path, PathBuf};

/// Métadonnées d'un modèle GGUF détecté localement.
#[derive(Debug, Clone, Serialize)]
pub struct LocalModel {
    /// Nom de fichier (ex: "mistral-7b-instruct-v0.2.Q4_K_M.gguf").
    pub filename: String,
    /// Chemin complet vers le fichier.
    pub path: PathBuf,
    /// Taille du fichier en octets.
    pub size_bytes: u64,
    /// Taille affichable (ex: "4.1 GB").
    pub size_display: String,
    /// Identifiant lisible dérivé du nom de fichier.
    pub display_name: String,
}

/// Gestionnaire de modèles GGUF locaux.
#[derive(Debug, Clone)]
pub struct ModelManager {
    models_dir: PathBuf,
}

impl ModelManager {
    /// Crée un ModelManager pour le dossier donné. Crée le dossier s'il n'existe pas.
    pub fn new(models_dir: impl Into<PathBuf>) -> Self {
        let dir = models_dir.into();
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                tracing::warn!("Impossible de créer le dossier modèles {dir:?} : {e}");
            }
        }
        Self { models_dir: dir }
    }

    /// Scanne le dossier pour trouver tous les fichiers .gguf.
    pub fn scan_models(&self) -> Vec<LocalModel> {
        let mut models = Vec::new();

        let entries = match std::fs::read_dir(&self.models_dir) {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Impossible de scanner {dir:?} : {e}", dir = self.models_dir);
                return models;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) == Some("gguf") {
                if let Ok(meta) = std::fs::metadata(&path) {
                    let filename = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();

                    let size_bytes = meta.len();
                    let display_name = derive_display_name(&filename);

                    models.push(LocalModel {
                        filename: filename.clone(),
                        path: path.clone(),
                        size_bytes,
                        size_display: format_size(size_bytes),
                        display_name,
                    });
                }
            }
        }

        // Trier par taille (plus petit d'abord — plus rapide à charger)
        models.sort_by_key(|m| m.size_bytes);
        models
    }

    /// Trouve un modèle par nom de fichier (exact ou partiel).
    pub fn find_model(&self, query: &str) -> Option<LocalModel> {
        let models = self.scan_models();
        let query_lower = query.to_lowercase();

        // Match exact d'abord
        if let Some(m) = models.iter().find(|m| m.filename == query) {
            return Some(m.clone());
        }

        // Match partiel (contient)
        models
            .into_iter()
            .find(|m| m.filename.to_lowercase().contains(&query_lower))
    }

    /// Recommande le meilleur modèle basé sur la RAM disponible.
    pub fn recommend_model(&self, available_ram_mb: u64) -> Option<LocalModel> {
        let models = self.scan_models();

        // Prendre le plus gros modèle qui rentre dans 75% de la RAM
        let usable_ram_bytes = (available_ram_mb as u64) * 1024 * 1024 * 3 / 4;

        models
            .into_iter()
            .rev() // plus gros d'abord (la liste est triée par taille croissante)
            .find(|m| m.size_bytes < usable_ram_bytes)
    }

    /// Chemin du dossier modèles.
    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }
}

/// Dérive un nom d'affichage à partir du nom de fichier GGUF.
fn derive_display_name(filename: &str) -> String {
    filename
        .trim_end_matches(".gguf")
        .replace('-', " ")
        .replace('_', " ")
}

/// Formate une taille en octets en texte lisible.
fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{} MB", bytes / MB)
    }
}
