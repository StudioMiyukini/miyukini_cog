// @id: MUIE-InvPanel @do: inventory-panel-organism @role: organism @layer: 6 @human: miyuk

//! Inventory panel organism -- equipment slots + 10x4 item grid + gold display.

use egui::{Color32, Context, Pos2, Vec2};
use miyuki_ui_tokens::palette::d2::D2_PALETTE;
use crate::atoms::quality_text::ItemQuality;
use crate::atoms::slot_frame::CELL_PX;
use crate::convert::rgba_to_color32;
use crate::molecules::equip_slot::{EquipItemData, EquipPosition, EquipSlot};

/// Grid dimensions.
pub const GRID_COLS: usize = 10;
/// Grid rows.
pub const GRID_ROWS: usize = 4;

/// Item data for the inventory grid.
#[derive(Debug, Clone)]
pub struct GridItemData {
    /// Item name.
    pub name: String,
    /// Item quality.
    pub quality: ItemQuality,
    /// Width in cells.
    pub width: u32,
    /// Height in cells.
    pub height: u32,
    /// Grid column position.
    pub grid_x: i32,
    /// Grid row position.
    pub grid_y: i32,
    /// Whether identified.
    pub identified: bool,
}

/// Equipment data for all paperdoll slots.
#[derive(Debug, Clone, Default)]
pub struct EquipmentData {
    /// Head (helm).
    pub head: Option<EquipItemData>,
    /// Body (armor).
    pub body: Option<EquipItemData>,
    /// Belt.
    pub belt: Option<EquipItemData>,
    /// Boots.
    pub boots: Option<EquipItemData>,
    /// Gloves.
    pub gloves: Option<EquipItemData>,
    /// Amulet.
    pub amulet: Option<EquipItemData>,
    /// Left ring.
    pub ring_left: Option<EquipItemData>,
    /// Right ring.
    pub ring_right: Option<EquipItemData>,
    /// Main hand weapon.
    pub main_hand: Option<EquipItemData>,
    /// Off hand.
    pub off_hand: Option<EquipItemData>,
}

/// Complete inventory panel data.
pub struct InventoryPanelData {
    /// Items in the inventory grid.
    pub items: Vec<GridItemData>,
    /// Equipment slots.
    pub equipment: EquipmentData,
    /// Gold carried.
    pub gold: i64,
}

/// The inventory panel organism.
pub struct InventoryPanel;

impl InventoryPanel {
    /// Draw the inventory panel as a window.
    pub fn show(ctx: &Context, is_open: &mut bool, data: &InventoryPanelData) {
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

                // Equipment section
                ui.group(|ui| {
                    let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                    ui.label(egui::RichText::new("Equipement").color(gold).size(12.0));
                    Self::draw_equipment(ui, &data.equipment);
                });

                ui.separator();

                // Inventory grid
                ui.group(|ui| {
                    let gold = rgba_to_color32(&D2_PALETTE.accent_primary);
                    ui.label(egui::RichText::new("Sac").color(gold).size(12.0));
                    Self::draw_grid(ui, &data.items);
                });

                ui.separator();

                // Gold
                let gold_text = format!("Or : {}", data.gold);
                let gold_bright = rgba_to_color32(&D2_PALETTE.text_high);
                ui.label(egui::RichText::new(&gold_text).color(gold_bright).size(11.0));
            });
    }

    /// Draw the equipment slots (paperdoll).
    fn draw_equipment(ui: &mut egui::Ui, equip: &EquipmentData) {
        // Row 1: Head, Body, Belt, Boots, Gloves
        ui.horizontal(|ui| {
            let slots: [(EquipPosition, &Option<EquipItemData>); 5] = [
                (EquipPosition::Head, &equip.head),
                (EquipPosition::Body, &equip.body),
                (EquipPosition::Belt, &equip.belt),
                (EquipPosition::Boots, &equip.boots),
                (EquipPosition::Gloves, &equip.gloves),
            ];
            for (pos, item_opt) in slots {
                let mut slot = EquipSlot::new(pos);
                if let Some(item) = item_opt {
                    slot = slot.item(item);
                }
                slot.show(ui);
            }
        });

        // Row 2: Main Hand, Amulet, Ring L, Ring R, Off Hand
        ui.horizontal(|ui| {
            let slots: [(EquipPosition, &Option<EquipItemData>); 5] = [
                (EquipPosition::MainHand, &equip.main_hand),
                (EquipPosition::Amulet, &equip.amulet),
                (EquipPosition::RingLeft, &equip.ring_left),
                (EquipPosition::RingRight, &equip.ring_right),
                (EquipPosition::OffHand, &equip.off_hand),
            ];
            for (pos, item_opt) in slots {
                let mut slot = EquipSlot::new(pos);
                if let Some(item) = item_opt {
                    slot = slot.item(item);
                }
                slot.show(ui);
            }
        });
    }

    /// Draw the 10x4 inventory grid.
    fn draw_grid(ui: &mut egui::Ui, items: &[GridItemData]) {
        let grid_w = GRID_COLS as f32 * CELL_PX;
        let grid_h = GRID_ROWS as f32 * CELL_PX;

        let (resp, painter) = ui.allocate_painter(Vec2::new(grid_w, grid_h), egui::Sense::click());
        let origin = resp.rect.min;

        // Grid background
        painter.rect_filled(resp.rect, 2.0, Color32::from_rgb(15, 12, 8));

        // Grid lines
        let line_color = Color32::from_rgb(40, 30, 15);
        for col in 0..=GRID_COLS {
            let x = origin.x + col as f32 * CELL_PX;
            painter.line_segment(
                [Pos2::new(x, origin.y), Pos2::new(x, origin.y + grid_h)],
                egui::Stroke::new(0.5, line_color),
            );
        }
        for row in 0..=GRID_ROWS {
            let y = origin.y + row as f32 * CELL_PX;
            painter.line_segment(
                [Pos2::new(origin.x, y), Pos2::new(origin.x + grid_w, y)],
                egui::Stroke::new(0.5, line_color),
            );
        }

        // Items
        for item in items {
            if item.grid_x < 0 || item.grid_y < 0 {
                continue;
            }
            let x = origin.x + item.grid_x as f32 * CELL_PX;
            let y = origin.y + item.grid_y as f32 * CELL_PX;
            let w = item.width as f32 * CELL_PX;
            let h = item.height as f32 * CELL_PX;
            let rect = egui::Rect::from_min_size(Pos2::new(x, y), Vec2::new(w, h));

            let border_color = item.quality.color();
            painter.rect_filled(rect, 2.0, Color32::from_rgba_premultiplied(30, 25, 15, 200));
            painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, border_color));

            if item.identified {
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
                    Color32::from_rgb(200, 190, 160),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_dimensions() {
        assert_eq!(GRID_COLS, 10);
        assert_eq!(GRID_ROWS, 4);
    }

    #[test]
    fn test_grid_item_data() {
        let item = GridItemData {
            name: "Shako".to_string(),
            quality: ItemQuality::Unique,
            width: 2,
            height: 2,
            grid_x: 0,
            grid_y: 0,
            identified: true,
        };
        assert_eq!(item.quality, ItemQuality::Unique);
    }

    #[test]
    fn test_equipment_data_default() {
        let equip = EquipmentData::default();
        assert!(equip.head.is_none());
        assert!(equip.body.is_none());
    }
}
