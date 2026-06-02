//! A minimal, deterministic ECS. Entity-keyed component stores plus shared
//! resources live in one `World`; systems are plain `fn(&mut World)` run by the
//! `App` scheduler. `BTreeMap` storage keeps iteration order stable so the
//! seeded RNG stays reproducible.

use std::collections::{BTreeMap, BTreeSet};

use crate::components::cc::{Interruptible, Knockdown, Pushback};
use crate::components::equipment::{EquipmentSlots, Inventory};
use crate::components::item::GroundItem;
use crate::components::mercenary::{MercInventory, MercState, MercenaryTag};
use crate::components::mob::{AggroRange, BossState, MobTag, MobType};
use crate::components::npc::{Merchant, NpcInfo, NpcKind};
use crate::components::projectile::Projectile;
use crate::components::summon::SummonTag;
use crate::components::player::{ClassType, PlayerTag, Progression, Stats};
use crate::components::quest::QuestLog;
use crate::components::skill::{
    ActiveAura, ArrowRainField, AuraBonus, BlizzardField, Buff, ChargingShot, ComboLock, Converted,
    DefDebuff, Dot, ElementResists, FlameWall, FrostArmor, HealField, HunterInstinctBuff,
    LightningShield, MeteorField, OrderBonuses, OrderBuff, Pardoned, Resist, Resurrecting,
    SkillSlot, SkillTree, Slow, Trap, Unstoppable,
};
use crate::components::world::{
    Chest, DiscoveredWaypoints, MapData, ObstacleKind, Waypoint, WaypointTeleportCast, ZoneId,
};
use crate::components::{Combat, Faction, Health, Mana, MoveTarget, Position2D};
use crate::data::GameData;
use crate::plugins::camera::Camera;
use crate::plugins::network::NetworkState;
use crate::plugins::player::PlayerInput;
use crate::render::animation::AnimationController;
use crate::render::figures::Direction;
use crate::rng::Rng;

/// Opaque entity handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Entity(pub u64);

/// A scheduled loot drop produced when a mob dies.
#[derive(Debug, Clone, Copy)]
pub struct DeathDrop {
    pub pos: Position2D,
    pub mob_def_id: u32,
    /// Level of the dead mob — drives item level (and thus level_req) of drops.
    pub mob_level: u32,
    /// Elite kill: bumps the drop count and biases rarity upward.
    pub elite: bool,
}

/// Visual category for an entity, used to colour death/feedback VFX without
/// the sim depending on the render palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualKind {
    Melee,
    Ranged,
    Caster,
    Boss,
    Player,
}

/// Render-facing feedback emitted by sim systems (combat, death, loot, skills)
/// and consumed by the renderer each frame to drive animations, particles,
/// floating numbers, and corpses. Headless mode clears these per tick.
#[derive(Debug, Clone)]
pub enum RenderEvent {
    /// An attack resolved against `target` at `pos`.
    Damage {
        attacker: Entity,
        target: Entity,
        pos: Position2D,
        amount: f32,
        crit: bool,
        dodged: bool,
    },
    /// An entity died at `pos`.
    Death { pos: Position2D, kind: VisualKind },
    /// Loot dropped at `pos` with the given rarity colour name.
    Loot { pos: Position2D, rarity: &'static str },
    /// A skill was cast (drives skill VFX in Phase 4).
    Skill { caster: Entity, from: Position2D, to: Position2D, fx: SkillFx },
    /// A cast/action on `target` was interrupted (white flash + "Interrompu!").
    Interrupt { target: Entity, pos: Position2D },
    /// An incoming attack was blocked by a shield / dual-wield parry at `pos`.
    Block { pos: Position2D },
}

/// Visual flavour for a cast skill.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkillFx {
    Slash,
    Whirlwind,
    Buff,
    Charge,
    Fire,
    Lightning,
    Frost,
    Blink,
    Arrow,
    Trap,
    Dodge,
    ArrowRain,
    Summon,
    /// Radiant beam / smite (Clerc light magic).
    Holy,
    /// Restorative green-gold burst (heal field / sacred circle).
    Heal,
    /// Falling fire from above (meteor shower).
    Meteor,
    /// Venomous green spray (poison arrow / jaw trap).
    Poison,
}

/// The whole game state: components + resources.
pub struct World {
    next_id: u64,

    // ── Component stores ────────────────────────────────────────────────
    pub positions: BTreeMap<Entity, Position2D>,
    pub factions: BTreeMap<Entity, Faction>,
    pub health: BTreeMap<Entity, Health>,
    pub mana: BTreeMap<Entity, Mana>,
    pub move_targets: BTreeMap<Entity, MoveTarget>,
    pub combat: BTreeMap<Entity, Combat>,
    pub player_tags: BTreeMap<Entity, PlayerTag>,
    pub classes: BTreeMap<Entity, ClassType>,
    pub stats: BTreeMap<Entity, Stats>,
    pub progression: BTreeMap<Entity, Progression>,
    pub equipment: BTreeMap<Entity, EquipmentSlots>,
    pub inventory: BTreeMap<Entity, Inventory>,
    pub skills: BTreeMap<Entity, SkillSlot>,
    pub skill_trees: BTreeMap<Entity, SkillTree>,
    /// Per-player set of unlocked branches identified by `(originating class,
    /// branch index)`. Native branches are NOT stored — they are always
    /// implicitly unlocked. Buying a foreign branch costs one skill point.
    pub unlocked_branches: BTreeMap<Entity, Vec<(ClassType, u32)>>,
    pub mob_tags: BTreeMap<Entity, MobTag>,
    pub mob_types: BTreeMap<Entity, MobType>,
    /// Links a live mob entity to its `SpawnPoint::stable_id()`, so the death
    /// system can record it into `killed_mobs` and the next visit to the zone
    /// can skip respawning it.
    pub spawn_ids: BTreeMap<Entity, u64>,
    pub aggro: BTreeMap<Entity, AggroRange>,
    pub ground_items: BTreeMap<Entity, GroundItem>,
    /// Client-side procedural animation state (render only).
    pub animations: BTreeMap<Entity, AnimationController>,
    /// Facing direction, updated by movement (travel vector) and combat (toward
    /// the target). Read by the renderer to pick the directional pose.
    pub directions: BTreeMap<Entity, Direction>,
    /// Active attack buffs (e.g. War Cry).
    pub buffs: BTreeMap<Entity, Buff>,
    /// Active movement slows (e.g. Frost Nova).
    pub slows: BTreeMap<Entity, Slow>,
    /// Active invulnerability windows (seconds remaining; e.g. Dodge).
    pub invuln: BTreeMap<Entity, f32>,
    /// Active stuns (seconds remaining): the entity cannot act while > 0.
    pub stuns: BTreeMap<Entity, f32>,
    /// Active renversements (knockdowns): the entity is grounded and helpless.
    pub knockdowns: BTreeMap<Entity, Knockdown>,
    /// Active reculs (pushbacks): the entity is shoved and cannot act.
    pub pushbacks: BTreeMap<Entity, Pushback>,
    /// In-progress interruptible casts, keyed by the casting entity.
    pub interruptibles: BTreeMap<Entity, Interruptible>,
    /// Uninterruptible-animation windows (e.g. the Soldat melee techniques):
    /// the entity ignores stuns/interruption while present.
    pub unstoppable: BTreeMap<Entity, Unstoppable>,
    /// Active locked melee combos ("Estocs"): a fixed run of guaranteed strikes
    /// that also locks the owner's input until it finishes.
    pub combo_locks: BTreeMap<Entity, ComboLock>,
    /// Active poison damage-over-time effects.
    pub dots: BTreeMap<Entity, Dot>,
    /// Active defence-reduction debuffs.
    pub def_debuffs: BTreeMap<Entity, DefDebuff>,
    /// Active damage-reduction blessings.
    pub resist: BTreeMap<Entity, Resist>,
    /// Active battle orders (Soldat "Ordres"), each entity holding any number of
    /// distinct concurrent orders.
    pub order_buffs: BTreeMap<Entity, Vec<OrderBuff>>,
    /// Active healing fields, keyed by their owning caster.
    pub heal_fields: BTreeMap<Entity, HealField>,
    /// Deployed traps.
    pub traps: BTreeMap<Entity, Trap>,
    /// Per-element damage resistances (Mage elemental branches read these).
    pub elem_resists: BTreeMap<Entity, ElementResists>,
    /// Active ground fire rectangles (Mage "Mur de flammes"), keyed by a spawned
    /// zone entity.
    pub flame_walls: BTreeMap<Entity, FlameWall>,
    /// Active chilling fields (Mage "Blizzard"), keyed by a spawned zone entity.
    pub blizzards: BTreeMap<Entity, BlizzardField>,
    /// Active meteor barrages (Mage "Pluie de météores"), keyed by a spawned
    /// zone entity.
    pub meteor_fields: BTreeMap<Entity, MeteorField>,
    /// Active frost-armour self-buffs (Mage "Armure de givre"), keyed by owner.
    pub frost_armor: BTreeMap<Entity, FrostArmor>,
    /// Active lightning-shield self-buffs (Mage "Bouclier de foudre"), keyed by
    /// owner.
    pub lightning_shield: BTreeMap<Entity, LightningShield>,
    /// In-progress charged archer shots, keyed by their owner.
    pub charging_shots: BTreeMap<Entity, ChargingShot>,
    /// Active "Instinct du chasseur" buffs, keyed by owner.
    pub hunter_instinct: BTreeMap<Entity, HunterInstinctBuff>,
    /// Active arrow-rain zones (Archer "Pluie de flèches"), keyed by a spawned
    /// zone entity.
    pub arrow_rains: BTreeMap<Entity, ArrowRainField>,
    /// The Clerc's currently toggled support aura (at most one per caster).
    pub active_auras: BTreeMap<Entity, ActiveAura>,
    /// Per-recipient aggregated aura bonuses, refreshed by aura ticks and read
    /// live by combat/movement.
    pub aura_bonuses: BTreeMap<Entity, AuraBonus>,
    /// Mana reserved by the currently active aura (deducted from `mana.max`).
    pub aura_reserved_mana: BTreeMap<Entity, f32>,
    /// Mobs the Clerc has "Converted": they fight under [`Faction::Player`]
    /// until the timer expires, then revert to their original faction.
    pub converted: BTreeMap<Entity, Converted>,
    /// Mobs the Clerc has "Pardoned": every attacker hitting them heals.
    pub pardoned: BTreeMap<Entity, Pardoned>,
    /// In-progress Résurrection casts, keyed by the casting Clerc.
    pub resurrecting: BTreeMap<Entity, Resurrecting>,
    /// NPC kinds (the merchant).
    pub npcs: BTreeMap<Entity, NpcKind>,
    /// Per-NPC display name + dialogue line.
    pub npc_info: BTreeMap<Entity, NpcInfo>,
    /// Merchant stocks, keyed by the NPC entity.
    pub merchants: BTreeMap<Entity, Merchant>,
    /// Per-boss special-attack state (slam/charge cooldowns + enrage latch).
    pub boss_state: BTreeMap<Entity, BossState>,
    /// Summoned allies (player invocations).
    pub summon_tags: BTreeMap<Entity, SummonTag>,
    /// Hired mercenary companions.
    pub mercenary_tags: BTreeMap<Entity, MercenaryTag>,
    /// Mercenary four-slot equipment.
    pub merc_inventory: BTreeMap<Entity, MercInventory>,
    /// Mercenary progression + ability/respawn timers.
    pub merc_states: BTreeMap<Entity, MercState>,
    /// In-flight ranged projectiles (homing). Not ECS entities: they have no
    /// id and live only in this flat vector for the duration of their flight.
    pub projectiles: Vec<Projectile>,
    /// Procedurally-placed chests, keyed by entity so the standard despawn path
    /// applies. Each chest carries its own position too (the entity also has a
    /// [`Position2D`] so the renderer can pick it up the usual way).
    pub chests: BTreeMap<Entity, Chest>,
    /// Procedurally-placed non-walkable scenery for the current map. Mirrored
    /// from [`MapData::obstacles`] every time the zone is entered.
    pub obstacles: Vec<(f32, f32, ObstacleKind)>,
    /// Entities flagged as "elite" by the procedural pack roller. The death
    /// system reads this to grant bonus XP and crank up loot rarity.
    pub elite_mobs: BTreeSet<Entity>,

    // ── Resources ───────────────────────────────────────────────────────
    pub map: MapData,
    pub data: GameData,
    pub rng: Rng,
    pub camera: Camera,
    pub input: PlayerInput,
    pub net: NetworkState,
    pub log: Vec<String>,
    pub tick: u64,
    pub dt: f32,
    pub player_entity: Option<Entity>,
    pub loot_queue: Vec<DeathDrop>,
    /// Render-facing feedback drained by the renderer each frame.
    pub render_events: Vec<RenderEvent>,
    /// Class the player will spawn as (set before `run_startup`).
    pub chosen_class: ClassType,
    /// The zone the player is currently in.
    pub zone: ZoneId,
    /// A requested zone transition (target zone + arrival tile), applied by the
    /// zone system next tick.
    pub pending_zone: Option<(ZoneId, (i32, i32))>,
    /// Per-zone list of `SpawnPoint::stable_id()`s the player has already
    /// killed. Slain mobs stay dead across zone transitions for the lifetime
    /// of the current run (cleared on new-game/load).
    pub killed_mobs: BTreeMap<String, Vec<u64>>,
    /// Active quests.
    pub quests: QuestLog,
    /// All waypoint beacons across every (non-boss) zone.
    pub waypoints: Vec<Waypoint>,
    /// Per-player set of waypoint ids the player has activated. Each player
    /// has their own list so multiplayer keeps progress separate.
    pub discovered_wps: BTreeMap<Entity, DiscoveredWaypoints>,
    /// In-progress waypoint teleport casts (1s, interruptible by damage).
    pub wp_teleport_casts: BTreeMap<Entity, WaypointTeleportCast>,
    /// Floating-text feedback drained by the renderer ("Waypoint découvert !").
    pub floating_texts: Vec<FloatingText>,
    /// `(player, wp_id, wp_name)` triplets recorded by the waypoint system and
    /// drained by the network layer to broadcast `WaypointDiscovered`.
    pub waypoint_events: Vec<(Entity, u32, String)>,
    /// Run-wide seed used by the procedural map generator. Defaults to the
    /// World's RNG seed at construction time, and is overridden by the save
    /// system to a hash of `SaveData::player_id` so a given run reproduces the
    /// same procedural maps after a reload.
    pub game_seed: u64,
    /// Current difficulty (0 = Normal). Reshuffles procedural maps when bumped.
    pub difficulty: u32,
    /// The most recent enemy the player fought (see [`LastEnemy`]); drives the
    /// "danger" readout on the character sheet. UI-only, reset on new game.
    pub last_enemy: Option<LastEnemy>,
    /// Dual-wield swing parity, keyed by attacker: `true` means the next melee
    /// swing comes from the off-hand weapon. Flipped after each swing so a
    /// two-weapon fighter alternates hands (and their individual damage).
    pub dual_hand: BTreeMap<Entity, bool>,
    /// Town-portal return point `(zone, tile)`: set when the player opens a
    /// portal to town, consumed when they portal back from the village.
    pub tp_return: Option<(ZoneId, (i32, i32))>,
}

/// A snapshot of the most recent enemy the player traded blows with, kept so
/// the character sheet can show the danger it poses even after it dies (and is
/// despawned). Pure UI/feedback state — never serialized.
#[derive(Debug, Clone)]
pub struct LastEnemy {
    pub name: String,
    pub level: u32,
    pub damage_min: f32,
    pub damage_max: f32,
    /// The enemy's Attack Rating — drives the sheet's "chance to be hit" and the
    /// "damage taken per hit" readouts against this specific foe.
    pub attack_rating: f32,
}

/// A short-lived floating piece of text rendered above a world position.
#[derive(Debug, Clone)]
pub struct FloatingText {
    pub text: String,
    pub pos: Position2D,
    pub color: [f32; 4],
    pub ttl: f32,
}

impl World {
    pub fn new(map: MapData, data: GameData, seed: u64) -> Self {
        let zone = map.zone;
        Self {
            next_id: 1,
            positions: BTreeMap::new(),
            factions: BTreeMap::new(),
            health: BTreeMap::new(),
            mana: BTreeMap::new(),
            move_targets: BTreeMap::new(),
            combat: BTreeMap::new(),
            player_tags: BTreeMap::new(),
            classes: BTreeMap::new(),
            stats: BTreeMap::new(),
            progression: BTreeMap::new(),
            equipment: BTreeMap::new(),
            inventory: BTreeMap::new(),
            skills: BTreeMap::new(),
            skill_trees: BTreeMap::new(),
            unlocked_branches: BTreeMap::new(),
            mob_tags: BTreeMap::new(),
            mob_types: BTreeMap::new(),
            spawn_ids: BTreeMap::new(),
            aggro: BTreeMap::new(),
            ground_items: BTreeMap::new(),
            animations: BTreeMap::new(),
            directions: BTreeMap::new(),
            buffs: BTreeMap::new(),
            slows: BTreeMap::new(),
            invuln: BTreeMap::new(),
            stuns: BTreeMap::new(),
            knockdowns: BTreeMap::new(),
            pushbacks: BTreeMap::new(),
            interruptibles: BTreeMap::new(),
            unstoppable: BTreeMap::new(),
            combo_locks: BTreeMap::new(),
            dots: BTreeMap::new(),
            def_debuffs: BTreeMap::new(),
            resist: BTreeMap::new(),
            order_buffs: BTreeMap::new(),
            heal_fields: BTreeMap::new(),
            traps: BTreeMap::new(),
            elem_resists: BTreeMap::new(),
            flame_walls: BTreeMap::new(),
            blizzards: BTreeMap::new(),
            meteor_fields: BTreeMap::new(),
            frost_armor: BTreeMap::new(),
            lightning_shield: BTreeMap::new(),
            charging_shots: BTreeMap::new(),
            hunter_instinct: BTreeMap::new(),
            arrow_rains: BTreeMap::new(),
            active_auras: BTreeMap::new(),
            aura_bonuses: BTreeMap::new(),
            aura_reserved_mana: BTreeMap::new(),
            converted: BTreeMap::new(),
            pardoned: BTreeMap::new(),
            resurrecting: BTreeMap::new(),
            npcs: BTreeMap::new(),
            npc_info: BTreeMap::new(),
            merchants: BTreeMap::new(),
            boss_state: BTreeMap::new(),
            summon_tags: BTreeMap::new(),
            mercenary_tags: BTreeMap::new(),
            merc_inventory: BTreeMap::new(),
            merc_states: BTreeMap::new(),
            projectiles: Vec::new(),
            chests: BTreeMap::new(),
            obstacles: Vec::new(),
            elite_mobs: BTreeSet::new(),
            map,
            data,
            rng: Rng::new(seed),
            camera: Camera::default(),
            input: PlayerInput::default(),
            net: NetworkState::default(),
            log: Vec::new(),
            tick: 0,
            dt: 0.0,
            player_entity: None,
            loot_queue: Vec::new(),
            render_events: Vec::new(),
            chosen_class: ClassType::Soldat,
            zone,
            pending_zone: None,
            killed_mobs: BTreeMap::new(),
            quests: QuestLog::act1(),
            waypoints: crate::data::maps::all_waypoints(),
            discovered_wps: BTreeMap::new(),
            wp_teleport_casts: BTreeMap::new(),
            floating_texts: Vec::new(),
            waypoint_events: Vec::new(),
            game_seed: seed,
            difficulty: 0,
            last_enemy: None,
            dual_hand: BTreeMap::new(),
            tp_return: None,
        }
    }

    /// Take all queued render-feedback events (used by the renderer per frame).
    pub fn drain_render_events(&mut self) -> Vec<RenderEvent> {
        std::mem::take(&mut self.render_events)
    }

    /// Allocate a fresh entity id.
    pub fn spawn(&mut self) -> Entity {
        let e = Entity(self.next_id);
        self.next_id += 1;
        e
    }

    /// Remove an entity from every component store.
    pub fn despawn(&mut self, e: Entity) {
        self.positions.remove(&e);
        self.factions.remove(&e);
        self.health.remove(&e);
        self.mana.remove(&e);
        self.move_targets.remove(&e);
        self.combat.remove(&e);
        self.player_tags.remove(&e);
        self.classes.remove(&e);
        self.stats.remove(&e);
        self.progression.remove(&e);
        self.equipment.remove(&e);
        self.inventory.remove(&e);
        self.skills.remove(&e);
        self.skill_trees.remove(&e);
        self.unlocked_branches.remove(&e);
        self.mob_tags.remove(&e);
        self.mob_types.remove(&e);
        self.spawn_ids.remove(&e);
        self.aggro.remove(&e);
        self.ground_items.remove(&e);
        self.animations.remove(&e);
        self.dual_hand.remove(&e);
        self.directions.remove(&e);
        self.buffs.remove(&e);
        self.slows.remove(&e);
        self.invuln.remove(&e);
        self.stuns.remove(&e);
        self.knockdowns.remove(&e);
        self.pushbacks.remove(&e);
        self.interruptibles.remove(&e);
        self.unstoppable.remove(&e);
        self.combo_locks.remove(&e);
        self.dots.remove(&e);
        self.def_debuffs.remove(&e);
        self.resist.remove(&e);
        self.order_buffs.remove(&e);
        self.heal_fields.remove(&e);
        self.traps.remove(&e);
        self.elem_resists.remove(&e);
        self.flame_walls.remove(&e);
        self.blizzards.remove(&e);
        self.meteor_fields.remove(&e);
        self.frost_armor.remove(&e);
        self.lightning_shield.remove(&e);
        self.charging_shots.remove(&e);
        self.hunter_instinct.remove(&e);
        self.arrow_rains.remove(&e);
        self.active_auras.remove(&e);
        self.aura_bonuses.remove(&e);
        self.aura_reserved_mana.remove(&e);
        self.converted.remove(&e);
        self.pardoned.remove(&e);
        self.resurrecting.remove(&e);
        self.npcs.remove(&e);
        self.npc_info.remove(&e);
        self.merchants.remove(&e);
        self.boss_state.remove(&e);
        self.summon_tags.remove(&e);
        self.mercenary_tags.remove(&e);
        self.merc_inventory.remove(&e);
        self.merc_states.remove(&e);
        self.discovered_wps.remove(&e);
        self.wp_teleport_casts.remove(&e);
        self.chests.remove(&e);
        self.elite_mobs.remove(&e);
    }

    /// Snapshot of all live chest entities.
    pub fn chest_entities(&self) -> Vec<Entity> {
        self.chests.keys().copied().collect()
    }

    /// Snapshot of all living mob entities.
    pub fn mob_entities(&self) -> Vec<Entity> {
        self.mob_tags.keys().copied().collect()
    }

    /// Snapshot of all summon entities.
    pub fn summon_entities(&self) -> Vec<Entity> {
        self.summon_tags.keys().copied().collect()
    }

    /// Snapshot of all mercenary entities.
    pub fn mercenary_entities(&self) -> Vec<Entity> {
        self.mercenary_tags.keys().copied().collect()
    }

    /// Snapshot of all ground-item entities.
    pub fn ground_item_entities(&self) -> Vec<Entity> {
        self.ground_items.keys().copied().collect()
    }

    /// Sum of every active battle order's bonuses on `e` (zero if none). Read by
    /// combat (damage/defence/attack-speed) and summon movement to fold the
    /// Soldat "Ordres" into the live stats.
    #[must_use]
    pub fn order_bonuses(&self, e: Entity) -> OrderBonuses {
        let mut acc = OrderBonuses::default();
        if let Some(list) = self.order_buffs.get(&e) {
            for b in list {
                acc.move_speed_pct += b.bonuses.move_speed_pct;
                acc.def_pct += b.bonuses.def_pct;
                acc.hp_pct += b.bonuses.hp_pct;
                acc.atk_speed_pct += b.bonuses.atk_speed_pct;
                acc.dmg_pct += b.bonuses.dmg_pct;
            }
        }
        acc
    }

    pub fn log_event(&mut self, msg: impl Into<String>) {
        self.log.push(msg.into());
    }

    /// Drain and return the accumulated event log lines.
    pub fn drain_log(&mut self) -> Vec<String> {
        std::mem::take(&mut self.log)
    }
}
