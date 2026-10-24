//! @id     miou_llm_bridge_skills_file_ops
//! @do     definir_default_sandbox
//! @role   orchestration
//! @layer  app
//! @human  miou-llm-bridge — module « skills file ops ».

//! Skill : Opérations sur les fichiers.
//!
//! Permet aux agents de lire, écrire, lister et rechercher dans les fichiers.
//! Toutes les opérations sont restreintes au sandbox configuré.

use super::{SkillInfo, SkillResult};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Répertoire sandbox par défaut (relatif au data_dir).
const DEFAULT_SANDBOX: &str = "workspace";

/// Taille maximale de lecture (1 MB).
const MAX_READ_SIZE: u64 = 1_048_576;

/// Métadonnées du skill.
pub fn info() -> SkillInfo {
    let mut params = HashMap::new();
    params.insert(
        "action".into(),
        "read | write | list | search | exists | mkdir | delete".into(),
    );
    params.insert(
        "path".into(),
        "Chemin relatif au sandbox (obligatoire)".into(),
    );
    params.insert("content".into(), "Contenu à écrire (pour write)".into());
    params.insert(
        "pattern".into(),
        "Pattern de recherche (pour search)".into(),
    );
    params.insert(
        "recursive".into(),
        "Recherche récursive (pour list/search, défaut: false)".into(),
    );

    SkillInfo {
        id: "file_ops".into(),
        name: "Opérations Fichiers".into(),
        description: "Lire, écrire, lister et rechercher des fichiers dans le sandbox.".into(),
        parameters: params,
        min_security_level: "sandboxed",
    }
}

/// Exécute une opération fichier.
pub async fn execute(params: &HashMap<String, serde_json::Value>) -> SkillResult {
    let action = params
        .get("action")
        .and_then(|v| v.as_str())
        .unwrap_or("read");

    let path_str = params.get("path").and_then(|v| v.as_str()).unwrap_or("");

    if path_str.is_empty() && action != "list" {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Paramètre 'path' requis".into()),
            duration_ms: 0,
        };
    }

    let sandbox = resolve_sandbox();
    // Fail-closed : tout chemin non résolu/non vérifiable dans le sandbox est refusé.
    let full_path = match resolve_in_sandbox(&sandbox, path_str) {
        Ok(p) => p,
        Err(msg) => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(msg),
                duration_ms: 0,
            };
        }
    };

    match action {
        "read" => read_file(&full_path).await,
        "write" => {
            let content = params.get("content").and_then(|v| v.as_str()).unwrap_or("");
            write_file(&full_path, content).await
        }
        "list" => {
            let target = &full_path;
            let recursive = params
                .get("recursive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            list_dir(target, recursive).await
        }
        "search" => {
            let pattern = params.get("pattern").and_then(|v| v.as_str()).unwrap_or("");
            search_files(&full_path, pattern).await
        }
        "exists" => {
            let exists = full_path.exists();
            SkillResult {
                success: true,
                output: serde_json::json!({ "exists": exists, "path": path_str }),
                error: None,
                duration_ms: 0,
            }
        }
        "mkdir" => mkdir(&full_path).await,
        "delete" => delete_file(&full_path).await,
        _ => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Action inconnue : {action}")),
            duration_ms: 0,
        },
    }
}

/// Résout `rel` dans `sandbox` et garantit que le résultat y reste (fail-closed).
///
/// - chemin absolu / préfixe / composant `..` : refusé d'emblée ;
/// - sandbox créé si absent puis canonicalisé (échec => refus) ;
/// - cible existante : `canonicalize` doit rester sous le sandbox ;
/// - cible inexistante (ex: écriture d'un nouveau fichier) : on remonte au
///   plus proche ancêtre existant, qu'on canonicalise (échec => refus, y
///   compris lien symbolique pendant) et qui doit être dans le sandbox.
pub(crate) fn resolve_in_sandbox(sandbox: &Path, rel: &str) -> Result<PathBuf, String> {
    use std::path::Component;
    let deny = |m: &str| Err(format!("{m} — accès refusé"));

    let rel_path = Path::new(rel);
    for c in rel_path.components() {
        match c {
            Component::Normal(_) | Component::CurDir => {}
            Component::ParentDir => return deny("Composant '..' interdit"),
            Component::RootDir | Component::Prefix(_) => return deny("Chemin absolu interdit"),
        }
    }

    if std::fs::create_dir_all(sandbox).is_err() {
        return deny("Sandbox inaccessible");
    }
    let Ok(sandbox_canon) = sandbox.canonicalize() else {
        return deny("Sandbox non résoluble");
    };

    let full = sandbox_canon.join(rel_path);

    // Plus proche ancêtre existant (lien symbolique pendant inclus via symlink_metadata).
    let mut probe: &Path = &full;
    loop {
        if std::fs::symlink_metadata(probe).is_ok() {
            let Ok(canon) = probe.canonicalize() else {
                return deny("Chemin non résoluble");
            };
            if !canon.starts_with(&sandbox_canon) {
                return deny("Chemin hors du sandbox");
            }
            break;
        }
        match probe.parent() {
            Some(parent) => probe = parent,
            None => return deny("Chemin non vérifiable"),
        }
    }
    Ok(full)
}

/// Résout le répertoire sandbox.
pub(crate) fn resolve_sandbox() -> PathBuf {
    if let Ok(data_dir) = std::env::var("MIYUKINI_DATA_DIR") {
        Path::new(&data_dir).join(DEFAULT_SANDBOX)
    } else {
        PathBuf::from(DEFAULT_SANDBOX)
    }
}

async fn read_file(path: &Path) -> SkillResult {
    match tokio::fs::metadata(path).await {
        Ok(meta) if meta.len() > MAX_READ_SIZE => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!(
                    "Fichier trop volumineux ({} bytes, max {} bytes)",
                    meta.len(),
                    MAX_READ_SIZE
                )),
                duration_ms: 0,
            };
        }
        Err(e) => {
            return SkillResult {
                success: false,
                output: serde_json::Value::Null,
                error: Some(format!("Fichier introuvable : {e}")),
                duration_ms: 0,
            };
        }
        _ => {}
    }

    match tokio::fs::read_to_string(path).await {
        Ok(content) => SkillResult {
            success: true,
            output: serde_json::json!({
                "content": content,
                "path": path.display().to_string(),
                "size": content.len(),
            }),
            error: None,
            duration_ms: 0,
        },
        Err(e) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur lecture : {e}")),
            duration_ms: 0,
        },
    }
}

async fn write_file(path: &Path, content: &str) -> SkillResult {
    // Créer les répertoires parents si nécessaires
    if let Some(parent) = path.parent() {
        let _ = tokio::fs::create_dir_all(parent).await;
    }

    match tokio::fs::write(path, content).await {
        Ok(()) => SkillResult {
            success: true,
            output: serde_json::json!({
                "path": path.display().to_string(),
                "size": content.len(),
                "action": "written",
            }),
            error: None,
            duration_ms: 0,
        },
        Err(e) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur écriture : {e}")),
            duration_ms: 0,
        },
    }
}

async fn list_dir(path: &Path, recursive: bool) -> SkillResult {
    let mut entries = Vec::new();

    if !path.exists() {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Répertoire introuvable".into()),
            duration_ms: 0,
        };
    }

    collect_entries(path, path, recursive, &mut entries, 0).await;

    SkillResult {
        success: true,
        output: serde_json::json!({
            "path": path.display().to_string(),
            "count": entries.len(),
            "entries": entries,
        }),
        error: None,
        duration_ms: 0,
    }
}

#[async_recursion::async_recursion]
async fn collect_entries(
    base: &Path,
    dir: &Path,
    recursive: bool,
    entries: &mut Vec<serde_json::Value>,
    depth: u32,
) {
    // Limite de profondeur pour éviter les boucles infinies
    if depth > 10 {
        return;
    }

    let mut read_dir = match tokio::fs::read_dir(dir).await {
        Ok(rd) => rd,
        Err(_) => return,
    };

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let path = entry.path();
        let rel_path = path.strip_prefix(base).unwrap_or(&path);
        let is_dir = path.is_dir();

        entries.push(serde_json::json!({
            "name": entry.file_name().to_string_lossy(),
            "path": rel_path.display().to_string(),
            "is_dir": is_dir,
        }));

        if is_dir && recursive {
            collect_entries(base, &path, recursive, entries, depth + 1).await;
        }
    }
}

async fn search_files(path: &Path, pattern: &str) -> SkillResult {
    if pattern.is_empty() {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Paramètre 'pattern' requis pour la recherche".into()),
            duration_ms: 0,
        };
    }

    let mut matches = Vec::new();
    search_recursive(path, pattern, &mut matches, 0).await;

    SkillResult {
        success: true,
        output: serde_json::json!({
            "pattern": pattern,
            "count": matches.len(),
            "matches": matches,
        }),
        error: None,
        duration_ms: 0,
    }
}

#[async_recursion::async_recursion]
async fn search_recursive(
    dir: &Path,
    pattern: &str,
    matches: &mut Vec<serde_json::Value>,
    depth: u32,
) {
    if depth > 10 || matches.len() >= 100 {
        return;
    }

    let mut read_dir = match tokio::fs::read_dir(dir).await {
        Ok(rd) => rd,
        Err(_) => return,
    };

    let pattern_lower = pattern.to_lowercase();

    while let Ok(Some(entry)) = read_dir.next_entry().await {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.to_lowercase().contains(&pattern_lower) {
            matches.push(serde_json::json!({
                "name": name,
                "path": path.display().to_string(),
                "is_dir": path.is_dir(),
            }));
        }

        if path.is_dir() {
            search_recursive(&path, pattern, matches, depth + 1).await;
        }
    }
}

async fn mkdir(path: &Path) -> SkillResult {
    match tokio::fs::create_dir_all(path).await {
        Ok(()) => SkillResult {
            success: true,
            output: serde_json::json!({ "path": path.display().to_string(), "action": "created" }),
            error: None,
            duration_ms: 0,
        },
        Err(e) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur création : {e}")),
            duration_ms: 0,
        },
    }
}

async fn delete_file(path: &Path) -> SkillResult {
    if !path.exists() {
        return SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some("Fichier introuvable".into()),
            duration_ms: 0,
        };
    }

    let result = if path.is_dir() {
        tokio::fs::remove_dir_all(path).await
    } else {
        tokio::fs::remove_file(path).await
    };

    match result {
        Ok(()) => SkillResult {
            success: true,
            output: serde_json::json!({ "path": path.display().to_string(), "action": "deleted" }),
            error: None,
            duration_ms: 0,
        },
        Err(e) => SkillResult {
            success: false,
            output: serde_json::Value::Null,
            error: Some(format!("Erreur suppression : {e}")),
            duration_ms: 0,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("maia-fileops-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn chemin_inexistant_dans_sandbox_accepte() {
        let sb = tmp("ok");
        let p = resolve_in_sandbox(&sb, "nouveau/sous/fichier.txt").unwrap();
        assert!(p.starts_with(sb.canonicalize().unwrap()));
        assert!(resolve_in_sandbox(&sb, "").is_ok());
        let _ = std::fs::remove_dir_all(&sb);
    }

    #[test]
    fn composant_parent_refuse_meme_inexistant() {
        let sb = tmp("dotdot");
        assert!(resolve_in_sandbox(&sb, "../evil.txt").is_err());
        assert!(resolve_in_sandbox(&sb, "a/../../evil.txt").is_err());
        assert!(resolve_in_sandbox(&sb, "inexistant/../x").is_err());
        let _ = std::fs::remove_dir_all(&sb);
    }

    #[test]
    fn chemin_absolu_refuse() {
        let sb = tmp("abs");
        assert!(resolve_in_sandbox(&sb, "/etc/passwd").is_err());
        let _ = std::fs::remove_dir_all(&sb);
    }

    #[cfg(unix)]
    #[test]
    fn symlink_sortant_refuse() {
        let sb = tmp("sym");
        let outside = tmp("sym-out");
        std::os::unix::fs::symlink(&outside, sb.join("lien")).unwrap();
        // Existant via lien, et inexistant sous le lien : refus dans les deux cas.
        assert!(resolve_in_sandbox(&sb, "lien").is_err());
        assert!(resolve_in_sandbox(&sb, "lien/nouveau.txt").is_err());
        // Lien pendant vers l'extérieur : canonicalize échoue => refus.
        std::os::unix::fs::symlink(outside.join("absent"), sb.join("pendant")).unwrap();
        assert!(resolve_in_sandbox(&sb, "pendant").is_err());
        let _ = std::fs::remove_dir_all(&sb);
        let _ = std::fs::remove_dir_all(&outside);
    }
}
