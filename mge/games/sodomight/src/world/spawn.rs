// @id: Sodomight-World-Spawn @do: entity-spawning @role: back-end @layer: 4 @human: miyuk
//! Spawning logic for monsters and item drops in the Sodomight world.

use super::SodomightWorld;
use super::types::{
    DEFAULT_ATTACK_RANGE, DEFAULT_SIGHT_RANGE, MonsterRecord, WorldError,
};
use crate::content::MonsterDef;
use mge_arpg_ai::{AggroRange, AiAgent};
use mge_arpg_entity::{Health, ItemDrop, Level, Position, Team};
use mge_ecs::EntityId;

impl SodomightWorld {
    // -------------------------------------------------------------------
    // Spawning
    // -------------------------------------------------------------------

    /// Spawn a monster in the world.
    ///
    /// Returns the `EntityId` of the newly created monster entity.
    pub fn spawn_monster(
        &mut self,
        name: &str,
        x: f32,
        y: f32,
        level: u8,
        max_health: u32,
    ) -> Result<EntityId, WorldError> {
        let record = MonsterRecord {
            name: name.to_string(),
            position: Position::new(x, y),
            health: Health::new(max_health),
            level: Level::new(level),
            team: Team::ENEMY,
        };

        let entity_id = self
            .ecs
            .spawn_with_1(record)
            .map_err(|e| WorldError::EcsError(e.to_string()))?;

        // Create an AI agent for this monster.
        let agent = AiAgent::new(
            entity_id.index,
            AggroRange::new(DEFAULT_SIGHT_RANGE, DEFAULT_ATTACK_RANGE),
        );
        self.ai_agents.insert(entity_id, agent);

        Ok(entity_id)
    }

    /// Spawn a monster from a [`MonsterDef`], using its stats for XP, loot,
    /// speed, and aggro range.
    ///
    /// This is the preferred way to spawn content-defined monsters. The
    /// original [`spawn_monster`](Self::spawn_monster) remains available for
    /// test helpers and ad-hoc spawning.
    pub fn spawn_monster_from_def(
        &mut self,
        def: &MonsterDef,
        x: f32,
        y: f32,
    ) -> Result<EntityId, WorldError> {
        let health_u32 = u32::try_from(def.health.max(0)).unwrap_or(1);
        let record = MonsterRecord {
            name: def.name.clone(),
            position: Position::new(x, y),
            health: Health::new(health_u32),
            level: Level::new(def.level),
            team: Team::ENEMY,
        };

        let entity_id = self
            .ecs
            .spawn_with_1(record)
            .map_err(|e| WorldError::EcsError(e.to_string()))?;

        // Use the def's aggro range, fall back to defaults for attack range.
        let agent = AiAgent::new(
            entity_id.index,
            AggroRange::new(def.aggro_range, DEFAULT_ATTACK_RANGE),
        );
        self.ai_agents.insert(entity_id, agent);

        // Store per-monster overrides for death handling.
        self.monster_xp_rewards.insert(entity_id, def.xp_reward);
        self.monster_tc_ids
            .insert(entity_id, def.tc_id.clone());

        Ok(entity_id)
    }

    // -------------------------------------------------------------------
    // Item drop spawning
    // -------------------------------------------------------------------

    /// Spawn an item drop entity at `(x, y)`.
    ///
    /// Creates a new ECS entity whose sole component is [`ItemDrop`]. The
    /// returned [`EntityId`] can be passed to [`pickup_item`](Self::pickup_item)
    /// when the player walks nearby.
    pub fn spawn_item_drop(
        &mut self,
        item_id: &str,
        quality_color: [f32; 4],
        x: f32,
        y: f32,
    ) -> EntityId {
        let drop = ItemDrop::new(item_id, quality_color, x, y, self.game_tick);
        // spawn_with_1 only fails on allocation; the ECS is infallible here.
        self.ecs.spawn_with_1(drop).unwrap_or_else(|_| {
            // Unreachable in practice — the ECS allocator is not bounded.
            panic!("ECS allocation failed while spawning ItemDrop")
        })
    }
}
