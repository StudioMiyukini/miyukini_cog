//! @id mge.rpg.inventory.v1.events
//! @role event
//! @layer plugin
//! @domain rpg

use mge_ecs::EntityId;
use mge_event::Event;

use crate::components::EquipSlot;

/// @id mge.rpg.inventory.v1.event.itemaddedevent
/// @fields owner:EntityId,item:EntityId,slot:usize
/// Emis quand un item est ajouté à un inventaire.
#[derive(Debug, Clone)]
pub struct ItemAddedEvent {
    pub owner: EntityId,
    pub item: EntityId,
    pub slot: usize,
}

impl Event for ItemAddedEvent {}

/// @id mge.rpg.inventory.v1.event.itemremovedevent
/// @fields owner:EntityId,item:EntityId,slot:usize
/// Emis quand un item est retiré d'un inventaire.
#[derive(Debug, Clone)]
pub struct ItemRemovedEvent {
    pub owner: EntityId,
    pub item: EntityId,
    pub slot: usize,
}

impl Event for ItemRemovedEvent {}

/// @id mge.rpg.inventory.v1.event.itemequippedevent
/// @fields owner:EntityId,item:EntityId,slot:EquipSlot
/// Emis quand un item est équipé.
#[derive(Debug, Clone)]
pub struct ItemEquippedEvent {
    pub owner: EntityId,
    pub item: EntityId,
    pub slot: EquipSlot,
}

impl Event for ItemEquippedEvent {}

/// @id mge.rpg.inventory.v1.event.itemunequippedevent
/// @fields owner:EntityId,item:EntityId,slot:EquipSlot
/// Emis quand un item est déséquipé.
#[derive(Debug, Clone)]
pub struct ItemUnequippedEvent {
    pub owner: EntityId,
    pub item: EntityId,
    pub slot: EquipSlot,
}

impl Event for ItemUnequippedEvent {}
