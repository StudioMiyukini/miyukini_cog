use serde::{Deserialize, Serialize};

/// Equipment slot on the paper-doll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipSlot {
    Head,
    Chest,
    Gloves,
    Belt,
    Boots,
    Amulet,
    RingLeft,
    RingRight,
    WeaponMain,
    WeaponOff,
}

/// A lightweight item reference for UI purposes.
/// Full item data lives in the gameplay/ECS layer; the UI only needs identifiers and display info.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ItemRef {
    pub id: u64,
    pub name: String,
    pub icon: String,
    /// Grid width in cells (1 or 2).
    pub grid_w: u8,
    /// Grid height in cells (1-4).
    pub grid_h: u8,
}

/// A cell in the inventory grid. `None` means empty.
pub type GridCell = Option<u64>;

/// Grid-based inventory (like D2: items occupy w x h cells).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryGrid {
    width: u8,
    height: u8,
    cells: Vec<GridCell>,
}

impl InventoryGrid {
    pub fn new(width: u8, height: u8) -> Self {
        let size = usize::from(width) * usize::from(height);
        Self { width, height, cells: vec![None; size] }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    fn index(&self, x: u8, y: u8) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(usize::from(y) * usize::from(self.width) + usize::from(x))
        } else {
            None
        }
    }

    /// Check if an item of given size can be placed at `(x, y)`.
    pub fn can_place(&self, x: u8, y: u8, w: u8, h: u8) -> bool {
        if x + w > self.width || y + h > self.height {
            return false;
        }
        for dy in 0..h {
            for dx in 0..w {
                if let Some(idx) = self.index(x + dx, y + dy) {
                    if self.cells[idx].is_some() {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    /// Place an item. Returns `false` if space is occupied.
    pub fn place(&mut self, x: u8, y: u8, item: &ItemRef) -> bool {
        if !self.can_place(x, y, item.grid_w, item.grid_h) {
            return false;
        }
        for dy in 0..item.grid_h {
            for dx in 0..item.grid_w {
                if let Some(idx) = self.index(x + dx, y + dy) {
                    self.cells[idx] = Some(item.id);
                }
            }
        }
        true
    }

    /// Remove an item by ID, clearing all cells it occupies.
    pub fn remove(&mut self, item_id: u64) -> bool {
        let mut found = false;
        for cell in &mut self.cells {
            if *cell == Some(item_id) {
                *cell = None;
                found = true;
            }
        }
        found
    }

    /// Find the top-left position of an item by ID.
    pub fn find_item(&self, item_id: u64) -> Option<(u8, u8)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(idx) = self.index(x, y) {
                    if self.cells[idx] == Some(item_id) {
                        return Some((x, y));
                    }
                }
            }
        }
        None
    }

    /// Auto-find a free position for the given item size.
    pub fn auto_place(&mut self, item: &ItemRef) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.can_place(x, y, item.grid_w, item.grid_h) {
                    return self.place(x, y, item);
                }
            }
        }
        false
    }

    pub fn cell_at(&self, x: u8, y: u8) -> GridCell {
        self.index(x, y).and_then(|i| self.cells[i])
    }

    /// Count of occupied cells.
    pub fn occupied_cells(&self) -> usize {
        self.cells.iter().filter(|c| c.is_some()).count()
    }
}

/// Equipment paper-doll state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Equipment {
    pub head: Option<ItemRef>,
    pub chest: Option<ItemRef>,
    pub gloves: Option<ItemRef>,
    pub belt: Option<ItemRef>,
    pub boots: Option<ItemRef>,
    pub amulet: Option<ItemRef>,
    pub ring_left: Option<ItemRef>,
    pub ring_right: Option<ItemRef>,
    pub weapon_main: Option<ItemRef>,
    pub weapon_off: Option<ItemRef>,
}

impl Equipment {
    pub fn get(&self, slot: EquipSlot) -> &Option<ItemRef> {
        match slot {
            EquipSlot::Head => &self.head,
            EquipSlot::Chest => &self.chest,
            EquipSlot::Gloves => &self.gloves,
            EquipSlot::Belt => &self.belt,
            EquipSlot::Boots => &self.boots,
            EquipSlot::Amulet => &self.amulet,
            EquipSlot::RingLeft => &self.ring_left,
            EquipSlot::RingRight => &self.ring_right,
            EquipSlot::WeaponMain => &self.weapon_main,
            EquipSlot::WeaponOff => &self.weapon_off,
        }
    }

    pub fn set(&mut self, slot: EquipSlot, item: Option<ItemRef>) -> Option<ItemRef> {
        let target = match slot {
            EquipSlot::Head => &mut self.head,
            EquipSlot::Chest => &mut self.chest,
            EquipSlot::Gloves => &mut self.gloves,
            EquipSlot::Belt => &mut self.belt,
            EquipSlot::Boots => &mut self.boots,
            EquipSlot::Amulet => &mut self.amulet,
            EquipSlot::RingLeft => &mut self.ring_left,
            EquipSlot::RingRight => &mut self.ring_right,
            EquipSlot::WeaponMain => &mut self.weapon_main,
            EquipSlot::WeaponOff => &mut self.weapon_off,
        };
        std::mem::replace(target, item)
    }

    /// Count equipped items.
    pub fn equipped_count(&self) -> usize {
        [
            &self.head,
            &self.chest,
            &self.gloves,
            &self.belt,
            &self.boots,
            &self.amulet,
            &self.ring_left,
            &self.ring_right,
            &self.weapon_main,
            &self.weapon_off,
        ]
        .iter()
        .filter(|s| s.is_some())
        .count()
    }
}

/// Drag-and-drop state for inventory operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragPayload {
    /// Dragging from inventory grid.
    FromGrid { item_id: u64, origin_x: u8, origin_y: u8 },
    /// Dragging from equipment slot.
    FromEquip { slot: EquipSlot },
}

/// Current drag state.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DragState {
    pub active: Option<DragPayload>,
}

impl DragState {
    pub fn start_grid_drag(&mut self, item_id: u64, x: u8, y: u8) {
        self.active = Some(DragPayload::FromGrid { item_id, origin_x: x, origin_y: y });
    }

    pub fn start_equip_drag(&mut self, slot: EquipSlot) {
        self.active = Some(DragPayload::FromEquip { slot });
    }

    pub fn cancel(&mut self) {
        self.active = None;
    }

    pub fn is_dragging(&self) -> bool {
        self.active.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: u64, w: u8, h: u8) -> ItemRef {
        ItemRef { id, name: format!("item_{id}"), icon: format!("icon_{id}"), grid_w: w, grid_h: h }
    }

    #[test]
    fn grid_place_and_find() {
        let mut grid = InventoryGrid::new(10, 4);
        let sword = make_item(1, 1, 3);
        assert!(grid.place(0, 0, &sword));
        assert_eq!(grid.find_item(1), Some((0, 0)));
        assert_eq!(grid.occupied_cells(), 3);
    }

    #[test]
    fn grid_rejects_overlap() {
        let mut grid = InventoryGrid::new(10, 4);
        let a = make_item(1, 2, 2);
        let b = make_item(2, 2, 2);
        assert!(grid.place(0, 0, &a));
        assert!(!grid.place(1, 1, &b));
    }

    #[test]
    fn grid_auto_place() {
        let mut grid = InventoryGrid::new(4, 4);
        let a = make_item(1, 2, 2);
        let b = make_item(2, 2, 2);
        let c = make_item(3, 2, 2);
        assert!(grid.auto_place(&a));
        assert!(grid.auto_place(&b));
        assert!(grid.auto_place(&c));
        assert_eq!(grid.occupied_cells(), 12);
    }

    #[test]
    fn grid_remove() {
        let mut grid = InventoryGrid::new(10, 4);
        let item = make_item(1, 2, 3);
        grid.place(0, 0, &item);
        assert!(grid.remove(1));
        assert_eq!(grid.occupied_cells(), 0);
        assert!(grid.find_item(1).is_none());
    }

    #[test]
    fn equipment_swap() {
        let mut equip = Equipment::default();
        let helm = make_item(1, 1, 1);
        let old = equip.set(EquipSlot::Head, Some(helm.clone()));
        assert!(old.is_none());
        assert_eq!(equip.equipped_count(), 1);
        let new_helm = make_item(2, 1, 1);
        let returned = equip.set(EquipSlot::Head, Some(new_helm));
        assert_eq!(returned.as_ref().map(|i| i.id), Some(1));
    }

    #[test]
    fn drag_state_lifecycle() {
        let mut drag = DragState::default();
        assert!(!drag.is_dragging());
        drag.start_grid_drag(42, 3, 1);
        assert!(drag.is_dragging());
        drag.cancel();
        assert!(!drag.is_dragging());
    }
}
