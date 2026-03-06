use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerProfile {
    pub display_name: String,
    pub last_scene: String,
    pub level: u32,
}

#[derive(Debug, Clone)]
pub struct SaveManager {
    root: PathBuf,
}

impl SaveManager {
    pub fn new(game_id: &str) -> Result<Self> {
        let base = env::var_os("LOCALAPPDATA").map_or_else(
            || env::current_dir().expect("cwd available").join(".runtime"),
            PathBuf::from,
        );
        let root = base.join("Miyukini-COG").join(game_id);
        let manager = Self { root };
        manager.ensure_layout()?;
        Ok(manager)
    }

    pub fn ensure_layout(&self) -> Result<()> {
        fs::create_dir_all(self.saves_dir())
            .with_context(|| format!("failed to create {}", self.saves_dir().display()))?;
        fs::create_dir_all(self.logs_dir())
            .with_context(|| format!("failed to create {}", self.logs_dir().display()))?;
        Ok(())
    }

    pub fn saves_dir(&self) -> PathBuf {
        self.root.join("saves")
    }

    pub fn logs_dir(&self) -> PathBuf {
        self.root.join("logs")
    }

    pub fn autosave_path(&self) -> PathBuf {
        self.saves_dir().join("autosave.ron")
    }

    pub fn save_profile(&self, profile: &PlayerProfile) -> Result<()> {
        let payload = ron::to_string(profile).context("failed to serialize player profile")?;
        fs::write(self.autosave_path(), payload)
            .with_context(|| format!("failed to write {}", self.autosave_path().display()))
    }

    pub fn load_profile(&self) -> Result<Option<PlayerProfile>> {
        let path = self.autosave_path();
        if !path.exists() {
            return Ok(None);
        }

        let raw = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let profile =
            ron::from_str(&raw).with_context(|| format!("failed to parse {}", path.display()))?;
        Ok(Some(profile))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_roundtrip_works() {
        let game_id = format!("sodomight-test-{}", std::process::id());
        let manager = SaveManager::new(&game_id).expect("save manager");
        let profile = PlayerProfile {
            display_name: "Miyuki".into(),
            last_scene: "act1_rogue_camp".into(),
            level: 3,
        };

        manager.save_profile(&profile).expect("save profile");
        let restored = manager.load_profile().expect("load profile").expect("profile");
        assert_eq!(restored.display_name, "Miyuki");
        assert_eq!(restored.level, 3);
    }
}
