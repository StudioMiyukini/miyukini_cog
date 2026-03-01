// @id: sd-impl-errors @do: implement @role: back-end @layer: 2 @human: miyuk

//! Error types for the render crate.

/// Result alias for render operations.
pub type RenderResult<T> = Result<T, RenderError>;

/// All error types produced by `mge-render`.
#[derive(Debug, thiserror::Error)]
pub enum RenderError {
    /// A wgpu operation failed.
    #[error("wgpu error: {message}")]
    Wgpu {
        /// Description of the wgpu failure.
        message: String,
    },

    /// The swap-chain surface texture could not be acquired.
    #[error("surface error: {message}")]
    SurfaceError {
        /// Description of the surface failure.
        message: String,
    },

    /// A texture atlas was requested but not found in the registry.
    #[error("atlas not found: {message}")]
    AtlasNotFound {
        /// The atlas identifier that was missing.
        message: String,
    },

    /// A shader failed to compile or load.
    #[error("shader error: {message}")]
    ShaderError {
        /// Description of the shader failure.
        message: String,
    },

    /// An image could not be decoded.
    #[error("image load error: {message}")]
    ImageLoad {
        /// Description of the image decoding failure.
        message: String,
    },

    /// A TOML descriptor could not be parsed.
    #[error("toml parse error: {message}")]
    TomlParse {
        /// Description of the TOML parse failure.
        message: String,
    },
}
