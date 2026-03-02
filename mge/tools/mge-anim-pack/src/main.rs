// @id: MGE-AnimPack-Main @do: cli-entry @role: back-end @layer: 6 @human: francois
//! CLI entry point for the animation atlas packer.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

    match cli.command {
        Command::Pack {
            input,
            output,
            entity,
            max_size,
            margin,
            mirror,
        } => {
            eprintln!(
                "pack: input={} output={} entity={entity} max_size={max_size} margin={margin} mirror={mirror}",
                input.display(),
                output.display(),
            );
            eprintln!("TODO: implement pack pipeline");
        }
        Command::Validate { input } => {
            eprintln!("validate: input={}", input.display());
            eprintln!("TODO: implement validate");
        }
        Command::Info { input } => {
            eprintln!("info: input={}", input.display());
            eprintln!("TODO: implement info");
        }
        Command::ImportPng {
            input,
            output,
            entity,
            size_class,
            action,
            directions,
            fps,
            aseprite,
        } => {
            eprintln!(
                "import-png: input={} output={} entity={entity} size_class={size_class} action={action} directions={directions} fps={fps} aseprite={aseprite}",
                input.display(),
                output.display(),
            );
            eprintln!("TODO: implement import-png");
        }
    }
}
