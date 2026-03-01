//! Inventory panel: 10x4 grid + equipment slots (D2-style silhouette layout).

use egui::{Color32, Context, Pos2, Vec2};

use crate::theme::{item_quality_color, D2Colors};

// ---------------------------------------------------------------------------
// Data structs
// ---------------------------------------------------------------------------

/// An item displayed in the inventory grid.
#[derive(Debug, Clone)]
pub struct UiItem {
    /// Unique item identifier.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Quality string (`"normal"`, `"magic"`, `"rare"`, `"unique"`, `"set"`, `"rune_word"`).
    pub quality: String,
    /// Width in grid cells (1..4).
    pub width: u32,
    /// Height in grid cells (1..4).
    pub height: u32,
    /// Grid column position.
    pub grid_x: i32,
    /// Grid row position.
    pub grid_y: i32,
    /// Icon atlas key.
    pub icon_name: String,
    /// Whether the item has been identified.
    pub is_identified: bool,
}

/// Equipment slots on the character silhouette.
#[derive(Default)]
pub struct EquipSlots {
    /// Head armor.
    pub head: Option<UiItem>,
    /// Chest armor.
    pub chest: Option<UiItem>,
    /// Belt.
    pub belt: Option<UiItem>,
    /// Boots.
    pub boots: Option<UiItem>,
    /// Gloves.
    pub gloves: Option<UiItem>,
    /// Amulet.
    pub amulet: Option<UiItem>,
    /// Left ring.
    pub ring_left: Option<UiItem>,
    /// Right ring.
    pub ring_right: Option<UiItem>,
    /// Main-hand weapon.
    pub main_hand: Option<UiItem>,
    /// Off-hand (shield / weapon).
    pub off_hand: Option<UiItem>,
}

/// Number of columns in the inventory grid.
pub const GRID_COLS: usize = 10;
/// Number of rows in the inventory grid.
pub const GRID_ROWS: usize = 4;
/// Pixel size of a single grid cell.
pub const CELL_SIZE: f32 = 28.0;

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw the full inventory panel (equipment + grid + gold).
pub fn draw_inventory(
    ctx: &Context,
    is_open: &mut bool,
    items: &[UiItem],
    equip: &EquipSlots,
    gold: i64,
) {
    if !*is_open {
        return;
    }

    egui::Window::new("Inventaire")
        .resizable(false)
        .collapsible(false)
        .title_bar(true)
        .open(is_open)
        .default_pos([400.0, 50.0])
        .show(ctx, |ui| {
            ui.set_min_width(340.0);

            // Equipment slots (D2 silhouette layout)
            ui.group(|ui| {
                ui.label(
                    egui::RichText::new("Equipement")
                        .color(D2Colors::GOLD)
                        .size(12.0),
                );
                draw_equip_slots(ui, equip);
            });

            ui.separator();

            // Inventory grid 10x4
            ui.group(|ui| {
                ui.label(
                    egui::RichText::new("Sac")
                        .color(D2Colors::GOLD)
                        .size(12.0),
                );
                draw_grid(ui, items);
            });

            ui.separator();

            let gold_text = format!("Or : {gold}");
            ui.label(
                egui::RichText::new(&gold_text)
                    .color(D2Colors::GOLD_BRIGHT)
                    .size(11.0),
            );
        });
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Draw the 10x4 inventory grid with items.
fn draw_grid(ui: &mut egui::Ui, items: &[UiItem]) {
    let grid_w = GRID_COLS as f32 * CELL_SIZE;
    let grid_h = GRID_ROWS as f32 * CELL_SIZE;

    let (resp, painter) = ui.allocate_painter(Vec2::new(grid_w, grid_h), egui::Sense::click());
    let origin = resp.rect.min;

    // Grid background
    painter.rect_filled(resp.rect, 2.0, Color32::from_rgb(15, 12, 8));

    // Vertical grid lines
    for col in 0..=GRID_COLS {
        let x = origin.x + col as f32 * CELL_SIZE;
        painter.line_segment(
            [Pos2::new(x, origin.y), Pos2::new(x, origin.y + grid_h)],
            egui::Stroke::new(0.5, Color32::from_rgb(40, 30, 15)),
        );
    }
    // Horizontal grid lines
    for row in 0..=GRID_ROWS {
        let y = origin.y + row as f32 * CELL_SIZE;
        painter.line_segment(
            [Pos2::new(origin.x, y), Pos2::new(origin.x + grid_w, y)],
            egui::Stroke::new(0.5, Color32::from_rgb(40, 30, 15)),
        );
    }

    // Items in the grid
    for item in items {
        if item.grid_x < 0 || item.grid_y < 0 {
            continue;
        }
        let x = origin.x + item.grid_x as f32 * CELL_SIZE;
        let y = origin.y + item.grid_y as f32 * CELL_SIZE;
        let w = item.width as f32 * CELL_SIZE;
        let h = item.height as f32 * CELL_SIZE;
        let rect = egui::Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h));

        let border_color = item_quality_color(&item.quality);
        painter.rect_filled(
            rect,
            2.0,
            Color32::from_rgba_premultiplied(30, 25, 15, 200),
        );
        painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border_color));

        // Abbreviated name or "?" if unidentified
        if item.is_identified {
            let short_name: String = item.name.chars().take(4).collect();
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &short_name,
                egui::FontId::proportional(9.0),
                border_color,
            );
        } else {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "?",
                egui::FontId::proportional(10.0),
                D2Colors::TEXT_NORMAL,
            );
        }
    }
}

/// Draw the equipment slot panel (two horizontal rows).
fn draw_equip_slots(ui: &mut egui::Ui, equip: &EquipSlots) {
    let slot_size = Vec2::new(38.0, 38.0);

    // Row 1: Head, Chest, Belt, Boots, Gloves
    ui.horizontal(|ui| {
        for (label, item) in [
            ("Tete", equip.head.as_ref()),
            ("Corps", equip.chest.as_ref()),
            ("Ceinture", equip.belt.as_ref()),
            ("Bottes", equip.boots.as_ref()),
            ("Gants", equip.gloves.as_ref()),
        ] {
            draw_single_equip_slot(ui, slot_size, label, item);
        }
    });

    // Row 2: Main Hand, Amulet, Ring L, Ring R, Off Hand
    ui.horizontal(|ui| {
        for (label, item) in [
            ("Main G", equip.main_hand.as_ref()),
            ("Amulette", equip.amulet.as_ref()),
            ("Anneau G", equip.ring_left.as_ref()),
            ("Anneau D", equip.ring_right.as_ref()),
            ("Main D", equip.off_hand.as_ref()),
        ] {
            draw_single_equip_slot(ui, slot_size, label, item);
        }
    });
}

/// Draw a single equipment slot with optional item.
fn draw_single_equip_slot(
    ui: &mut egui::Ui,
    slot_size: Vec2,
    label: &str,
    item: Option<&UiItem>,
) {
    let (resp, painter) = ui.allocate_painter(slot_size, egui::Sense::hover());
    let rect = resp.rect;
    let bg = if resp.hovered() {
        D2Colors::SLOT_HOVER
    } else {
        D2Colors::SLOT_EMPTY
    };
    painter.rect_filled(rect, 3.0, bg);
    painter.rect_stroke(rect, 3.0, egui::Stroke::new(1.0, D2Colors::PANEL_BORDER));

    if let Some(it) = item {
        painter.rect_filled(rect.shrink(4.0), 2.0, Color32::from_rgb(50, 40, 20));
        let short_name: String = it.name.chars().take(4).collect();
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            &short_name,
            egui::FontId::proportional(8.0),
            item_quality_color(&it.quality),
        );
    } else {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::proportional(8.0),
            Color32::from_rgb(80, 70, 50),
        );
    }
}
