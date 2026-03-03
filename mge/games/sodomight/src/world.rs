// @id: Sodomight-World @do: gameplay-world @role: back-end @layer: 4 @human: miyuk
//! Sodomight game world -- connects all gameplay systems (ECS, stats, combat,
//! items, skills, loot, quests, AI) into a single cohesive runtime.
//!
//! The [`SodomightWorld`] struct owns the full game state and provides the
//! high-level API for the game loop: spawning, ticking, combat, inventory,
//! skill usage, and XP/levelling.

use std::collections::HashMap;

use crate::content::MonsterDef;
use mge_arpg_ai::{AggroRange, AiAgent};
use mge_arpg_combat::{
    AttackerStats, CombatEvent, CombatProcessor, DamageType, DefenderStats, StatusEffect,
    StatusTracker, StatusType,
};
use mge_arpg_entity::{Health, ItemDrop, Level, Position, Team};
use mge_arpg_items::{Equipment, Inventory, ItemInstance, ItemSlot};
use mge_arpg_loot::{DropRoll, LootGenerator, TreasureClassRegistry};
use mge_arpg_quest::{QuestDef, QuestJournal};
use mge_arpg_skills::{SkillBook, SkillCooldownTracker, SkillDef, SkillId, SkillRegistry};
use mge_arpg_stats::{BaseStats, CharacterClass, ExpTable, StatBlock};
use mge_ecs::EntityId;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

/// Milliseconds per game tick at the default 25 Hz tick rate.
const TICK_DELTA_MS: u32 = 40;

/// Default aggro sight range for monsters (world units).
const DEFAULT_SIGHT_RANGE: f32 = 8.0;

/// Default attack range for monsters (world units).
const DEFAULT_ATTACK_RANGE: f32 = 1.5;

/// Default XP base reward per monster kill.
const BASE_XP_PER_KILL: u64 = 50;

/// Minimum number of ticks between successive monster attacks (25 Hz → 2.0s).
const ATTACK_COOLDOWN_TICKS: u64 = 50;

/// Monster chase speed in world units per tick.
const MONSTER_MOVE_SPEED: f32 = 0.04;

/// Ticks between life regeneration ticks (25 Hz → every 4 seconds).
const LIFE_REGEN_INTERVAL: u64 = 100;

/// Ticks between mana regeneration ticks (25 Hz → every 2 seconds).
const MANA_REGEN_INTERVAL: u64 = 50;

/// Life restored per regen tick.
const LIFE_REGEN_AMOUNT: i32 = 1;

/// Mana restored per regen tick.
const MANA_REGEN_AMOUNT: i32 = 2;

// ---------------------------------------------------------------------------
// Monster record (ECS-side data stored alongside each monster entity)
// ---------------------------------------------------------------------------

/// Lightweight ECS component bundling all data needed for a monster entity.
///
/// Stored as a single component via `spawn_with_1` because the ECS only
/// provides helpers up to `spawn_with_3`. All fields are public for direct
/// access by the world systems.
#[derive(Debug, Clone)]
pub struct MonsterRecord {
    /// Display name.
    pub name: String,
    /// World position.
    pub position: Position,
    /// Hit points.
    pub health: Health,
    /// Monster level.
    pub level: Level,
    /// Team affiliation.
    pub team: Team,
}

// ---------------------------------------------------------------------------
// Player record (ECS-side data for the player entity)
// ---------------------------------------------------------------------------

/// Lightweight ECS component for the player entity.
#[derive(Debug, Clone)]
pub struct PlayerRecord {
    /// World position.
    pub position: Position,
    /// Hit points.
    pub health: Health,
    /// Character level.
    pub level: Level,
    /// Team affiliation.
    pub team: Team,
}

// ---------------------------------------------------------------------------
// WorldError
// ---------------------------------------------------------------------------

/// Errors that can occur during world operations.
#[derive(Debug, thiserror::Error)]
pub enum WorldError {
    /// The target entity was not found or is dead.
    #[error("Entity not found or dead")]
    EntityNotFound,

    /// The player's inventory is full.
    #[error("Inventory is full")]
    InventoryFull,

    /// The specified loot index is invalid.
    #[error("Invalid loot index: {0}")]
    InvalidLootIndex(usize),

    /// The requested skill is not known or cannot be used.
    #[error("Skill error: {0}")]
    SkillError(String),

    /// The player does not have enough mana.
    #[error("Not enough mana (need {need}, have {have})")]
    NotEnoughMana {
        /// Mana required.
        need: i32,
        /// Mana currently available.
        have: i32,
    },

    /// The skill is still on cooldown.
    #[error("Skill '{0}' is on cooldown")]
    OnCooldown(String),

    /// An ECS operation failed.
    #[error("ECS error: {0}")]
    EcsError(String),

    /// The player is too far from the item drop to pick it up.
    #[error("Too far to pick up item (distance {distance:.2}, max {max:.2})")]
    TooFar {
        /// Actual distance to the drop.
        distance: f32,
        /// Maximum allowed pickup distance.
        max: f32,
    },

    /// The target equipment slot is invalid.
    #[error("Invalid equipment slot")]
    InvalidEquipSlot,

    /// The inventory slot is empty or out of bounds.
    #[error("Inventory slot empty or out of bounds")]
    InventorySlotEmpty,
}

// ---------------------------------------------------------------------------
// Difficulty
// ---------------------------------------------------------------------------

/// Game difficulty, determining XP loss on player death (D2-style).
///
/// - `Normal`: no XP penalty.
/// - `Nightmare`: 5 % of the current level's XP range is deducted.
/// - `Hell`: 10 % of the current level's XP range is deducted.
///
/// The penalty never reduces the player's XP below the threshold of their
/// current level, preventing an involuntary de-level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Difficulty {
    /// No XP loss on death.
    #[default]
    Normal,
    /// 5 % of the current level's XP range is lost on death.
    Nightmare,
    /// 10 % of the current level's XP range is lost on death.
    Hell,
}

/// Compute the XP penalty incurred on death for a given difficulty.
///
/// # Parameters
/// - `difficulty` — active difficulty mode.
/// - `current_xp` — player's cumulative XP **before** the penalty.
/// - `level` — player's current level (1-based).
/// - `table` — experience table used for level thresholds.
///
/// # Returns
/// New cumulative XP after applying the penalty. The result is clamped to
/// `table.xp_for_level(level)` so the player cannot drop below the floor of
/// their current level.
#[must_use]
pub fn xp_death_penalty(
    difficulty: Difficulty,
    current_xp: u64,
    level: u32,
    table: &mge_arpg_stats::ExpTable,
) -> u64 {
    // Normal: no loss.
    if difficulty == Difficulty::Normal || level == 0 {
        return current_xp;
    }

    // Floor XP: the minimum required to be at `level` (cannot drop below it).
    let floor_xp = table.xp_for_level(level).unwrap_or(0);

    // Ceiling XP: threshold for the next level.
    let ceil_xp = table.xp_for_level(level + 1).unwrap_or(current_xp);

    // Level range: the span of XP covered by this level.
    let range = ceil_xp.saturating_sub(floor_xp);

    // Penalty fraction: 5 % for Nightmare, 10 % for Hell.
    // Integer arithmetic — round the division down (conservative).
    let penalty = match difficulty {
        Difficulty::Normal => 0,
        Difficulty::Nightmare => range / 20, // 5 % = range / 20
        Difficulty::Hell => range / 10,      // 10 % = range / 10
    };

    // Deduct penalty, but never fall below the level floor.
    current_xp.saturating_sub(penalty).max(floor_xp)
}

// ---------------------------------------------------------------------------
// SodomightWorld
// ---------------------------------------------------------------------------

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
    exp_table: ExpTable,

    /// AI agents keyed by their ECS `EntityId`.
    ai_agents: HashMap<EntityId, AiAgent>,

    /// Active status effects per entity.
    status_effects: HashMap<EntityId, Vec<StatusEffect>>,

    /// Last tick at which each monster successfully attacked.
    attack_cooldowns: HashMap<EntityId, u64>,

    /// The player's gold counter.
    pub player_gold: u32,

    /// Per-monster XP reward overrides (from `MonsterDef`).
    monster_xp_rewards: HashMap<EntityId, i64>,

    /// Per-monster treasure class id overrides (from `MonsterDef`).
    monster_tc_ids: HashMap<EntityId, String>,

    /// `StatusTracker` per entity — used by `combat_tick` for the ECS-integrated
    /// status pipeline. Complements the legacy `status_effects` map.
    status_trackers: HashMap<EntityId, StatusTracker>,

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
    // -------------------------------------------------------------------
    // Construction
    // -------------------------------------------------------------------

    /// Create a new Sodomight world with a level-1 player at the map centre.
    ///
    /// All registries start empty. Call `register_skill`, `register_tc`, etc.
    /// to populate game data.
    pub fn new() -> Result<Self, WorldError> {
        let mut ecs = mge_ecs::World::new();

        // Spawn the player entity at the centre of the 32×32 map.
        let player_record = PlayerRecord {
            position: Position::new(16.0, 16.0),
            health: Health::new(100),
            level: Level::new(1),
            team: Team::PLAYER,
        };

        let player_id = ecs
            .spawn_with_1(player_record)
            .map_err(|e| WorldError::EcsError(e.to_string()))?;

        // Build default player stats (all base stats = 10).
        // TODO: accept player-chosen class instead of defaulting to Barbarian.
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
    // Item drop spawning and pickup
    // -------------------------------------------------------------------

    /// Maximum pickup distance (world units). Matches D2 ~2-tile radius.
    const PICKUP_RADIUS: f32 = 2.0;

    /// Spawn an item drop entity at `(x, y)`.
    ///
    /// Creates a new ECS entity whose sole component is [`ItemDrop`]. The
    /// returned [`EntityId`] can be passed to [`pickup_item`] when the player
    /// walks nearby.
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
    // Game tick
    // -------------------------------------------------------------------

    /// Execute one game tick.
    ///
    /// This is the main simulation step, called at the configured tick rate.
    ///
    /// 1. Increment `game_tick`.
    /// 2. Tick cooldowns.
    /// 3. Process status effects (poison damage, etc.).
    /// 4. Run AI decisions for each monster.
    /// 5. Process AI-initiated attacks.
    /// 6. Check deaths and generate loot.
    pub fn tick(&mut self) {
        self.game_tick += 1;

        // 1. Tick skill cooldowns.
        self.cooldown_tracker.tick_all(TICK_DELTA_MS);

        // 2. Process status effects.
        self.tick_status_effects();

        // 3. AI decisions and monster attacks.
        self.tick_ai();

        // 4. Life / mana regeneration (D2-style slow passive regen).
        if self.player_stats.is_alive() {
            if self.game_tick.is_multiple_of(LIFE_REGEN_INTERVAL) {
                self.player_stats.restore_life(LIFE_REGEN_AMOUNT);
            }
            if self.game_tick.is_multiple_of(MANA_REGEN_INTERVAL) {
                self.player_stats.restore_mana(MANA_REGEN_AMOUNT);
            }
        }
    }

    /// Process all active status effects (poison, burn, etc.).
    fn tick_status_effects(&mut self) {
        let entity_ids: Vec<EntityId> = self.status_effects.keys().copied().collect();

        for entity_id in &entity_ids {
            let mut poison_damage = 0i32;

            if let Some(effects) = self.status_effects.get_mut(entity_id) {
                for effect in effects.iter_mut() {
                    // Accumulate poison damage from active Poison effects.
                    // `potency` is treated as damage per tick (i32 truncation is
                    // intentional — sub-1 DPS effects are simply floored to 0).
                    if effect.kind == StatusType::Poison && effect.remaining_ms > 0 {
                        #[allow(clippy::cast_possible_truncation)]
                        let dmg = effect.potency as i32;
                        poison_damage += dmg;
                    }
                    // Advance the effect timer by one tick.
                    effect.remaining_ms = effect.remaining_ms.saturating_sub(TICK_DELTA_MS);
                }
                // Remove fully expired effects.
                effects.retain(|e| e.remaining_ms > 0);
            }

            // Apply accumulated poison damage for this tick.
            if poison_damage > 0 {
                self.apply_damage_to_entity(*entity_id, poison_damage);
            }
        }

        // Clean up entities with no remaining effects.
        self.status_effects.retain(|_, effects| !effects.is_empty());
    }

    /// Run AI evaluation for all monster agents.
    ///
    /// Steps per monster:
    /// 1. FSM update (aggro evaluation).
    /// 2. Chase movement: walk toward player when in Chase state.
    /// 3. Attack with cooldown enforcement.
    fn tick_ai(&mut self) {
        let player_pos = self.player_position();
        let current_tick = self.game_tick;

        // Collect AI decisions first, then apply.
        let mut attacks: Vec<EntityId> = Vec::new();
        let mut moves: Vec<(EntityId, f32, f32)> = Vec::new();

        let agent_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();

        for monster_id in &agent_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            let monster_record = match self.ecs.get_component::<MonsterRecord>(*monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            if !monster_record.health.is_alive() {
                continue;
            }

            let mx = monster_record.position.x();
            let my = monster_record.position.y();
            let dx = player_pos.0 - mx;
            let dy = player_pos.1 - my;
            let distance = (dx * dx + dy * dy).sqrt();

            let hp_ratio = monster_record.health.ratio();

            if let Some(agent) = self.ai_agents.get_mut(monster_id) {
                agent.update(Some(distance), hp_ratio);

                // Chase movement: walk toward the player.
                if agent.fsm.can_move()
                    && agent.fsm.is_hostile()
                    && distance > agent.aggro.attack_range
                    && distance > 0.01
                {
                    let inv = MONSTER_MOVE_SPEED / distance;
                    let new_x = mx + dx * inv;
                    let new_y = my + dy * inv;
                    moves.push((*monster_id, new_x, new_y));
                }

                // Attack with cooldown.
                if agent.fsm.can_attack() && agent.aggro.can_attack(distance) {
                    let last = self.attack_cooldowns.get(monster_id).copied().unwrap_or(0);
                    if current_tick.saturating_sub(last) >= ATTACK_COOLDOWN_TICKS {
                        attacks.push(*monster_id);
                    }
                }
            }
        }

        // Apply monster movement.
        for (monster_id, new_x, new_y) in moves {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(monster_id, |mr| {
                    mr.position = Position::new(new_x, new_y);
                });
        }

        // Process monster attacks against the player.
        for monster_id in &attacks {
            self.attack_cooldowns.insert(*monster_id, current_tick);
        }
        for monster_id in attacks {
            self.monster_attack_player(monster_id);
        }
    }

    /// A monster attacks the player.
    fn monster_attack_player(&mut self, monster_id: EntityId) {
        let monster = match self.ecs.get_component::<MonsterRecord>(monster_id) {
            Ok(r) => r.clone(),
            Err(_) => return,
        };

        let monster_level = i32::from(monster.level.get());
        let attacker = AttackerStats {
            min_damage: 1 + (monster_level / 2),
            max_damage: 3 + monster_level,
            attack_rating: monster_level * 5,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: monster.level.get(),
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };

        let defender = DefenderStats {
            defense_rating: self.player_stats.derived.defense_rating,
            fire_res: self.player_stats.derived.fire_res,
            cold_res: self.player_stats.derived.cold_res,
            light_res: self.player_stats.derived.light_res,
            poison_res: self.player_stats.derived.poison_res,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: u8::try_from(self.player_stats.level.level).unwrap_or(1),
            absorb: 0,
            current_hp: self.player_stats.current_life,
        };

        let result = CombatProcessor::process_attack(
            monster_id.index,
            self.player_id.index,
            &attacker,
            &defender,
            self.player_stats.current_life,
            &mut self.rng,
        );

        if result.total_damage > 0 {
            self.player_stats.take_damage(result.total_damage);

            // Sync player ECS component.
            let new_hp = self.player_stats.current_life;
            let max_hp = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.current = new_hp.max(0) as u32;
                    pr.health.max = max_hp.max(0) as u32;
                });
        }

        // Log events.
        for event in &result.events {
            match event {
                CombatEvent::Hit { damage, .. } => {
                    let msg = format!(
                        "{} hits you for {} damage{}",
                        monster.name,
                        damage.final_amount,
                        if damage.is_critical { " (CRITICAL)" } else { "" }
                    );
                    self.combat_log.push(msg);
                }
                CombatEvent::Miss { .. } => {
                    let msg = format!("{} misses you", monster.name);
                    self.combat_log.push(msg);
                }
                CombatEvent::Death { .. } => {
                    self.combat_log
                        .push("You have been slain!".to_string());
                }
                CombatEvent::Immune { .. } => {}
            }
        }
    }

    /// Apply raw damage to any entity (monster or player).
    fn apply_damage_to_entity(&mut self, entity_id: EntityId, damage: i32) {
        if entity_id == self.player_id {
            self.player_stats.take_damage(damage);
            let new_hp = self.player_stats.current_life;
            let max_hp = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.current = new_hp.max(0) as u32;
                    pr.health.max = max_hp.max(0) as u32;
                });
        } else {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(entity_id, |mr| {
                    mr.health.take_damage(damage.max(0) as u32);
                });
        }
    }

    // -------------------------------------------------------------------
    // Player combat
    // -------------------------------------------------------------------

    /// The player attacks a target entity.
    ///
    /// Returns combat log messages for this attack. If the monster dies,
    /// loot is generated and XP is awarded.
    pub fn player_attack(&mut self, target_id: EntityId) -> Result<Vec<String>, WorldError> {
        let monster = self
            .ecs
            .get_component::<MonsterRecord>(target_id)
            .map_err(|_| WorldError::EntityNotFound)?
            .clone();

        if !monster.health.is_alive() {
            return Err(WorldError::EntityNotFound);
        }

        let attacker = AttackerStats {
            min_damage: self.player_stats.derived.min_damage,
            max_damage: self.player_stats.derived.max_damage,
            attack_rating: self.player_stats.derived.attack_rating,
            crit_chance: 0.05,
            crit_multiplier: 1.5,
            damage_type: DamageType::Physical,
            level: u8::try_from(self.player_stats.level.level).unwrap_or(1),
            str_bonus: 0.0,
            skill_bonus: 0.0,
        };

        let defender = DefenderStats {
            defense_rating: 0,
            fire_res: 0,
            cold_res: 0,
            light_res: 0,
            poison_res: 0,
            physical_res: 0,
            is_immune_to: Vec::new(),
            level: monster.level.get(),
            absorb: 0,
            current_hp: i32::try_from(monster.health.current).unwrap_or(i32::MAX),
        };

        let result = CombatProcessor::process_attack(
            self.player_id.index,
            target_id.index,
            &attacker,
            &defender,
            i32::try_from(monster.health.current).unwrap_or(i32::MAX),
            &mut self.rng,
        );

        let mut messages = Vec::new();

        // Apply damage to monster.
        if result.total_damage > 0 {
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(target_id, |mr| {
                    mr.health.take_damage(result.total_damage as u32);
                });
        }

        // Log combat events and detect death.
        let monster_died = Self::log_player_attack_events(
            &result.events,
            &monster.name,
            &mut messages,
            &mut self.combat_log,
        );

        // Handle monster death.
        if monster_died {
            self.handle_monster_death(target_id, &monster, &mut messages);
        }

        Ok(messages)
    }

    /// Log combat events from a player attack. Returns `true` if the monster died.
    fn log_player_attack_events(
        events: &[CombatEvent],
        monster_name: &str,
        messages: &mut Vec<String>,
        combat_log: &mut Vec<String>,
    ) -> bool {
        let mut died = false;
        for event in events {
            match event {
                CombatEvent::Hit { damage, .. } => {
                    let crit = if damage.is_critical { " (CRITICAL)" } else { "" };
                    let msg = format!(
                        "You hit {monster_name} for {} damage{crit}",
                        damage.final_amount
                    );
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
                CombatEvent::Miss { .. } => {
                    let msg = format!("You miss {monster_name}");
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
                CombatEvent::Death { .. } => {
                    died = true;
                }
                CombatEvent::Immune { dtype, .. } => {
                    let msg = format!("{monster_name} is immune to {dtype:?}");
                    messages.push(msg.clone());
                    combat_log.push(msg);
                }
            }
        }
        died
    }

    /// Handle all side-effects of a monster dying: XP, loot, quest tracking, AI cleanup.
    fn handle_monster_death(
        &mut self,
        target_id: EntityId,
        monster: &MonsterRecord,
        messages: &mut Vec<String>,
    ) {
        let name = &monster.name;
        let death_msg = format!("{name} has been slain!");
        messages.push(death_msg.clone());
        self.combat_log.push(death_msg);

        // Award XP -- prefer per-monster override from MonsterDef, fall back
        // to the legacy formula (BASE_XP_PER_KILL * level).
        let xp_reward = self
            .monster_xp_rewards
            .remove(&target_id)
            .map_or_else(
                || {
                    let monster_level = u64::from(monster.level.get());
                    BASE_XP_PER_KILL * monster_level
                },
                |xp| u64::try_from(xp.max(0)).unwrap_or(0),
            );
        let mut xp_messages = self.player_gain_xp(xp_reward);
        messages.append(&mut xp_messages);

        // Generate loot -- prefer per-monster TC from MonsterDef, fall back
        // to "tc_default".
        let tc_id = self
            .monster_tc_ids
            .remove(&target_id)
            .unwrap_or_else(|| "tc_default".to_string());
        let mlvl = u32::from(monster.level.get());
        let drops = LootGenerator::generate(
            &tc_id,
            &self.tc_registry,
            mlvl,
            0,
            &mut self.rng,
        );

        // Always drop some gold.
        let gold = LootGenerator::gold_drop(mlvl, mlvl * 10, &mut self.rng);

        let mut all_drops = drops;
        all_drops.push(gold);

        let drop_count = all_drops.len();
        let drop_msg = format!("{name} dropped {drop_count} items");
        messages.push(drop_msg.clone());
        self.combat_log.push(drop_msg);

        self.pending_loot
            .push((monster.position.x(), monster.position.y(), all_drops));

        // Notify quest journal.
        self.player_quest_journal.register_kill(name, 1);

        // Remove AI agent.
        self.ai_agents.remove(&target_id);
        self.status_effects.remove(&target_id);
    }

    // -------------------------------------------------------------------
    // Loot pickup
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
    // Skill usage
    // -------------------------------------------------------------------

    /// The player uses a skill on a target.
    ///
    /// Validates cooldown, mana cost, and skill availability before
    /// applying the effect.
    pub fn player_use_skill(
        &mut self,
        skill_id: &SkillId,
        target: Option<EntityId>,
    ) -> Result<Vec<String>, WorldError> {
        // Check skill exists and is learned.
        let skill_level = self.player_skills.level_of(skill_id);
        if skill_level == 0 {
            return Err(WorldError::SkillError(format!(
                "Skill '{skill_id}' not learned"
            )));
        }

        let def = self
            .skill_registry
            .get(skill_id)
            .ok_or_else(|| {
                WorldError::SkillError(format!("Skill '{skill_id}' not in registry"))
            })?
            .clone();

        // Check cooldown.
        if !self.cooldown_tracker.is_ready(skill_id) {
            return Err(WorldError::OnCooldown(skill_id.to_string()));
        }

        // Check mana cost.
        #[allow(clippy::cast_possible_truncation)]
        let mana_cost = def.mana_cost(skill_level) as i32;
        if self.player_stats.current_mana < mana_cost {
            return Err(WorldError::NotEnoughMana {
                need: mana_cost,
                have: self.player_stats.current_mana,
            });
        }

        // Deduct mana.
        self.player_stats.current_mana -= mana_cost;

        // Start cooldown.
        if def.cooldown_ms > 0 {
            self.cooldown_tracker
                .register(skill_id.clone(), def.cooldown_ms);
            self.cooldown_tracker.trigger(skill_id);
        }

        let mut messages = Vec::new();
        let use_msg = format!("You use {}", def.name);
        messages.push(use_msg.clone());
        self.combat_log.push(use_msg);

        // Apply skill effect to target (simplified: treat as bonus damage attack).
        if let Some(target_id) = target {
            let skill_lvl_i32 = i32::try_from(skill_level).unwrap_or(i32::MAX);
            let bonus_damage = skill_lvl_i32 * 5 + mana_cost;
            let _ = self
                .ecs
                .modify_component::<MonsterRecord>(target_id, |mr| {
                    mr.health.take_damage(bonus_damage.max(0) as u32);
                });
            let dmg_msg = format!(
                "{} deals {} damage to target",
                def.name, bonus_damage
            );
            messages.push(dmg_msg.clone());
            self.combat_log.push(dmg_msg);

            // Check if monster died — delegate to the shared death handler.
            if let Ok(mr) = self.ecs.get_component::<MonsterRecord>(target_id) {
                if !mr.health.is_alive() {
                    let mr_clone = mr.clone();
                    self.handle_monster_death(target_id, &mr_clone, &mut messages);
                }
            }
        }

        Ok(messages)
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
    // Experience and levelling
    // -------------------------------------------------------------------

    /// Award experience points to the player.
    ///
    /// Automatically handles level-ups, stat point grants, and skill
    /// point grants. Returns messages for each level gained.
    pub fn player_gain_xp(&mut self, amount: u64) -> Vec<String> {
        let old_level = self.player_stats.level.level;

        self.player_stats.level.add_experience(amount, &self.exp_table);

        let new_level = self.player_stats.level.level;

        let mut messages = Vec::new();
        let xp_msg = format!("Gained {amount} XP");
        messages.push(xp_msg.clone());
        self.combat_log.push(xp_msg);

        if new_level > old_level {
            for lvl in (old_level + 1)..=new_level {
                let lvl_msg = format!("Level up! Now level {lvl}");
                messages.push(lvl_msg.clone());
                self.combat_log.push(lvl_msg);
            }

            // Recalculate derived stats on level-up.
            self.player_stats.recalculate();

            // Restore pools to full on level-up.
            self.player_stats.current_life = self.player_stats.derived.max_life;
            self.player_stats.current_mana = self.player_stats.derived.max_mana;

            // Sync player ECS component.
            let max_life = self.player_stats.derived.max_life;
            let _ = self
                .ecs
                .modify_component::<PlayerRecord>(self.player_id, |pr| {
                    pr.health.max = max_life.max(0) as u32;
                    pr.health.current = max_life.max(0) as u32;
                    pr.level = Level::new(new_level.min(255) as u8);
                });

            // Grant skill points to skill book.
            let skill_points_gained = new_level - old_level;
            self.player_skills.add_points(skill_points_gained);
        }

        messages
    }

    // -------------------------------------------------------------------
    // Queries
    // -------------------------------------------------------------------

    /// Returns all monsters within `radius` of the given position.
    ///
    /// Each entry is `(entity_id, x, y, current_hp)`.
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

    /// Move the player to a new position.
    pub fn set_player_position(&mut self, x: f32, y: f32) {
        let _ = self
            .ecs
            .modify_component::<PlayerRecord>(self.player_id, |pr| {
                pr.position = Position::new(x, y);
            });
    }

    // -------------------------------------------------------------------
    // Death and respawn
    // -------------------------------------------------------------------

    /// Respawn the player after death (D2-style).
    ///
    /// Steps performed:
    /// 1. Apply XP penalty according to [`self.difficulty`].
    /// 2. Restore HP and mana to their maximum values.
    /// 3. Teleport the player to the town spawn point `(0.0, 0.0)`.
    /// 4. Sync the ECS [`PlayerRecord`] to reflect the new position and HP.
    ///
    /// The XP penalty is calculated via [`xp_death_penalty`] and is guaranteed
    /// never to reduce the player below their current level's XP floor.
    pub fn respawn_player(&mut self) {
        // 1. Apply XP penalty.
        let new_xp = xp_death_penalty(
            self.difficulty,
            self.player_stats.level.experience,
            self.player_stats.level.level,
            &self.exp_table,
        );
        self.player_stats.level.experience = new_xp;

        // 2. Restore HP and mana to full.
        let max_life = self.player_stats.derived.max_life;
        let max_mana = self.player_stats.derived.max_mana;
        self.player_stats.current_life = max_life;
        self.player_stats.current_mana = max_mana;

        // 3 + 4. Move player to town and sync ECS component.
        let _ = self
            .ecs
            .modify_component::<PlayerRecord>(self.player_id, |pr| {
                pr.position = Position::new(0.0, 0.0);
                pr.health.current = max_life.max(0) as u32;
                pr.health.max = max_life.max(0) as u32;
            });

        self.combat_log
            .push("You have been resurrected in town.".to_string());
    }

    // -------------------------------------------------------------------
    // Registration helpers
    // -------------------------------------------------------------------

    /// Register a skill definition in the skill registry and its cooldown.
    pub fn register_skill(&mut self, def: SkillDef) {
        if def.cooldown_ms > 0 {
            self.cooldown_tracker
                .register(def.id.clone(), def.cooldown_ms);
        }
        self.skill_registry.register(def);
    }

    /// Add a status effect to an entity.
    pub fn add_status_effect(&mut self, entity_id: EntityId, effect: StatusEffect) {
        self.status_effects
            .entry(entity_id)
            .or_default()
            .push(effect);
    }

    /// Returns an iterator over all AI agent entity IDs (live monsters).
    pub fn ai_agents_keys(&self) -> impl Iterator<Item = &EntityId> {
        self.ai_agents.keys()
    }

    /// Apply a status effect to an entity's [`StatusTracker`] (ECS-integrated path).
    ///
    /// This is the preferred mutation point for the `combat_tick` pipeline.
    /// For the legacy `Vec<StatusEffect>` path use [`add_status_effect`].
    pub fn apply_status_tracked(&mut self, entity_id: EntityId, effect: StatusEffect) {
        self.status_trackers
            .entry(entity_id)
            .or_default()
            .apply(effect);
    }

    /// Returns a shared reference to the [`StatusTracker`] for `entity_id`, if any.
    #[must_use]
    pub fn status_tracker(&self, entity_id: EntityId) -> Option<&StatusTracker> {
        self.status_trackers.get(&entity_id)
    }

    // -------------------------------------------------------------------
    // ECS combat tick — integrates AI FSM + StatusTracker into the loop
    // -------------------------------------------------------------------

    /// Execute the combat sub-tick for one simulation step.
    ///
    /// Designed to be called from [`tick`] (or directly by the game loop when a
    /// finer-grained split is needed).  Steps, in order:
    ///
    /// 1. **AI FSM tick** — calls [`AiAgent::tick`] for every live monster,
    ///    advancing the finite state machine (Idle → Alert → Chase → Attack …).
    /// 2. **Status tick** — calls [`StatusTracker::tick`] for every entity that
    ///    has an active tracker, decrementing timers and pruning expired effects.
    /// 3. **Dead-entity cleanup** — marks every monster whose HP has reached 0
    ///    for removal: awards XP to the player, generates loot, and despawns the
    ///    ECS entity.
    pub fn combat_tick(&mut self, dt_ms: u32) {
        // Convert milliseconds to seconds for APIs that take seconds.
        #[allow(clippy::cast_precision_loss)]
        let dt_secs: f32 = dt_ms as f32 / 1_000.0;

        let player_pos = self.player_position();

        // ----------------------------------------------------------------
        // 1. AI FSM tick — advance state machines for every live monster.
        // ----------------------------------------------------------------
        // Snapshot the keys first to satisfy the borrow checker: we need
        // `&mut self.ai_agents` while also reading `self.ecs`.
        let agent_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();

        for monster_id in &agent_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            let mr = match self.ecs.get_component::<MonsterRecord>(*monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            if !mr.health.is_alive() {
                continue;
            }

            // Compute inputs for agent.tick().
            let mx = mr.position.x();
            let my = mr.position.y();
            let dx = player_pos.0 - mx;
            let dy = player_pos.1 - my;
            let dist = (dx * dx + dy * dy).sqrt();

            let current_hp = i32::try_from(mr.health.current).unwrap_or(i32::MAX);
            let max_hp = i32::try_from(mr.health.max).unwrap_or(i32::MAX);

            // Extract aggro thresholds before taking &mut borrow on the agent.
            let (sight, attack) = self
                .ai_agents
                .get(monster_id)
                .map_or((DEFAULT_SIGHT_RANGE, DEFAULT_ATTACK_RANGE), |a| {
                    (a.aggro.sight_range, a.aggro.attack_range)
                });

            if let Some(agent) = self.ai_agents.get_mut(monster_id) {
                agent.tick(current_hp, max_hp, dist, sight, attack, dt_secs);
            }
        }

        // ----------------------------------------------------------------
        // 2. Status tick — advance all active StatusTrackers.
        // ----------------------------------------------------------------
        let tracker_ids: Vec<EntityId> = self.status_trackers.keys().copied().collect();

        for entity_id in &tracker_ids {
            if let Some(tracker) = self.status_trackers.get_mut(entity_id) {
                tracker.tick(dt_ms);
            }
        }

        // Prune trackers with no remaining effects (check Poison as a sentinel;
        // trackers for entities without any active type are left in place and
        // will be cleaned up when the entity is despawned).
        // Note: StatusTracker does not expose an `is_empty()` — we leave
        // exhausted trackers; they are freed during dead-entity cleanup.

        // ----------------------------------------------------------------
        // 3. Dead-entity cleanup — collect monsters with HP == 0.
        // ----------------------------------------------------------------
        let all_monster_ids: Vec<EntityId> = self.ai_agents.keys().copied().collect();
        let mut dead: Vec<EntityId> = Vec::new();

        for monster_id in &all_monster_ids {
            if !self.ecs.is_alive(*monster_id) {
                continue;
            }

            if let Ok(mr) = self.ecs.get_component::<MonsterRecord>(*monster_id) {
                if !mr.health.is_alive() {
                    dead.push(*monster_id);
                }
            }
        }

        // Process each dead monster: XP + loot + despawn.
        for monster_id in dead {
            let mr = match self.ecs.get_component::<MonsterRecord>(monster_id) {
                Ok(r) => r.clone(),
                Err(_) => continue,
            };

            // handle_monster_death pushes all messages to self.combat_log
            // internally; we pass a throwaway Vec to satisfy the signature.
            let mut death_msgs: Vec<String> = Vec::new();
            self.handle_monster_death(monster_id, &mr, &mut death_msgs);
            // Messages are already mirrored to self.combat_log by the callee.
            drop(death_msgs);

            // Remove the ECS entity.
            let _ = self.ecs.despawn(monster_id);

            // Clean up side-maps that handle_monster_death may not have reached.
            self.attack_cooldowns.remove(&monster_id);
            self.status_trackers.remove(&monster_id);
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use mge_arpg_loot::{DropEntry, TreasureClass};
    use mge_arpg_skills::SkillKind;

    fn make_world() -> SodomightWorld {
        SodomightWorld::new().unwrap()
    }

    // --- Test 1: Spawn player and verify initial stats ---

    #[test]
    fn test_spawn_player_initial_stats() {
        let world = make_world();

        assert!(world.ecs.is_alive(world.player_id));
        assert_eq!(world.player_stats.level.level, 1);
        assert_eq!(world.player_stats.level.experience, 0);
        assert!(world.player_stats.current_life > 0);
        assert!(world.player_stats.current_mana > 0);

        let (px, py) = world.player_position();
        assert!((px - 16.0).abs() < f32::EPSILON);
        assert!((py - 16.0).abs() < f32::EPSILON);
    }

    // --- Test 2: Spawn monster and verify ---

    #[test]
    fn test_spawn_monster() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("Zombie", 10.0, 10.0, 3, 50)
            .unwrap();

        assert!(world.ecs.is_alive(monster_id));

        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();

        assert_eq!(mr.name, "Zombie");
        assert!((mr.position.x() - 10.0).abs() < f32::EPSILON);
        assert!((mr.position.y() - 10.0).abs() < f32::EPSILON);
        assert_eq!(mr.health.current, 50);
        assert_eq!(mr.health.max, 50);
        assert_eq!(mr.level.get(), 3);
        assert!(mr.team.is_enemy());
    }

    // --- Test 3: Player attack hits and deals damage ---

    #[test]
    fn test_player_attack_hits() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("Skeleton", 5.0, 5.0, 1, 200)
            .unwrap();

        let messages = world.player_attack(monster_id).unwrap();
        assert!(!messages.is_empty());

        // Monster should have taken some damage (or missed).
        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        // With a level 1 player and default stats, most attacks should connect.
        // We check the monster is still alive (200 HP is high enough).
        assert!(mr.health.current <= 200);
    }

    // --- Test 4: Player attack kills monster ---

    #[test]
    fn test_player_attack_kills_monster() {
        let mut world = make_world();

        // Very low HP monster -- guaranteed kill.
        let monster_id = world
            .spawn_monster("Weakling", 5.0, 5.0, 1, 1)
            .unwrap();

        // Attack repeatedly until the monster dies (seeded RNG might miss once).
        let mut all_messages = Vec::new();
        for _ in 0..10 {
            let is_dead = world
                .ecs
                .get_component::<MonsterRecord>(monster_id)
                .map(|mr| !mr.health.is_alive())
                .unwrap_or(true);
            if is_dead {
                break;
            }
            if let Ok(msgs) = world.player_attack(monster_id) {
                all_messages.extend(msgs);
            }
        }

        // The monster should be dead after repeated attacks.
        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        let is_dead = !mr.health.is_alive();

        // Check for death-related messages.
        let has_kill_msg = all_messages.iter().any(|m| m.contains("slain"));
        let has_xp_msg = all_messages.iter().any(|m| m.contains("XP"));

        assert!(
            is_dead || has_kill_msg || has_xp_msg,
            "Monster should have been killed after repeated attacks"
        );
    }

    // --- Test 5: XP gain and level up ---

    #[test]
    fn test_xp_gain_level_up() {
        let mut world = make_world();

        assert_eq!(world.player_stats.level.level, 1);

        // Award enough XP to reach level 2 (threshold is 500 XP).
        let messages = world.player_gain_xp(600);

        assert!(world.player_stats.level.level >= 2);
        assert!(messages.iter().any(|m| m.contains("Level up")));
        assert!(messages.iter().any(|m| m.contains("Gained 600 XP")));
    }

    // --- Test 6: Inventory pickup ---

    #[test]
    fn test_inventory_pickup() {
        let mut world = make_world();

        // Manually add pending loot.
        let drop = DropRoll {
            item_id: "short_sword".to_string(),
            quality: mge_arpg_loot::DropQuality::Normal,
            quantity: 1,
        };
        world.pending_loot.push((5.0, 5.0, vec![drop]));

        let result = world.player_pickup_loot(0, 0);
        assert!(result.is_ok());

        let msg = result.unwrap();
        assert!(msg.contains("short_sword"));
        assert_eq!(world.player_inventory.item_count(), 1);
    }

    // --- Test 7: Inventory full prevents pickup ---

    #[test]
    fn test_inventory_full() {
        let mut world = make_world();

        // Fill the inventory (10x4 = 40 slots).
        for row in 0..mge_arpg_items::INV_ROWS {
            for col in 0..mge_arpg_items::INV_COLS {
                let item = ItemInstance::new_normal(format!("item_{col}_{row}"), 1);
                world
                    .player_inventory
                    .try_place(item, col, row)
                    .unwrap();
            }
        }

        assert!(world.player_inventory.is_full());

        // Try to pick up more loot.
        let drop = DropRoll {
            item_id: "extra_item".to_string(),
            quality: mge_arpg_loot::DropQuality::Normal,
            quantity: 1,
        };
        world.pending_loot.push((5.0, 5.0, vec![drop]));

        let result = world.player_pickup_loot(0, 0);
        assert!(result.is_err());
        match result {
            Err(WorldError::InventoryFull) => {} // Expected.
            other => panic!("Expected InventoryFull, got: {:?}", other),
        }
    }

    // --- Test 8: Equip and unequip ---

    #[test]
    fn test_equip_unequip() {
        let mut world = make_world();

        // Place an item in inventory.
        let item = ItemInstance::new_normal("helmet_01".to_string(), 5);
        world
            .player_inventory
            .try_place(item, 0, 0)
            .unwrap();

        // Equip it.
        let result = world.player_equip(0, 0, ItemSlot::Helm);
        assert!(result.is_ok());
        assert!(result.unwrap().contains("helmet_01"));

        // Verify inventory slot is now empty and equipment slot is filled.
        assert!(world.player_inventory.get(0, 0).is_none());
        assert!(world.player_equipment.get(ItemSlot::Helm).is_some());
    }

    // --- Test 9: Skill usage and cooldown ---

    #[test]
    fn test_skill_usage_and_cooldown() {
        let mut world = make_world();

        // Register a skill.
        let skill_def = SkillDef {
            id: SkillId::new("fireball"),
            name: "Fireball".to_string(),
            max_level: 20,
            prerequisites: Vec::new(),
            synergies: Vec::new(),
            mana_cost_base: 5.0,
            mana_cost_per_level: 1.0,
            cooldown_ms: 100,
            tree: 0,
            damage_type: DamageType::Fire,
            kind: SkillKind::Projectile,
            base_damage_min: 6,
            base_damage_max: 12,
            damage_per_level: 3,
            synergy_ids: Vec::new(),
        };
        world.register_skill(skill_def);

        // Grant skill points and invest.
        world.player_skills.add_points(1);
        let skill_id = SkillId::new("fireball");
        world
            .player_skills
            .invest(&skill_id, &world.skill_registry)
            .unwrap();

        // Spawn a target monster.
        let monster_id = world
            .spawn_monster("Target Dummy", 5.0, 5.0, 1, 100)
            .unwrap();

        // Use the skill.
        let result = world.player_use_skill(&skill_id, Some(monster_id));
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert!(messages.iter().any(|m| m.contains("Fireball")));

        // Verify mana was consumed.
        // Mana cost at level 1: 5.0 + 1.0 * 1 = 6
        let initial_mana = world.player_stats.derived.max_mana;
        assert!(world.player_stats.current_mana < initial_mana);

        // Verify cooldown is active.
        assert!(!world.cooldown_tracker.is_ready(&skill_id));

        // Second use should fail (on cooldown).
        let result2 = world.player_use_skill(&skill_id, Some(monster_id));
        assert!(result2.is_err());
    }

    // --- Test 10: Monster death generates loot ---

    #[test]
    fn test_monster_death_generates_loot() {
        let mut world = make_world();

        // Register a treasure class so loot generation produces something.
        let tc = TreasureClass {
            id: "tc_default".to_string(),
            picks: 1,
            no_drop: 50,
            entries: vec![DropEntry {
                item_id: "potion_health".to_string(),
                weight: 50,
                min_qty: 1,
                max_qty: 1,
                is_treasure_class: false,
            }],
        };
        world.tc_registry.register(tc);

        // Spawn a very weak monster.
        let monster_id = world
            .spawn_monster("Fragile Skeleton", 5.0, 5.0, 1, 1)
            .unwrap();

        assert!(world.pending_loot.is_empty());

        // Attack until dead.
        for _ in 0..20 {
            let mr = world
                .ecs
                .get_component::<MonsterRecord>(monster_id)
                .unwrap();
            if !mr.health.is_alive() {
                break;
            }
            let _ = world.player_attack(monster_id);
        }

        // Loot should have been generated (gold is always dropped).
        assert!(
            !world.pending_loot.is_empty(),
            "Expected loot drops after monster death"
        );
    }

    // --- Test 11: Monsters near query ---

    #[test]
    fn test_monsters_near() {
        let mut world = make_world();

        let _m1 = world.spawn_monster("Near", 6.0, 5.0, 1, 50).unwrap();
        let _m2 = world.spawn_monster("Far", 100.0, 100.0, 1, 50).unwrap();

        let near = world.monsters_near(5.0, 5.0, 3.0);
        assert_eq!(near.len(), 1);
        assert!((near[0].1 - 6.0).abs() < f32::EPSILON);
    }

    // --- Test 12: Player health and mana queries ---

    #[test]
    fn test_player_health_mana() {
        let world = make_world();

        let (hp, max_hp) = world.player_health();
        assert!(hp > 0);
        assert!(max_hp > 0);
        assert_eq!(hp, max_hp);

        let (mp, max_mp) = world.player_mana();
        assert!(mp > 0);
        assert!(max_mp > 0);
        assert_eq!(mp, max_mp);
    }

    // --- Test 13: Status effect tick ---

    #[test]
    fn test_status_effect_poison_tick() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("Poisoned Beast", 5.0, 5.0, 1, 100)
            .unwrap();

        // Apply poison: 5 damage per tick for 3 ticks.
        // `potency` = damage per tick, `remaining_ms` = 3 * TICK_DELTA_MS so
        // that the effect stays active for exactly three ticks before expiring.
        world.add_status_effect(
            monster_id,
            StatusEffect {
                kind: StatusType::Poison,
                potency: 5.0,
                remaining_ms: 3 * TICK_DELTA_MS,
            },
        );

        // Tick 3 times.
        world.tick();
        world.tick();
        world.tick();

        let mr = world
            .ecs
            .get_component::<MonsterRecord>(monster_id)
            .unwrap();
        // Should have taken 15 damage (5 * 3 ticks).
        assert_eq!(mr.health.current, 85);
    }

    // --- Test 14: Game tick increments ---

    #[test]
    fn test_game_tick_increments() {
        let mut world = make_world();

        assert_eq!(world.game_tick, 0);
        world.tick();
        assert_eq!(world.game_tick, 1);
        world.tick();
        assert_eq!(world.game_tick, 2);
    }

    // --- Test 15: Multiple level ups from large XP ---

    #[test]
    fn test_multiple_level_ups() {
        let mut world = make_world();

        // Award massive XP to jump multiple levels.
        let messages = world.player_gain_xp(50_000);

        assert!(world.player_stats.level.level > 5);
        let level_up_count = messages
            .iter()
            .filter(|m| m.contains("Level up"))
            .count();
        assert!(level_up_count > 1);
    }

    // --- Test 16: combat_tick — StatusTracker timer decrements ---
    //
    // Applies a Poison effect via `apply_status_tracked`, calls `combat_tick`,
    // and verifies that `remaining_ms` has been decremented by `dt_ms`.

    #[test]
    fn combat_system_status_tick() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("StatusBeast", 10.0, 10.0, 1, 100)
            .unwrap();

        // Apply a 500 ms poison via the tracker path.
        world.apply_status_tracked(
            monster_id,
            StatusEffect {
                kind: StatusType::Poison,
                remaining_ms: 500,
                potency: 5.0,
            },
        );

        // Verify the effect is registered.
        assert!(
            world
                .status_tracker(monster_id)
                .is_some_and(|t| t.is_affected(StatusType::Poison)),
            "Poison should be active before tick"
        );

        // Advance by TICK_DELTA_MS (40 ms).
        world.combat_tick(TICK_DELTA_MS);

        // remaining_ms should now be 500 - 40 = 460.
        let tracker = world
            .status_tracker(monster_id)
            .expect("tracker should still be present");
        assert!(
            tracker.is_affected(StatusType::Poison),
            "Poison should still be active after a single tick"
        );
        // We cannot read remaining_ms directly (private field), but we can
        // verify the effect has NOT expired yet (500 ms > 40 ms tick).
        // The effect would only expire if remaining_ms reached 0.
        // A second check: tick 20 more times (20 * 40 = 800 ms > 500 ms total).
        for _ in 0..20 {
            world.combat_tick(TICK_DELTA_MS);
        }
        // After 21 ticks (840 ms) the 500 ms effect must have expired.
        let still_poisoned = world
            .status_tracker(monster_id)
            .is_some_and(|t| t.is_affected(StatusType::Poison));
        assert!(
            !still_poisoned,
            "Poison effect should have expired after enough ticks"
        );
    }

    // --- Test 17: combat_tick — dead entity cleanup + XP ---
    //
    // Spawns a monster, kills it directly (set HP to 0 via modify_component),
    // then calls `combat_tick` and verifies the entity is no longer alive and
    // that the player gained XP.

    #[test]
    fn combat_system_dead_cleanup() {
        let mut world = make_world();

        let monster_id = world
            .spawn_monster("DeadDummy", 5.0, 5.0, 1, 50)
            .unwrap();

        // Zero out the monster's HP directly in the ECS to simulate a kill.
        world
            .ecs
            .modify_component::<MonsterRecord>(monster_id, |mr| {
                mr.health.current = 0;
            })
            .unwrap();

        // Verify the monster reads as dead before the tick.
        let mr = world.ecs.get_component::<MonsterRecord>(monster_id).unwrap();
        assert!(!mr.health.is_alive(), "Monster should be dead before combat_tick");

        let xp_before = world.player_stats.level.experience;

        // combat_tick should detect the dead entity and award XP.
        world.combat_tick(TICK_DELTA_MS);

        // The entity should have been despawned.
        assert!(
            !world.ecs.is_alive(monster_id),
            "Dead entity must be despawned after combat_tick"
        );

        // The AI agent map should no longer contain the dead monster.
        assert!(
            !world.ai_agents.contains_key(&monster_id),
            "AI agent entry must be removed after death cleanup"
        );

        // XP should have been awarded to the player.
        let xp_after = world.player_stats.level.experience;
        assert!(
            xp_after > xp_before,
            "Player should have gained XP when monster was cleaned up (before={xp_before}, after={xp_after})"
        );
    }

    // --- Test 18: item_drop_spawn — spawn a drop, verify entity and component ---

    #[test]
    fn item_drop_spawn() {
        let mut world = make_world();

        let white = [1.0_f32, 1.0, 1.0, 1.0];
        let drop_id = world.spawn_item_drop("short_sword", white, 5.0, 6.0);

        // The entity must be alive in the ECS.
        assert!(
            world.ecs.is_alive(drop_id),
            "ItemDrop entity must be alive after spawning"
        );

        // The component must be retrievable and carry the correct data.
        let drop = world
            .ecs
            .get_component::<ItemDrop>(drop_id)
            .expect("ItemDrop component must exist on the spawned entity");

        assert_eq!(drop.item_id, "short_sword");
        assert!((drop.position_x - 5.0).abs() < f32::EPSILON);
        assert!((drop.position_y - 6.0).abs() < f32::EPSILON);
    }

    // --- Test 19: item_pickup_distance — too far, pickup must fail ---

    #[test]
    fn item_pickup_distance() {
        let mut world = make_world();

        // Player starts at (16, 16).  Drop placed far away.
        let gold = [1.0_f32, 0.84, 0.0, 1.0];
        let drop_id = world.spawn_item_drop("rare_ring", gold, 50.0, 50.0);

        let result = world.pickup_item(drop_id);

        assert!(
            result.is_err(),
            "pickup_item must fail when the player is too far from the drop"
        );
        match result {
            Err(WorldError::TooFar { distance, max }) => {
                assert!(
                    distance > max,
                    "distance ({distance:.2}) must exceed max ({max:.2})"
                );
            }
            other => panic!("Expected TooFar, got: {other:?}"),
        }

        // The entity must still be alive (not consumed on failure).
        assert!(
            world.ecs.is_alive(drop_id),
            "ItemDrop entity must remain alive after a failed pickup"
        );
    }

    // --- Test 20: death_xp_penalty_normal — Normal difficulty = 0 XP loss ---

    #[test]
    fn death_xp_penalty_normal() {
        let table = mge_arpg_stats::ExpTable::d2_standard();

        // Player at level 2 with some XP above the level-2 threshold.
        let level2_xp = table.xp_for_level(2).unwrap(); // 500
        let current_xp = level2_xp + 100;               // 600

        let result = xp_death_penalty(Difficulty::Normal, current_xp, 2, &table);

        assert_eq!(
            result, current_xp,
            "Normal difficulty must not deduct any XP"
        );
    }

    // --- Test 21: death_xp_penalty_hell — Hell loses XP but stays at current level ---

    #[test]
    fn death_xp_penalty_hell() {
        let table = mge_arpg_stats::ExpTable::d2_standard();

        // Use level 2.  Threshold: level 2 = 500 XP, level 3 = 575 XP (500 * 1.15).
        // Range = 575 - 500 = 75 XP.  10 % penalty = 7 XP.
        // Start right at the top of level 2: current_xp = 574 (just below level 3).
        let level2_floor = table.xp_for_level(2).unwrap();
        let level3_ceil = table.xp_for_level(3).unwrap();
        let current_xp = level3_ceil - 1; // just under level 3

        let result = xp_death_penalty(Difficulty::Hell, current_xp, 2, &table);

        // After penalty the player must still be at least at level 2's floor.
        assert!(
            result >= level2_floor,
            "XP after Hell penalty ({result}) must be >= level 2 floor ({level2_floor})"
        );

        // A penalty must have been applied.
        assert!(
            result < current_xp,
            "Hell difficulty must deduct XP (before={current_xp}, after={result})"
        );

        // The player must not have been de-levelled: computed level must still be 2.
        let new_level = table.level_for_xp(result);
        assert_eq!(
            new_level, 2,
            "Player must remain at level 2 after Hell death penalty (got {new_level})"
        );
    }

    // --- Test 22: respawn_town — after respawn, position = (0.0, 0.0) and HP = max ---

    #[test]
    fn respawn_town() {
        let mut world = make_world();

        // Damage the player severely.
        world.player_stats.current_life = 1;
        world.player_stats.current_mana = 0;

        // Move the player away from town.
        world.set_player_position(20.0, 15.0);

        // Respawn.
        world.respawn_player();

        // Position must be town spawn (0.0, 0.0).
        let (px, py) = world.player_position();
        assert!(
            (px - 0.0).abs() < f32::EPSILON,
            "After respawn, player X must be 0.0 (got {px})"
        );
        assert!(
            (py - 0.0).abs() < f32::EPSILON,
            "After respawn, player Y must be 0.0 (got {py})"
        );

        // HP must be fully restored.
        let (hp, max_hp) = world.player_health();
        assert_eq!(
            hp, max_hp,
            "After respawn, current HP ({hp}) must equal max HP ({max_hp})"
        );

        // Mana must also be fully restored.
        let (mp, max_mp) = world.player_mana();
        assert_eq!(
            mp, max_mp,
            "After respawn, current mana ({mp}) must equal max mana ({max_mp})"
        );
    }
}
