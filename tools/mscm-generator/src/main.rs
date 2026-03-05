use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    id: String,
    file: String,
    start_line: usize,
    end_line: usize,
    role: Option<String>,
    layer: Option<String>,
    do_action: Option<String>,
    human: Option<String>,
    depends: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Registry {
    version: String,
    mscm_version: String,
    generated_at: String,
    files_count: usize,
    blocks_count: usize,
    integrity: String,
}

/// Découvre tous les répertoires `src` des crates sous `crates/` et `tools/`.
fn discover_crate_src_dirs() -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut dirs = Vec::new();
    for parent in &["crates", "tools"] {
        let parent_path = PathBuf::from(parent);
        if !parent_path.exists() {
            continue;
        }
        for entry in fs::read_dir(&parent_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let cargo_toml = path.join("Cargo.toml");
                let src = path.join("src");
                if cargo_toml.is_file() && src.is_dir() {
                    dirs.push(src);
                }
            }
        }
    }
    Ok(dirs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output_dir = PathBuf::from("mscm_index");

    // Créer le dossier de sortie
    fs::create_dir_all(&output_dir)?;

    // Découverte dynamique de tous les crates (workspace)
    let crate_src_dirs = discover_crate_src_dirs()?;

    let mut blocks = Vec::new();
    let mut files_processed = HashSet::new();

    for crate_dir in &crate_src_dirs {
        scan_directory(crate_dir, &mut blocks, &mut files_processed)?;
    }

    // Générer les fichiers d'index
    generate_registry(&blocks, &output_dir, files_processed.len())?;
    generate_blocks(&blocks, &output_dir)?;
    generate_hierarchy(&blocks, &output_dir)?;
    generate_graph(&blocks, &output_dir)?;
    generate_flows(&blocks, &output_dir)?;
    generate_domains(&blocks, &output_dir)?;
    generate_layers(&blocks, &output_dir)?;
    generate_dependencies(&blocks, &output_dir)?;
    generate_files(&blocks, &output_dir)?;
    generate_stats(&blocks, &output_dir, files_processed.len())?;

    println!("Index MSCM généré avec succès dans {:?}", output_dir);
    println!(
        "Total: {} blocs dans {} fichiers",
        blocks.len(),
        files_processed.len()
    );

    Ok(())
}

fn scan_directory(
    dir: &Path,
    blocks: &mut Vec<Block>,
    files_processed: &mut HashSet<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            scan_directory(&path, blocks, files_processed)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            files_processed.insert(path.clone());
            parse_file(&path, blocks)?;
        }
    }

    Ok(())
}

fn parse_file(file_path: &Path, blocks: &mut Vec<Block>) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let lines: Vec<&str> = content.lines().collect();

    // Normaliser le chemin du fichier (relatif à la racine du workspace)
    let file_path_str = file_path
        .to_str()
        .ok_or("Invalid file path")?
        .replace('\\', "/");

    // Regex pour parser les annotations MSCM
    // Supporte /// et //!, avec ou sans : après @id/@role/etc.
    let id_re = Regex::new(r"(?:///|//!)\s*@id:?\s*(.+)")?;
    let role_re = Regex::new(r"(?:///|//!)\s*@role:?\s*(.+)")?;
    let layer_re = Regex::new(r"(?:///|//!)\s*@layer:?\s*(.+)")?;
    let do_re = Regex::new(r"(?:///|//!)\s*@do:?\s*(.+)")?;
    let human_re = Regex::new(r"(?:///|//!)\s*@human:?\s*(.+)")?;
    let depends_re = Regex::new(r"(?:///|//!)\s*@depends:?\s*(.+)")?;

    let mut current_block: Option<Block> = None;
    let mut in_comment_block = false;

    for (line_num, line) in lines.iter().enumerate() {
        let line_num = line_num + 1; // 1-indexed
        let trimmed = line.trim();

        // Détecter le début d'un bloc MSCM (@id)
        if let Some(caps) = id_re.captures(line) {
            // Sauvegarder le bloc précédent s'il existe
            if let Some(mut block) = current_block.take() {
                // Trouver la dernière ligne de code du bloc précédent
                block.end_line = find_block_end(&lines, block.start_line - 1);
                blocks.push(block);
            }

            // Nouveau bloc
            let id = caps.get(1).unwrap().as_str().trim().to_string();
            in_comment_block = true;
            current_block = Some(Block {
                id,
                file: file_path_str.clone(),
                start_line: line_num,
                end_line: line_num, // Sera mis à jour plus tard
                role: None,
                layer: None,
                do_action: None,
                human: None,
                depends: Vec::new(),
            });
        }

        // Parser les autres annotations pour le bloc courant
        if let Some(ref mut block) = current_block {
            if let Some(caps) = role_re.captures(line) {
                block.role = Some(caps.get(1).unwrap().as_str().trim().to_string());
            } else if let Some(caps) = layer_re.captures(line) {
                block.layer = Some(caps.get(1).unwrap().as_str().trim().to_string());
            } else if let Some(caps) = do_re.captures(line) {
                block.do_action = Some(caps.get(1).unwrap().as_str().trim().to_string());
            } else if let Some(caps) = human_re.captures(line) {
                block.human = Some(caps.get(1).unwrap().as_str().trim().to_string());
            } else if let Some(caps) = depends_re.captures(line) {
                let deps_str = caps.get(1).unwrap().as_str().trim();
                // Les dépendances peuvent être séparées par des virgules ou des espaces
                let deps: Vec<String> = deps_str
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                block.depends.extend(deps);
            }

            // Détecter la fin du bloc de commentaires MSCM
            // Le bloc se termine quand on trouve une ligne de code Rust (pas un commentaire)
            if in_comment_block
                && !trimmed.starts_with("///")
                && !trimmed.starts_with("//!")
                && !trimmed.is_empty()
            {
                // On a trouvé le début du code, le bloc se termine ici
                block.end_line = find_block_end(&lines, line_num - 1);
                in_comment_block = false;
            }
        }
    }

    // Sauvegarder le dernier bloc
    if let Some(mut block) = current_block {
        if block.end_line == block.start_line {
            block.end_line = find_block_end(&lines, block.start_line - 1);
        }
        blocks.push(block);
    }

    Ok(())
}

fn find_block_end(lines: &[&str], start_idx: usize) -> usize {
    // Chercher la dernière ligne de code non vide après le bloc de commentaires
    // On cherche jusqu'à la prochaine définition (pub fn, pub struct, impl, etc.)
    // ou jusqu'à la fin du fichier
    let mut end_line = start_idx + 1;

    for (idx, line) in lines.iter().enumerate().skip(start_idx) {
        let trimmed = line.trim();

        // Ignorer les lignes vides et les commentaires
        if trimmed.is_empty() || trimmed.starts_with("///") || trimmed.starts_with("//!") {
            continue;
        }

        // Si on trouve une nouvelle définition (pub, struct, enum, trait, impl, mod, use)
        // et qu'on a déjà trouvé du code, on s'arrête
        if idx > start_idx + 1
            && (trimmed.starts_with("pub ")
                || trimmed.starts_with("struct ")
                || trimmed.starts_with("enum ")
                || trimmed.starts_with("trait ")
                || trimmed.starts_with("impl ")
                || trimmed.starts_with("mod ")
                || trimmed.starts_with("use "))
        {
            // Retourner la ligne précédente
            return idx;
        }

        // Si on trouve du code Rust (contient des accolades, point-virgule, etc.)
        if trimmed.contains('{') || trimmed.contains('}') || trimmed.contains(';') {
            end_line = idx + 1;
        }
    }

    // Si on n'a pas trouvé de fin claire, utiliser la dernière ligne du fichier
    if end_line <= start_idx {
        end_line = lines.len();
    }

    end_line
}

fn generate_registry(
    blocks: &[Block],
    output_dir: &Path,
    files_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let registry = Registry {
        version: "mip_v1".to_string(),
        mscm_version: "v1".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        files_count,
        blocks_count: blocks.len(),
        integrity: "ok".to_string(),
    };

    let json = serde_json::to_string_pretty(&registry)?;
    fs::write(output_dir.join("registry.json"), json)?;

    Ok(())
}

fn generate_blocks(blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let blocks_json: Vec<serde_json::Value> = blocks
        .iter()
        .map(|b| {
            let mut obj = serde_json::Map::new();
            obj.insert("id".to_string(), serde_json::Value::String(b.id.clone()));
            obj.insert(
                "file".to_string(),
                serde_json::Value::String(b.file.clone()),
            );
            obj.insert(
                "start_line".to_string(),
                serde_json::Value::Number(b.start_line.into()),
            );
            obj.insert(
                "end_line".to_string(),
                serde_json::Value::Number(b.end_line.into()),
            );

            if let Some(ref do_action) = b.do_action {
                obj.insert(
                    "do".to_string(),
                    serde_json::Value::String(do_action.clone()),
                );
            }

            if let Some(ref role) = b.role {
                obj.insert("role".to_string(), serde_json::Value::String(role.clone()));
            }

            if let Some(ref layer) = b.layer {
                obj.insert(
                    "layer".to_string(),
                    serde_json::Value::String(layer.clone()),
                );
            }

            if let Some(ref human) = b.human {
                obj.insert(
                    "human".to_string(),
                    serde_json::Value::String(human.clone()),
                );
            }

            serde_json::Value::Object(obj)
        })
        .collect();

    let json = serde_json::to_string_pretty(&blocks_json)?;
    fs::write(output_dir.join("blocks.json"), json)?;

    Ok(())
}

fn generate_hierarchy(
    blocks: &[Block],
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut hierarchy: HashMap<String, Vec<String>> = HashMap::new();

    // Construire la hiérarchie basée sur les dépendances
    // Un bloc parent est celui qui est référencé dans @depends
    for block in blocks {
        if !block.depends.is_empty() {
            for dep in &block.depends {
                hierarchy
                    .entry(dep.clone())
                    .or_default()
                    .push(block.id.clone());
            }
        }
    }

    let json = serde_json::to_string_pretty(&hierarchy)?;
    fs::write(output_dir.join("hierarchy.json"), json)?;

    Ok(())
}

fn generate_graph(blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        if !block.depends.is_empty() {
            graph.insert(block.id.clone(), block.depends.clone());
        }
    }

    let json = serde_json::to_string_pretty(&graph)?;
    fs::write(output_dir.join("graph.json"), json)?;

    Ok(())
}

fn generate_flows(_blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Pour Phase 1, on génère un flow vide car les flows métier ne sont pas encore définis
    let flows: HashMap<String, Vec<String>> = HashMap::new();

    let json = serde_json::to_string_pretty(&flows)?;
    fs::write(output_dir.join("flows.json"), json)?;

    Ok(())
}

fn generate_domains(blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut domains: HashMap<String, Vec<String>> = HashMap::new();

    // Grouper par domaine basé sur le préfixe de l'ID (ex: kernel_*, config_*, etc.)
    for block in blocks {
        let domain = block.id.split('_').next().unwrap_or("unknown").to_string();

        domains.entry(domain).or_default().push(block.id.clone());
    }

    let json = serde_json::to_string_pretty(&domains)?;
    fs::write(output_dir.join("domains.json"), json)?;

    Ok(())
}

fn generate_layers(blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut layers: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        let layer = block.layer.as_deref().unwrap_or("unknown").to_string();
        layers.entry(layer).or_default().push(block.id.clone());
    }

    let json = serde_json::to_string_pretty(&layers)?;
    fs::write(output_dir.join("layers.json"), json)?;

    Ok(())
}

fn generate_dependencies(
    blocks: &[Block],
    output_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        if !block.depends.is_empty() {
            dependencies.insert(block.id.clone(), block.depends.clone());
        }
    }

    let json = serde_json::to_string_pretty(&dependencies)?;
    fs::write(output_dir.join("dependencies.json"), json)?;

    Ok(())
}

fn generate_files(blocks: &[Block], output_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut files: HashMap<String, Vec<String>> = HashMap::new();

    for block in blocks {
        files
            .entry(block.file.clone())
            .or_default()
            .push(block.id.clone());
    }

    let json = serde_json::to_string_pretty(&files)?;
    fs::write(output_dir.join("files.json"), json)?;

    Ok(())
}

fn generate_stats(
    blocks: &[Block],
    output_dir: &Path,
    files_count: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut layers_set = HashSet::new();
    let mut roles_set = HashSet::new();
    let mut max_depth = 0;

    for block in blocks {
        if let Some(ref layer) = block.layer {
            layers_set.insert(layer.clone());
        }
        if let Some(ref role) = block.role {
            roles_set.insert(role.clone());
        }
    }

    // Calculer la profondeur maximale (simplifié pour Phase 1)
    // On compte le nombre de niveaux de dépendances
    let mut depth_map: HashMap<String, usize> = HashMap::new();
    for block in blocks {
        let depth = calculate_depth(block, blocks, &mut depth_map);
        max_depth = max_depth.max(depth);
    }

    let stats = serde_json::json!({
        "blocks": blocks.len(),
        "files": files_count,
        "depth_max": max_depth,
        "domains": count_domains(blocks),
        "layers": layers_set.len(),
        "roles": roles_set.len(),
    });

    let json = serde_json::to_string_pretty(&stats)?;
    fs::write(output_dir.join("stats.json"), json)?;

    Ok(())
}

fn calculate_depth(
    block: &Block,
    all_blocks: &[Block],
    depth_map: &mut HashMap<String, usize>,
) -> usize {
    if let Some(&depth) = depth_map.get(&block.id) {
        return depth;
    }

    if block.depends.is_empty() {
        depth_map.insert(block.id.clone(), 1);
        return 1;
    }

    let max_dep_depth = block
        .depends
        .iter()
        .filter_map(|dep_id| {
            all_blocks
                .iter()
                .find(|b| b.id == *dep_id)
                .map(|dep_block| calculate_depth(dep_block, all_blocks, depth_map))
        })
        .max()
        .unwrap_or(0);

    let depth = max_dep_depth + 1;
    depth_map.insert(block.id.clone(), depth);
    depth
}

fn count_domains(blocks: &[Block]) -> usize {
    let mut domains = HashSet::new();
    for block in blocks {
        let domain = block.id.split('_').next().unwrap_or("unknown").to_string();
        domains.insert(domain);
    }
    domains.len()
}
