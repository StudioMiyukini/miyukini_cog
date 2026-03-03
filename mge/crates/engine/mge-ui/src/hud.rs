// @id: MGE-UI-HUD @do: hud-orbs @role: front-end @layer: 3 @human: miyuk

//! In-game HUD: life/mana orbs, skill hotbar (8 slots), belt (4 slots),
//! experience bar, level indicator, and gold display.
//!
//! ## HUD orb state
//!
//! [`OrbState`] tracks current/max values with a smoothly lerped `display_percent`
//! suitable for filling the circular orb sprite. [`HudState`] bundles the HP and
//! mana orbs and provides a single `update` call driven by the game loop `dt`.

use egui::{Color32, Context, Pos2, Rect, Vec2};

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// OrbState
// ---------------------------------------------------------------------------

/// Animated state for a single life or mana orb.
///
/// `display_percent` lags behind the real fill ratio so the orb drains /
/// refills with a smooth visual interpolation instead of jumping instantly.
#[derive(Debug, Clone, PartialEq)]
pub struct OrbState {
    /// Raw current value (e.g. current HP).
    pub current: f32,
    /// Raw maximum value (e.g. max HP).
    pub max: f32,
    /// Smoothly interpolated fill ratio used for rendering (0.0 ..= 1.0).
    pub display_percent: f32,
}

impl OrbState {
    /// Create a new `OrbState` with both `current` and `display_percent`
    /// initialised to the exact fill ratio — no initial lerp lag.
    #[must_use]
    pub fn new(current: f32, max: f32) -> Self {
        let display_percent = if max > 0.0 {
            (current / max).clamp(0.0, 1.0)
        } else {
            0.0
        };
        Self {
            current,
            max,
            display_percent,
        }
    }

    /// Advance the orb state for one frame.
    ///
    /// Updates `current` and `max`, then smoothly moves `display_percent`
    /// toward the true fill ratio at `lerp_speed` (units: fraction per second).
    /// Pass `lerp_speed = 1.0` to cover the full 0→1 range in one second.
    pub fn update(&mut self, current: f32, max: f32, lerp_speed: f32) {
        self.current = current;
        self.max = max;

        let target = if max > 0.0 {
            (current / max).clamp(0.0, 1.0)
        } else {
            0.0
        };

        // Linear interpolation toward target; clamp result to valid range.
        let delta = target - self.display_percent;
        self.display_percent = (self.display_percent + delta * lerp_speed).clamp(0.0, 1.0);
    }

    /// Return the clamped fill ratio (0.0 ..= 1.0) based on raw values.
    ///
    /// Returns `0.0` when `max` is zero to avoid division by zero.
    #[must_use]
    pub fn percent(&self) -> f32 {
        if self.max <= 0.0 {
            return 0.0;
        }
        (self.current / self.max).clamp(0.0, 1.0)
    }
}

// ---------------------------------------------------------------------------
// HudState
// ---------------------------------------------------------------------------

/// Persistent per-frame state for the full HUD (HP + mana orbs).
///
/// Keep one `HudState` alive for the lifetime of a game session and call
/// [`HudState::update`] every frame with the latest values and delta time.
#[derive(Debug, Clone, PartialEq)]
pub struct HudState {
    /// Animated HP orb.
    pub hp_orb: OrbState,
    /// Animated mana orb.
    pub mana_orb: OrbState,
}

impl HudState {
    /// Create a new `HudState` with both orbs at zero (max = 1 placeholder).
    ///
    /// Call [`HudState::update`] immediately after creation with real values.
    #[must_use]
    pub fn new() -> Self {
        Self {
            hp_orb: OrbState::new(0.0, 1.0),
            mana_orb: OrbState::new(0.0, 1.0),
        }
    }

    /// Update both orbs for the current frame.
    ///
    /// `dt` is the frame delta time in seconds. The lerp speed is fixed at
    /// `4.0` (full range in ~250 ms), which matches the classic D2 feel.
    pub fn update(
        &mut self,
        hp_current: f32,
        hp_max: f32,
        mana_current: f32,
        mana_max: f32,
        dt: f32,
    ) {
        let lerp_speed = 4.0 * dt;
        self.hp_orb.update(hp_current, hp_max, lerp_speed);
        self.mana_orb.update(mana_current, mana_max, lerp_speed);
    }
}

impl Default for HudState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Data structs
// ---------------------------------------------------------------------------

/// Player data required to render the HUD.
pub struct HudData<'a> {
    /// Current life points.
    pub life_cur: i32,
    /// Maximum life points.
    pub life_max: i32,
    /// Current mana points.
    pub mana_cur: i32,
    /// Maximum mana points.
    pub mana_max: i32,
    /// Character level.
    pub level: i32,
    /// Experience progress toward next level (0.0 .. 1.0).
    pub experience_pct: f32,
    /// Gold held.
    pub gold: i64,
    /// The 8 skill hotbar slots.
    pub skill_slots: &'a [SkillSlotData; 8],
    /// The 4 belt (potion) slots.
    pub belt_slots: &'a [BeltSlotData; 4],
}

/// Data for a single skill hotbar slot.
#[derive(Default, Clone)]
pub struct SkillSlotData {
    /// Skill identifier (None = empty slot).
    pub skill_id: Option<String>,
    /// Icon atlas name.
    pub icon_name: Option<String>,
    /// Cooldown progress (0.0 = ready, 1.0 = full cooldown).
    pub cooldown_pct: f32,
    /// Mana cost to cast.
    pub mana_cost: i32,
}

/// Data for a single belt (potion) slot.
#[derive(Default, Clone)]
pub struct BeltSlotData {
    /// Item identifier (None = empty slot).
    pub item_id: Option<String>,
    /// Icon atlas name.
    pub icon_name: Option<String>,
    /// Stack quantity.
    pub quantity: u32,
}

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw the full D2-style HUD on the screen.
///
/// `screen_w` / `screen_h` are the current window dimensions in logical pixels.
pub fn draw_hud(ctx: &Context, data: &HudData<'_>, screen_w: f32, screen_h: f32) {
    // -- Bottom bar (main HUD panel) --
    egui::Area::new("hud_bottom".into())
        .fixed_pos(Pos2::new(0.0, screen_h - 80.0))
        .show(ctx, |ui| {
            ui.set_width(screen_w);

            // Background
            let rect = ui.max_rect();
            ui.painter().rect_filled(
                rect,
                0.0,
                Color32::from_rgba_premultiplied(10, 8, 5, 230),
            );
            ui.painter().rect_stroke(
                rect,
                0.0,
                egui::Stroke::new(1.0, D2Colors::PANEL_BORDER),
            );

            ui.horizontal(|ui| {
                // Life orb (left)
                draw_orb(ui, data.life_cur, data.life_max, D2Colors::RED_LIFE, "Vie");

                ui.add_space(8.0);

                // Skill hotbar (center)
                draw_skill_hotbar(ui, data.skill_slots, data.mana_cur);

                ui.add_space(8.0);

                // Belt (quick potions)
                draw_belt(ui, data.belt_slots);

                ui.add_space(8.0);

                // Mana orb (right)
                draw_orb(
                    ui,
                    data.mana_cur,
                    data.mana_max,
                    D2Colors::BLUE_MANA,
                    "Mana",
                );
            });
        });

    // -- Experience bar (very bottom) --
    egui::Area::new("hud_xp".into())
        .fixed_pos(Pos2::new(0.0, screen_h - 6.0))
        .show(ctx, |ui| {
            let bar_rect = Rect::from_min_size(
                Pos2::ZERO,
                Vec2::new(screen_w * data.experience_pct, 5.0),
            );
            ui.painter().rect_filled(bar_rect, 0.0, D2Colors::GOLD);
        });

    // -- Level indicator (bottom-left) --
    let level_label = format!("Nv {}", data.level);
    egui::Area::new("hud_level".into())
        .fixed_pos(Pos2::new(6.0, screen_h - 18.0))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(&level_label)
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
        });

    // -- Gold display (bottom-right) --
    let gold_label = format!("Or : {}", data.gold);
    egui::Area::new("hud_gold".into())
        .fixed_pos(Pos2::new(screen_w - 120.0, screen_h - 18.0))
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(&gold_label)
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
        });
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Draw a life or mana orb with proportional fill from bottom to top.
fn draw_orb(ui: &mut egui::Ui, cur: i32, max: i32, color: Color32, label: &str) {
    let orb_size = Vec2::new(48.0, 64.0);
    let (resp, painter) = ui.allocate_painter(orb_size, egui::Sense::hover());
    let rect = resp.rect;

    // Orb background
    painter.rect_filled(rect, egui::Rounding::same(24.0), Color32::from_rgb(20, 15, 10));

    // Proportional fill (bottom to top)
    let max_safe = max.max(1);
    let pct = (cur as f32 / max_safe as f32).clamp(0.0, 1.0);
    let fill_h = orb_size.y * pct;
    let fill_rect = Rect::from_min_size(
        Pos2::new(rect.min.x, rect.max.y - fill_h),
        Vec2::new(orb_size.x, fill_h),
    );
    painter.rect_filled(fill_rect, egui::Rounding::same(24.0), color);

    // Border
    painter.rect_stroke(
        rect,
        egui::Rounding::same(24.0),
        egui::Stroke::new(1.5, D2Colors::PANEL_BORDER),
    );

    // Value text
    let value_text = format!("{cur}/{max}");
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        &value_text,
        egui::FontId::proportional(10.0),
        Color32::WHITE,
    );

    // Tooltip on hover
    let tooltip_text = format!("{label} : {cur}/{max}");
    resp.on_hover_text(tooltip_text);
}

/// Draw the 8-slot skill hotbar.
fn draw_skill_hotbar(ui: &mut egui::Ui, slots: &[SkillSlotData; 8], mana_cur: i32) {
    let slot_size = Vec2::new(40.0, 40.0);
    ui.horizontal(|ui| {
        for (i, slot) in slots.iter().enumerate() {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::click());
            let rect = resp.rect;

            // Slot background
            let bg = if resp.hovered() {
                D2Colors::SLOT_HOVER
            } else {
                D2Colors::SLOT_EMPTY
            };
            painter.rect_filled(rect, 3.0, bg);
            painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

            if slot.icon_name.is_some() {
                // In production: draw the icon from the sprite atlas.
                // Placeholder: colored rectangle.
                painter.rect_filled(rect.shrink(4.0), 2.0, Color32::from_rgb(80, 80, 120));
            }

            // Cooldown overlay
            if slot.cooldown_pct > 0.0 {
                let cd_rect = Rect::from_min_size(
                    rect.min,
                    Vec2::new(slot_size.x, slot_size.y * slot.cooldown_pct),
                );
                painter.rect_filled(
                    cd_rect,
                    3.0,
                    Color32::from_rgba_premultiplied(0, 0, 0, 160),
                );
            }

            // Slot number (bottom-left corner)
            let slot_num = (i + 1).to_string();
            painter.text(
                Pos2::new(rect.min.x + 2.0, rect.max.y - 2.0),
                egui::Align2::LEFT_BOTTOM,
                &slot_num,
                egui::FontId::proportional(9.0),
                D2Colors::GOLD,
            );

            // Mana cost (bottom-right corner, red if insufficient)
            if slot.mana_cost > 0 {
                let cost_color = if mana_cur >= slot.mana_cost {
                    D2Colors::BLUE_MANA
                } else {
                    D2Colors::RED_LIFE
                };
                let cost_text = slot.mana_cost.to_string();
                painter.text(
                    Pos2::new(rect.max.x - 2.0, rect.max.y - 2.0),
                    egui::Align2::RIGHT_BOTTOM,
                    &cost_text,
                    egui::FontId::proportional(9.0),
                    cost_color,
                );
            }
        }
    });
}

/// Draw the 4-slot belt (quick potion access).
fn draw_belt(ui: &mut egui::Ui, slots: &[BeltSlotData; 4]) {
    let slot_size = Vec2::new(36.0, 36.0);
    ui.vertical(|ui| {
        for (i, slot) in slots.iter().enumerate() {
            let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::click());
            let rect = resp.rect;

            let bg = if resp.hovered() {
                D2Colors::SLOT_HOVER
            } else {
                D2Colors::SLOT_EMPTY
            };
            painter.rect_filled(rect, 2.0, bg);
            painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

            // Potion quantity
            if slot.quantity > 0 {
                // Placeholder: red rectangle for potions.
                painter.rect_filled(rect.shrink(5.0), 2.0, Color32::from_rgb(160, 30, 30));
                let qty_text = slot.quantity.to_string();
                painter.text(
                    rect.center_bottom() - Vec2::new(0.0, 2.0),
                    egui::Align2::CENTER_BOTTOM,
                    &qty_text,
                    egui::FontId::proportional(9.0),
                    Color32::WHITE,
                );
            }

            // Key binding label (5/6/7/8)
            let key_label = (5 + i).to_string();
            painter.text(
                Pos2::new(rect.min.x + 2.0, rect.min.y + 2.0),
                egui::Align2::LEFT_TOP,
                &key_label,
                egui::FontId::proportional(8.0),
                D2Colors::GOLD,
            );
        }
    });
}

// ---------------------------------------------------------------------------
// PotionType (TASK-057)
// ---------------------------------------------------------------------------

/// Type of potion occupying a belt slot.
///
/// Used by [`BeltState`] to track which potions the player has equipped in the
/// 4 quick-access belt slots (keys 5-8 in classic D2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PotionType {
    /// Small healing potion — restores 70 HP.
    HpSmall,
    /// Medium healing potion — restores 140 HP.
    HpMedium,
    /// Small mana potion — restores 40 mana.
    ManaSmall,
    /// Medium mana potion — restores 80 mana.
    ManaMedium,
    /// Rejuvenation potion — restores 35% HP and mana (special case).
    Rejuvenation,
}

impl PotionType {
    /// Returns the `(hp_heal, mana_heal)` pair for this potion.
    ///
    /// Rejuvenation is a percentage-based heal; by convention this method
    /// returns `(0, 0)` as a sentinel — callers must handle that variant
    /// separately (apply 35 % of max HP and max mana).
    #[must_use]
    pub fn heal_amount(&self) -> (i32, i32) {
        match self {
            Self::HpSmall => (70, 0),
            Self::HpMedium => (140, 0),
            Self::ManaSmall => (0, 40),
            Self::ManaMedium => (0, 80),
            // 35 % — special case; sentinel (0,0) signals percentage healing.
            Self::Rejuvenation => (0, 0),
        }
    }
}

// ---------------------------------------------------------------------------
// BeltState (TASK-057)
// ---------------------------------------------------------------------------

/// State of the player's 4-slot potion belt.
///
/// The belt maps directly to keys 5-8. Each slot holds at most one potion
/// type at a time (D2 stacks are represented externally; this tracks the
/// equipped type only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeltState {
    /// The four belt slots; `None` = empty.
    pub slots: [Option<PotionType>; 4],
}

impl BeltState {
    /// Create a new `BeltState` with all slots empty.
    #[must_use]
    pub fn new() -> Self {
        Self { slots: [None; 4] }
    }

    /// Set the potion in `index` (clamped to `0..=3`).
    ///
    /// Passing `potion = None` clears the slot.
    pub fn set_slot(&mut self, index: usize, potion: Option<PotionType>) {
        let i = index.min(3);
        self.slots[i] = potion;
    }

    /// Consume the potion in `index` (clamped to `0..=3`).
    ///
    /// Returns the [`PotionType`] that was in the slot (or `None` if empty),
    /// and leaves the slot empty.
    pub fn use_slot(&mut self, index: usize) -> Option<PotionType> {
        let i = index.min(3);
        self.slots[i].take()
    }
}

impl Default for BeltState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// XpBarState (TASK-059)
// ---------------------------------------------------------------------------

/// Persistent state for the experience bar shown at the bottom of the screen.
///
/// [`XpBarState::update`] recomputes the fill ratio each frame; the result is
/// clamped to `0.0..=1.0` and exposed via [`XpBarState::progress`].
#[derive(Debug, Clone, PartialEq)]
pub struct XpBarState {
    /// Current player level.
    pub level: u32,
    /// Fill ratio for the XP bar (`0.0` = no progress, `1.0` = level-up ready).
    progress: f32,
}

impl XpBarState {
    /// Create a new `XpBarState` at level 1 with no XP progress.
    #[must_use]
    pub fn new() -> Self {
        Self {
            level: 1,
            progress: 0.0,
        }
    }

    /// Recompute the XP bar progress from raw XP values.
    ///
    /// When `next_level_xp` is zero (e.g. at max level), `progress` is forced
    /// to `1.0` — the bar displays as full.
    pub fn update(&mut self, level: u32, current_xp: u64, next_level_xp: u64) {
        self.level = level;
        self.progress = if next_level_xp == 0 {
            1.0
        } else {
            (current_xp as f32 / next_level_xp as f32).clamp(0.0, 1.0)
        };
    }

    /// Return the clamped XP fill ratio (`0.0..=1.0`).
    #[must_use]
    pub fn progress(&self) -> f32 {
        self.progress.clamp(0.0, 1.0)
    }
}

impl Default for XpBarState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::{BeltState, OrbState, PotionType, XpBarState};

    /// Clamping test: current > max must return percent = 1.0.
    #[test]
    fn orb_percent_clamp() {
        let orb = OrbState::new(150.0, 100.0);
        assert!(
            (orb.percent() - 1.0).abs() < f32::EPSILON,
            "percent should be clamped to 1.0, got {}",
            orb.percent()
        );
    }

    /// Division-by-zero guard: max = 0 must return 0.0 without panicking.
    #[test]
    fn orb_div_by_zero() {
        let orb = OrbState::new(50.0, 0.0);
        assert!(
            orb.percent().abs() < f32::EPSILON,
            "percent should be 0.0 when max is zero, got {}",
            orb.percent()
        );
    }

    /// Lerp direction test: after one update, display_percent must move toward
    /// the target ratio (not away from it, and not stay at the same value).
    #[test]
    fn orb_lerp_update() {
        // Start at 0% fill.
        let mut orb = OrbState::new(0.0, 100.0);
        assert!(
            orb.display_percent.abs() < f32::EPSILON,
            "initial display_percent should be 0.0"
        );

        // Ask for 80% fill with a large lerp_speed so we get noticeable movement.
        orb.update(80.0, 100.0, 0.5);

        // display_percent must have moved toward 0.8 (i.e. > 0.0).
        assert!(
            orb.display_percent > 0.0,
            "display_percent should have moved toward target after update, got {}",
            orb.display_percent
        );
        // Must not overshoot.
        assert!(
            orb.display_percent <= 0.8 + f32::EPSILON,
            "display_percent must not overshoot target 0.8, got {}",
            orb.display_percent
        );
    }

    // -----------------------------------------------------------------------
    // TASK-057 — BeltState tests
    // -----------------------------------------------------------------------

    /// Using a potion slot returns it and leaves the slot empty.
    #[test]
    fn belt_use_potion() {
        let mut belt = BeltState::new();
        belt.set_slot(0, Some(PotionType::HpSmall));

        let taken = belt.use_slot(0);
        assert_eq!(
            taken,
            Some(PotionType::HpSmall),
            "use_slot(0) should return Some(HpSmall)"
        );
        assert_eq!(
            belt.slots[0], None,
            "slot 0 should be None after use_slot"
        );
    }

    /// Setting a slot with a different potion overwrites the previous value.
    #[test]
    fn belt_stack_potions() {
        let mut belt = BeltState::new();
        // Fill all 4 slots.
        belt.set_slot(0, Some(PotionType::HpSmall));
        belt.set_slot(1, Some(PotionType::HpMedium));
        belt.set_slot(2, Some(PotionType::ManaSmall));
        belt.set_slot(3, Some(PotionType::ManaMedium));

        // Overwrite slot 1 — simulates replacing a consumed potion.
        belt.set_slot(1, Some(PotionType::Rejuvenation));
        assert_eq!(
            belt.slots[1],
            Some(PotionType::Rejuvenation),
            "set_slot should overwrite the previous potion"
        );

        // Other slots must be unchanged.
        assert_eq!(belt.slots[0], Some(PotionType::HpSmall));
        assert_eq!(belt.slots[2], Some(PotionType::ManaSmall));
        assert_eq!(belt.slots[3], Some(PotionType::ManaMedium));
    }

    // -----------------------------------------------------------------------
    // TASK-059 — XpBarState tests
    // -----------------------------------------------------------------------

    /// 50 XP out of 100 required → progress = 0.5.
    #[test]
    fn xp_bar_progress() {
        let mut xp = XpBarState::new();
        xp.update(10, 50, 100);
        let got = xp.progress();
        assert!(
            (got - 0.5).abs() < f32::EPSILON,
            "50/100 XP should yield progress 0.5, got {got}"
        );
    }

    /// At max level (next_level_xp = 0) → progress must be clamped to 1.0.
    #[test]
    fn xp_bar_level_99() {
        let mut xp = XpBarState::new();
        xp.update(99, 0, 0);
        let got = xp.progress();
        assert!(
            (got - 1.0).abs() < f32::EPSILON,
            "max level with next_level_xp=0 should yield progress 1.0, got {got}"
        );
    }
}
