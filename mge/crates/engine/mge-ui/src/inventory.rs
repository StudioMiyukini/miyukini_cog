// @id: MGE-UI-Inventory @do: inventory-grid @role: front-end @layer: 2 @human: miyuk
//! Inventory panel: 10x4 grid + equipment slots (D2-style silhouette layout).
//!
//! This module provides two layers:
//!
//! - **Data model** (`InventoryGrid`, `InventorySlot`, `ItemSize`, `InventoryError`):
//!   pure logic for placement, removal, swap and collision checks.
//! - **UI rendering** (`draw_inventory`, `draw_grid`, …): egui draw calls consuming
//!   the data model.

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

// u8 aliases used internally by the data model (same values, different type).
const COLS: u8 = 10;
const ROWS: u8 = 4;

// ---------------------------------------------------------------------------
// Data model — ItemSize
// ---------------------------------------------------------------------------

/// Grid footprint of an item (width x height in cells).
///
/// Width is 1–2 columns; height is 1–4 rows, matching the 10×4 inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ItemSize {
    /// Width in grid columns (1-2).
    pub width: u8,
    /// Height in grid rows (1-4).
    pub height: u8,
}

/// Return the canonical grid footprint for a given `base_type` string.
///
/// Unknown types default to 1×1.
pub fn item_size(base_type: &str) -> ItemSize {
    match base_type {
        "belt" | "boots" | "gloves" | "helm" | "circlet"
        | "shield" | "buckler" => ItemSize { width: 2, height: 2 },

        "sword" | "axe" | "mace" | "scepter" | "wand" | "dagger" => ItemSize { width: 1, height: 3 },

        "bow" | "crossbow" | "staff" | "polearm" | "spear" => ItemSize { width: 2, height: 4 },

        "armor" | "plate" | "robe" => ItemSize { width: 2, height: 3 },

        // ring, amulet, gem, rune, jewel, potion, scroll, key, and any unknown type
        _ => ItemSize { width: 1, height: 1 },
    }
}

// ---------------------------------------------------------------------------
// Data model — InventorySlot
// ---------------------------------------------------------------------------

/// A single item placed in the inventory grid.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InventorySlot {
    /// Unique item identifier (UUID v4 string).
    pub item_id: String,
    /// Base type key used to derive size and display.
    pub base_type: String,
    /// Top-left column of the item (0-9).
    pub col: u8,
    /// Top-left row of the item (0-3).
    pub row: u8,
    /// Cached footprint of this item.
    pub size: ItemSize,
}

// ---------------------------------------------------------------------------
// Data model — InventoryError
// ---------------------------------------------------------------------------

/// Errors that can occur when placing or manipulating items in the grid.
#[derive(Debug, thiserror::Error)]
pub enum InventoryError {
    /// Another item already occupies at least one of the required cells.
    #[error("Item does not fit at ({col}, {row}): overlap")]
    Overlap {
        /// Column of the conflicting cell.
        col: u8,
        /// Row of the conflicting cell.
        row: u8,
    },
    /// The item footprint would extend outside the grid boundaries.
    #[error("Item does not fit at ({col}, {row}): out of bounds")]
    OutOfBounds {
        /// Column where the boundary is exceeded.
        col: u8,
        /// Row where the boundary is exceeded.
        row: u8,
    },
    /// No item occupies the requested cell.
    #[error("No item at ({col}, {row})")]
    Empty {
        /// Column queried.
        col: u8,
        /// Row queried.
        row: u8,
    },
}

// ---------------------------------------------------------------------------
// Data model — InventoryGrid
// ---------------------------------------------------------------------------

/// 10×4 grid that tracks item placement with O(1) cell lookup.
///
/// Each cell stores the index into `items` of the occupying item, or `None`.
/// Invariant: every cell covered by `items[i]` stores `Some(i)`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InventoryGrid {
    /// `occupied[row][col]` = `Some(item_index)` or `None`.
    occupied: [[Option<usize>; GRID_COLS]; GRID_ROWS],
    /// Ordered list of placed items.
    items: Vec<InventorySlot>,
}

impl InventoryGrid {
    /// Create an empty grid.
    pub fn new() -> Self {
        Self {
            occupied: [[None; GRID_COLS]; GRID_ROWS],
            items: Vec::new(),
        }
    }

    /// Place a new item at `(col, row)`.
    ///
    /// Returns the index of the newly placed item on success.
    ///
    /// # Errors
    ///
    /// - `InventoryError::OutOfBounds` if the footprint exceeds grid limits.
    /// - `InventoryError::Overlap` if any covered cell is already occupied.
    pub fn place_item(
        &mut self,
        item_id: impl Into<String>,
        base_type: impl Into<String>,
        col: u8,
        row: u8,
    ) -> Result<usize, InventoryError> {
        let base_type = base_type.into();
        let size = item_size(&base_type);

        // Bounds check: every cell the item covers must be inside the grid.
        let end_col = col.saturating_add(size.width);
        let end_row = row.saturating_add(size.height);
        if end_col > COLS || end_row > ROWS {
            return Err(InventoryError::OutOfBounds { col, row });
        }

        // Overlap check.
        for r in row..end_row {
            for c in col..end_col {
                if self.occupied[r as usize][c as usize].is_some() {
                    return Err(InventoryError::Overlap { col: c, row: r });
                }
            }
        }

        // Commit: register item and mark all covered cells.
        let index = self.items.len();
        self.items.push(InventorySlot {
            item_id: item_id.into(),
            base_type,
            col,
            row,
            size,
        });

        for r in row..end_row {
            for c in col..end_col {
                self.occupied[r as usize][c as usize] = Some(index);
            }
        }

        Ok(index)
    }

    /// Remove the item at `index` and clear all cells it occupied.
    ///
    /// Returns the removed `InventorySlot`, or `None` if `index` is out of
    /// range. Because removal uses swap-remove for O(1) performance, all
    /// indices above `index` are invalidated; callers must refresh any cached
    /// indices after calling this method.
    pub fn remove_item(&mut self, index: usize) -> Option<InventorySlot> {
        if index >= self.items.len() {
            return None;
        }

        // Clear the cells occupied by the item being removed.
        let slot = self.items[index].clone();
        let end_col = slot.col + slot.size.width;
        let end_row = slot.row + slot.size.height;
        for r in slot.row..end_row {
            for c in slot.col..end_col {
                self.occupied[r as usize][c as usize] = None;
            }
        }

        // Swap-remove: the last item moves to position `index`.
        let last = self.items.len() - 1;
        if index != last {
            // Re-label every cell of the swapped item to point at `index`.
            let swapped = self.items[last].clone();
            let sw_end_col = swapped.col + swapped.size.width;
            let sw_end_row = swapped.row + swapped.size.height;
            for r in swapped.row..sw_end_row {
                for c in swapped.col..sw_end_col {
                    if self.occupied[r as usize][c as usize] == Some(last) {
                        self.occupied[r as usize][c as usize] = Some(index);
                    }
                }
            }
        }

        self.items.swap_remove(index);
        Some(slot)
    }

    /// Return a reference to the item occupying cell `(col, row)`, or `None`.
    pub fn item_at(&self, col: u8, row: u8) -> Option<&InventorySlot> {
        if col >= COLS || row >= ROWS {
            return None;
        }
        let idx = self.occupied[row as usize][col as usize]?;
        self.items.get(idx)
    }

    /// Check whether an item of `base_type` can be placed at `(col, row)`
    /// without modifying the grid.
    pub fn can_place(&self, base_type: &str, col: u8, row: u8) -> bool {
        let size = item_size(base_type);
        let end_col = col.saturating_add(size.width);
        let end_row = row.saturating_add(size.height);

        if end_col > COLS || end_row > ROWS {
            return false;
        }

        for r in row..end_row {
            for c in col..end_col {
                if self.occupied[r as usize][c as usize].is_some() {
                    return false;
                }
            }
        }
        true
    }

    /// Return the number of items currently in the grid.
    pub fn item_count(&self) -> usize {
        self.items.len()
    }

    /// Swap the positions of items `idx_a` and `idx_b`.
    ///
    /// Both items are removed from their current positions and re-placed at
    /// each other's positions. If the second placement fails (e.g. because the
    /// footprints no longer fit after the first is removed), the operation is
    /// rolled back by reinserting the first item at its original position.
    ///
    /// # Errors
    ///
    /// Returns `InventoryError::OutOfBounds` or `InventoryError::Overlap` if
    /// the swap is geometrically impossible and cannot be completed.
    pub fn swap_items(&mut self, idx_a: usize, idx_b: usize) -> Result<(), InventoryError> {
        // Snapshot positions before any mutation.
        let (col_a, row_a, type_a, id_a);
        let (col_b, row_b, type_b, id_b);
        {
            let Some(a) = self.items.get(idx_a) else { return Ok(()) };
            col_a = a.col;
            row_a = a.row;
            type_a = a.base_type.clone();
            id_a = a.item_id.clone();

            let Some(b) = self.items.get(idx_b) else { return Ok(()) };
            col_b = b.col;
            row_b = b.row;
            type_b = b.base_type.clone();
            id_b = b.item_id.clone();
        }

        // Remove both items (higher index first to keep lower index stable).
        let (hi, lo) = if idx_a > idx_b { (idx_a, idx_b) } else { (idx_b, idx_a) };
        self.remove_item(hi);
        self.remove_item(lo);

        // Place A at B's old position, then B at A's old position.
        match self.place_item(id_a.clone(), type_a.clone(), col_b, row_b) {
            Ok(_) => {}
            Err(e) => {
                // Roll back: restore A and B at their original positions.
                // Best-effort; ignore errors during rollback.
                let _ = self.place_item(id_a, type_a, col_a, row_a);
                let _ = self.place_item(id_b, type_b, col_b, row_b);
                return Err(e);
            }
        }

        match self.place_item(id_b.clone(), type_b.clone(), col_a, row_a) {
            Ok(_) => Ok(()),
            Err(e) => {
                // Roll back both.
                // Remove the A that was just placed (it is now the last item).
                let last = self.items.len().saturating_sub(1);
                self.remove_item(last);
                let _ = self.place_item(id_a, type_a, col_a, row_a);
                let _ = self.place_item(id_b, type_b, col_b, row_b);
                Err(e)
            }
        }
    }
}

impl Default for InventoryGrid {
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

    #[test]
    fn inventory_place_item() {
        let mut grid = InventoryGrid::new();
        let idx = grid.place_item("item-001", "ring", 0, 0).expect("place failed");
        assert_eq!(idx, 0);
        assert_eq!(grid.item_count(), 1);
    }

    #[test]
    fn inventory_overlap_reject() {
        let mut grid = InventoryGrid::new();
        grid.place_item("item-001", "belt", 0, 0).expect("first place failed");
        // belt = 2x2; a second 2x2 at (1,1) overlaps at cell (1,1)
        let result = grid.place_item("item-002", "belt", 1, 1);
        assert!(
            matches!(result, Err(InventoryError::Overlap { .. })),
            "expected Overlap, got {result:?}"
        );
    }

    #[test]
    fn inventory_out_of_bounds() {
        let mut grid = InventoryGrid::new();
        // bow = 2x4; placing at col=9 would require columns 9..11 (out of 0-9)
        let result = grid.place_item("item-001", "bow", 9, 0);
        assert!(
            matches!(result, Err(InventoryError::OutOfBounds { .. })),
            "expected OutOfBounds, got {result:?}"
        );
    }

    #[test]
    fn inventory_remove() {
        let mut grid = InventoryGrid::new();
        let idx = grid.place_item("item-001", "ring", 0, 0).expect("place failed");
        let removed = grid.remove_item(idx);
        assert!(removed.is_some());
        assert_eq!(removed.unwrap().item_id, "item-001");
        assert_eq!(grid.item_count(), 0);
    }

    #[test]
    fn inventory_can_place() {
        let mut grid = InventoryGrid::new();
        // Empty grid: any valid footprint should fit.
        assert!(grid.can_place("ring", 0, 0));
        // Fill (0,0).
        grid.place_item("item-001", "ring", 0, 0).expect("place failed");
        // That cell is now occupied.
        assert!(!grid.can_place("ring", 0, 0));
        // Adjacent cell is still free.
        assert!(grid.can_place("ring", 1, 0));
    }

    #[test]
    fn inventory_item_at() {
        let mut grid = InventoryGrid::new();
        grid.place_item("item-sword", "sword", 2, 0).expect("place failed");
        // sword = 1x3; all three cells should resolve to the same item.
        let at_top = grid.item_at(2, 0).expect("expected item at (2,0)");
        assert_eq!(at_top.item_id, "item-sword");

        let at_mid = grid.item_at(2, 1).expect("expected item at (2,1)");
        assert_eq!(at_mid.item_id, "item-sword");

        // Cell outside the sword footprint should be empty.
        assert!(grid.item_at(3, 0).is_none());
    }
}

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
