// @id: MGE-AnimPack-Errors @do: error-types @role: back-end @layer: 6 @human: francois
//! Error types for the animation atlas packer.

use std::path::PathBuf;

/// All errors that can occur in the animation atlas packer.
#[derive(Debug, thiserror::Error)]
pub enum AnimPackError {
    /// Failed to read or write a file.
    #[error("I/O error on '{path}': {source}")]
    Io {
        /// The path that caused the error.
        path: PathBuf,
        /// The underlying I/O error.
        source: std::io::Error,
    },

    /// Failed to parse an Aseprite file.
    #[error("Aseprite parse error: {0}")]
    AsepriteParse(String),

    /// A filename does not match the expected naming convention.
    #[error("Invalid frame name: {0}")]
    InvalidFrameName(String),

    /// The atlas packer could not fit all frames.
    #[error("Atlas overflow: frames do not fit in {max_size}x{max_size} atlas")]
    AtlasOverflow {
        /// Maximum atlas dimension.
        max_size: u32,
    },

    /// No frames were provided to pack.
    #[error("No frames provided")]
    NoFrames,

    /// A frame has invalid pixel data.
    #[error("Invalid pixel data for frame '{name}': expected {expected} bytes, got {got}")]
    InvalidPixelData {
        /// Frame identifier.
        name: String,
        /// Expected byte count.
        expected: usize,
        /// Actual byte count.
        got: usize,
    },

    /// An unknown size class was requested.
    #[error("Unknown size class: '{0}'")]
    UnknownSizeClass(String),

    /// TOML serialisation failed.
    #[error("TOML serialisation error: {0}")]
    TomlSerialize(String),

    /// Image decode/encode error.
    #[error("Image error: {0}")]
    Image(String),

    /// Aseprite export is not yet implemented.
    #[error("Aseprite export not yet implemented: {0}")]
    ExportNotImplemented(String),
}

impl AnimPackError {
    /// Create an I/O error with a path context.
    pub fn io(path: impl Into<PathBuf>, source: std::io::Error) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}
