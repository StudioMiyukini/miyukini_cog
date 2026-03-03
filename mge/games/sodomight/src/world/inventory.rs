// @id: Sodomight-World-Inventory @do: loot-equipment @role: back-end @layer: 4 @human: miyuk
//! Loot pickup, item pickup, equipment management, and potion usage.

use super::SodomightWorld;
use super::types::{PlayerRecord, WorldError};
use mge_arpg_entity::ItemDrop;
use mge_arpg_items::{ItemInstance, ItemSlot};
use mge_ecs::EntityId;

impl SodomightWorld {
    // -------------------------------------------------------------------
    // Item pickup (ECS-based drops)
    // -------------------------------------------------------------------

    /// Maximum pickup distance (world units). Matches D2 ~2-tile radius.
    const PICKUP_RADIUS: f32 = 2.0;

    /// Attempt to pick up an item drop entity.
    ///
    /// Checks that the player is within [`PICKUP_RADIUS`] of the drop. If so,
    /// the drop entity is despawned and the `item_id` is returned so the caller
    /// can add the item to the player's inventory.
    ///
    /// # Errors
    /// - [`WorldError::EntityNotFound`] if `drop_entity_id` is no longer alive.
    /// - [`WorldError::TooFar`] if the player is more than `PICKUP_RADIUS` world
    ///   units away from the drop.
    pub fn pickup_item(&mut self, drop_entity_id: EntityId) -> Result<String, WorldError> {
        // Verify the entity is alive and get the drop data.
        let drop = self
            .ecs
            .get_component::<ItemDrop>(drop_entity_id)
            .map_err(|_| WorldError::EntityNotFound)?
            .clone();

        // Check proximity.
        let (px, py) = self.player_position();
        let dx = drop.position_x - px;
        let dy = drop.position_y - py;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > Self::PICKUP_RADIUS {
            return Err(WorldError::TooFar {
                distance: dist,
                max: Self::PICKUP_RADIUS,
            });
        }

        // Despawn the drop entity.
        let _ = self.ecs.despawn(drop_entity_id);

        Ok(drop.item_id)
    }

    // -------------------------------------------------------------------
    // Loot pickup (pending_loot piles)
    // -------------------------------------------------------------------

    /// Attempt to pick up a loot drop from the ground.
    ///
    /// The `loot_index` indexes into `pending_loot`, and `drop_index`
    /// selects a specific item within that pile. If successful, the item
    /// is added to the player's inventory.
    pub fn player_pickup_loot(
        &mut self,
        loot_index: usize,
        drop_index: usize,
    ) -> Result<String, WorldError> {
        let pile = self
            .pending_loot
            .get(loot_index)
            .ok_or(WorldError::InvalidLootIndex(loot_index))?;

        let drop = pile
            .2
            .get(drop_index)
            .ok_or(WorldError::InvalidLootIndex(drop_index))?;

        if drop.item_id == "gold" {
            self.player_gold = self.player_gold.saturating_add(drop.quantity);
            let msg = format!("Picked up {} gold (total: {})", drop.quantity, self.player_gold);
            self.combat_log.push(msg.clone());
            // Remove the drop from the pile.
            let pile_mut = &mut self.pending_loot[loot_index];
            pile_mut.2.remove(drop_index);
            if pile_mut.2.is_empty() {
                self.pending_loot.remove(loot_index);
            }
            return Ok(msg);
        }

        // Create an item instance from the drop.
        let item = ItemInstance::new_normal(drop.item_id.clone(), 1);

        let slot = self
            .player_inventory
            .find_free_slot()
            .ok_or(WorldError::InventoryFull)?;

        self.player_inventory
            .try_place(item, slot.0, slot.1)
            .map_err(|_| WorldError::InventoryFull)?;

        let msg = format!("Picked up {}", drop.item_id);
        self.combat_log.push(msg.clone());

        // Remove the drop from the pile.
        let pile_mut = &mut self.pending_loot[loot_index];
        pile_mut.2.remove(drop_index);
        if pile_mut.2.is_empty() {
            self.pending_loot.remove(loot_index);
        }

        Ok(msg)
    }

    // -------------------------------------------------------------------
    // Equipment
    // -------------------------------------------------------------------

    /// Equip an item from the inventory into an equipment slot.
    ///
    /// The item at `(inv_col, inv_row)` is moved to the equipment slot.
    /// If the equipment slot already holds an item, it is swapped back
    /// into the inventory slot.
    pub fn player_equip(
        &mut self,
        inv_col: usize,
        inv_row: usize,
        equip_slot: ItemSlot,
    ) -> Result<String, WorldError> {
        let item = self
            .player_inventory
            .remove(inv_col, inv_row)
            .ok_or(WorldError::InventorySlotEmpty)?;

        let item_name = item.base_id.clone();

        // Equip returns the previously equipped item (if any).
        let previous = self.player_equipment.equip(equip_slot, item);

        // If there was a previously equipped item, put it in the freed inventory slot.
        if let Some(prev_item) = previous {
            let _ = self.player_inventory.try_place(prev_item, inv_col, inv_row);
        }

        // Recalculate derived stats.
        self.player_stats.recalculate();

        // Sync player ECS health values.
        let max_life = self.player_stats.derived.max_life;
        let current_life = self.player_stats.current_life;
        let _ = self
            .ecs
            .modify_component::<PlayerRecord>(self.player_id, |pr| {
                pr.health.max = max_life.max(0) as u32;
                pr.health.current = current_life.max(0) as u32;
            });

        let msg = format!("Equipped {item_name} in {equip_slot:?}");
        self.combat_log.push(msg.clone());

        Ok(msg)
    }

    // -------------------------------------------------------------------
    // Potions
    // -------------------------------------------------------------------

    /// Use a health potion from the player's inventory.
    ///
    /// Scans the inventory for the first `minor_health_potion` and consumes it,
    /// restoring 50 HP. Returns a message describing the result.
    pub fn use_health_potion(&mut self) -> Result<String, WorldError> {
        let slot = self
            .player_inventory
            .find_item("minor_health_potion")
            .ok_or(WorldError::SkillError("No health potions".to_string()))?;
        let _ = self.player_inventory.remove(slot.0, slot.1);
        self.player_stats.restore_life(50);
        let msg = format!(
            "Used health potion (HP: {}/{})",
            self.player_stats.current_life, self.player_stats.derived.max_life
        );
        self.combat_log.push(msg.clone());
        Ok(msg)
    }

    /// Use a mana potion from the player's inventory.
    ///
    /// Scans the inventory for the first `minor_mana_potion` and consumes it,
    /// restoring 30 MP. Returns a message describing the result.
    pub fn use_mana_potion(&mut self) -> Result<String, WorldError> {
        let slot = self
            .player_inventory
            .find_item("minor_mana_potion")
            .ok_or(WorldError::SkillError("No mana potions".to_string()))?;
        let _ = self.player_inventory.remove(slot.0, slot.1);
        self.player_stats.restore_mana(30);
        let msg = format!(
            "Used mana potion (MP: {}/{})",
            self.player_stats.current_mana, self.player_stats.derived.max_mana
        );
        self.combat_log.push(msg.clone());
        Ok(msg)
    }
}
