// @id: MGE-ARPG-Combat-Events @do: combat-events @role: back-end @layer: 3 @human: miyuk
//! Combat events emitted by the combat pipeline.

use crate::damage::{DamageRoll, DamageType};

/// An event produced by the combat pipeline for a single attack resolution.
#[derive(Debug, Clone)]
pub enum CombatEvent {
    /// The attack connected and dealt damage.
    Hit {
        /// Entity ID of the attacker.
        attacker_id: u32,
        /// Entity ID of the defender.
        defender_id: u32,
        /// Full damage roll result.
        damage: DamageRoll,
    },
    /// The attack missed.
    Miss {
        /// Entity ID of the attacker.
        attacker_id: u32,
        /// Entity ID of the defender.
        defender_id: u32,
    },
    /// The defender died from the attack.
    Death {
        /// Entity ID of the dead entity.
        entity_id: u32,
        /// Entity ID of the killer.
        killer_id: u32,
    },
    /// The defender is immune to the damage type.
    Immune {
        /// Entity ID of the immune defender.
        defender_id: u32,
        /// The damage type the defender is immune to.
        dtype: DamageType,
    },
}

/// Aggregate result of a single attack resolution.
#[derive(Debug, Clone)]
pub struct CombatResult {
    /// All events generated during this attack.
    pub events: Vec<CombatEvent>,
    /// Total damage dealt (0 if missed or immune).
    pub total_damage: i32,
    /// Whether the attack hit.
    pub hit: bool,
}
