// @id: MGE-Studio-Atlas @do: atlas-editor @role: back-end @layer: 6 @human: miyuk
//! Atlas editor state: manage frame entries for sprite sheet creation.

use crate::error::StudioError;

/// A single frame entry in the atlas editor.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FrameEntry {
    /// Frame identifier (used in animation definitions).
    pub id: String,
    /// Source image file path (relative to assets dir).
    pub source_path: String,
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
}

impl FrameEntry {
    /// Create a new frame entry.
    pub fn new(
        id: impl Into<String>,
        source_path: impl Into<String>,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            id: id.into(),
            source_path: source_path.into(),
            width,
            height,
        }
    }
}

/// Atlas editor state: list of frames to pack into a sprite sheet.
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct AtlasEditorState {
    /// Output atlas name (used as filename without extension).
    pub atlas_name: String,
    /// Frames to include in the atlas.
    pub frames: Vec<FrameEntry>,
    /// Maximum atlas side length (power of two recommended).
    pub max_size: u32,
    /// Pixel margin between frames.
    pub margin: u32,
}

impl AtlasEditorState {
    /// Create a new atlas editor state with sensible defaults.
    pub fn new(atlas_name: impl Into<String>) -> Self {
        Self {
            atlas_name: atlas_name.into(),
            frames: Vec::new(),
            max_size: 2048,
            margin: 1,
        }
    }

    /// Add a frame to the atlas. Returns error if the frame has zero dimensions.
    pub fn add_frame(&mut self, frame: FrameEntry) -> Result<(), StudioError> {
        if frame.width == 0 || frame.height == 0 {
            return Err(StudioError::AtlasError {
                message: format!("Frame '{}' has zero dimensions", frame.id),
            });
        }
        self.frames.push(frame);
        Ok(())
    }

    /// Remove a frame by id. Returns true if found and removed.
    pub fn remove_frame(&mut self, id: &str) -> bool {
        if let Some(pos) = self.frames.iter().position(|f| f.id == id) {
            self.frames.remove(pos);
            true
        } else {
            false
        }
    }

    /// Find a frame by id.
    pub fn find_frame(&self, id: &str) -> Option<&FrameEntry> {
        self.frames.iter().find(|f| f.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_frame() {
        let mut atlas = AtlasEditorState::new("test_atlas");
        let frame = FrameEntry::new("idle_0", "sprites/idle_0.png", 64, 64);
        assert!(atlas.add_frame(frame).is_ok());
        assert_eq!(atlas.frames.len(), 1);
    }

    #[test]
    fn test_add_frame_zero_size() {
        let mut atlas = AtlasEditorState::new("test_atlas");
        let frame = FrameEntry::new("bad", "sprites/bad.png", 0, 64);
        let result = atlas.add_frame(frame);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, StudioError::AtlasError { .. }));
    }

    #[test]
    fn test_remove_frame_exists() {
        let mut atlas = AtlasEditorState::new("test_atlas");
        let frame = FrameEntry::new("idle_0", "sprites/idle_0.png", 64, 64);
        atlas.add_frame(frame).unwrap();
        assert!(atlas.remove_frame("idle_0"));
        assert!(atlas.frames.is_empty());
    }

    #[test]
    fn test_remove_frame_not_exists() {
        let mut atlas = AtlasEditorState::new("test_atlas");
        assert!(!atlas.remove_frame("nonexistent"));
    }

    #[test]
    fn test_find_frame() {
        let mut atlas = AtlasEditorState::new("test_atlas");
        let frame = FrameEntry::new("walk_0", "sprites/walk_0.png", 48, 48);
        atlas.add_frame(frame).unwrap();
        let found = atlas.find_frame("walk_0");
        assert!(found.is_some());
        assert_eq!(found.unwrap().width, 48);
        assert!(atlas.find_frame("missing").is_none());
    }
}
