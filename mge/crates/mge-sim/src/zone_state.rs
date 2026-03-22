use serde::{Deserialize, Serialize};

use crate::entity::*;
use crate::gather_node::GatherNode;

/// All authoritative state for a single zone instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneState {
    pub zone_id: ZoneId,
    pub enemies: Vec<Enemy>,
    pub breakables: Vec<Breakable>,
    pub shrines: Vec<Shrine>,
    pub chests: Vec<Chest>,
    pub projectiles: Vec<Projectile>,
    pub drops: Vec<ItemDrop>,
    pub lightning_bolts: Vec<LightningBolt>,
    pub floats: Vec<DamageFloat>,
    /// Harvest nodes (trees, ore, herbs, etc.)
    pub gather_nodes: Vec<GatherNode>,
}

impl ZoneState {
    pub fn new(zone_id: ZoneId) -> Self {
        Self {
            zone_id,
            enemies: Vec::new(),
            breakables: Vec::new(),
            shrines: Vec::new(),
            chests: Vec::new(),
            projectiles: Vec::new(),
            drops: Vec::new(),
            lightning_bolts: Vec::new(),
            floats: Vec::new(),
            gather_nodes: Vec::new(),
        }
    }

    /// Clear transient data (on zone enter/leave).
    pub fn clear_transient(&mut self) {
        self.projectiles.clear();
        self.drops.clear();
        self.lightning_bolts.clear();
        self.floats.clear();
    }
}
