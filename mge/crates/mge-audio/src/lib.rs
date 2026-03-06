use serde::{Deserialize, Serialize};

pub type AudioBackend = kira::DefaultBackend;

/// Audio cue identifiers linked to gameplay events.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AudioCue {
    // Ambience
    TownAmbience,
    FieldAmbience,
    CaveAmbience,
    // UI
    UiClick,
    UiHover,
    InventoryOpen,
    InventoryClose,
    QuestComplete,
    LevelUp,
    // Combat
    SwordSwing,
    SwordHit,
    BowRelease,
    ArrowImpact,
    SpellCast,
    SpellImpact,
    MonsterHit,
    MonsterDeath,
    PlayerHurt,
    PlayerDeath,
    // World
    Footstep,
    DoorOpen,
    ChestOpen,
    PortalActivate,
    GoldPickup,
    ItemPickup,
    ItemDrop,
    // Custom named cue (for data-driven extensions).
    Named(String),
}

/// Audio bus category for volume mixing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BusCategory {
    Master,
    Music,
    Sfx,
    Ambience,
    Ui,
    Voice,
}

/// Volume settings per bus.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusVolume {
    pub category: BusCategory,
    pub volume: f32,
    pub muted: bool,
}

impl BusVolume {
    pub fn effective_volume(&self) -> f32 {
        if self.muted {
            0.0
        } else {
            self.volume.clamp(0.0, 1.0)
        }
    }
}

/// Audio mixer managing bus volumes and cue routing.
#[derive(Debug, Clone)]
pub struct AudioMixer {
    buses: Vec<BusVolume>,
    pending_cues: Vec<(AudioCue, BusCategory)>,
}

impl Default for AudioMixer {
    fn default() -> Self {
        Self {
            buses: vec![
                BusVolume { category: BusCategory::Master, volume: 1.0, muted: false },
                BusVolume { category: BusCategory::Music, volume: 0.7, muted: false },
                BusVolume { category: BusCategory::Sfx, volume: 1.0, muted: false },
                BusVolume { category: BusCategory::Ambience, volume: 0.5, muted: false },
                BusVolume { category: BusCategory::Ui, volume: 0.8, muted: false },
                BusVolume { category: BusCategory::Voice, volume: 1.0, muted: false },
            ],
            pending_cues: Vec::new(),
        }
    }
}

impl AudioMixer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue a cue to be played on the next audio tick.
    pub fn play_cue(&mut self, cue: AudioCue) {
        let category = cue_category(&cue);
        self.pending_cues.push((cue, category));
    }

    /// Drain pending cues for the audio backend to process.
    pub fn drain_cues(&mut self) -> Vec<(AudioCue, BusCategory, f32)> {
        let master_vol = self.bus_volume(BusCategory::Master);
        let buses = &self.buses;
        self.pending_cues
            .drain(..)
            .map(|(cue, cat)| {
                let bus_vol = buses
                    .iter()
                    .find(|b| b.category == cat)
                    .map_or(1.0, BusVolume::effective_volume);
                let final_vol = master_vol * bus_vol;
                (cue, cat, final_vol)
            })
            .collect()
    }

    pub fn set_volume(&mut self, category: BusCategory, volume: f32) {
        if let Some(bus) = self.buses.iter_mut().find(|b| b.category == category) {
            bus.volume = volume.clamp(0.0, 1.0);
        }
    }

    pub fn set_muted(&mut self, category: BusCategory, muted: bool) {
        if let Some(bus) = self.buses.iter_mut().find(|b| b.category == category) {
            bus.muted = muted;
        }
    }

    pub fn bus_volume(&self, category: BusCategory) -> f32 {
        self.buses.iter().find(|b| b.category == category).map_or(1.0, BusVolume::effective_volume)
    }

    pub fn pending_count(&self) -> usize {
        self.pending_cues.len()
    }
}

/// Map a cue to its bus category.
fn cue_category(cue: &AudioCue) -> BusCategory {
    match cue {
        AudioCue::TownAmbience | AudioCue::FieldAmbience | AudioCue::CaveAmbience => {
            BusCategory::Ambience
        }
        AudioCue::UiClick
        | AudioCue::UiHover
        | AudioCue::InventoryOpen
        | AudioCue::InventoryClose
        | AudioCue::QuestComplete
        | AudioCue::LevelUp => BusCategory::Ui,
        _ => BusCategory::Sfx,
    }
}

// Keep backward compat for sodomight main.rs which uses AudioBus.
#[derive(Debug, Clone)]
pub struct AudioBus {
    bus_name: String,
    mixer: AudioMixer,
}

impl AudioBus {
    pub fn new(bus_name: impl Into<String>) -> Self {
        Self { bus_name: bus_name.into(), mixer: AudioMixer::new() }
    }

    pub fn bus_name(&self) -> &str {
        &self.bus_name
    }

    pub fn push_cue(&mut self, cue: AudioCue) {
        self.mixer.play_cue(cue);
    }

    pub fn cue_count(&self) -> usize {
        self.mixer.pending_count()
    }

    pub fn mixer(&self) -> &AudioMixer {
        &self.mixer
    }

    pub fn mixer_mut(&mut self) -> &mut AudioMixer {
        &mut self.mixer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixer_routes_cues_to_categories() {
        let mut mixer = AudioMixer::new();
        mixer.play_cue(AudioCue::SwordSwing);
        mixer.play_cue(AudioCue::UiClick);
        mixer.play_cue(AudioCue::TownAmbience);

        let drained = mixer.drain_cues();
        assert_eq!(drained.len(), 3);
        assert_eq!(drained[0].1, BusCategory::Sfx);
        assert_eq!(drained[1].1, BusCategory::Ui);
        assert_eq!(drained[2].1, BusCategory::Ambience);
    }

    #[test]
    fn muted_bus_returns_zero_volume() {
        let mut mixer = AudioMixer::new();
        mixer.set_muted(BusCategory::Sfx, true);
        mixer.play_cue(AudioCue::SwordHit);

        let drained = mixer.drain_cues();
        assert!(drained[0].2 < f32::EPSILON, "muted bus should yield 0 volume");
    }

    #[test]
    fn volume_clamps_to_range() {
        let mut mixer = AudioMixer::new();
        mixer.set_volume(BusCategory::Master, 5.0);
        assert!((mixer.bus_volume(BusCategory::Master) - 1.0).abs() < f32::EPSILON);
        mixer.set_volume(BusCategory::Master, -1.0);
        assert!(mixer.bus_volume(BusCategory::Master) < f32::EPSILON);
    }

    #[test]
    fn drain_clears_pending() {
        let mut mixer = AudioMixer::new();
        mixer.play_cue(AudioCue::Footstep);
        assert_eq!(mixer.pending_count(), 1);
        let _ = mixer.drain_cues();
        assert_eq!(mixer.pending_count(), 0);
    }

    #[test]
    fn named_cue_maps_to_sfx() {
        let cat = cue_category(&AudioCue::Named("custom_explosion".into()));
        assert_eq!(cat, BusCategory::Sfx);
    }

    #[test]
    fn audio_bus_backward_compat() {
        let mut bus = AudioBus::new("master");
        bus.push_cue(AudioCue::TownAmbience);
        assert_eq!(bus.cue_count(), 1);
        assert_eq!(bus.bus_name(), "master");
    }
}
