use serde::{Deserialize, Serialize};

/// System menu section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuSection {
    Pause,
    Video,
    Audio,
    Gameplay,
    Controls,
}

/// A configurable option entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OptionValue {
    Toggle(bool),
    Slider { value: f32, min: f32, max: f32 },
    Choice { selected: usize, options: Vec<String> },
}

/// A single option in the settings menu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionEntry {
    pub key: String,
    pub label: String,
    pub value: OptionValue,
}

/// System menu state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMenu {
    pub active_section: MenuSection,
    pub sections: Vec<MenuSection>,
    pub entries: Vec<OptionEntry>,
    pub has_unsaved_changes: bool,
}

impl Default for SystemMenu {
    fn default() -> Self {
        Self {
            active_section: MenuSection::Pause,
            sections: vec![
                MenuSection::Pause,
                MenuSection::Video,
                MenuSection::Audio,
                MenuSection::Gameplay,
                MenuSection::Controls,
            ],
            entries: Vec::new(),
            has_unsaved_changes: false,
        }
    }
}

impl SystemMenu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_section(&mut self, section: MenuSection) {
        self.active_section = section;
    }

    pub fn add_entry(&mut self, entry: OptionEntry) {
        self.entries.push(entry);
    }

    /// Find an option by key.
    pub fn get_entry(&self, key: &str) -> Option<&OptionEntry> {
        self.entries.iter().find(|e| e.key == key)
    }

    /// Modify an option by key. Marks unsaved changes.
    pub fn set_toggle(&mut self, key: &str, value: bool) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            if let OptionValue::Toggle(ref mut v) = entry.value {
                *v = value;
                self.has_unsaved_changes = true;
                return true;
            }
        }
        false
    }

    pub fn set_slider(&mut self, key: &str, value: f32) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            if let OptionValue::Slider { value: ref mut v, min, max } = entry.value {
                *v = value.clamp(min, max);
                self.has_unsaved_changes = true;
                return true;
            }
        }
        false
    }

    pub fn set_choice(&mut self, key: &str, index: usize) -> bool {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.key == key) {
            if let OptionValue::Choice { ref mut selected, ref options } = entry.value {
                if index < options.len() {
                    *selected = index;
                    self.has_unsaved_changes = true;
                    return true;
                }
            }
        }
        false
    }

    /// Mark changes as saved.
    pub fn mark_saved(&mut self) {
        self.has_unsaved_changes = false;
    }

    /// Build default video settings.
    pub fn default_video_entries() -> Vec<OptionEntry> {
        vec![
            OptionEntry {
                key: "fullscreen".into(),
                label: "Fullscreen".into(),
                value: OptionValue::Toggle(false),
            },
            OptionEntry {
                key: "vsync".into(),
                label: "VSync".into(),
                value: OptionValue::Toggle(true),
            },
            OptionEntry {
                key: "resolution".into(),
                label: "Resolution".into(),
                value: OptionValue::Choice {
                    selected: 0,
                    options: vec!["1280x720".into(), "1920x1080".into(), "2560x1440".into()],
                },
            },
        ]
    }

    /// Build default audio settings.
    pub fn default_audio_entries() -> Vec<OptionEntry> {
        vec![
            OptionEntry {
                key: "master_volume".into(),
                label: "Master Volume".into(),
                value: OptionValue::Slider { value: 1.0, min: 0.0, max: 1.0 },
            },
            OptionEntry {
                key: "music_volume".into(),
                label: "Music Volume".into(),
                value: OptionValue::Slider { value: 0.7, min: 0.0, max: 1.0 },
            },
            OptionEntry {
                key: "sfx_volume".into(),
                label: "SFX Volume".into(),
                value: OptionValue::Slider { value: 1.0, min: 0.0, max: 1.0 },
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_menu_sections() {
        let menu = SystemMenu::new();
        assert_eq!(menu.sections.len(), 5);
        assert_eq!(menu.active_section, MenuSection::Pause);
    }

    #[test]
    fn toggle_option() {
        let mut menu = SystemMenu::new();
        menu.add_entry(OptionEntry {
            key: "fullscreen".into(),
            label: "Fullscreen".into(),
            value: OptionValue::Toggle(false),
        });
        assert!(menu.set_toggle("fullscreen", true));
        assert!(menu.has_unsaved_changes);
        if let Some(entry) = menu.get_entry("fullscreen") {
            if let OptionValue::Toggle(v) = entry.value {
                assert!(v);
            }
        }
    }

    #[test]
    fn slider_clamps() {
        let mut menu = SystemMenu::new();
        menu.add_entry(OptionEntry {
            key: "volume".into(),
            label: "Volume".into(),
            value: OptionValue::Slider { value: 0.5, min: 0.0, max: 1.0 },
        });
        assert!(menu.set_slider("volume", 2.0));
        if let Some(entry) = menu.get_entry("volume") {
            if let OptionValue::Slider { value, .. } = entry.value {
                assert!((value - 1.0).abs() < f32::EPSILON);
            }
        }
    }

    #[test]
    fn choice_rejects_oob() {
        let mut menu = SystemMenu::new();
        menu.add_entry(OptionEntry {
            key: "res".into(),
            label: "Resolution".into(),
            value: OptionValue::Choice {
                selected: 0,
                options: vec!["720p".into(), "1080p".into()],
            },
        });
        assert!(!menu.set_choice("res", 5));
        assert!(menu.set_choice("res", 1));
    }
}
