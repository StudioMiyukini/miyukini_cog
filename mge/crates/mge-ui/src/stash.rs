use serde::{Deserialize, Serialize};

use crate::inventory::{InventoryGrid, ItemRef};

/// A stash tab with its own inventory grid.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StashTab {
    pub name: String,
    pub grid: InventoryGrid,
}

impl StashTab {
    pub fn new(name: impl Into<String>, width: u8, height: u8) -> Self {
        Self { name: name.into(), grid: InventoryGrid::new(width, height) }
    }
}

/// Multi-tab stash (shared or personal).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stash {
    tabs: Vec<StashTab>,
    active_tab: usize,
    max_tabs: usize,
}

impl Stash {
    pub fn new(initial_tabs: usize, tab_width: u8, tab_height: u8, max_tabs: usize) -> Self {
        let tabs = (0..initial_tabs)
            .map(|i| StashTab::new(format!("Tab {}", i + 1), tab_width, tab_height))
            .collect();
        Self { tabs, active_tab: 0, max_tabs }
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn active_tab_index(&self) -> usize {
        self.active_tab
    }

    pub fn set_active_tab(&mut self, index: usize) -> bool {
        if index < self.tabs.len() {
            self.active_tab = index;
            true
        } else {
            false
        }
    }

    pub fn active_grid(&self) -> &InventoryGrid {
        &self.tabs[self.active_tab].grid
    }

    pub fn active_grid_mut(&mut self) -> &mut InventoryGrid {
        &mut self.tabs[self.active_tab].grid
    }

    pub fn tab(&self, index: usize) -> Option<&StashTab> {
        self.tabs.get(index)
    }

    /// Add a new tab if below max.
    pub fn add_tab(&mut self, name: impl Into<String>, width: u8, height: u8) -> bool {
        if self.tabs.len() >= self.max_tabs {
            return false;
        }
        self.tabs.push(StashTab::new(name, width, height));
        true
    }

    /// Transfer an item from inventory grid to stash (auto-place in active tab).
    pub fn transfer_in(
        &mut self,
        source: &mut InventoryGrid,
        item_id: u64,
        item: &ItemRef,
    ) -> bool {
        if source.find_item(item_id).is_none() {
            return false;
        }
        if self.active_grid_mut().auto_place(item) {
            source.remove(item_id);
            true
        } else {
            false
        }
    }

    /// Transfer an item from stash to inventory grid (auto-place).
    pub fn transfer_out(&mut self, dest: &mut InventoryGrid, item_id: u64, item: &ItemRef) -> bool {
        if self.active_grid().find_item(item_id).is_none() {
            return false;
        }
        if dest.auto_place(item) {
            self.active_grid_mut().remove(item_id);
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_item(id: u64) -> ItemRef {
        ItemRef { id, name: format!("item_{id}"), icon: format!("icon_{id}"), grid_w: 1, grid_h: 1 }
    }

    #[test]
    fn stash_creation_and_tabs() {
        let stash = Stash::new(3, 10, 10, 5);
        assert_eq!(stash.tab_count(), 3);
        assert_eq!(stash.active_tab_index(), 0);
    }

    #[test]
    fn switch_tabs() {
        let mut stash = Stash::new(3, 10, 10, 5);
        assert!(stash.set_active_tab(2));
        assert_eq!(stash.active_tab_index(), 2);
        assert!(!stash.set_active_tab(5));
    }

    #[test]
    fn add_tab_respects_max() {
        let mut stash = Stash::new(2, 10, 10, 3);
        assert!(stash.add_tab("Extra", 10, 10));
        assert!(!stash.add_tab("Too many", 10, 10));
    }

    #[test]
    fn transfer_in_and_out() {
        let mut inv = InventoryGrid::new(10, 4);
        let item = make_item(1);
        inv.auto_place(&item);

        let mut stash = Stash::new(1, 10, 10, 3);
        assert!(stash.transfer_in(&mut inv, 1, &item));
        assert!(inv.find_item(1).is_none());
        assert!(stash.active_grid().find_item(1).is_some());

        assert!(stash.transfer_out(&mut inv, 1, &item));
        assert!(inv.find_item(1).is_some());
        assert!(stash.active_grid().find_item(1).is_none());
    }
}
