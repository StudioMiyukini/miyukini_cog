// @id: MGE-Platform-Error @do: platform-error @role: back-end @layer: 1 @human: miyuk
//! Platform error types.

/// Errors that can occur during platform operations.
#[derive(Debug, thiserror::Error)]
pub enum PlatformError {
    /// Failed to create the application window.
    #[error("Window creation failed: {message}")]
    WindowCreation {
        /// Description of the window creation failure.
        message: String,
    },
    /// Failed to initialize the GPU backend.
    #[error("GPU initialization failed: {message}")]
    GpuInit {
        /// Description of the GPU initialization failure.
        message: String,
    },
    /// A required platform feature is not available.
    #[error("Unsupported feature: {feature}")]
    UnsupportedFeature {
        /// Name of the unsupported feature.
        feature: String,
    },
    /// The event loop encountered an error.
    #[error("Event loop error: {message}")]
    EventLoop {
        /// Description of the event loop error.
        message: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_creation_error() {
        let err = PlatformError::WindowCreation {
            message: "no display".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("Window creation failed"));
        assert!(msg.contains("no display"));
    }

    #[test]
    fn test_gpu_init_error() {
        let err = PlatformError::GpuInit {
            message: "no adapter".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("GPU initialization failed"));
        assert!(msg.contains("no adapter"));
    }

    #[test]
    fn test_unsupported_feature() {
        let err = PlatformError::UnsupportedFeature {
            feature: "ray_tracing".to_string(),
        };
        let msg = format!("{err}");
        assert!(msg.contains("Unsupported feature"));
        assert!(msg.contains("ray_tracing"));
    }
}
