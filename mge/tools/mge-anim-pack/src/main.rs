// @id: MGE-AnimPack-Main @do: cli-entry @role: back-end @layer: 6 @human: francois
//! CLI entry point for the animation atlas packer.

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};

use mge_anim_pack::aseprite;
use mge_anim_pack::errors::AnimPackError;
use mge_anim_pack::mirror;
use mge_anim_pack::naming;
use mge_anim_pack::packer;
use mge_anim_pack::toml_output;

/// A named frame: (name, RGBA pixels, width, height).
type NamedFrame = (String, Vec<u8>, u32, u32);

/// MGE Animation Atlas Packer.
///
/// Transforms Aseprite (.ase) files or individual PNG frames into
/// texture atlases with TOML descriptors.
#[derive(Parser, Debug)]
#[command(name = "mge-anim-pack", version, about)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Available subcommands.
#[derive(Subcommand, Debug)]
enum Command {
    /// Pack animation frames into an atlas with TOML descriptors.
    Pack {
        /// Input path: an .ase file or a directory of PNG frames.
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory for the atlas PNG and TOML files.
        #[arg(short, long)]
        output: PathBuf,

        /// Entity identifier (e.g. "fallen", "skeleton").
        #[arg(short, long)]
        entity: String,

        /// Maximum atlas dimension in pixels.
        #[arg(long, default_value = "2048")]
        max_size: u32,

        /// Pixel margin between packed frames.
        #[arg(long, default_value = "1")]
        margin: u32,

        /// Generate 8-direction mirrors from 4 rendered directions.
        #[arg(long, default_value = "false")]
        mirror: bool,
    },

    /// Validate an existing atlas TOML descriptor.
    Validate {
        /// Path to the atlas TOML file to validate.
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Display information about an Aseprite file.
    Info {
        /// Path to the .ase file.
        #[arg(short, long)]
        input: PathBuf,
    },

    /// Import individual PNG frames with size presets.
    ImportPng {
        /// Input directory containing PNG files.
        #[arg(short, long)]
        input: PathBuf,

        /// Output directory.
        #[arg(short, long)]
        output: PathBuf,

        /// Entity identifier.
        #[arg(short, long)]
        entity: String,

        /// Size class preset: small, medium, humanoid, large, boss.
        #[arg(long, default_value = "humanoid")]
        size_class: String,

        /// Animation action/state (idle, walk, attack, ...).
        #[arg(short, long, default_value = "idle")]
        action: String,

        /// Number of directions (4 or 8).
        #[arg(short, long, default_value = "4")]
        directions: u32,

        /// Frames per second.
        #[arg(long, default_value = "12")]
        fps: u32,

        /// Also generate an Aseprite (.ase) file.
        #[arg(long, default_value = "false")]
        aseprite: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Pack {
            input,
            output,
            entity,
            max_size,
            margin,
            mirror: do_mirror,
        } => run_pack(&input, &output, &entity, max_size, margin, do_mirror),
        Command::Validate { input } => run_validate(&input),
        Command::Info { input } => run_info(&input),
        Command::ImportPng {
            input,
            output,
            entity,
            size_class,
            action,
            directions,
            fps,
            aseprite: _aseprite,
        } => {
            let _ = (&input, &output, &entity, &size_class, &action, directions, fps);
            eprintln!("import-png: not yet implemented in this sprint");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// Run the `pack` pipeline:
/// 1. Load input (Aseprite or PNG directory)
/// 2. Validate naming + generate mirrors
/// 3. Pack into atlas
/// 4. Write atlas PNG + TOML files
fn run_pack(
    input: &Path,
    output: &Path,
    entity: &str,
    max_size: u32,
    margin: u32,
    do_mirror: bool,
) -> Result<(), AnimPackError> {
    let mut frames = load_input_frames(input, entity)?;
    if frames.is_empty() {
        return Err(AnimPackError::NoFrames);
    }
    eprintln!("Loaded {} frames", frames.len());

    validate_frame_names(&frames);

    if do_mirror {
        let mirrors = mirror::generate_mirrors(&frames)?;
        eprintln!("Generated {} mirror frames", mirrors.len());
        frames.extend(mirrors);
    }

    let pack_result = pack_and_assemble(&frames, max_size, margin)?;
    write_output_files(output, entity, &pack_result, &frames)?;

    eprintln!("Pack complete.");
    Ok(())
}

/// Load frames from an Aseprite file or a PNG directory.
fn load_input_frames(input: &Path, entity: &str) -> Result<Vec<NamedFrame>, AnimPackError> {
    let is_ase = input
        .extension()
        .is_some_and(|ext| ext == "ase" || ext == "aseprite");

    if is_ase {
        load_aseprite_frames(input, entity)
    } else if input.is_dir() {
        load_png_directory(input)
    } else {
        Err(AnimPackError::Io {
            path: input.to_path_buf(),
            source: std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Input must be an .ase/.aseprite file or a directory of PNGs",
            ),
        })
    }
}

/// Load frames from an Aseprite file using tags as action names.
fn load_aseprite_frames(input: &Path, entity: &str) -> Result<Vec<NamedFrame>, AnimPackError> {
    let ase_data = aseprite::load_aseprite(input)?;
    eprintln!(
        "Loaded Aseprite file: {}x{}, {} frames, {} tags",
        ase_data.width,
        ase_data.height,
        ase_data.frames.len(),
        ase_data.tags.len(),
    );

    if ase_data.tags.is_empty() {
        return Ok(ase_data
            .frames
            .into_iter()
            .map(|f| {
                let name = naming::format_frame_name(entity, "default", "s", f.index);
                (name, f.pixels, ase_data.width, ase_data.height)
            })
            .collect());
    }

    let mut result = Vec::new();
    for tag in &ase_data.tags {
        for frame_idx in tag.from_frame..=tag.to_frame {
            let local_idx = frame_idx - tag.from_frame;
            let name = naming::format_frame_name(entity, &tag.name, "s", local_idx);
            if let Some(frame) = ase_data.frames.get(frame_idx as usize) {
                result.push((
                    name,
                    frame.pixels.clone(),
                    ase_data.width,
                    ase_data.height,
                ));
            }
        }
    }
    Ok(result)
}

/// Load PNG files from a directory, sorted by name.
fn load_png_directory(dir: &Path) -> Result<Vec<NamedFrame>, AnimPackError> {
    let mut entries: Vec<_> = std::fs::read_dir(dir)
        .map_err(|e| AnimPackError::io(dir, e))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "png") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    entries.sort();

    let mut frames = Vec::with_capacity(entries.len());
    for path in &entries {
        let img = image::open(path)
            .map_err(|e| AnimPackError::Image(format!("{}: {e}", path.display())))?;
        let rgba = img.to_rgba8();
        let (w, h) = rgba.dimensions();
        let name = path
            .file_name()
            .map_or_else(|| "unknown.png".to_string(), |n| n.to_string_lossy().to_string());
        frames.push((name, rgba.into_raw(), w, h));
    }

    Ok(frames)
}

/// Validate frame names and print warnings for invalid ones.
fn validate_frame_names(frames: &[NamedFrame]) {
    let names: Vec<&str> = frames.iter().map(|(n, _, _, _)| n.as_str()).collect();
    let errors = naming::validate_naming(&names);
    for err in &errors {
        eprintln!("Warning: {err}");
    }
}

/// Packed atlas result with pixel data.
struct PackedAtlas {
    result: packer::PackResult,
    pixels: Vec<u8>,
}

/// Pack frames and assemble the atlas pixel buffer.
fn pack_and_assemble(
    frames: &[NamedFrame],
    max_size: u32,
    margin: u32,
) -> Result<PackedAtlas, AnimPackError> {
    let frame_specs: Vec<packer::FrameSpec> = frames
        .iter()
        .map(|(name, _, w, h)| (name.clone(), *w, *h))
        .collect();
    let result = packer::pack_frames(&frame_specs, max_size, margin)?;
    eprintln!(
        "Atlas packed: {}x{}, {} frames",
        result.atlas_width,
        result.atlas_height,
        result.frames.len(),
    );

    let pixels = assemble_atlas_pixels(&result, frames);
    Ok(PackedAtlas { result, pixels })
}

/// Assemble the final atlas pixel buffer from packed frame positions.
fn assemble_atlas_pixels(
    pack_result: &packer::PackResult,
    frames: &[NamedFrame],
) -> Vec<u8> {
    let mut atlas =
        vec![0u8; (pack_result.atlas_width * pack_result.atlas_height * 4) as usize];

    for packed in &pack_result.frames {
        if let Some((_name, pixels, w, _h)) = frames.iter().find(|(n, _, _, _)| n == &packed.name)
        {
            for row in 0..packed.height {
                let src_start = (row * w * 4) as usize;
                let src_end = src_start + (packed.width * 4) as usize;
                let dst_row = packed.y + row;
                let dst_start = ((dst_row * pack_result.atlas_width + packed.x) * 4) as usize;
                let dst_end = dst_start + (packed.width * 4) as usize;
                if src_end <= pixels.len() && dst_end <= atlas.len() {
                    atlas[dst_start..dst_end].copy_from_slice(&pixels[src_start..src_end]);
                }
            }
        }
    }

    atlas
}

/// Write atlas PNG, atlas TOML, and animations TOML to the output directory.
fn write_output_files(
    output: &Path,
    entity: &str,
    packed: &PackedAtlas,
    frames: &[NamedFrame],
) -> Result<(), AnimPackError> {
    std::fs::create_dir_all(output).map_err(|e| AnimPackError::io(output, e))?;

    // Write atlas PNG.
    let atlas_png_path = output.join(format!("{entity}_atlas.png"));
    image::save_buffer(
        &atlas_png_path,
        &packed.pixels,
        packed.result.atlas_width,
        packed.result.atlas_height,
        image::ColorType::Rgba8,
    )
    .map_err(|e| AnimPackError::Image(format!("{}: {e}", atlas_png_path.display())))?;
    eprintln!("Wrote atlas PNG: {}", atlas_png_path.display());

    // Write atlas TOML.
    let atlas_id = format!("{entity}_atlas");
    let atlas_toml = toml_output::AtlasOutput {
        atlas_id: atlas_id.clone(),
        width: packed.result.atlas_width,
        height: packed.result.atlas_height,
        frames: packed
            .result
            .frames
            .iter()
            .map(|f| toml_output::AtlasFrameOutput {
                name: f.name.clone(),
                x: f.x,
                y: f.y,
                w: f.width,
                h: f.height,
            })
            .collect(),
    };
    let atlas_toml_str = toml_output::write_atlas_toml(&atlas_toml)?;
    let atlas_toml_path = output.join(format!("{entity}_atlas.toml"));
    std::fs::write(&atlas_toml_path, atlas_toml_str)
        .map_err(|e| AnimPackError::io(&atlas_toml_path, e))?;
    eprintln!("Wrote atlas TOML: {}", atlas_toml_path.display());

    // Write animations TOML.
    let clips = build_animation_clips(frames, 12);
    let anim_toml_str = toml_output::write_animations_toml(entity, &atlas_id, &clips)?;
    let anim_toml_path = output.join(format!("{entity}_animations.toml"));
    std::fs::write(&anim_toml_path, anim_toml_str)
        .map_err(|e| AnimPackError::io(&anim_toml_path, e))?;
    eprintln!("Wrote animations TOML: {}", anim_toml_path.display());

    Ok(())
}

/// Build animation clips by grouping frames by action+direction.
fn build_animation_clips(
    frames: &[NamedFrame],
    default_fps: u32,
) -> Vec<toml_output::AnimClipOutput> {
    use std::collections::BTreeMap;

    let mut groups: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (name, _, _, _) in frames {
        if let Ok((_entity, action, dir, _frame)) = naming::parse_frame_name(name) {
            let clip_key = format!("{action}_{dir}");
            groups.entry(clip_key).or_default().push(name.clone());
        }
    }

    groups
        .into_iter()
        .map(|(clip_name, mut frame_names)| {
            frame_names.sort();
            toml_output::AnimClipOutput {
                name: clip_name,
                fps: default_fps,
                looping: true,
                frame_names,
            }
        })
        .collect()
}

/// Run the `validate` command.
fn run_validate(input: &Path) -> Result<(), AnimPackError> {
    let content =
        std::fs::read_to_string(input).map_err(|e| AnimPackError::io(input, e))?;

    let atlas: toml_output::AtlasOutput =
        toml::from_str(&content).map_err(|e| AnimPackError::TomlSerialize(e.to_string()))?;

    eprintln!("Atlas: {} ({}x{})", atlas.atlas_id, atlas.width, atlas.height);
    eprintln!("Frames: {}", atlas.frames.len());

    let mut errors = 0;
    for frame in &atlas.frames {
        if frame.x + frame.w > atlas.width || frame.y + frame.h > atlas.height {
            eprintln!(
                "  ERROR: Frame '{}' at ({},{}) size {}x{} exceeds atlas bounds",
                frame.name, frame.x, frame.y, frame.w, frame.h
            );
            errors += 1;
        }
    }

    let names: Vec<&str> = atlas.frames.iter().map(|f| f.name.as_str()).collect();
    let naming_errors = naming::validate_naming(&names);
    for err in &naming_errors {
        eprintln!("  NAMING: {err}");
    }
    errors += naming_errors.len();

    if errors == 0 {
        eprintln!("Validation passed.");
    } else {
        eprintln!("{errors} error(s) found.");
    }

    Ok(())
}

/// Run the `info` command.
fn run_info(input: &Path) -> Result<(), AnimPackError> {
    let ase_data = aseprite::load_aseprite(input)?;

    eprintln!("File: {}", input.display());
    eprintln!("Size: {}x{}", ase_data.width, ase_data.height);
    eprintln!("Frames: {}", ase_data.frames.len());
    eprintln!("Tags: {}", ase_data.tags.len());

    for (i, tag) in ase_data.tags.iter().enumerate() {
        eprintln!(
            "  Tag {i}: '{}' frames {}-{}",
            tag.name, tag.from_frame, tag.to_frame
        );
    }

    for frame in &ase_data.frames {
        eprintln!(
            "  Frame {}: {}ms, {} bytes",
            frame.index,
            frame.duration_ms,
            frame.pixels.len()
        );
    }

    Ok(())
}
