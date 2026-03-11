use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

pub struct InitLog {
    pub created: Vec<String>,
    pub skipped: Vec<String>,
    pub errors:  Vec<String>,
}
impl InitLog {
    fn new() -> Self { Self { created: vec![], skipped: vec![], errors: vec![] } }
    pub fn summary(&self) -> String {
        format!(
            "{} fichier(s) cree(s), {} ignore(s){}",
            self.created.len(), self.skipped.len(),
            if self.errors.is_empty() { String::new() }
            else { format!(", {} erreur(s)", self.errors.len()) }
        )
    }
}

/// Initialise la structure de base (equivalent init-sequence-base.ps1).
pub fn init_base(seq_path: &Path, template_root: &Path) -> Result<InitLog> {
    let mut log = InitLog::new();
    let (leaf, date) = parse_leaf(seq_path)?;
    let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let base_dir  = template_root.join("base");
    for sub in ["briefs", "metrics", "phases/p0/temps"] {
        let dir = seq_path.join(sub.replace('/', std::path::MAIN_SEPARATOR_STR));
        std::fs::create_dir_all(&dir).with_context(|| format!("mkdir {}", dir.display()))?;
    }
    if base_dir.exists() {
        apply_delta(&base_dir, seq_path, &leaf, &date, &timestamp, &mut log)?;
    }
    Ok(log)
}

/// Applique les deltas CN..C1 (equivalent init-sequence-by-complexity.ps1).
pub fn init_by_complexity(seq_path: &Path, template_root: &Path, complexity: u8) -> Result<InitLog> {
    let mut log = InitLog::new();
    let (leaf, date) = parse_leaf(seq_path)?;
    let timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    for level in (1..=complexity).rev() {
        let delta_dir = template_root.join(format!("c{level}"));
        if delta_dir.exists() {
            apply_delta(&delta_dir, seq_path, &leaf, &date, &timestamp, &mut log)?;
        }
    }
    Ok(log)
}

/// Regenere .mip/sequences/index.json.
pub fn rebuild_index(mip_root: &Path) -> Result<()> {
    let seq_dir = mip_root.join(".mip").join("sequences");
    if !seq_dir.exists() {
        anyhow::bail!("Dossier sequences introuvable : {}", seq_dir.display());
    }
    let now = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
    let mut rows: Vec<serde_json::Value> = Vec::new();
    let mut entries: Vec<_> = std::fs::read_dir(&seq_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .filter(|e| e.file_name() != "_template")
        .collect();
    entries.sort_by_key(|e| e.file_name());
    for entry in entries {
        let name         = entry.file_name().to_string_lossy().to_string();
        let (date, slug) = parse_name_parts(&name);
        let seq_path     = entry.path();
        let metrics       = read_first_json(&seq_path.join("metrics"));
        let brief_content = read_first_md_content(&seq_path.join("briefs"));
        let brief_path    = find_first_md_rel(&seq_path.join("briefs"), &name);
        let manifest      = read_json_file(&seq_path.join("ui").join("manifest.json"));
        let task_type = extract_type(&metrics, &brief_content);
        let status    = extract_str(&manifest, "identity.status");
        let cur_phase = extract_str(&manifest, "identity.current_phase");
        let sec_score = extract_f64(&metrics, "counters.security_score") as i64;
        rows.push(serde_json::json!({
            "name":          name,
            "date":          date,
            "slug":          slug,
            "type":          task_type,
            "status":        status,
            "current_phase": cur_phase,
            "brief_path":    brief_path.unwrap_or_default(),
            "ui_path":       format!("./sequences/{name}/ui/index.html"),
            "metrics_path":  format!("./sequences/{name}/metrics/"),
            "security_score": sec_score,
        }));
    }
    let count = rows.len();
    let index = serde_json::json!({
        "generated_at": now,
        "schema":       "mip-sequences-index-v2",
        "count":        count,
        "sequences":    rows,
    });
    let out = seq_dir.join("index.json");
    std::fs::write(&out, serde_json::to_string_pretty(&index)?)?;
    tracing::info!("Index regenere : {} ({} sequences)", out.display(), count);
    Ok(())
}

fn apply_delta(
    delta_dir: &Path, seq_root: &Path,
    leaf: &str, date: &str, timestamp: &str,
    log: &mut InitLog,
) -> Result<()> {
    for entry in walkdir(delta_dir) {
        let rel = entry.strip_prefix(delta_dir)
            .map(|p| p.to_string_lossy().replace('\\', "/"))
            .unwrap_or_default();
        let dest_rel = rel.replace("DATE-SLUG", leaf);
        let dest = seq_root.join(dest_rel.replace('/', std::path::MAIN_SEPARATOR_STR));
        if dest.exists() { log.skipped.push(dest.to_string_lossy().to_string()); continue; }
        if let Some(parent) = dest.parent() { std::fs::create_dir_all(parent)?; }
        let raw = std::fs::read_to_string(&entry)
            .with_context(|| format!("Lecture {}", entry.display()))?;
        let content = substitute(&raw, leaf, date, timestamp);
        std::fs::write(&dest, content.as_bytes())
            .with_context(|| format!("Ecriture {}", dest.display()))?;
        log.created.push(dest.to_string_lossy().to_string());
    }
    Ok(())
}

fn walkdir(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = std::fs::read_dir(dir) else { return out; };
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_dir() { out.extend(walkdir(&path)); } else { out.push(path); }
    }
    out
}

fn substitute(s: &str, leaf: &str, date: &str, ts: &str) -> String {
    s.replace("DATE-SLUG", leaf).replace("THE-DATE", date).replace("__TIMESTAMP__", ts)
}

fn parse_leaf(seq_path: &Path) -> Result<(String, String)> {
    let leaf = seq_path.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .context("Chemin invalide")?;
    let date = if leaf.len() >= 10 { leaf[..10].to_string() } else { String::new() };
    Ok((leaf, date))
}

fn parse_name_parts(name: &str) -> (String, String) {
    if name.len() > 10 && name.chars().nth(4) == Some('-') && name.chars().nth(7) == Some('-') {
        (name[..10].to_string(), if name.len() > 11 { name[11..].to_string() } else { String::new() })
    } else {
        (String::new(), name.to_string())
    }
}

fn read_first_json(dir: &Path) -> Option<serde_json::Value> {
    let entry = std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "json"))
        .min_by_key(|e| e.file_name())?;
    serde_json::from_str(&std::fs::read_to_string(entry.path()).ok()?).ok()
}

fn read_json_file(path: &Path) -> Option<serde_json::Value> {
    serde_json::from_str(&std::fs::read_to_string(path).ok()?).ok()
}

fn read_first_md_content(dir: &Path) -> String {
    std::fs::read_dir(dir).ok()
        .and_then(|mut r| r.find(|e| {
            e.as_ref().map(|e| e.path().extension().is_some_and(|x| x == "md")).unwrap_or(false)
        }))
        .and_then(|e| e.ok())
        .and_then(|e| std::fs::read_to_string(e.path()).ok())
        .unwrap_or_default()
}

fn find_first_md_rel(dir: &Path, seq_name: &str) -> Option<String> {
    let entry = std::fs::read_dir(dir).ok()?.filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|x| x == "md"))
        .min_by_key(|e| e.file_name())?;
    Some(format!("./sequences/{seq_name}/briefs/{}", entry.file_name().to_string_lossy()))
}

fn extract_type(metrics: &Option<serde_json::Value>, brief: &str) -> String {
    if let Some(m) = metrics {
        if let Some(t) = m.pointer("/project/class").and_then(|v| v.as_str()) {
            if matches!(t, "T1" | "T2" | "T3" | "T4" | "T5") { return t.to_string(); }
        }
    }
    for line in brief.lines() {
        for prefix in ["Classe : ", "Classification MIP : ", "Classe:"] {
            if let Some(pos) = line.find(prefix) {
                let t = line[pos + prefix.len()..].trim();
                if t.len() >= 2 && t.starts_with('T') && matches!(&t[..2], "T1" | "T2" | "T3" | "T4" | "T5") {
                    return t[..2].to_string();
                }
            }
        }
    }
    "UNKNOWN".to_string()
}

fn extract_str(json: &Option<serde_json::Value>, pointer: &str) -> String {
    let ptr = format!("/{}", pointer.replace('.', "/"));
    json.as_ref().and_then(|v| v.pointer(&ptr)).and_then(|v| v.as_str()).unwrap_or("").to_string()
}

fn extract_f64(json: &Option<serde_json::Value>, pointer: &str) -> f64 {
    let ptr = format!("/{}", pointer.replace('.', "/"));
    json.as_ref().and_then(|v| v.pointer(&ptr)).and_then(|v| v.as_f64()).unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_substitute() {
        let r = substitute("DATE-SLUG / THE-DATE / __TIMESTAMP__", "2026-03-07-t", "2026-03-07", "TS");
        assert_eq!(r, "2026-03-07-t / 2026-03-07 / TS");
    }

    #[test]
    fn test_parse_leaf() {
        let tmp = TempDir::new().unwrap();
        let seq = tmp.path().join("2026-03-07-myseq");
        std::fs::create_dir_all(&seq).unwrap();
        let (leaf, date) = parse_leaf(&seq).unwrap();
        assert_eq!(leaf, "2026-03-07-myseq");
        assert_eq!(date, "2026-03-07");
    }

    #[test]
    fn test_extract_type() {
        let t = extract_type(&None, "Classe : T4\nDomaine : fullstack");
        assert_eq!(t, "T4");
    }
}
