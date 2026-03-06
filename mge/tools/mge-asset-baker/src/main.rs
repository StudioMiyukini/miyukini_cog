use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Metadata for a single sprite in the baked atlas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BakedSprite {
    pub name: String,
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub source_hash: String,
}

/// Atlas manifest produced by the baker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlasManifest {
    pub atlas_name: String,
    pub width: u32,
    pub height: u32,
    pub sprites: Vec<BakedSprite>,
}

/// Input descriptor for the baker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BakeInput {
    pub atlas_name: String,
    pub atlas_size: u32,
    pub source_dir: String,
}

/// Scan a directory for image files and collect their metadata.
pub fn scan_sources(dir: &Path) -> Result<Vec<(String, u64)>> {
    let mut entries = Vec::new();
    if !dir.exists() {
        return Ok(entries);
    }
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if matches!(ext.to_lowercase().as_str(), "png" | "bmp" | "tga") {
                    let name =
                        path.file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_owned();
                    let meta = fs::metadata(&path)?;
                    entries.push((name, meta.len()));
                }
            }
        }
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(entries)
}

/// Simple hash of file contents (FNV-1a 64-bit).
pub fn hash_bytes(data: &[u8]) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for &byte in data {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// Validate naming conventions: lowercase, underscores, no spaces.
pub fn validate_name(name: &str) -> Result<()> {
    if name.is_empty() {
        anyhow::bail!("empty sprite name");
    }
    for ch in name.chars() {
        if !ch.is_ascii_lowercase() && !ch.is_ascii_digit() && ch != '_' {
            anyhow::bail!("invalid character '{ch}' in sprite name '{name}' (expected lowercase/digit/underscore)");
        }
    }
    Ok(())
}

/// Pack sprites into an atlas using a simple shelf algorithm.
/// Returns placement positions. Does not produce image data (placeholder).
pub fn pack_shelf(sprites: &[(String, u16, u16)], atlas_size: u32) -> Result<Vec<BakedSprite>> {
    let mut placed = Vec::new();
    let mut shelf_x: u32 = 0;
    let mut shelf_y: u32 = 0;
    let mut shelf_height: u32 = 0;

    for (name, w, h) in sprites {
        let w32 = u32::from(*w);
        let h32 = u32::from(*h);

        if shelf_x + w32 > atlas_size {
            // Next shelf
            shelf_y += shelf_height;
            shelf_x = 0;
            shelf_height = 0;
        }

        if shelf_y + h32 > atlas_size {
            anyhow::bail!(
                "atlas overflow: cannot fit '{name}' ({w}x{h}) in {atlas_size}x{atlas_size}"
            );
        }

        #[allow(clippy::cast_possible_truncation)]
        placed.push(BakedSprite {
            name: name.clone(),
            x: shelf_x as u16,
            y: shelf_y as u16,
            w: *w,
            h: *h,
            source_hash: String::new(),
        });

        shelf_x += w32;
        shelf_height = shelf_height.max(h32);
    }

    Ok(placed)
}

/// Write the atlas manifest to a JSON file.
pub fn write_manifest(manifest: &AtlasManifest, output: &Path) -> Result<()> {
    let json =
        serde_json::to_string_pretty(manifest).context("failed to serialize atlas manifest")?;
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, json).with_context(|| format!("failed to write {}", output.display()))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mge-asset-baker <input.json>");
        eprintln!("  input.json: BakeInput descriptor");
        std::process::exit(1);
    }

    let input_path = PathBuf::from(&args[1]);
    let raw = fs::read_to_string(&input_path)
        .with_context(|| format!("reading {}", input_path.display()))?;
    let input: BakeInput =
        serde_json::from_str(&raw).with_context(|| format!("parsing {}", input_path.display()))?;

    let source_dir = PathBuf::from(&input.source_dir);
    let sources = scan_sources(&source_dir)?;
    eprintln!("Found {} source files in {}", sources.len(), source_dir.display());

    // For now, use placeholder sizes (64x64) since we don't decode images yet.
    let sprites: Vec<(String, u16, u16)> =
        sources.iter().map(|(name, _)| (name.clone(), 64, 64)).collect();

    for (name, _, _) in &sprites {
        validate_name(name)?;
    }

    let placed = pack_shelf(&sprites, input.atlas_size)?;

    let manifest = AtlasManifest {
        atlas_name: input.atlas_name.clone(),
        width: input.atlas_size,
        height: input.atlas_size,
        sprites: placed,
    };

    let output_path = source_dir.join(format!("{}_atlas.json", input.atlas_name));
    write_manifest(&manifest, &output_path)?;
    eprintln!("Wrote manifest to {}", output_path.display());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_name_accepts_valid() {
        assert!(validate_name("warrior_idle_s").is_ok());
        assert!(validate_name("fire_01").is_ok());
    }

    #[test]
    fn validate_name_rejects_invalid() {
        assert!(validate_name("").is_err());
        assert!(validate_name("Warrior").is_err());
        assert!(validate_name("has space").is_err());
        assert!(validate_name("has-dash").is_err());
    }

    #[test]
    fn hash_bytes_deterministic() {
        let h1 = hash_bytes(b"hello");
        let h2 = hash_bytes(b"hello");
        assert_eq!(h1, h2);
        let h3 = hash_bytes(b"world");
        assert_ne!(h1, h3);
    }

    #[test]
    fn pack_shelf_fits_sprites() {
        let sprites = vec![("a".into(), 64, 64), ("b".into(), 64, 64), ("c".into(), 64, 64)];
        let placed = pack_shelf(&sprites, 256).expect("should fit");
        assert_eq!(placed.len(), 3);
        assert_eq!(placed[0].x, 0);
        assert_eq!(placed[1].x, 64);
        assert_eq!(placed[2].x, 128);
    }

    #[test]
    fn pack_shelf_wraps_to_next_row() {
        let sprites = vec![("a".into(), 200, 100), ("b".into(), 200, 100)];
        let placed = pack_shelf(&sprites, 256).expect("should fit");
        assert_eq!(placed[0].y, 0);
        assert_eq!(placed[1].y, 100); // wrapped to next shelf
    }

    #[test]
    fn pack_shelf_overflow_errors() {
        let sprites = vec![("big".into(), 300, 300)];
        assert!(pack_shelf(&sprites, 256).is_err());
    }

    #[test]
    fn manifest_roundtrip() {
        let manifest = AtlasManifest {
            atlas_name: "test".into(),
            width: 256,
            height: 256,
            sprites: vec![BakedSprite {
                name: "warrior".into(),
                x: 0,
                y: 0,
                w: 64,
                h: 64,
                source_hash: "abc123".into(),
            }],
        };
        let json = serde_json::to_string(&manifest).expect("serialize");
        let restored: AtlasManifest = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(restored.atlas_name, "test");
        assert_eq!(restored.sprites.len(), 1);
    }
}
