// @id: Sodomight-World-State @do: world-state @role: back-end @layer: 4 @human: miyuk
//! `SodomightWorld` struct definition, construction, and basic query methods.

use super::types::{Difficulty, MonsterRecord, PlayerRecord, WorldError};
use std::collections::HashMap;

use mge_arpg_ai::AiAgent;
use mge_arpg_combat::{StatusEffect, StatusTracker};
use mge_arpg_entity::{Health, Level, Position, Team};
use mge_arpg_items::{Equipment, Inventory};
use mge_arpg_loot::{DropRoll, TreasureClassRegistry};
use mge_arpg_quest::{QuestDef, QuestJournal};
use mge_arpg_skills::{SkillBook, SkillCooldownTracker, SkillRegistry};
use mge_arpg_stats::{BaseStats, CharacterClass, ExpTable, StatBlock};
use mge_ecs::EntityId;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Complete game state for a Sodomight session.
///
/// Encapsulates the ECS world, player data, registries, and all runtime
/// state needed by the game loop.
pub struct SodomightWorld {
    /// The ECS world storing all entities and their components.
    pub ecs: mge_ecs::World,

    /// The player's entity ID in the ECS.
    pub player_id: EntityId,

    /// The player's full stat block (base + derived + level/XP).
    pub player_stats: StatBlock,

    /// The player's grid-based inventory.
    pub player_inventory: Inventory,

    /// The player's equipment slots.
    pub player_equipment: Equipment,

    /// The player's skill book (invested skill levels).
    pub player_skills: SkillBook,

    /// The player's quest journal.
    pub player_quest_journal: QuestJournal,

    /// Cooldown tracker for all skills.
    pub cooldown_tracker: SkillCooldownTracker,

    /// Treasure class registry for loot generation.
    pub tc_registry: TreasureClassRegistry,

    /// All quest definitions available in the game.
    pub quest_registry: Vec<QuestDef>,

    /// All skill definitions available in the game.
    pub skill_registry: SkillRegistry,

    /// Combat log messages (most recent last).
    pub combat_log: Vec<String>,

    /// Pending loot drops on the ground: `(x, y, drops)`.
    pub pending_loot: Vec<(f32, f32, Vec<DropRoll>)>,

    /// Current game tick counter.
    pub game_tick: u64,

    /// Seeded RNG for determinism.
    pub rng: ChaCha8Rng,

    /// Experience table for level-up thresholds.
    pub(crate) exp_table: ExpTable,

    /// AI agents keyed by their ECS `EntityId`.
    pub(crate) ai_agents: HashMap<EntityId, AiAgent>,

    /// Active status effects per entity.
    pub(crate) status_effects: HashMap<EntityId, Vec<StatusEffect>>,

    /// Last tick at which each monster successfully attacked.
    pub(crate) attack_cooldowns: HashMap<EntityId, u64>,

    /// The player's gold counter.
    pub player_gold: u32,

    /// Per-monster XP reward overrides (from `MonsterDef`).
    pub(crate) monster_xp_rewards: HashMap<EntityId, i64>,

    /// Per-monster treasure class id overrides (from `MonsterDef`).
    pub(crate) monster_tc_ids: HashMap<EntityId, String>,

    /// `StatusTracker` per entity -- used by `combat_tick` for the ECS-integrated
    /// status pipeline. Complements the legacy `status_effects` map.
    pub(crate) status_trackers: HashMap<EntityId, StatusTracker>,

    /// Active difficulty level, governing XP loss on player death.
    pub difficulty: Difficulty,
}

impl std::fmt::Debug for SodomightWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SodomightWorld")
            .field("player_id", &self.player_id)
            .field("game_tick", &self.game_tick)
            .field("entity_count", &self.ecs.entity_count())
            .field("combat_log_len", &self.combat_log.len())
            .field("pending_loot_len", &self.pending_loot.len())
            .finish_non_exhaustive()
    }
}

impl SodomightWorld {
    /// Create a new Sodomight world with a level-1 player at the map centre.
    ///
    /// All registries start empty. Call `register_skill`, `register_tc`, etc.
    /// to populate game data.
    pub fn new() -> Result<Self, WorldError> {
        let mut ecs = mge_ecs::World::new();

        let player_record = PlayerRecord {
            position: Position::new(16.0, 16.0),
            health: Health::new(100),
            level: Level::new(1),
            team: Team::PLAYER,
        };

        let player_id = ecs
            .spawn_with_1(player_record)
            .map_err(|e| WorldError::EcsError(e.to_string()))?;

        let player_stats = StatBlock::new(BaseStats::default(), CharacterClass::Barbarian);

        Ok(Self {
            ecs,
            player_id,
            player_stats,
            player_inventory: Inventory::new(),
            player_equipment: Equipment::new(),
            player_skills: SkillBook::new(0),
            player_quest_journal: QuestJournal::new(),
            cooldown_tracker: SkillCooldownTracker::new(),
            tc_registry: TreasureClassRegistry::new(),
            quest_registry: Vec::new(),
            skill_registry: SkillRegistry::new(),
            combat_log: Vec::new(),
            pending_loot: Vec::new(),
            game_tick: 0,
            rng: ChaCha8Rng::from_entropy(),
            exp_table: ExpTable::d2_standard(),
            ai_agents: HashMap::new(),
            status_effects: HashMap::new(),
            attack_cooldowns: HashMap::new(),
            player_gold: 0,
            monster_xp_rewards: HashMap::new(),
            monster_tc_ids: HashMap::new(),
            status_trackers: HashMap::new(),
            difficulty: Difficulty::Normal,
        })
    }

    /// Returns all monsters within `radius` of the given position.
    pub fn monsters_near(
        &self,
        x: f32,
        y: f32,
        radius: f32,
    ) -> Vec<(EntityId, f32, f32, u32)> {
        let radius_sq = radius * radius;
        let mut result = Vec::new();

        for &entity_id in self.ai_agents.keys() {
            if !self.ecs.is_alive(entity_id) {
                continue;
            }

            if let Ok(mr) = self.ecs.get_component::<MonsterRecord>(entity_id) {
                if !mr.health.is_alive() {
                    continue;
                }

                let dx = mr.position.x() - x;
                let dy = mr.position.y() - y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq <= radius_sq {
                    result.push((entity_id, mr.position.x(), mr.position.y(), mr.health.current));
                }
            }
        }

        result
    }

    /// Returns the player's current world position.
    pub fn player_position(&self) -> (f32, f32) {
        self.ecs
            .get_component::<PlayerRecord>(self.player_id)
            .map(|pr| (pr.position.x(), pr.position.y()))
            .unwrap_or((16.0, 16.0))
    }

    /// Returns the player's current and maximum health.
    pub fn player_health(&self) -> (i32, i32) {
        (
            self.player_stats.current_life,
            self.player_stats.derived.max_life,
        )
    }

    /// Returns the player's current and maximum mana.
    pub fn player_mana(&self) -> (i32, i32) {
        (
            self.player_stats.current_mana,
            self.player_stats.derived.max_mana,
        )
    }

    /// Returns an iterator over all AI agent entity IDs (live monsters).
    pub fn ai_agents_keys(&self) -> impl Iterator<Item = &EntityId> {
        self.ai_agents.keys()
    }

    /// Returns a shared reference to the [`StatusTracker`] for `entity_id`, if any.
    #[must_use]
    pub fn status_tracker(&self, entity_id: EntityId) -> Option<&StatusTracker> {
        self.status_trackers.get(&entity_id)
    }
}
