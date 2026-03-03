// @id: MGE-Render-Overhead @do: implement @role: back-end @layer: 2 @human: francois

//! World-space overhead UI: floating combat text, emotes, and progress bars.
//!
//! This module provides three overlay systems rendered above entities in
//! world-space, inspired by Ragnarok Online:
//!
//! - [`FloatingText`] / [`FloatingTextManager`] -- damage numbers, heals,
//!   XP gains, and other combat / interaction feedback.
//! - [`SpriteEmote`] / [`EmoteManager`] -- icon emotes above entities
//!   (happy, angry, question, etc.).
//! - [`OverheadProgressBar`] / [`ProgressBarManager`] -- progress bars for
//!   gathering, crafting, and casting.
//!
//! All managers are CPU-side; actual rendering is delegated to the client
//! which reads the public accessors and produces GPU draw calls.

// ---------------------------------------------------------------------------
// FloatingTextKind
// ---------------------------------------------------------------------------

/// Type of floating text, each with a distinct visual preset.
///
/// Visual style inspired by Ragnarok Online: damage in red, critical in
/// white-on-red, dodge/block as keyword labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FloatingTextKind {
    /// Physical damage (red, 20px) — Ragnarok Online style.
    Damage,
    /// Critical hit (white text, 26px, shake) — rendered with red background
    /// quad by the client.
    Critical,
    /// Evade / dodge (yellow #FFD700, 16px, "DODGE").
    Evade,
    /// Blocked hit (cyan #66CCFF, 18px, "BLOCK").
    Block,
    /// Heal (green #44FF44, 16px, "+HP").
    Heal,
    /// Gathered resource (gold #FFD700, 14px, "+3 Iron Ore").
    Gathered,
    /// Success feedback (green #22CC22, 16px).
    Success,
    /// Failure feedback (red #FF4444, 16px).
    Failure,
    /// Experience gained (purple #AA66FF, 14px).
    Experience,
}

// ---------------------------------------------------------------------------
// FloatingTextPreset
// ---------------------------------------------------------------------------

/// Visual presets for a floating text kind.
#[derive(Debug, Clone, Copy)]
pub struct FloatingTextPreset {
    /// RGBA colour (linear, 0.0..1.0).
    pub color: [f32; 4],
    /// Font size in logical pixels.
    pub font_size: f32,
    /// Rise speed in pixels per second (upward).
    pub rise_speed: f32,
    /// Total lifetime in seconds before removal.
    pub lifetime: f32,
    /// Whether horizontal shake is applied (e.g. critical hits).
    pub has_shake: bool,
}

impl FloatingTextKind {
    /// Return the visual preset for this kind.
    #[must_use]
    pub fn preset(self) -> FloatingTextPreset {
        match self {
            Self::Damage => FloatingTextPreset {
                color: [1.0, 0.15, 0.15, 1.0], // bright red (RO style)
                font_size: 20.0,
                rise_speed: 45.0,
                lifetime: 1.2,
                has_shake: false,
            },
            Self::Critical => FloatingTextPreset {
                color: [1.0, 1.0, 1.0, 1.0], // white text (rendered on red bg)
                font_size: 26.0,
                rise_speed: 50.0,
                lifetime: 1.5,
                has_shake: true,
            },
            Self::Evade => FloatingTextPreset {
                color: [1.0, 0.84, 0.0, 1.0], // gold/yellow (RO "MISS"/"DODGE")
                font_size: 16.0,
                rise_speed: 35.0,
                lifetime: 1.0,
                has_shake: false,
            },
            Self::Block => FloatingTextPreset {
                color: [0.4, 0.8, 1.0, 1.0], // cyan-blue
                font_size: 18.0,
                rise_speed: 35.0,
                lifetime: 1.0,
                has_shake: false,
            },
            Self::Heal => FloatingTextPreset {
                color: [0.267, 1.0, 0.267, 1.0],
                font_size: 16.0,
                rise_speed: 40.0,
                lifetime: 1.2,
                has_shake: false,
            },
            Self::Gathered => FloatingTextPreset {
                color: [1.0, 0.843, 0.0, 1.0],
                font_size: 14.0,
                rise_speed: 30.0,
                lifetime: 2.0,
                has_shake: false,
            },
            Self::Success => FloatingTextPreset {
                color: [0.133, 0.8, 0.133, 1.0],
                font_size: 16.0,
                rise_speed: 35.0,
                lifetime: 1.5,
                has_shake: false,
            },
            Self::Failure => FloatingTextPreset {
                color: [1.0, 0.267, 0.267, 1.0],
                font_size: 16.0,
                rise_speed: 35.0,
                lifetime: 1.5,
                has_shake: false,
            },
            Self::Experience => FloatingTextPreset {
                color: [0.667, 0.4, 1.0, 1.0],
                font_size: 14.0,
                rise_speed: 30.0,
                lifetime: 2.0,
                has_shake: false,
            },
        }
    }
}

// ---------------------------------------------------------------------------
// FloatingText
// ---------------------------------------------------------------------------

/// A floating text element in world space.
///
/// Created via [`FloatingText::new`] which applies the visual preset for
/// the given [`FloatingTextKind`]. The client ticks the manager each frame
/// to update `age`, `y_offset`, and `opacity`.
pub struct FloatingText {
    /// World-space origin `[x, y]`.
    pub world_pos: [f32; 2],
    /// Text content (e.g. "42", "MISS", "+3 Iron Ore").
    pub text: String,
    /// The kind of floating text (determines colour, size, etc.).
    pub kind: FloatingTextKind,
    /// Elapsed time in seconds since spawn.
    pub age: f32,
    /// Total lifetime before removal (seconds).
    pub lifetime: f32,
    /// Current vertical offset (increases with rise speed).
    pub y_offset: f32,
    /// Current opacity (1.0 at spawn, fades to 0.0 at lifetime).
    pub opacity: f32,
    /// Rise speed in pixels per second.
    pub rise_speed: f32,
    /// Font size in logical pixels.
    pub font_size: f32,
    /// RGBA colour.
    pub color: [f32; 4],
    /// Whether horizontal shake is applied.
    pub has_shake: bool,
}

impl FloatingText {
    /// Create a new floating text from a kind preset.
    #[must_use]
    pub fn new(world_pos: [f32; 2], text: String, kind: FloatingTextKind) -> Self {
        let preset = kind.preset();
        Self {
            world_pos,
            text,
            kind,
            age: 0.0,
            lifetime: preset.lifetime,
            y_offset: 0.0,
            opacity: 1.0,
            rise_speed: preset.rise_speed,
            font_size: preset.font_size,
            color: preset.color,
            has_shake: preset.has_shake,
        }
    }
}

// ---------------------------------------------------------------------------
// FloatingTextManager
// ---------------------------------------------------------------------------

/// Manages a pool of floating texts with automatic cleanup.
///
/// The manager enforces a maximum capacity. When the cap is reached, the
/// oldest text is removed before spawning a new one.
pub struct FloatingTextManager {
    /// Active floating texts.
    texts: Vec<FloatingText>,
    /// Maximum number of simultaneous texts.
    max_texts: usize,
}

impl FloatingTextManager {
    /// Create a new manager with the given capacity.
    #[must_use]
    pub fn new(max_texts: usize) -> Self {
        Self {
            texts: Vec::with_capacity(max_texts),
            max_texts,
        }
    }

    /// Spawn a new floating text at the given world position.
    ///
    /// If the pool is full, the oldest text is evicted first.
    pub fn spawn(&mut self, world_pos: [f32; 2], text: String, kind: FloatingTextKind) {
        if self.texts.len() >= self.max_texts {
            self.texts.remove(0);
        }
        self.texts.push(FloatingText::new(world_pos, text, kind));
    }

    /// Advance all texts by `dt` seconds and remove expired ones.
    pub fn tick(&mut self, dt: f32) {
        for text in &mut self.texts {
            text.age += dt;
            text.y_offset += text.rise_speed * dt;
            text.opacity = 1.0 - (text.age / text.lifetime);
            if text.opacity < 0.0 {
                text.opacity = 0.0;
            }
        }
        self.texts.retain(|t| t.age < t.lifetime);
    }

    /// Number of active floating texts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.texts.len()
    }

    /// Returns `true` when no floating texts are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.texts.is_empty()
    }

    /// Read-only access to the active texts for rendering.
    #[must_use]
    pub fn texts(&self) -> &[FloatingText] {
        &self.texts
    }
}

// ---------------------------------------------------------------------------
// EmoteKind
// ---------------------------------------------------------------------------

/// Predefined emote icon types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmoteKind {
    /// Happy / smile.
    Happy,
    /// Angry / rage.
    Angry,
    /// Sad / cry.
    Sad,
    /// Confused / dizzy.
    Confused,
    /// Exclamation mark (alert).
    Exclamation,
    /// Question mark (puzzled).
    Question,
    /// Heart (love).
    Heart,
    /// Sleep / ZZZ.
    Sleep,
    /// Skull (death / danger).
    Skull,
    /// Coin (loot / money).
    Coin,
}

impl EmoteKind {
    /// Index in the emote atlas (32x32px each, 10x1 grid).
    #[must_use]
    pub fn atlas_index(self) -> u32 {
        match self {
            Self::Happy => 0,
            Self::Angry => 1,
            Self::Sad => 2,
            Self::Confused => 3,
            Self::Exclamation => 4,
            Self::Question => 5,
            Self::Heart => 6,
            Self::Sleep => 7,
            Self::Skull => 8,
            Self::Coin => 9,
        }
    }
}

// ---------------------------------------------------------------------------
// SpriteEmote
// ---------------------------------------------------------------------------

/// An emote icon displayed above an entity in world space.
pub struct SpriteEmote {
    /// World-space position `[x, y]`.
    pub world_pos: [f32; 2],
    /// The emote icon type.
    pub kind: EmoteKind,
    /// Elapsed time since spawn (seconds).
    pub age: f32,
    /// Total lifetime before removal (seconds).
    pub lifetime: f32,
    /// Current opacity (fades in the last 0.5s).
    pub opacity: f32,
    /// Vertical bounce offset (sinusoidal).
    pub bounce_offset: f32,
    /// Entity ID for deduplication (one emote per entity).
    pub entity_id: u32,
}

// ---------------------------------------------------------------------------
// EmoteManager
// ---------------------------------------------------------------------------

/// Manages sprite emotes with one-per-entity deduplication.
pub struct EmoteManager {
    /// Active emotes.
    emotes: Vec<SpriteEmote>,
}

impl EmoteManager {
    /// Create a new emote manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            emotes: Vec::new(),
        }
    }

    /// Spawn an emote for the given entity.
    ///
    /// If the entity already has an active emote, it is replaced.
    pub fn spawn(&mut self, entity_id: u32, world_pos: [f32; 2], kind: EmoteKind) {
        self.emotes.retain(|e| e.entity_id != entity_id);
        self.emotes.push(SpriteEmote {
            world_pos,
            kind,
            age: 0.0,
            lifetime: 3.0,
            opacity: 1.0,
            bounce_offset: 0.0,
            entity_id,
        });
    }

    /// Advance all emotes by `dt` seconds, apply bounce and fade, and
    /// remove expired ones.
    pub fn tick(&mut self, dt: f32) {
        for emote in &mut self.emotes {
            emote.age += dt;
            // Bounce: amplitude 4px, period 0.5s.
            emote.bounce_offset = (emote.age * std::f32::consts::TAU / 0.5).sin() * 4.0;
            // Fade out in the last 0.5s.
            let remaining = emote.lifetime - emote.age;
            emote.opacity = if remaining < 0.5 {
                (remaining / 0.5).max(0.0)
            } else {
                1.0
            };
        }
        self.emotes.retain(|e| e.age < e.lifetime);
    }

    /// Number of active emotes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.emotes.len()
    }

    /// Returns `true` when no emotes are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.emotes.is_empty()
    }

    /// Read-only access to the active emotes for rendering.
    #[must_use]
    pub fn emotes(&self) -> &[SpriteEmote] {
        &self.emotes
    }
}

impl Default for EmoteManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// OverheadProgressBar
// ---------------------------------------------------------------------------

/// Colour preset: gathering (green).
pub const GATHERING_COLOR: [f32; 4] = [0.133, 0.8, 0.133, 1.0];
/// Colour preset: crafting (blue).
pub const CRAFTING_COLOR: [f32; 4] = [0.267, 0.533, 1.0, 1.0];
/// Colour preset: casting (purple).
pub const CASTING_COLOR: [f32; 4] = [0.533, 0.267, 0.8, 1.0];
/// Colour preset: loading / generic (white).
pub const LOADING_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 1.0];

/// A progress bar rendered above an entity in world space.
pub struct OverheadProgressBar {
    /// World-space position `[x, y]`.
    pub world_pos: [f32; 2],
    /// Current progress (clamped to 0.0..=1.0).
    pub progress: f32,
    /// Bar width in logical pixels (default 40.0).
    pub width: f32,
    /// Bar height in logical pixels (default 6.0).
    pub height: f32,
    /// Background colour (RGBA).
    pub bg_color: [f32; 4],
    /// Fill colour (RGBA).
    pub fill_color: [f32; 4],
    /// Whether the bar is visible.
    pub visible: bool,
    /// Entity ID (one bar per entity).
    pub entity_id: u32,
}

// ---------------------------------------------------------------------------
// ProgressBarManager
// ---------------------------------------------------------------------------

/// Manages overhead progress bars with one-per-entity semantics.
pub struct ProgressBarManager {
    /// Active progress bars.
    bars: Vec<OverheadProgressBar>,
}

impl ProgressBarManager {
    /// Create a new progress bar manager.
    #[must_use]
    pub fn new() -> Self {
        Self {
            bars: Vec::new(),
        }
    }

    /// Create or update a progress bar for the given entity.
    ///
    /// Progress is clamped to `[0.0, 1.0]`.
    pub fn set(
        &mut self,
        entity_id: u32,
        world_pos: [f32; 2],
        progress: f32,
        fill_color: [f32; 4],
    ) {
        let progress = progress.clamp(0.0, 1.0);
        if let Some(bar) = self.bars.iter_mut().find(|b| b.entity_id == entity_id) {
            bar.progress = progress;
            bar.fill_color = fill_color;
            bar.world_pos = world_pos;
            bar.visible = true;
        } else {
            self.bars.push(OverheadProgressBar {
                world_pos,
                progress,
                width: 40.0,
                height: 6.0,
                bg_color: [0.2, 0.2, 0.2, 0.8],
                fill_color,
                visible: true,
                entity_id,
            });
        }
    }

    /// Remove the progress bar for the given entity.
    pub fn remove(&mut self, entity_id: u32) {
        self.bars.retain(|b| b.entity_id != entity_id);
    }

    /// Number of active progress bars.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bars.len()
    }

    /// Returns `true` when no progress bars are active.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bars.is_empty()
    }

    /// Read-only access to the active bars for rendering.
    #[must_use]
    pub fn bars(&self) -> &[OverheadProgressBar] {
        &self.bars
    }
}

impl Default for ProgressBarManager {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // === S1c-T01 : FloatingTextKind + FloatingText ===

    #[test]
    fn floating_text_kind_presets() {
        // Each kind should have a distinct colour and lifetime.
        let all_kinds = [
            FloatingTextKind::Damage,
            FloatingTextKind::Critical,
            FloatingTextKind::Evade,
            FloatingTextKind::Block,
            FloatingTextKind::Heal,
            FloatingTextKind::Gathered,
            FloatingTextKind::Success,
            FloatingTextKind::Failure,
            FloatingTextKind::Experience,
        ];

        for (i, kind_a) in all_kinds.iter().enumerate() {
            let preset_a = kind_a.preset();
            // Lifetime must be positive.
            assert!(
                preset_a.lifetime > 0.0,
                "{kind_a:?} has non-positive lifetime"
            );
            // Font size must be positive.
            assert!(
                preset_a.font_size > 0.0,
                "{kind_a:?} has non-positive font_size"
            );
            // Rise speed must be positive.
            assert!(
                preset_a.rise_speed > 0.0,
                "{kind_a:?} has non-positive rise_speed"
            );
            // Each kind should differ from every other in at least colour or lifetime.
            for kind_b in &all_kinds[i + 1..] {
                let preset_b = kind_b.preset();
                let same_color = preset_a.color == preset_b.color;
                let same_lifetime =
                    (preset_a.lifetime - preset_b.lifetime).abs() < f32::EPSILON;
                assert!(
                    !same_color || !same_lifetime,
                    "{kind_a:?} and {kind_b:?} have identical colour AND lifetime"
                );
            }
        }
    }

    #[test]
    fn floating_text_new_defaults() {
        let ft = FloatingText::new([10.0, 20.0], "42".to_string(), FloatingTextKind::Damage);
        assert!((ft.age - 0.0).abs() < f32::EPSILON, "age should be 0");
        assert!(
            (ft.opacity - 1.0).abs() < f32::EPSILON,
            "opacity should be 1.0"
        );
        assert!(
            (ft.y_offset - 0.0).abs() < f32::EPSILON,
            "y_offset should be 0"
        );
        assert_eq!(ft.kind, FloatingTextKind::Damage);
        assert_eq!(ft.text, "42");
        assert_eq!(ft.world_pos, [10.0, 20.0]);
    }

    #[test]
    fn floating_text_critical_has_shake() {
        let ft = FloatingText::new([0.0, 0.0], "999".to_string(), FloatingTextKind::Critical);
        assert!(ft.has_shake, "Critical should have shake");
        assert!(
            (ft.font_size - 26.0).abs() < f32::EPSILON,
            "Critical font_size should be 26"
        );
    }

    // === S1c-T02 : FloatingTextManager ===

    #[test]
    fn floating_text_spawn_and_tick() {
        let mut mgr = FloatingTextManager::new(100);
        mgr.spawn([0.0, 0.0], "10".to_string(), FloatingTextKind::Damage);
        assert_eq!(mgr.len(), 1);

        mgr.tick(0.5);
        let t = &mgr.texts()[0];
        assert!(t.y_offset > 0.0, "y_offset should increase after tick");
        assert!(t.opacity < 1.0, "opacity should decrease after tick");
        assert!((t.age - 0.5).abs() < f32::EPSILON, "age should be 0.5");
    }

    #[test]
    fn floating_text_removal_after_lifetime() {
        let mut mgr = FloatingTextManager::new(100);
        mgr.spawn([0.0, 0.0], "1".to_string(), FloatingTextKind::Evade);
        // Evade lifetime = 1.0s
        assert_eq!(mgr.len(), 1);

        mgr.tick(1.5);
        assert_eq!(
            mgr.len(),
            0,
            "text should be removed after exceeding lifetime"
        );
    }

    #[test]
    fn floating_text_stacking() {
        let mut mgr = FloatingTextManager::new(100);
        mgr.spawn([5.0, 5.0], "A".to_string(), FloatingTextKind::Damage);
        mgr.spawn([5.0, 5.0], "B".to_string(), FloatingTextKind::Heal);
        mgr.spawn([5.0, 5.0], "C".to_string(), FloatingTextKind::Critical);
        assert_eq!(mgr.len(), 3, "3 texts at same pos should all exist");
    }

    #[test]
    fn floating_text_cap() {
        let mut mgr = FloatingTextManager::new(2);
        mgr.spawn([0.0, 0.0], "A".to_string(), FloatingTextKind::Damage);
        mgr.spawn([0.0, 0.0], "B".to_string(), FloatingTextKind::Damage);
        mgr.spawn([0.0, 0.0], "C".to_string(), FloatingTextKind::Damage);
        assert_eq!(mgr.len(), 2, "should cap at max_texts=2");
        // The oldest ("A") should have been evicted.
        assert_eq!(mgr.texts()[0].text, "B");
        assert_eq!(mgr.texts()[1].text, "C");
    }

    #[test]
    fn floating_text_manager_is_empty() {
        let mgr = FloatingTextManager::new(10);
        assert!(mgr.is_empty());
    }

    // === S1c-T03 : EmoteKind + SpriteEmote + EmoteManager ===

    #[test]
    fn emote_kind_atlas_indices_unique() {
        let all = [
            EmoteKind::Happy,
            EmoteKind::Angry,
            EmoteKind::Sad,
            EmoteKind::Confused,
            EmoteKind::Exclamation,
            EmoteKind::Question,
            EmoteKind::Heart,
            EmoteKind::Sleep,
            EmoteKind::Skull,
            EmoteKind::Coin,
        ];
        let mut indices: Vec<u32> = all.iter().map(|k| k.atlas_index()).collect();
        let original_len = indices.len();
        indices.sort_unstable();
        indices.dedup();
        assert_eq!(
            indices.len(),
            original_len,
            "all atlas indices must be unique"
        );
    }

    #[test]
    fn emote_spawn_and_bounce() {
        let mut mgr = EmoteManager::new();
        mgr.spawn(1, [0.0, 0.0], EmoteKind::Happy);
        assert_eq!(mgr.len(), 1);

        mgr.tick(0.3);
        let e = &mgr.emotes()[0];
        // After 0.3s of sinusoidal bounce, offset should be non-zero.
        assert!(
            e.bounce_offset.abs() > f32::EPSILON,
            "bounce_offset should be non-zero after tick"
        );
        // Opacity should still be 1.0 (we are well before the last 0.5s of 3.0s lifetime).
        assert!(
            (e.opacity - 1.0).abs() < f32::EPSILON,
            "opacity should be 1.0 before fade window"
        );
    }

    #[test]
    fn emote_replaces_existing() {
        let mut mgr = EmoteManager::new();
        mgr.spawn(42, [0.0, 0.0], EmoteKind::Happy);
        mgr.spawn(42, [1.0, 1.0], EmoteKind::Angry);
        assert_eq!(mgr.len(), 1, "same entity_id should replace existing emote");
        assert_eq!(mgr.emotes()[0].kind, EmoteKind::Angry);
    }

    #[test]
    fn emote_fade_out() {
        let mut mgr = EmoteManager::new();
        mgr.spawn(1, [0.0, 0.0], EmoteKind::Question);
        // lifetime=3.0, tick to 2.8 => remaining=0.2 < 0.5 => should fade.
        mgr.tick(2.8);
        assert_eq!(mgr.len(), 1, "emote should still be alive at 2.8s");
        let e = &mgr.emotes()[0];
        assert!(
            e.opacity < 1.0,
            "opacity should be < 1.0 in fade window"
        );
        assert!(e.opacity > 0.0, "opacity should still be > 0 at 2.8s");
    }

    #[test]
    fn emote_removal_after_lifetime() {
        let mut mgr = EmoteManager::new();
        mgr.spawn(1, [0.0, 0.0], EmoteKind::Sleep);
        mgr.tick(3.5);
        assert_eq!(
            mgr.len(),
            0,
            "emote should be removed after exceeding lifetime"
        );
    }

    #[test]
    fn emote_manager_default() {
        let mgr = EmoteManager::default();
        assert!(mgr.is_empty());
    }

    // === S1c-T04 : OverheadProgressBar + ProgressBarManager ===

    #[test]
    fn progress_bar_set_and_update() {
        let mut mgr = ProgressBarManager::new();
        mgr.set(1, [0.0, 0.0], 0.3, GATHERING_COLOR);
        assert_eq!(mgr.len(), 1);
        assert!((mgr.bars()[0].progress - 0.3).abs() < f32::EPSILON);

        // Update the same entity.
        mgr.set(1, [0.0, 0.0], 0.7, GATHERING_COLOR);
        assert_eq!(mgr.len(), 1, "should update in-place, not create new");
        assert!((mgr.bars()[0].progress - 0.7).abs() < f32::EPSILON);
    }

    #[test]
    fn progress_bar_remove() {
        let mut mgr = ProgressBarManager::new();
        mgr.set(1, [0.0, 0.0], 0.5, CRAFTING_COLOR);
        assert_eq!(mgr.len(), 1);
        mgr.remove(1);
        assert_eq!(mgr.len(), 0, "bar should be removed");
    }

    #[test]
    fn progress_bar_clamp() {
        let mut mgr = ProgressBarManager::new();
        mgr.set(1, [0.0, 0.0], 1.5, CASTING_COLOR);
        assert!(
            (mgr.bars()[0].progress - 1.0).abs() < f32::EPSILON,
            "progress > 1.0 should be clamped to 1.0"
        );

        mgr.set(2, [0.0, 0.0], -0.5, LOADING_COLOR);
        assert!(
            (mgr.bars()[1].progress - 0.0).abs() < f32::EPSILON,
            "progress < 0.0 should be clamped to 0.0"
        );
    }

    #[test]
    fn progress_bar_default_dimensions() {
        let mut mgr = ProgressBarManager::new();
        mgr.set(1, [0.0, 0.0], 0.5, GATHERING_COLOR);
        let bar = &mgr.bars()[0];
        assert!((bar.width - 40.0).abs() < f32::EPSILON, "default width=40");
        assert!((bar.height - 6.0).abs() < f32::EPSILON, "default height=6");
        assert!(bar.visible, "should be visible after set");
    }

    #[test]
    fn progress_bar_manager_default() {
        let mgr = ProgressBarManager::default();
        assert!(mgr.is_empty());
    }

    #[test]
    fn progress_bar_multiple_entities() {
        let mut mgr = ProgressBarManager::new();
        mgr.set(1, [0.0, 0.0], 0.2, GATHERING_COLOR);
        mgr.set(2, [10.0, 10.0], 0.5, CRAFTING_COLOR);
        mgr.set(3, [20.0, 20.0], 0.8, CASTING_COLOR);
        assert_eq!(mgr.len(), 3);
        mgr.remove(2);
        assert_eq!(mgr.len(), 2);
    }

    // === Colour constants ===

    #[test]
    fn colour_constants_alpha_is_one() {
        assert!((GATHERING_COLOR[3] - 1.0).abs() < f32::EPSILON);
        assert!((CRAFTING_COLOR[3] - 1.0).abs() < f32::EPSILON);
        assert!((CASTING_COLOR[3] - 1.0).abs() < f32::EPSILON);
        assert!((LOADING_COLOR[3] - 1.0).abs() < f32::EPSILON);
    }
}
