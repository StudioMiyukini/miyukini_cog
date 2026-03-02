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

    /// The instanced sprite batcher exceeded its maximum capacity.
    #[error("batcher overflow: capacity {capacity}, attempted {attempted}")]
    BatcherOverflow {
        /// Maximum allowed instances.
        capacity: usize,
        /// Number of instances that were attempted.
        attempted: usize,
    },

    /// The texture array has no remaining layers.
    #[error("texture array full: max layers {max_layers}")]
    TextureArrayFull {
        /// Maximum number of layers the texture array supports.
        max_layers: u32,
    },

    /// A shader failed to compile.
    #[error("shader compilation failed: {details}")]
    ShaderCompilationFailed {
        /// Detailed shader compilation error message.
        details: String,
    },

    /// A texture had unexpected dimensions.
    #[error(
        "invalid texture size: expected {expected_w}x{expected_h}, got {got_w}x{got_h}"
    )]
    InvalidTextureSize {
        /// Expected width.
        expected_w: u32,
        /// Expected height.
        expected_h: u32,
        /// Actual width.
        got_w: u32,
        /// Actual height.
        got_h: u32,
    },
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_error_implements_display() {
        let err = RenderError::BatcherOverflow {
            capacity: 100,
            attempted: 101,
        };
        let msg = format!("{err}");
        assert!(msg.contains("100"));
        assert!(msg.contains("101"));
    }

    #[test]
    fn render_error_implements_error_trait() {
        let err = RenderError::TextureArrayFull { max_layers: 16 };
        // std::error::Error is implemented by thiserror.
        let _as_error: &dyn std::error::Error = &err;
        let msg = format!("{err}");
        assert!(msg.contains("16"));
    }

    #[test]
    fn render_error_shader_compilation_failed_display() {
        let err = RenderError::ShaderCompilationFailed {
            details: "syntax error at line 42".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("syntax error at line 42"));
    }

    #[test]
    fn render_error_invalid_texture_size_display() {
        let err = RenderError::InvalidTextureSize {
            expected_w: 512,
            expected_h: 512,
            got_w: 256,
            got_h: 256,
        };
        let msg = format!("{err}");
        assert!(msg.contains("512x512"));
        assert!(msg.contains("256x256"));
    }
}
