//! In-game HUD: life/mana orbs, skill hotbar (8 slots), belt (4 slots),
//! experience bar, level indicator, and gold display.

use egui::{Color32, Context, Pos2, Rect, Vec2};

use crate::theme::D2Colors;

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
