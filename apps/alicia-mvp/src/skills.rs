//! Systeme de skills Markdown pour Alicia.
//!
//! Les skills sont des fichiers `.md` dans `{workspace}/skills/`
//! avec un frontmatter YAML minimal qui definit un persona/comportement.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Definition d'un skill charge depuis un fichier Markdown.
#[derive(Debug, Clone)]
pub struct SkillDef {
    pub name: String,
    pub description: String,
    /// System prompt (corps du markdown apres le frontmatter).
    pub system_prompt: String,
    /// Outils autorises (None = tous).
    pub allowed_tools: Option<Vec<String>>,
}

/// Cache de skills charges depuis le workspace.
pub struct SkillStore {
    skills_dir: PathBuf,
    cache: HashMap<String, SkillDef>,
}

impl SkillStore {
    /// Scanne le dossier `{workspace}/skills/` et charge tous les `.md`.
    pub fn scan(workspace: &Path) -> Self {
        let skills_dir = workspace.join("skills");
        let mut cache = HashMap::new();

        if skills_dir.is_dir() {
            if let Ok(entries) = std::fs::read_dir(&skills_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|e| e.to_str()) == Some("md") {
                        match std::fs::read_to_string(&path) {
                            Ok(content) => {
                                if let Some(skill) = parse_skill_md(&content) {
                                    println!("  [Skills] Charge: {} — {}", skill.name, skill.description);
                                    cache.insert(skill.name.clone(), skill);
                                }
                            }
                            Err(e) => {
                                println!("  [Skills] Erreur lecture {:?}: {e}", path.file_name());
                            }
                        }
                    }
                }
            }
        }

        if cache.is_empty() {
            println!("  [Skills] Aucun skill trouve dans {}", skills_dir.display());
        } else {
            println!("  [Skills] {} skill(s) charge(s)", cache.len());
        }

        Self { skills_dir, cache }
    }

    /// Recharge les skills depuis le disque.
    pub fn reload(&mut self) {
        *self = Self::scan(
            self.skills_dir
                .parent()
                .unwrap_or(&self.skills_dir),
        );
    }

    pub fn find(&self, name: &str) -> Option<&SkillDef> {
        self.cache.get(name)
    }

    pub fn list(&self) -> Vec<(&str, &str)> {
        self.cache
            .values()
            .map(|s| (s.name.as_str(), s.description.as_str()))
            .collect()
    }
}

/// Parse un fichier Markdown avec frontmatter YAML minimal.
///
/// Format attendu :
/// ```text
/// ---
/// name: expert-rust
/// description: Expert Rust pour revue de code
/// tools: read_file, write_file, run_command
/// ---
/// Tu es un expert Rust senior...
/// ```
fn parse_skill_md(content: &str) -> Option<SkillDef> {
    let trimmed = content.trim();

    // Trouver le frontmatter entre les deux ---
    if !trimmed.starts_with("---") {
        return None;
    }

    let after_first = &trimmed[3..];
    let end_idx = after_first.find("---")?;
    let frontmatter = &after_first[..end_idx];
    let body = after_first[end_idx + 3..].trim();

    // Parser le frontmatter ligne par ligne
    let mut name = String::new();
    let mut description = String::new();
    let mut allowed_tools: Option<Vec<String>> = None;

    for line in frontmatter.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            match key {
                "name" => name = value.to_string(),
                "description" => description = value.to_string(),
                "tools" => {
                    let tools: Vec<String> = value
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect();
                    if !tools.is_empty() {
                        allowed_tools = Some(tools);
                    }
                }
                _ => {} // Ignorer les cles inconnues
            }
        }
    }

    if name.is_empty() {
        return None;
    }

    if description.is_empty() {
        description = format!("Skill: {name}");
    }

    let system_prompt = if body.is_empty() {
        format!("Tu es {name}.")
    } else {
        body.to_string()
    };

    Some(SkillDef {
        name,
        description,
        system_prompt,
        allowed_tools,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_skill_md() {
        let md = r#"---
name: expert-rust
description: Expert Rust pour revue de code
tools: read_file, write_file, run_command
---
Tu es un expert Rust senior. Tu analyses le code avec rigueur."#;

        let skill = parse_skill_md(md).unwrap();
        assert_eq!(skill.name, "expert-rust");
        assert_eq!(skill.description, "Expert Rust pour revue de code");
        assert_eq!(
            skill.allowed_tools,
            Some(vec![
                "read_file".to_string(),
                "write_file".to_string(),
                "run_command".to_string()
            ])
        );
        assert!(skill.system_prompt.contains("expert Rust senior"));
    }

    #[test]
    fn test_parse_no_tools() {
        let md = r#"---
name: poete
description: Ecrit de la poesie
---
Tu es un poete francais."#;

        let skill = parse_skill_md(md).unwrap();
        assert_eq!(skill.name, "poete");
        assert!(skill.allowed_tools.is_none());
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_skill_md("pas de frontmatter").is_none());
        assert!(parse_skill_md("---\n---\n").is_none()); // pas de name
    }
}
