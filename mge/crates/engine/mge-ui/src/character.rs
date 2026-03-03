// @id: MGE-UI-Character @do: paperdoll @role: front-end @layer: 2 @human: miyuk
//! Character stats panel (D2-style): base stats, derived stats, resistances.
//! Also provides the paperdoll data model: `EquipSlot`, `EquippedItem`,
//! `PaperdollState`, `EquipError`, and the `can_equip` helper.

use egui::Context;

use crate::theme::D2Colors;

// ---------------------------------------------------------------------------
// Data
// ---------------------------------------------------------------------------

/// Character data displayed in the stats panel.
pub struct CharacterData {
    /// Character name.
    pub name: String,
    /// Class name.
    pub class: String,
    /// Current level.
    pub level: i32,
    /// Strength.
    pub strength: i32,
    /// Dexterity.
    pub dexterity: i32,
    /// Vitality.
    pub vitality: i32,
    /// Energy.
    pub energy: i32,
    /// Stat points available to distribute.
    pub unspent_points: i32,
    /// Base life (before modifiers).
    pub life_base: i32,
    /// Base mana (before modifiers).
    pub mana_base: i32,
    /// Total defense.
    pub defense: i32,
    /// Minimum weapon damage.
    pub damage_min: i32,
    /// Maximum weapon damage.
    pub damage_max: i32,
    /// Attack rating.
    pub attack_rating: i32,
    /// Fire resistance (%).
    pub fire_res: i32,
    /// Cold resistance (%).
    pub cold_res: i32,
    /// Lightning resistance (%).
    pub lightning_res: i32,
    /// Poison resistance (%).
    pub poison_res: i32,
}

// ---------------------------------------------------------------------------
// Public draw entry-point
// ---------------------------------------------------------------------------

/// Draw the character stats panel.
pub fn draw_character_panel(ctx: &Context, is_open: &mut bool, data: &CharacterData) {
    if !*is_open {
        return;
    }

    egui::Window::new("Personnage")
        .resizable(false)
        .collapsible(false)
        .open(is_open)
        .default_pos([20.0, 50.0])
        .min_width(220.0)
        .show(ctx, |ui| {
            // Header
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new(&data.name)
                        .color(D2Colors::GOLD_BRIGHT)
                        .size(14.0),
                );
                let class_level = format!("{} - Nv {}", data.class, data.level);
                ui.label(
                    egui::RichText::new(&class_level)
                        .color(D2Colors::TEXT_NORMAL)
                        .size(11.0),
                );
            });
            ui.separator();

            // Unspent points
            if data.unspent_points > 0 {
                let pts_text = format!("Points a distribuer : {}", data.unspent_points);
                ui.label(
                    egui::RichText::new(&pts_text)
                        .color(D2Colors::SKILL_ACTIVE)
                        .size(11.0),
                );
                ui.separator();
            }

            // Base stats
            ui.label(
                egui::RichText::new("-- Stats de base --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            let can_add = data.unspent_points > 0;
            egui::Grid::new("base_stats").num_columns(3).show(ui, |ui| {
                stat_row(ui, "Force", data.strength, can_add);
                stat_row(ui, "Dexterite", data.dexterity, can_add);
                stat_row(ui, "Vitalite", data.vitality, can_add);
                stat_row(ui, "Energie", data.energy, can_add);
            });
            ui.separator();

            // Derived stats
            ui.label(
                egui::RichText::new("-- Stats derivees --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            egui::Grid::new("derived_stats")
                .num_columns(2)
                .show(ui, |ui| {
                    derived_stat(ui, "Vie", data.life_base);
                    derived_stat(ui, "Mana", data.mana_base);
                    derived_stat(ui, "Defense", data.defense);
                    derived_stat(ui, "Attaque", data.attack_rating);
                    let dmg_label = format!("Degats {}-{}", data.damage_min, data.damage_max);
                    derived_stat(ui, &dmg_label, 0);
                });
            ui.separator();

            // Resistances
            ui.label(
                egui::RichText::new("-- Resistances --")
                    .color(D2Colors::GOLD)
                    .size(11.0),
            );
            egui::Grid::new("res_stats").num_columns(2).show(ui, |ui| {
                res_row(ui, "Feu", data.fire_res);
                res_row(ui, "Froid", data.cold_res);
                res_row(ui, "Foudre", data.lightning_res);
                res_row(ui, "Poison", data.poison_res);
            });
        });
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// A base stat row with optional "+" button for point distribution.
fn stat_row(ui: &mut egui::Ui, label: &str, value: i32, can_add: bool) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    let val_text = value.to_string();
    ui.label(
        egui::RichText::new(&val_text)
            .color(D2Colors::GOLD)
            .size(11.0),
    );
    if can_add {
        if ui.small_button("+").clicked() {
            // Signal to game: add 1 point to this stat
        }
    } else {
        ui.label("");
    }
    ui.end_row();
}

/// A derived (read-only) stat row.
fn derived_stat(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    if value > 0 {
        let val_text = value.to_string();
        ui.label(
            egui::RichText::new(&val_text)
                .color(D2Colors::GOLD)
                .size(11.0),
        );
    }
    ui.end_row();
}

/// A resistance row with color-coded value.
fn res_row(ui: &mut egui::Ui, label: &str, value: i32) {
    ui.label(
        egui::RichText::new(label)
            .color(D2Colors::TEXT_NORMAL)
            .size(11.0),
    );
    let color = if value >= 75 {
        D2Colors::GOLD_BRIGHT
    } else if value >= 0 {
        D2Colors::GOLD
    } else {
        D2Colors::RED_LIFE
    };
    let res_text = format!("{value}%");
    ui.label(egui::RichText::new(&res_text).color(color).size(11.0));
    ui.end_row();
}

// ---------------------------------------------------------------------------
// Paperdoll data model
// ---------------------------------------------------------------------------

use std::collections::HashMap;

/// The 10 equipment slots on the character paperdoll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum EquipSlot {
    /// Head armor slot.
    Helm,
    /// Body armor slot.
    Armor,
    /// Belt slot.
    Belt,
    /// Boots slot.
    Boots,
    /// Gloves slot.
    Gloves,
    /// Amulet slot.
    Amulet,
    /// First ring slot.
    Ring1,
    /// Second ring slot.
    Ring2,
    /// Main-hand weapon slot.
    Weapon,
    /// Off-hand / shield slot.
    Shield,
}

/// A single item occupying an equipment slot.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquippedItem {
    /// Unique item identifier (UUID v4 string).
    pub item_id: String,
    /// Base type key (e.g. `"helm"`, `"sword"`, `"ring"`).
    pub base_type: String,
    /// Localised display name shown in the UI.
    pub display_name: String,
}

/// Full paperdoll state: equipped items + unspent stat points.
///
/// All 10 slots are always present in the map; empty slots hold `None`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PaperdollState {
    /// Equipment slots — always contains an entry for every `EquipSlot` variant.
    pub slots: HashMap<EquipSlot, Option<EquippedItem>>,
    /// Stat points available to allocate.
    pub stat_points_available: u32,
    /// Strength points allocated this session.
    pub str_allocated: u32,
    /// Dexterity points allocated this session.
    pub dex_allocated: u32,
    /// Vitality points allocated this session.
    pub vit_allocated: u32,
    /// Energy points allocated this session.
    pub ene_allocated: u32,
}

/// Errors that can occur when equipping items or allocating stat points.
#[derive(Debug, thiserror::Error)]
pub enum EquipError {
    /// The item's base type is not compatible with the target slot.
    #[error("Item type '{base_type}' cannot go in {slot:?}")]
    WrongSlot {
        /// Base type of the item that was rejected.
        base_type: String,
        /// The slot that was targeted.
        slot: EquipSlot,
    },
    /// All available stat points have already been allocated.
    #[error("No stat points available")]
    NoStatPoints,
}

impl PaperdollState {
    /// Create an empty paperdoll with all slots unoccupied and 0 stat points.
    pub fn new() -> Self {
        let mut slots = HashMap::new();
        for slot in [
            EquipSlot::Helm,
            EquipSlot::Armor,
            EquipSlot::Belt,
            EquipSlot::Boots,
            EquipSlot::Gloves,
            EquipSlot::Amulet,
            EquipSlot::Ring1,
            EquipSlot::Ring2,
            EquipSlot::Weapon,
            EquipSlot::Shield,
        ] {
            slots.insert(slot, None);
        }
        Self {
            slots,
            stat_points_available: 0,
            str_allocated: 0,
            dex_allocated: 0,
            vit_allocated: 0,
            ene_allocated: 0,
        }
    }

    /// Equip `item` into `slot`.
    ///
    /// Returns the previously-equipped item (if any) on success so the caller
    /// can move it back to inventory.
    ///
    /// # Errors
    ///
    /// Returns [`EquipError::WrongSlot`] when the item's `base_type` is
    /// incompatible with `slot` according to [`can_equip`].
    pub fn equip(
        &mut self,
        slot: EquipSlot,
        item: EquippedItem,
    ) -> Result<Option<EquippedItem>, EquipError> {
        if !can_equip(slot, &item.base_type) {
            return Err(EquipError::WrongSlot {
                base_type: item.base_type.clone(),
                slot,
            });
        }
        let previous = self.slots.insert(slot, Some(item)).flatten();
        Ok(previous)
    }

    /// Remove and return the item in `slot`, leaving the slot empty.
    pub fn unequip(&mut self, slot: EquipSlot) -> Option<EquippedItem> {
        self.slots.insert(slot, None).flatten()
    }

    /// Return a reference to the item in `slot`, or `None` if empty.
    pub fn get(&self, slot: EquipSlot) -> Option<&EquippedItem> {
        self.slots.get(&slot).and_then(Option::as_ref)
    }

    /// Allocate one point to the named stat.
    ///
    /// Valid stat names: `"str"`, `"dex"`, `"vit"`, `"ene"`.
    ///
    /// # Errors
    ///
    /// Returns [`EquipError::NoStatPoints`] when `stat_points_available == 0`.
    pub fn allocate_stat(&mut self, stat: &str) -> Result<(), EquipError> {
        if self.stat_points_available == 0 {
            return Err(EquipError::NoStatPoints);
        }
        match stat {
            "str" => self.str_allocated += 1,
            "dex" => self.dex_allocated += 1,
            "vit" => self.vit_allocated += 1,
            "ene" => self.ene_allocated += 1,
            _ => {
                // Unknown stat name — treat as a no-op rather than panic.
                return Ok(());
            }
        }
        self.stat_points_available -= 1;
        Ok(())
    }
}

impl Default for PaperdollState {
    fn default() -> Self {
        Self::new()
    }
}

/// Return `true` when `base_type` is a legal item type for `slot`.
///
/// The mapping mirrors the Diablo II item classification:
///
/// | Slot | Accepted base types |
/// |------|---------------------|
/// | Helm | `"helm"`, `"circlet"`, `"crown"` |
/// | Armor | `"armor"`, `"plate"`, `"robe"` |
/// | Belt | `"belt"`, `"sash"` |
/// | Boots | `"boots"`, `"greaves"` |
/// | Gloves | `"gloves"`, `"gauntlets"` |
/// | Amulet | `"amulet"` |
/// | Ring1 / Ring2 | `"ring"` |
/// | Weapon | `"sword"`, `"axe"`, `"mace"`, `"dagger"`, `"wand"`, `"scepter"`, `"staff"`, `"bow"`, `"crossbow"`, `"spear"`, `"polearm"` |
/// | Shield | `"shield"`, `"buckler"` |
pub fn can_equip(slot: EquipSlot, base_type: &str) -> bool {
    match slot {
        EquipSlot::Helm => matches!(base_type, "helm" | "circlet" | "crown"),
        EquipSlot::Armor => matches!(base_type, "armor" | "plate" | "robe"),
        EquipSlot::Belt => matches!(base_type, "belt" | "sash"),
        EquipSlot::Boots => matches!(base_type, "boots" | "greaves"),
        EquipSlot::Gloves => matches!(base_type, "gloves" | "gauntlets"),
        EquipSlot::Amulet => matches!(base_type, "amulet"),
        EquipSlot::Ring1 | EquipSlot::Ring2 => matches!(base_type, "ring"),
        EquipSlot::Weapon => matches!(
            base_type,
            "sword"
                | "axe"
                | "mace"
                | "dagger"
                | "wand"
                | "scepter"
                | "staff"
                | "bow"
                | "crossbow"
                | "spear"
                | "polearm"
        ),
        EquipSlot::Shield => matches!(base_type, "shield" | "buckler"),
    }
}

// ---------------------------------------------------------------------------
// Tests — paperdoll
// ---------------------------------------------------------------------------

#[cfg(test)]
mod paperdoll_tests {
    use super::*;

    fn make_item(id: &str, base_type: &str, name: &str) -> EquippedItem {
        EquippedItem {
            item_id: id.to_string(),
            base_type: base_type.to_string(),
            display_name: name.to_string(),
        }
    }

    #[test]
    fn paperdoll_equip_helm() {
        let mut pd = PaperdollState::new();
        let item = make_item("item-001", "helm", "Iron Helm");
        pd.equip(EquipSlot::Helm, item).expect("equip helm failed");
        let got = pd.get(EquipSlot::Helm).expect("slot should be occupied");
        assert_eq!(got.item_id, "item-001");
        assert_eq!(got.display_name, "Iron Helm");
    }

    #[test]
    fn paperdoll_wrong_slot() {
        let mut pd = PaperdollState::new();
        let item = make_item("item-002", "helm", "Iron Helm");
        let result = pd.equip(EquipSlot::Boots, item);
        assert!(
            matches!(
                result,
                Err(EquipError::WrongSlot {
                    slot: EquipSlot::Boots,
                    ..
                })
            ),
            "expected WrongSlot, got {result:?}"
        );
    }

    #[test]
    fn paperdoll_stat_alloc() {
        let mut pd = PaperdollState::new();
        pd.stat_points_available = 5;
        pd.allocate_stat("str").expect("allocate str failed");
        assert_eq!(pd.stat_points_available, 4);
        assert_eq!(pd.str_allocated, 1);
    }

    #[test]
    fn paperdoll_no_stat_points() {
        let mut pd = PaperdollState::new();
        // stat_points_available is 0 by default
        let result = pd.allocate_stat("dex");
        assert!(
            matches!(result, Err(EquipError::NoStatPoints)),
            "expected NoStatPoints, got {result:?}"
        );
    }

    #[test]
    fn paperdoll_unequip() {
        let mut pd = PaperdollState::new();
        let item = make_item("item-003", "ring", "Ruby Ring");
        pd.equip(EquipSlot::Ring1, item).expect("equip ring failed");
        let removed = pd.unequip(EquipSlot::Ring1);
        assert!(removed.is_some(), "should have returned the item");
        assert_eq!(removed.expect("item").item_id, "item-003");
        assert!(pd.get(EquipSlot::Ring1).is_none(), "slot should be empty");
    }
}
