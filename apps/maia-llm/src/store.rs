//! Store des modèles GGUF — scan, index, recommandation par RAM.
//!
//! SHA-256 content-addressed : chaque .gguf peut avoir un fichier sidecar
//! `.gguf.sha256` avec son empreinte hex. Inspiré de miyucloud P3 dedup.
//! Le calcul est paresseux (à la demande) pour éviter de hasher des GB au démarrage.

use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::{Path, PathBuf};

/// Métadonnées d'un modèle GGUF détecté localement.
#[derive(Debug, Clone, Serialize)]
pub struct GgufModel {
    /// Nom de fichier (ex: "qwen3-4b-instruct-q4_k_m.gguf").
    pub filename: String,
    /// Chemin complet.
    pub path: PathBuf,
    /// Taille en octets.
    pub size_bytes: u64,
    /// Taille lisible (ex: "4.1 GB").
    pub size_display: String,
    /// Identifiant pour les requêtes API (sans extension).
    pub model_id: String,
    /// Nom d'affichage dérivé du filename.
    pub display_name: String,
    /// SHA-256 du contenu (calculé à la demande — None si non encore calculé).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
}

/// Store des modèles GGUF locaux.
#[derive(Debug, Clone)]
pub struct ModelStore {
    pub models_dir: PathBuf,
}

impl ModelStore {
    /// Crée le store pour le dossier donné. Crée le dossier si absent.
    pub fn new(models_dir: impl Into<PathBuf>) -> Self {
        let dir = models_dir.into();
        if !dir.exists() {
            if let Err(e) = std::fs::create_dir_all(&dir) {
                tracing::warn!("Impossible de créer le dossier modèles {dir:?} : {e}");
            }
        }
        Self { models_dir: dir }
    }

    /// Scanne le dossier et retourne tous les fichiers .gguf trouvés.
    /// Triés par taille croissante.
    pub fn scan(&self) -> Vec<GgufModel> {
        let mut models = Vec::new();

        let entries = match std::fs::read_dir(&self.models_dir) {
            Ok(e) => e,
            Err(e) => {
                tracing::warn!("Scan modèles échoué ({:?}) : {e}", self.models_dir);
                return models;
            }
        };

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("gguf") {
                continue;
            }
            if let Ok(meta) = std::fs::metadata(&path) {
                let filename = path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let size_bytes = meta.len();
                let model_id = filename.trim_end_matches(".gguf").to_string();
                let display_name = model_id.replace('-', " ").replace('_', " ");

                // Lire le SHA-256 depuis le sidecar .sha256 si disponible (évite de hasher les GB)
                let sha256 = read_sidecar_sha256(&path);

                models.push(GgufModel {
                    filename,
                    path,
                    size_bytes,
                    size_display: format_size(size_bytes),
                    model_id,
                    display_name,
                    sha256,
                });
            }
        }

        models.sort_by_key(|m| m.size_bytes);
        models
    }

    /// Trouve un modèle par model_id (exact) ou filename (partiel).
    pub fn find(&self, query: &str) -> Option<GgufModel> {
        let models = self.scan();
        let q = query.to_lowercase();

        // 1. Match exact sur model_id
        if let Some(m) = models.iter().find(|m| m.model_id == query) {
            return Some(m.clone());
        }
        // 2. Match exact sur filename
        if let Some(m) = models.iter().find(|m| m.filename == query) {
            return Some(m.clone());
        }
        // 3. Match partiel (insensible à la casse)
        models.into_iter().find(|m| m.filename.to_lowercase().contains(&q))
    }

    /// Recommande le modèle le plus grand tenant dans `ram_mb` Mo.
    pub fn recommend_for_ram(&self, ram_mb: u64) -> Option<GgufModel> {
        let models = self.scan();
        let usable_bytes = ram_mb * 1024 * 1024 * 3 / 4; // 75% de la RAM
        models.into_iter().rev().find(|m| m.size_bytes < usable_bytes)
    }

    /// Chemin du dossier modèles.
    pub fn models_dir(&self) -> &Path {
        &self.models_dir
    }
}

/// Lit le SHA-256 depuis un fichier sidecar `.gguf.sha256` s'il existe.
fn read_sidecar_sha256(gguf_path: &Path) -> Option<String> {
    let sidecar = gguf_path.with_extension("gguf.sha256");
    std::fs::read_to_string(&sidecar)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| s.len() == 64) // SHA-256 hex = 64 chars
}

/// Calcule le SHA-256 d'un fichier GGUF.
///
/// Lit par blocs de 1 MB pour ne pas consommer la RAM entière.
/// Ce calcul peut prendre plusieurs dizaines de secondes sur des modèles >10GB —
/// à appeler dans `tokio::task::spawn_blocking` exclusivement.
pub fn compute_sha256(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buf = vec![0u8; 1024 * 1024]; // 1 MB blocs
    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

/// Calcule le SHA-256 et l'écrit dans le fichier sidecar `.gguf.sha256`.
/// Retourne le hash. N'écrase pas un sidecar existant.
pub fn compute_sha256_and_cache(path: &Path) -> std::io::Result<String> {
    // Vérifier si le sidecar existe déjà
    if let Some(cached) = read_sidecar_sha256(path) {
        return Ok(cached);
    }
    let hash = compute_sha256(path)?;
    let sidecar = path.with_extension("gguf.sha256");
    if let Err(e) = std::fs::write(&sidecar, &hash) {
        tracing::warn!("Impossible d'écrire le sidecar SHA-256 {sidecar:?}: {e}");
    }
    Ok(hash)
}

/// Formate une taille en octets en texte lisible.
pub fn format_size(bytes: u64) -> String {
    const GB: u64 = 1024 * 1024 * 1024;
    const MB: u64 = 1024 * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else {
        format!("{} MB", bytes / MB)
    }
}
