// @id: MGE-Studio-Project @do: studio-project @role: back-end @layer: 6 @human: miyuk
//! Studio project manifest.

/// Metadata for a studio project.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectMeta {
    /// Project display name.
    pub name: String,
    /// Project version string.
    pub version: String,
    /// Path to the game data root directory.
    pub data_dir: String,
    /// Path to the assets directory.
    pub assets_dir: String,
}

impl Default for ProjectMeta {
    fn default() -> Self {
        Self {
            name: String::from("Untitled"),
            version: String::from("0.1.0"),
            data_dir: String::from("data/"),
            assets_dir: String::from("assets/"),
        }
    }
}

/// A studio project holding all editor state.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StudioProject {
    /// Project metadata.
    pub meta: ProjectMeta,
    /// Whether the project has unsaved changes.
    pub dirty: bool,
}

impl StudioProject {
    /// Create a new empty project with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            meta: ProjectMeta {
                name: name.into(),
                ..ProjectMeta::default()
            },
            dirty: false,
        }
    }

    /// Mark the project as having unsaved changes.
    pub fn mark_dirty(&mut self) {
        self.dirty = true;
    }

    /// Mark the project as saved (clear dirty flag).
    pub fn mark_saved(&mut self) {
        self.dirty = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_project() {
        let proj = StudioProject::new("Test");
        assert_eq!(proj.meta.name, "Test");
        assert!(!proj.dirty);
    }

    #[test]
    fn test_mark_dirty() {
        let mut proj = StudioProject::new("Test");
        proj.mark_dirty();
        assert!(proj.dirty);
    }

    #[test]
    fn test_mark_saved() {
        let mut proj = StudioProject::new("Test");
        proj.mark_dirty();
        assert!(proj.dirty);
        proj.mark_saved();
        assert!(!proj.dirty);
    }
}
